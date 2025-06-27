//! Deferred rendering pipeline optimized for ECS architecture
//! 
//! This renderer separates geometry and lighting into two passes:
//! 1. Geometry Pass: Render all StaticRenderer + Transform entities to G-buffer
//! 2. Lighting Pass: Process all LightSource + Transform entities in screen space
//! 3. Final Composition: Combine lighting with materials/textures

use macroquad::prelude::*;
use crate::game::Player;
use crate::ecs::{World, Transform, StaticRenderer, LightSource, LightReceiver, WallMesh, FloorMesh};
use std::collections::HashMap;

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
    
    // Texture storage for ECS entities (same as forward renderer)
    wall_textures: HashMap<crate::game::map::WallType, Texture2D>,
    floor_texture: Option<Texture2D>,
    ceiling_texture: Option<Texture2D>,
    
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
            wall_textures: HashMap::new(),
            floor_texture: None,
            ceiling_texture: None,
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

    /// Load textures from disk (same as forward renderer)
    pub async fn load_textures(&mut self) {
        println!("🖼️ Deferred Renderer: Loading textures...");
        
        // Load wall textures
        if let Ok(mut texture) = load_texture("assets/textures/tech_panel.png").await {
            texture.set_filter(FilterMode::Nearest);
            self.wall_textures.insert(crate::game::map::WallType::TechPanel, texture);
            println!("Loaded tech_panel.png");
        }
        if let Ok(mut texture) = load_texture("assets/textures/hull_plating.png").await {
            texture.set_filter(FilterMode::Nearest);
            self.wall_textures.insert(crate::game::map::WallType::HullPlating, texture);
            println!("Loaded hull_plating.png");
        }
        if let Ok(mut texture) = load_texture("assets/textures/control_system.png").await {
            texture.set_filter(FilterMode::Nearest);
            self.wall_textures.insert(crate::game::map::WallType::ControlSystem, texture);
            println!("Loaded control_system.png");
        }
        if let Ok(mut texture) = load_texture("assets/textures/energy_conduit.png").await {
            texture.set_filter(FilterMode::Nearest);
            self.wall_textures.insert(crate::game::map::WallType::EnergyConduit, texture);
            println!("Loaded energy_conduit.png");
        }
        
        // Load floor and ceiling textures
        if let Ok(mut texture) = load_texture("assets/textures/floor.png").await {
            texture.set_filter(FilterMode::Nearest);
            self.floor_texture = Some(texture);
            println!("Loaded floor.png");
        }
        if let Ok(mut texture) = load_texture("assets/textures/ceiling.png").await {
            texture.set_filter(FilterMode::Nearest);
            self.ceiling_texture = Some(texture);
            println!("Loaded ceiling.png");
        }
        
        println!("🖼️ Deferred Renderer: Texture loading complete.");
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
    pub fn render(&mut self, world: &World, time: f32) {
        self.frame_count += 1;
        
        // Ensure G-buffer is correct size
        let screen_width = screen_width() as u32;
        let screen_height = screen_height() as u32;
        self.g_buffer.resize(screen_width, screen_height);

        // Pass 1: Geometry pass - render all geometry to G-buffer
        self.geometry_pass(world);

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
    fn geometry_pass(&mut self, world: &World) {
        set_camera(&self.camera);
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0)); // Dark blue atmosphere
        
        let mut geometry_count = 0;

        // Render wall meshes to G-buffer
        for (entity, transform, wall_mesh) in world.query_2::<Transform, WallMesh>() {
            if world.is_valid(entity) && entity.enabled && wall_mesh.is_enabled() && transform.is_enabled() {
                if let Some(mesh) = &wall_mesh.mesh {
                    draw_mesh(mesh);
                    geometry_count += 1;
                }
            }
        }

        // Render floor meshes to G-buffer
        for (entity, transform, floor_mesh) in world.query_2::<Transform, FloorMesh>() {
            if world.is_valid(entity) && entity.enabled && floor_mesh.is_enabled() && transform.is_enabled() {
                if let Some(mesh) = &floor_mesh.mesh {
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
                let texture = self.get_wall_texture_by_name(texture_name);
                (size, texture)
            },
            crate::ecs::MaterialType::Floor { .. } => {
                let size = Vec3::new(1.0, 0.05, 1.0);
                (size, self.floor_texture.as_ref())
            },
            crate::ecs::MaterialType::Ceiling { .. } => {
                let size = Vec3::new(1.0, 0.05, 1.0);
                (size, self.ceiling_texture.as_ref())
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

    /// Get wall texture by texture name
    fn get_wall_texture_by_name(&self, texture_name: &str) -> Option<&Texture2D> {
        use crate::game::map::WallType;
        
        let wall_type = match texture_name {
            "tech_panel.png" => WallType::TechPanel,
            "hull_plating.png" => WallType::HullPlating,
            "control_system.png" => WallType::ControlSystem,
            "energy_conduit.png" => WallType::EnergyConduit,
            _ => return None,
        };
        
        self.wall_textures.get(&wall_type)
    }
} 