//! Mesh export system for converting generated meshes to external asset files

use std::fs::File;
use std::io::Write;
use crate::game::map::{Map, WallType};

/// Lightweight vertex data for export (without macroquad dependencies)
#[derive(Debug, Clone)]
pub struct ExportVertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}

/// Lightweight mesh data for export (without macroquad dependencies)
#[derive(Debug, Clone)]
pub struct ExportMesh {
    pub vertices: Vec<ExportVertex>,
    pub indices: Vec<u32>,
    pub material_name: String,
}

/// Mesh export formats supported by the system
#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    /// GLTF 2.0 format (modern, efficient, game-ready)
    Gltf,
}

/// Mesh exporter that can convert generated meshes to external files
pub struct MeshExporter {
    output_directory: String,
}

impl MeshExporter {
    /// Create a new mesh exporter
    pub fn new(output_directory: impl Into<String>) -> Self {
        Self {
            output_directory: output_directory.into(),
        }
    }

    /// Export a mesh to the specified format
    pub async fn export_mesh(
        &self,
        mesh: &ExportMesh,
        filename: &str,
        format: ExportFormat,
        wall_type: Option<WallType>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_directory)?;

        match format {
            ExportFormat::Gltf => self.export_gltf(mesh, filename, wall_type).await,
        }
    }

    /// Export mesh as GLTF 2.0 format
    async fn export_gltf(
        &self,
        mesh: &ExportMesh,
        filename: &str,
        wall_type: Option<WallType>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gltf_path = format!("{}/{}.gltf", self.output_directory, filename);
        let bin_path = format!("{}/{}.bin", self.output_directory, filename);
        
        let mut buffer_data = Vec::new();
        
        // Pack vertex data (position + uv + normal)
        for vertex in &mesh.vertices {
            // Position (3 floats)
            buffer_data.extend_from_slice(&vertex.position[0].to_le_bytes());
            buffer_data.extend_from_slice(&vertex.position[1].to_le_bytes());
            buffer_data.extend_from_slice(&vertex.position[2].to_le_bytes());
            
            // Normal (3 floats)
            buffer_data.extend_from_slice(&vertex.normal[0].to_le_bytes());
            buffer_data.extend_from_slice(&vertex.normal[1].to_le_bytes());
            buffer_data.extend_from_slice(&vertex.normal[2].to_le_bytes());
            
            // UV coordinates (2 floats)
            buffer_data.extend_from_slice(&vertex.uv[0].to_le_bytes());
            buffer_data.extend_from_slice(&vertex.uv[1].to_le_bytes());
        }
        
        // Pack index data
        let index_byte_offset = buffer_data.len();
        for &index in &mesh.indices {
            buffer_data.extend_from_slice(&index.to_le_bytes());
        }

        // Calculate bounds for the mesh
        let mut min_pos = [f32::INFINITY; 3];
        let mut max_pos = [f32::NEG_INFINITY; 3];
        
        for vertex in &mesh.vertices {
            for i in 0..3 {
                min_pos[i] = min_pos[i].min(vertex.position[i]);
                max_pos[i] = max_pos[i].max(vertex.position[i]);
            }
        }

        // Create GLTF JSON structure
        let material_name = wall_type.map(|w| format!("{:?}", w)).unwrap_or_else(|| "Default".to_string());
        let texture_filename = match wall_type {
            Some(WallType::TechPanel) => "tech_panel.png",
            Some(WallType::HullPlating) => "hull_plating.png", 
            Some(WallType::ControlSystem) => "control_system.png",
            Some(WallType::EnergyConduit) => "energy_conduit.png",
            _ => "default.png",
        };

        let gltf_json = serde_json::json!({
            "asset": {
                "version": "2.0",
                "generator": "GameByAI Mesh Exporter"
            },
            "scene": 0,
            "scenes": [
                {
                    "nodes": [0]
                }
            ],
            "nodes": [
                {
                    "mesh": 0,
                    "name": filename
                }
            ],
            "meshes": [
                {
                    "primitives": [
                        {
                            "attributes": {
                                "POSITION": 0,
                                "NORMAL": 1,
                                "TEXCOORD_0": 2
                            },
                            "indices": 3,
                            "material": 0
                        }
                    ],
                    "name": format!("{}Mesh", filename)
                }
            ],
            "materials": [
                {
                    "name": material_name,
                    "pbrMetallicRoughness": {
                        "baseColorTexture": {
                            "index": 0
                        },
                        "metallicFactor": 0.1,
                        "roughnessFactor": 0.9
                    }
                }
            ],
            "textures": [
                {
                    "sampler": 0,
                    "source": 0
                }
            ],
            "images": [
                {
                    "uri": texture_filename,
                    "name": texture_filename
                }
            ],
            "samplers": [
                {
                    "magFilter": 9728,
                    "minFilter": 9728,
                    "wrapS": 10497,
                    "wrapT": 10497
                }
            ],
            "accessors": [
                {
                    "bufferView": 0,
                    "componentType": 5126,
                    "count": mesh.vertices.len(),
                    "type": "VEC3",
                    "min": min_pos,
                    "max": max_pos
                },
                {
                    "bufferView": 1,
                    "componentType": 5126,
                    "count": mesh.vertices.len(),
                    "type": "VEC3"
                },
                {
                    "bufferView": 2,
                    "componentType": 5126,
                    "count": mesh.vertices.len(),
                    "type": "VEC2"
                },
                {
                    "bufferView": 3,
                    "componentType": 5125,
                    "count": mesh.indices.len(),
                    "type": "SCALAR"
                }
            ],
            "bufferViews": [
                {
                    "buffer": 0,
                    "byteOffset": 0,
                    "byteLength": mesh.vertices.len() * 12,
                    "byteStride": 32,
                    "target": 34962
                },
                {
                    "buffer": 0,
                    "byteOffset": 12,
                    "byteLength": mesh.vertices.len() * 12,
                    "byteStride": 32,
                    "target": 34962
                },
                {
                    "buffer": 0,
                    "byteOffset": 24,
                    "byteLength": mesh.vertices.len() * 8,
                    "byteStride": 32,
                    "target": 34962
                },
                {
                    "buffer": 0,
                    "byteOffset": index_byte_offset,
                    "byteLength": mesh.indices.len() * 4,
                    "target": 34963
                }
            ],
            "buffers": [
                {
                    "uri": format!("{}.bin", filename),
                    "byteLength": buffer_data.len()
                }
            ]
        });

        // Write GLTF JSON file
        let mut gltf_file = File::create(&gltf_path)?;
        gltf_file.write_all(gltf_json.to_string().as_bytes())?;

        // Write binary buffer file
        let mut bin_file = File::create(&bin_path)?;
        bin_file.write_all(&buffer_data)?;

        println!("✅ Exported GLTF mesh: {}", gltf_path);
        println!("   Binary data: {}", bin_path);
        println!("   Vertices: {}, Faces: {}", mesh.vertices.len(), mesh.indices.len() / 3);
        
        Ok(())
    }

    /// Generate wall mesh for specific type without macroquad dependencies
    pub fn generate_wall_mesh_for_type(&self, map: &Map, target_wall_type: WallType) -> ExportMesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut vertex_count = 0u32;

        for y in 0..map.height {
            for x in 0..map.width {
                if map.is_wall(x as i32, y as i32) {
                    let wall_type = map.get_wall_type(x as i32, y as i32);
                    
                    // Only include walls of the target type
                    if wall_type != target_wall_type {
                        continue;
                    }
                    
                    let pos = [x as f32 + 0.5, 1.0, y as f32 + 0.5];
                    
                    // Only create faces that are exposed (adjacent to empty space)
                    
                    // North face (positive Z)
                    if !map.is_wall(x as i32, (y + 1) as i32) {
                        self.add_wall_face(
                            &mut vertices,
                            &mut indices,
                            &mut vertex_count,
                            pos,
                            [
                                [-0.5, -1.0, 0.5],  // Bottom left
                                [0.5, -1.0, 0.5],   // Bottom right
                                [0.5, 1.0, 0.5],    // Top right  
                                [-0.5, 1.0, 0.5],   // Top left
                            ],
                            [0.0, 0.0, 1.0], // Normal
                        );
                    }
                    
                    // South face (negative Z)
                    if !map.is_wall(x as i32, (y as i32) - 1) {
                        self.add_wall_face(
                            &mut vertices,
                            &mut indices,
                            &mut vertex_count,
                            pos,
                            [
                                [0.5, -1.0, -0.5],   // Bottom left
                                [-0.5, -1.0, -0.5],  // Bottom right
                                [-0.5, 1.0, -0.5],   // Top right
                                [0.5, 1.0, -0.5],    // Top left
                            ],
                            [0.0, 0.0, -1.0], // Normal
                        );
                    }
                    
                    // East face (positive X)
                    if !map.is_wall((x + 1) as i32, y as i32) {
                        self.add_wall_face(
                            &mut vertices,
                            &mut indices,
                            &mut vertex_count,
                            pos,
                            [
                                [0.5, -1.0, 0.5],    // Bottom left
                                [0.5, -1.0, -0.5],   // Bottom right
                                [0.5, 1.0, -0.5],    // Top right
                                [0.5, 1.0, 0.5],     // Top left
                            ],
                            [1.0, 0.0, 0.0], // Normal
                        );
                    }
                    
                    // West face (negative X)
                    if !map.is_wall((x as i32) - 1, y as i32) {
                        self.add_wall_face(
                            &mut vertices,
                            &mut indices,
                            &mut vertex_count,
                            pos,
                            [
                                [-0.5, -1.0, -0.5],  // Bottom left
                                [-0.5, -1.0, 0.5],   // Bottom right
                                [-0.5, 1.0, 0.5],    // Top right
                                [-0.5, 1.0, -0.5],   // Top left
                            ],
                            [-1.0, 0.0, 0.0], // Normal
                        );
                    }
                }
            }
        }

        let material_name = format!("Material_{:?}", target_wall_type);

        println!("Generated {:?} wall mesh with {} vertices and {} faces", 
                 target_wall_type, vertices.len(), indices.len() / 3);

        ExportMesh {
            vertices,
            indices,
            material_name,
        }
    }

    /// Add a wall face to the mesh
    fn add_wall_face(
        &self,
        vertices: &mut Vec<ExportVertex>,
        indices: &mut Vec<u32>,
        vertex_count: &mut u32,
        center_pos: [f32; 3],
        local_positions: [[f32; 3]; 4],
        normal: [f32; 3],
    ) {
        // UV coordinates for a wall face
        let uvs = [
            [0.0, 1.0], // Bottom left
            [1.0, 1.0], // Bottom right  
            [1.0, 0.0], // Top right
            [0.0, 0.0], // Top left
        ];

        // Add vertices
        for i in 0..4 {
            let world_pos = [
                center_pos[0] + local_positions[i][0],
                center_pos[1] + local_positions[i][1],
                center_pos[2] + local_positions[i][2],
            ];
            
            vertices.push(ExportVertex {
                position: world_pos,
                uv: uvs[i],
                normal,
            });
        }

        // Add indices for two triangles (quad)
        let base = *vertex_count;
        indices.extend_from_slice(&[
            base, base + 1, base + 2,  // First triangle
            base, base + 2, base + 3,  // Second triangle
        ]);

        *vertex_count += 4;
    }

    /// Export all wall meshes for different wall types
    pub async fn export_all_wall_meshes(
        &self,
        map: &Map,
        format: ExportFormat,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for wall_type in [WallType::TechPanel, WallType::HullPlating, 
                        WallType::ControlSystem, WallType::EnergyConduit] {
            let mesh = self.generate_wall_mesh_for_type(map, wall_type);
            
            if !mesh.vertices.is_empty() {
                let filename = format!("wall_{:?}", wall_type).to_lowercase();
                self.export_mesh(&mesh, &filename, format, Some(wall_type)).await?;
            } else {
                println!("⚠️ Skipping empty mesh for wall type: {:?}", wall_type);
            }
        }
        Ok(())
    }
} 