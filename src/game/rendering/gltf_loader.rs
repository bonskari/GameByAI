//! GLTF loader for converting GLTF files to macroquad meshes

use macroquad::prelude::*;
use std::path::Path;
use futures;

/// Represents a loaded GLTF mesh ready for rendering
pub struct LoadedGltfMesh {
    pub mesh: Mesh,
    pub material_name: Option<String>,
    pub base_color_texture: Option<Texture2D>,
    pub normal_texture: Option<Texture2D>,
    pub metallic_roughness_texture: Option<Texture2D>,
}

/// GLTF loader that can parse GLTF files and extract mesh data
pub struct GltfLoader;

impl GltfLoader {
    /// Load a GLTF file and convert it to macroquad meshes
    pub async fn load_gltf(file_path: &str) -> Result<Vec<LoadedGltfMesh>, Box<dyn std::error::Error>> {
        println!("🔧 Loading GLTF file: {}", file_path);
        
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(format!("GLTF file not found: {}", file_path).into());
        }

        // Load GLTF file (we'll handle textures separately through our texture system)
        // Use import_slice to only load buffer data, not external textures
        let gltf_data = std::fs::read(path)?;
        let gltf = gltf::Gltf::from_slice(&gltf_data)?;
        
        // Load only the buffer data
        let mut buffers = Vec::new();
        for buffer in gltf.buffers() {
            match buffer.source() {
                gltf::buffer::Source::Uri(uri) => {
                    // Load buffer file relative to GLTF file
                    let buffer_path = path.parent()
                        .unwrap_or_else(|| Path::new("."))
                        .join(uri);
                    let buffer_data = std::fs::read(buffer_path)?;
                    buffers.push(gltf::buffer::Data(buffer_data));
                },
                gltf::buffer::Source::Bin => {
                    // For GLB files, the buffer is embedded
                    if let Some(blob) = gltf.blob.as_deref() {
                        buffers.push(gltf::buffer::Data(blob.to_vec()));
                    } else {
                        return Err("GLB file missing binary data".into());
                    }
                },
            }
        }
        let mut meshes = Vec::new();

