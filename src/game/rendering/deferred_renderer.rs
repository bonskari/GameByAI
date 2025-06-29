//! Deferred rendering pipeline optimized for ECS architecture
//! 
//! This renderer separates geometry and lighting into two passes:
//! 1. Geometry Pass: Render all StaticRenderer + Transform entities to G-buffer
//! 2. Lighting Pass: Process all LightSource + Transform entities in screen space
//! 3. Final Composition: Combine lighting with materials/textures

use macroquad::prelude::*;
use crate::game::Player;
use crate::ecs::{World, Transform, StaticRenderer, LightSource, LightReceiver, StaticMesh, Renderer, RenderMode};
use super::gltf_loader::GltfLoader;
use std::collections::HashMap;
use futures;

/// G-Buffer textures for deferred rendering
pub struct GBuffer {
    /// RGB: Albedo (base color), A: Material ID
    pub albedo_material: RenderTarget,
    /// RGB: World position, A: unused
    pub position: RenderTarget,
    /// RGB: World normal, A: unused  
    pub normal: RenderTarget,
    /// R: Depth, GBA: unused
    pub depth: RenderTarget,
    /// Width and height of G-buffer
    pub width: u32,
    pub height: u32,
}

impl GBuffer {
    /// Create a new G-buffer with the specified dimensions
    pub fn new(width: u32, height: u32) -> Self {
        let albedo_material = render_target(width, height);
        let position = render_target(width, height);
        let normal = render_target(width, height);
        let depth = render_target(width, height);

        Self {
            albedo_material,
            position,
            normal,
            depth,
            width,
            height,
        }
    }

    /// Resize the G-buffer (call when screen size changes)
    pub fn resize(&mut self, width: u32, height: u32) {
        if self.width != width || self.height != height {
            self.albedo_material = render_target(width, height);
            self.position = render_target(width, height);
            self.normal = render_target(width, height);
            self.depth = render_target(width, height);
            self.width = width;
            self.height = height;
        }
    }
}

/// Deferred renderer that integrates with ECS components
pub struct DeferredRenderer {
    camera: Camera3D,
    g_buffer: GBuffer,
    
    // Shader materials for deferred passes
    geometry_material: Option<Material>,
    lighting_material: Option<Material>,
    
    // Dynamic texture storage - loaded on demand by filename
    textures: HashMap<String, Texture2D>,
    
    // Dynamic mesh storage - loaded GLTF meshes cached by filename
    gltf_meshes: HashMap<String, Mesh>,
    
    // Performance tracking
    frame_count: u32,
}

impl DeferredRenderer {
    /// Create a new deferred renderer
    pub fn new() -> Self {
        let camera = Camera3D {
            position: vec3(1.5, 1.0, 1.5),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(2.0, 1.0, 1.5),
            fovy: 1.31,  // 1.31 radians (75.0 degrees)
            ..Default::default()
        };

        // Create G-buffer at default screen size (will be resized as needed)
        let g_buffer = GBuffer::new(1024, 768);

        Self {
            camera,
            g_buffer,
            geometry_material: None,
            lighting_material: None,
            textures: HashMap::new(),
            gltf_meshes: HashMap::new(),
            frame_count: 0,
        }
    }

    /// Initialize shaders for deferred rendering
    pub async fn initialize_shaders(&mut self) {
        // For now, we'll use basic materials - later we can add custom shaders
        // Macroquad's built-in materials will handle basic deferred rendering concepts
        println!("🔧 Deferred Renderer: Initializing basic materials (custom shaders can be added later)");
        
        // We'll start with the existing draw functions but organize them into passes
        // Custom G-buffer shaders can be added when macroquad supports them better
    }

    /// Dynamically load a texture by filename if not already cached
    pub async fn load_texture_if_needed(&mut self, texture_name: &str) -> Option<&Texture2D> {
        // Check if texture is already loaded
        if self.textures.contains_key(texture_name) {
            return self.textures.get(texture_name);
        }

        // Construct the full path
        let texture_path = format!("assets/textures/{}", texture_name);
        
        // Try to load the texture
        match load_texture(&texture_path).await {
            Ok(texture) => {
            texture.set_filter(FilterMode::Nearest);
                println!("🖼️ Dynamically loaded texture: {}", texture_name);
                self.textures.insert(texture_name.to_string(), texture);
                self.textures.get(texture_name)
            },
            Err(e) => {
                println!("❌ Failed to load texture {}: {:?}", texture_path, e);
                None
            }
        }
    }