        // Process each mesh in the GLTF file
        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                if let Some(loaded_mesh) = Self::process_primitive(&primitive, &buffers, &gltf, path).await? {
                    meshes.push(loaded_mesh);
                }
            }
        }

        println!("✅ Loaded {} meshes from GLTF file: {}", meshes.len(), file_path);
        Ok(meshes)
    }

    /// Process a single GLTF primitive and convert to macroquad mesh
    async fn process_primitive(
        primitive: &gltf::Primitive<'_>,
        buffers: &[gltf::buffer::Data],
        gltf: &gltf::Gltf,
        gltf_path: &Path,
    ) -> Result<Option<LoadedGltfMesh>, Box<dyn std::error::Error>> {
        // Get vertex attributes
        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
        
        // Extract positions (required)
        let positions: Vec<[f32; 3]> = if let Some(positions) = reader.read_positions() {
            positions.collect()
        } else {
            return Err("GLTF primitive missing position data".into());
        };

        // Extract normals (optional, generate if missing)
        let normals: Vec<[f32; 3]> = if let Some(normals) = reader.read_normals() {
            normals.collect()
        } else {
            // Generate default normals pointing up
            vec![[0.0, 1.0, 0.0]; positions.len()]
        };

        // Extract UVs (optional, use default if missing)
        let uvs: Vec<[f32; 2]> = if let Some(uvs) = reader.read_tex_coords(0) {
            uvs.into_f32().collect()
        } else {
            // Generate default UVs
            vec![[0.0, 0.0]; positions.len()]
        };

        // Extract indices (optional, generate if missing)
        let indices: Vec<u16> = if let Some(indices) = reader.read_indices() {
            indices.into_u32().map(|i| i as u16).collect()
        } else {
            // Generate indices for triangle list
            (0..positions.len() as u16).collect()
        };

        // Convert to macroquad vertex format
        // Apply global scale factor to make GLTF models appropriately sized
        const GLTF_SCALE_FACTOR: f32 = 0.01; // Makes models 100x smaller
        
        let mut vertices = Vec::new();
        for i in 0..positions.len() {
            vertices.push(Vertex {
                position: Vec3::new(
                    positions[i][0] * GLTF_SCALE_FACTOR, 
                    positions[i][1] * GLTF_SCALE_FACTOR, 
                    positions[i][2] * GLTF_SCALE_FACTOR
                ),
                uv: Vec2::new(uvs[i][0], uvs[i][1]),
                normal: Vec4::new(normals[i][0], normals[i][1], normals[i][2], 0.0),
                color: [255, 255, 255, 255], // Default white in u8 format
            });
        }

        // Check if mesh is too complex for macroquad's rendering system
        const MAX_VERTICES: usize = 150000;
        if vertices.len() > MAX_VERTICES {
            return Err(format!(
                "❌ MESH TOO COMPLEX: {} vertices (max: {})\n💡 Use a simpler model or convert to low-poly.\n🚫 This will cause render spam - game terminating!",
                vertices.len(), 
                MAX_VERTICES
            ).into());
        }

        println!("✅ GLTF mesh loaded: {} vertices, {} indices", vertices.len(), indices.len());

        // Load material textures if available
        let material = primitive.material();
        let (base_color_texture, normal_texture, metallic_roughness_texture) = 
            Self::load_material_textures(&material, gltf, gltf_path).await;

        // Create macroquad mesh with base color texture if available
        let mesh = Mesh {
            vertices,
            indices,
            texture: base_color_texture.clone(),
        };

        // Get material name if available
        let material_name = material.name().map(|s| s.to_string());

        Ok(Some(LoadedGltfMesh {
            mesh,
            material_name,
            base_color_texture,
            normal_texture,
            metallic_roughness_texture,
        }))
    }

    /// Load a single mesh from GLTF file (returns the first mesh found)
    pub async fn load_single_mesh(file_path: &str) -> Result<Mesh, Box<dyn std::error::Error>> {
        let meshes = Self::load_gltf(file_path).await?;
        
        if meshes.is_empty() {
            return Err(format!("No meshes found in GLTF file: {}", file_path).into());
        }

        // Return the first mesh by moving it out
        Ok(meshes.into_iter().next().unwrap().mesh)
    }

    /// Load a single mesh with materials from GLTF file (returns the first mesh found)
    pub async fn load_single_mesh_with_materials(file_path: &str) -> Result<LoadedGltfMesh, Box<dyn std::error::Error>> {
        let meshes = Self::load_gltf(file_path).await?;
        
        if meshes.is_empty() {
            return Err(format!("No meshes found in GLTF file: {}", file_path).into());
        }

        // Return the first mesh by moving it out
        Ok(meshes.into_iter().next().unwrap())
    }

    /// Load textures from GLTF material
    async fn load_material_textures(
        material: &gltf::Material<'_>,
        gltf: &gltf::Gltf,
        gltf_path: &Path,
    ) -> (Option<Texture2D>, Option<Texture2D>, Option<Texture2D>) {
        let base_dir = gltf_path.parent().unwrap_or_else(|| Path::new("."));

        // Create futures for all texture loading operations
        let base_color_future = async {
            if let Some(info) = material.pbr_metallic_roughness().base_color_texture() {
                Self::load_texture_from_gltf(info.texture(), gltf, base_dir).await
            } else {
                None
            }
        };

        let normal_future = async {
            if let Some(info) = material.normal_texture() {
                Self::load_texture_from_gltf(info.texture(), gltf, base_dir).await
            } else {
                None
            }
        };

        let metallic_roughness_future = async {
            if let Some(info) = material.pbr_metallic_roughness().metallic_roughness_texture() {
                Self::load_texture_from_gltf(info.texture(), gltf, base_dir).await
            } else {
                None
            }
        };

        // Load all textures in parallel
        let (base_color_texture, normal_texture, metallic_roughness_texture) = 
            futures::future::join3(base_color_future, normal_future, metallic_roughness_future).await;

        (base_color_texture, normal_texture, metallic_roughness_texture)
    }

    /// Load a single texture from GLTF
    async fn load_texture_from_gltf(
        texture: gltf::Texture<'_>,
        gltf: &gltf::Gltf,
        base_dir: &Path,
    ) -> Option<Texture2D> {
        let source = texture.source().source();
        match source {
            gltf::image::Source::Uri { uri, .. } => {
                    let texture_path = base_dir.join(uri);
                    if texture_path.exists() {
                        println!("🖼️ Loading GLTF texture: {}", texture_path.display());
                        match load_texture(texture_path.to_str().unwrap()).await {
                            Ok(tex) => {
                                println!("✅ Loaded GLTF texture: {}", uri);
                                Some(tex)
                            },
                            Err(e) => {
                                println!("❌ Failed to load GLTF texture {}: {}", uri, e);
                                None
                            }
                        }
                    } else {
                        println!("⚠️ GLTF texture not found: {}", texture_path.display());
                        None
                    }
                },
            gltf::image::Source::View { .. } => {
                println!("⚠️ Embedded GLTF textures not yet supported");
                None
            }
        }
    }

    /// Check if a file path is a GLTF file
    pub fn is_gltf_file(file_path: &str) -> bool {
        let path = Path::new(file_path);
        if let Some(extension) = path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            ext == "gltf" || ext == "glb"
        } else {
            false
        }
    }


} 