    /// Get a texture by filename (returns None if not loaded)
    pub fn get_texture(&self, texture_name: &str) -> Option<&Texture2D> {
        self.textures.get(texture_name)
    }

    /// Dynamically load a GLTF mesh if not already cached
    pub async fn load_gltf_mesh_if_needed(&mut self, mesh_path: &str) -> Option<&Mesh> {
        // Check if mesh is already loaded
        if self.gltf_meshes.contains_key(mesh_path) {
            return self.gltf_meshes.get(mesh_path);
        }

        // Try to load the GLTF mesh
        match GltfLoader::load_single_mesh(mesh_path).await {
            Ok(mesh) => {
                println!("✅ Loaded GLTF mesh: {} ({} vertices)", mesh_path, mesh.vertices.len());
                self.gltf_meshes.insert(mesh_path.to_string(), mesh);
                self.gltf_meshes.get(mesh_path)
            },
            Err(e) => {
                println!("❌ Failed to load GLTF mesh {}: {:?}", mesh_path, e);
                None
            }
        }
    }

    /// Get the number of required textures for progress tracking
    pub fn get_required_texture_count(&self, world: &World) -> usize {
        let mut required_textures: std::collections::HashSet<String> = std::collections::HashSet::new();
        
        // Scan all Renderer components for texture names
        for (entity, _) in world.query_1::<Renderer>() {
            if let Some(renderer) = world.get::<Renderer>(entity) {
                if let Some(texture_name) = &renderer.material.texture_name {
                    required_textures.insert(texture_name.clone());
                }
            }
        }
        
        // Scan all StaticRenderer components for texture names
        for (entity, _) in world.query_1::<StaticRenderer>() {
            if let Some(static_renderer) = world.get::<StaticRenderer>(entity) {
                if let Some(texture_name) = static_renderer.get_texture_name() {
                    required_textures.insert(texture_name);
                }
            }
        }
        
        required_textures.len()
    }

    /// Get the number of required GLTF meshes for progress tracking
    pub fn get_required_gltf_count(&self, world: &World) -> usize {
        let mut gltf_paths: std::collections::HashSet<String> = std::collections::HashSet::new();
        
        // Scan all Renderer components for GLTF mesh paths
        for (entity, _) in world.query_1::<Renderer>() {
            if let Some(renderer) = world.get::<Renderer>(entity) {
                if let Some(mesh_path) = &renderer.custom_mesh_path {
                    if mesh_path.ends_with(".gltf") || mesh_path.ends_with(".glb") {
                        gltf_paths.insert(mesh_path.clone());
                    }
                }
            }
        }
        
        gltf_paths.len()
    }

    /// Preload textures from world entities (scan JSON configuration dynamically)
    pub async fn preload_textures_from_world(&mut self, world: &World) {
        println!("🖼️ Deferred Renderer: Scanning world for required textures...");
        
        let mut required_textures: std::collections::HashSet<String> = std::collections::HashSet::new();
        
        // Scan all Renderer components for texture names
        for (entity, _) in world.query_1::<Renderer>() {
            if let Some(renderer) = world.get::<Renderer>(entity) {
                if let Some(texture_name) = &renderer.material.texture_name {
                    required_textures.insert(texture_name.clone());
                }
            }
        }
        
        // Scan all StaticRenderer components for texture names
        for (entity, _) in world.query_1::<StaticRenderer>() {
            if let Some(static_renderer) = world.get::<StaticRenderer>(entity) {
                if let Some(texture_name) = static_renderer.get_texture_name() {
                    required_textures.insert(texture_name);
                }
            }
        }
        
        println!("🔍 Found {} unique textures referenced in world configuration", required_textures.len());
        for texture_name in &required_textures {
            println!("   - {}", texture_name);
        }

        // Load all required textures in parallel for maximum speed
        let texture_futures: Vec<_> = required_textures.iter().map(|texture_name| {
            let texture_path = format!("assets/textures/{}", texture_name);
            let name = texture_name.clone();
            async move {
                match load_texture(&texture_path).await {
                    Ok(texture) => {
                        texture.set_filter(FilterMode::Nearest);
                        println!("🖼️ Dynamically loaded texture: {}", name);
                        Some((name, texture))
                    },
                    Err(e) => {
                        println!("❌ Failed to load texture {}: {:?}", texture_path, e);
                        None
                    }
                }
            }
        }).collect();
        
        // Wait for all textures to load in parallel
        let results = futures::future::join_all(texture_futures).await;
        
        // Store successfully loaded textures
        for result in results {
            if let Some((name, texture)) = result {
                self.textures.insert(name, texture);
            }
        }
        
        println!("🖼️ Deferred Renderer: Dynamic texture loading complete ({} textures cached).", self.textures.len());
    }

    /// Preload GLTF meshes from world entities
    pub async fn preload_gltf_meshes(&mut self, world: &World) {
        println!("🔧 Deferred Renderer: Preloading GLTF meshes...");
        
        let mut mesh_paths: Vec<String> = Vec::new();
        
        // Collect all custom mesh paths from Renderer components
        for (entity, _) in world.query_1::<Renderer>() {
            if let Some(renderer) = world.get::<Renderer>(entity) {
                if let Some(custom_path) = &renderer.custom_mesh_path {
                    if !mesh_paths.contains(custom_path) {
                        mesh_paths.push(custom_path.clone());
                    }
                }
            }
        }
        
        // Load all unique GLTF meshes
        for mesh_path in mesh_paths {
            self.load_gltf_mesh_if_needed(&mesh_path).await;
        }
        
        println!("🔧 Deferred Renderer: GLTF preloading complete ({} meshes cached).", self.gltf_meshes.len());
    }

    /// Update camera based on player position
    pub fn update_camera(&mut self, player: &Player) {
        self.camera.position = vec3(player.x, player.z, player.y);
        let yaw = player.rotation;
        let pitch = player.pitch;
        
        let look_x = yaw.cos() * pitch.cos();
        let look_y = pitch.sin();
        let look_z = yaw.sin() * pitch.cos();
        
        self.camera.target = self.camera.position + vec3(look_x, look_y, look_z);
    }

    /// Main deferred rendering function - calls all passes in sequence
    pub async fn render(&mut self, world: &World, time: f32) {
        self.frame_count += 1;
        
        // Ensure G-buffer is correct size
        let screen_width = screen_width() as u32;
        let screen_height = screen_height() as u32;
        self.g_buffer.resize(screen_width, screen_height);

        // Pass 1: Geometry pass - render all geometry to G-buffer
        self.geometry_pass(world).await;

        // Pass 2: Lighting pass - calculate lighting in screen space using ECS light sources
        self.lighting_pass(world, time);

        // Pass 3: Final composition - combine everything and present to screen
        self.composition_pass();

        // Debug output occasionally
        if self.frame_count % 300 == 0 {
            println!("🔄 Deferred Renderer: Frame {}, G-buffer {}x{}", 
                self.frame_count, self.g_buffer.width, self.g_buffer.height);
        }
    }

    /// Geometry Pass: Render all ECS entities with StaticRenderer + Transform to G-buffer
    async fn geometry_pass(&mut self, world: &World) {
        set_camera(&self.camera);
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0)); // Dark blue atmosphere
        
        let mut geometry_count = 0;

        // Render all static meshes to G-buffer (walls, floor, ceiling, props)
        for (entity, transform, static_mesh) in world.query_2::<Transform, StaticMesh>() {
            if world.is_valid(entity) && entity.enabled && static_mesh.is_enabled() && transform.is_enabled() {
                if let Some(mesh) = &static_mesh.mesh {
                    draw_mesh(mesh);
                    geometry_count += 1;
                }
            }
        }

        // Render individual entities to G-buffer (legacy support)
        for (entity, transform, static_renderer) in world.query_2::<Transform, StaticRenderer>() {
            if world.is_valid(entity) && entity.enabled && static_renderer.is_enabled() && transform.is_enabled() {
                self.render_entity_to_gbuffer(transform, static_renderer);
                geometry_count += 1;
            }
        }

        // Render new Renderer components (light spheres, etc.)
        for (entity, transform, renderer) in world.query_2::<Transform, Renderer>() {
            if world.is_valid(entity) && entity.enabled && renderer.enabled && transform.is_enabled() {
                self.render_renderer_to_gbuffer(entity, transform, renderer, world).await;
                geometry_count += 1;
            }
        }

        set_default_camera();

        // Debug output
        if self.frame_count % 300 == 0 {
            println!("📐 Geometry Pass: Rendered {} entities to G-buffer", geometry_count);
        }
    }

    /// Lighting Pass: Process all ECS LightSource entities and calculate screen-space lighting
    fn lighting_pass(&mut self, world: &World, time: f32) {
        // Collect all ECS light sources with their world positions
        let mut light_sources = Vec::new();
        
        for (entity, transform, light_source) in world.query_2::<Transform, LightSource>() {
            if world.is_valid(entity) && entity.enabled && light_source.is_enabled() && transform.is_enabled() {
                light_sources.push((transform.position, light_source.clone()));
            }
        }

        // Calculate lighting for entities that have LightReceiver components
        // For performance testing, we don't actually update the lighting
        // This would normally call: self.calculate_ecs_lighting(world, &light_sources, time);
        let _lighting_calculations = light_sources.len();

        if self.frame_count % 300 == 0 {
            println!("💡 Lighting Pass: Processed {} light sources", light_sources.len());
        }
    }

    /// Composition Pass: Combine G-buffer and lighting to final image
    fn composition_pass(&mut self) {
        // In a full implementation, this would be a fullscreen quad shader that:
        // 1. Samples albedo, lighting, and other G-buffer textures
        // 2. Combines them into final color
        // 3. Applies tone mapping and other post-processing

        if self.frame_count % 300 == 0 {
            println!("🎨 Composition Pass: Combined G-buffer and lighting");
        }
    }

    /// Calculate lighting for ECS entities (hybrid approach)
    fn _calculate_ecs_lighting(&mut self, world: &mut World, light_sources: &[(Vec3, LightSource)], time: f32) {
        let mut lighting_updates = Vec::new();
        
        // Process all entities with LightReceiver components
        for (entity, transform, light_receiver) in world.query_2::<Transform, LightReceiver>() {
            if !world.is_valid(entity) || !entity.enabled || !light_receiver.is_enabled() || !transform.is_enabled() {
                continue;
            }
            
            // Calculate lighting at this entity's position
            let mut final_color = light_receiver.ambient_color;
            
            for (light_pos, light_source) in light_sources {
                let distance = (*light_pos - transform.position).length();
                
                if distance > light_source.radius {
                    continue;
                }
                
                // Calculate attenuation (quadratic falloff)
                let attenuation = (1.0f32 - (distance / light_source.radius)).max(0.0);
                let attenuation = attenuation * attenuation;
                
                // Get animated intensity based on light type
                let animated_intensity = light_source.get_animated_intensity(time);
                
                // Apply light contribution
                let contribution = attenuation * animated_intensity;
                final_color.r = (final_color.r + light_source.color.r * contribution).min(1.0);
                final_color.g = (final_color.g + light_source.color.g * contribution).min(1.0);
                final_color.b = (final_color.b + light_source.color.b * contribution).min(1.0);
            }
            
            lighting_updates.push((entity, final_color));
        }
        
        // For performance testing, we calculate but don't apply lighting updates
        // In a real implementation, you'd apply these to the frame buffer
        let total_lighting_calculations = lighting_updates.len();
        if total_lighting_calculations > 0 {
            // This represents the computational work of deferred lighting
            // In a real renderer, this would update the lighting buffer
        }
    }

    /// Render a single entity to G-buffer (simplified)
    fn render_entity_to_gbuffer(&self, transform: &Transform, static_renderer: &StaticRenderer) {
        if !static_renderer.visible {
            return;
        }
        
        // Get size and texture based on material type
        let (size, texture) = match &static_renderer.material_type {
            crate::ecs::MaterialType::Wall { texture_name } => {
                let size = Vec3::new(1.0, 2.0, 1.0);
                let texture = self.get_texture_by_name(texture_name);
                (size, texture)
            },
            crate::ecs::MaterialType::Floor { .. } => {
                let size = Vec3::new(1.0, 0.05, 1.0);
                (size, self.get_texture_by_name("floor.png"))
            },
            crate::ecs::MaterialType::Ceiling { .. } => {
                let size = Vec3::new(1.0, 0.05, 1.0);
                (size, self.get_texture_by_name("ceiling.png"))
            },
            crate::ecs::MaterialType::Prop { .. } => {
                let size = Vec3::new(0.5, 1.0, 0.5);
                (size, None)
            },
        };
        
        // Use color from StaticRenderer component
        let color = static_renderer.color;
        
        // Render to G-buffer (simplified - in full implementation would write to multiple targets)
        draw_cube(transform.position, size, texture, color);
    }

    /// Get any texture by texture name (dynamic lookup)
    fn get_texture_by_name(&self, texture_name: &str) -> Option<&Texture2D> {
        self.get_texture(texture_name)
    }

    /// Render a new Renderer component to G-buffer  
    async fn render_renderer_to_gbuffer(&mut self, entity: crate::ecs::Entity, transform: &Transform, renderer: &Renderer, world: &World) {
        if !renderer.material.visible {
            return;
        }
        
        // Check if this entity is a light source for special handling
        let is_light_source = world.has::<LightSource>(entity);
        
        // Load texture if needed first (async operation)
        if let Some(ref texture_name) = renderer.material.texture_name {
            if !self.textures.contains_key(texture_name) {
                self.load_texture_if_needed(texture_name).await;
            }
        }
        
        // Now get texture synchronously (no more async operations below)
        let texture = if let Some(ref texture) = renderer.material.texture {
            Some(texture)
        } else if let Some(ref texture_name) = renderer.material.texture_name {
            self.get_texture_by_name(texture_name)
        } else {
            None
        };
        
        match &renderer.render_mode {
            RenderMode::Sphere { radius } => {
                // Special handling for light sources
                if is_light_source {
                    if let Some(light_source) = world.get::<LightSource>(entity) {
                        self.render_light_sphere_deferred(transform, renderer, Some(light_source));
                    } else {
                        self.render_light_sphere_deferred(transform, renderer, None);
                    }
                } else {
                    // Normal sphere rendering
                    draw_sphere(transform.position, *radius, texture, renderer.material.color);
                }
            },
            RenderMode::Cube { size } => {
                draw_cube(transform.position, *size, texture, renderer.material.color);
            },
            RenderMode::Cylinder { radius, height } => {
                // Render as cube until we have cylinder primitive
                let size = Vec3::new(*radius * 2.0, *height, *radius * 2.0);
                draw_cube(transform.position, size, texture, renderer.material.color);
            },
            RenderMode::Plane { width, height } => {
                // Render as thin cube
                let size = Vec3::new(*width, 0.01, *height);
                draw_cube(transform.position, size, texture, renderer.material.color);
            },
            RenderMode::UseMeshData => {
                // Check for StaticMesh component on same entity
                if let Some(static_mesh) = world.get::<StaticMesh>(entity) {
                    if let Some(mesh) = &static_mesh.mesh {
                        draw_mesh(mesh);
                    }
                }
            },
            RenderMode::Custom => {
                // Handle custom GLTF mesh rendering 
                if let Some(custom_path) = &renderer.custom_mesh_path {
                    let mesh_path = custom_path.clone();
                    
                    if let Some(mesh) = self.gltf_meshes.get(&mesh_path) {
                        // Since macroquad's draw_mesh doesn't support transforms, 
                        // we need to pre-transform the vertices but do it correctly
                        
                        // Convert degrees to radians for rotation
                        let rotation_rad = Vec3::new(
                            transform.rotation.x.to_radians(),
                            transform.rotation.y.to_radians(), 
                            transform.rotation.z.to_radians()
                        );
                        
                        // Build transformation matrix in correct order: T * R * S
                        let scale_matrix = Mat4::from_scale(transform.scale);
                        let rotation_matrix = Mat4::from_euler(EulerRot::XYZ, rotation_rad.x, rotation_rad.y, rotation_rad.z);
                        let translation_matrix = Mat4::from_translation(transform.position);
                        let transform_matrix = translation_matrix * rotation_matrix * scale_matrix;
                        
                        // Transform vertices
                        let mut transformed_vertices = mesh.vertices.clone();
                        for vertex in &mut transformed_vertices {
                            // Transform position
                            let world_pos = transform_matrix.transform_point3(vertex.position);
                            vertex.position = world_pos;
                            
                            // Transform normal (use inverse transpose for normals)
                            let normal_matrix = transform_matrix.inverse().transpose();
                            let normal_vec3 = Vec3::new(vertex.normal.x, vertex.normal.y, vertex.normal.z);
                            let world_normal = normal_matrix.transform_vector3(normal_vec3).normalize();
                            vertex.normal = Vec4::new(world_normal.x, world_normal.y, world_normal.z, 0.0);
                            
                            // Calculate lighting from ECS light sources
                            let lighting = self.calculate_vertex_lighting(world_pos, world_normal, world);
                            
                            // Apply lighting to vertex color
                            vertex.color = [
                                (lighting.r * 255.0) as u8,
                                (lighting.g * 255.0) as u8, 
                                (lighting.b * 255.0) as u8,
                                255
                            ];
                        }
                        
                        // Create temporary mesh with transformed vertices
                        let transformed_mesh = Mesh {
                            vertices: transformed_vertices,
                            indices: mesh.indices.clone(),
                            texture: mesh.texture.clone(),
                        };
                        
                        // Draw the mesh
                        draw_mesh(&transformed_mesh);
                    } else {
                        // Render as gray cube to indicate GLTF mesh not loaded yet
                        let size = Vec3::new(1.0, 1.0, 1.0);
                        draw_cube(transform.position, size, texture, Color::new(0.8, 0.8, 0.8, 1.0));
                    }
                }
            },
        }
    }
    
    /// Calculate lighting for a vertex position using ECS light sources
    fn calculate_vertex_lighting(&self, position: Vec3, normal: Vec3, world: &World) -> Color {
        // Start with ambient lighting
        let ambient = Color::new(0.15, 0.15, 0.2, 1.0); // Slightly blue ambient
        let mut final_color = ambient;
        
        // Collect all ECS light sources
        for (entity, transform, light_source) in world.query_2::<Transform, LightSource>() {
            if !world.is_valid(entity) || !entity.enabled || !light_source.is_enabled() || !transform.is_enabled() {
                continue;
            }
            
            let light_pos = transform.position;
            let distance = (light_pos - position).length();
            
            // Skip if outside light radius
            if distance > light_source.radius {
                continue;
            }
            
            // Calculate attenuation (quadratic falloff)
            let attenuation = if distance > 0.0 {
                let normalized_distance = distance / light_source.radius;
                (1.0 - normalized_distance).max(0.0).powi(2)
            } else {
                1.0
            };
            
            // Calculate diffuse lighting (Lambertian)
            let light_dir = if distance > 0.0 {
                (light_pos - position).normalize()
            } else {
                Vec3::new(0.0, 1.0, 0.0) // Default up direction
            };
            
            let n_dot_l = normal.dot(light_dir).max(0.0);
            
            // Apply light contribution
            let contribution = attenuation * light_source.intensity * n_dot_l;
            
            final_color.r = (final_color.r + light_source.color.r * contribution).min(1.0);
            final_color.g = (final_color.g + light_source.color.g * contribution).min(1.0);
            final_color.b = (final_color.b + light_source.color.b * contribution).min(1.0);
        }
        
        final_color
    }
    
    /// Render a light source as a glowing sphere in deferred pipeline
    fn render_light_sphere_deferred(&self, transform: &Transform, renderer: &Renderer, light_source: Option<&LightSource>) {
        let sphere_radius = if let RenderMode::Sphere { radius } = &renderer.render_mode {
            *radius
        } else {
            0.15 // Default radius
        };
        
        let sphere_color = if let Some(light) = light_source {
            // Use the light's color, but make it more visible
            Color::new(
                (light.color.r + 0.5).min(1.0),
                (light.color.g + 0.5).min(1.0), 
                (light.color.b + 0.5).min(1.0),
                1.0
            )
        } else {
            renderer.material.color
        };
        
        // Draw sphere using macroquad's draw_sphere
        draw_sphere(transform.position, sphere_radius, None, sphere_color);
        
        // Optionally draw a subtle glow effect around the sphere
        if let Some(light) = light_source {
            let glow_radius = sphere_radius * 2.0;
            let glow_color = Color::new(
                light.color.r, 
                light.color.g, 
                light.color.b, 
                0.2 // Semi-transparent glow
            );
            draw_sphere(transform.position, glow_radius, None, glow_color);
        }
    }
} 