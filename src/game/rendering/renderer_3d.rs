//! Clean 3D renderer with ECS-only rendering

use macroquad::prelude::*;
use crate::game::{Player};
use crate::game::map::WallType;
use crate::ecs::{World, Transform, StaticRenderer, Wall, Floor, Ceiling, LightReceiver, LightSource, Renderable, RenderData, RenderType, Renderer, RenderMode, StaticMesh};
use std::collections::HashMap;

/// Modern 3D renderer with ECS-based rendering only
pub struct Modern3DRenderer {
    camera: Camera3D,
    // Texture storage for ECS entities
    wall_textures: HashMap<WallType, Texture2D>,
    floor_texture: Option<Texture2D>,
    ceiling_texture: Option<Texture2D>,
}

impl Modern3DRenderer {
    /// Create a new renderer
    pub fn new() -> Self {
        let camera = Camera3D {
            position: vec3(1.5, 1.0, 1.5),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(2.0, 1.0, 1.5),
            fovy: 1.31,  // 1.31 radians (75.0 degrees) - macroquad expects radians
            ..Default::default()
        };

        Self {
            camera,
            wall_textures: HashMap::new(),
            floor_texture: None,
            ceiling_texture: None,
        }
    }

    /// Load textures from disk
    pub async fn load_textures(&mut self) {
        println!("Loading textures from disk...");
        
        // Try to load wall textures
        if let Ok(mut texture) = load_texture("assets/textures/tech_panel.png").await {
            texture.set_filter(FilterMode::Nearest);
            self.wall_textures.insert(WallType::TechPanel, texture);
            println!("Loaded tech_panel.png");
        }
        if let Ok(mut texture) = load_texture("assets/textures/hull_plating.png").await {
            texture.set_filter(FilterMode::Nearest);
            self.wall_textures.insert(WallType::HullPlating, texture);
            println!("Loaded hull_plating.png");
        }
        if let Ok(mut texture) = load_texture("assets/textures/control_system.png").await {
            texture.set_filter(FilterMode::Nearest);
            self.wall_textures.insert(WallType::ControlSystem, texture);
            println!("Loaded control_system.png");
        }
        if let Ok(mut texture) = load_texture("assets/textures/energy_conduit.png").await {
            texture.set_filter(FilterMode::Nearest);
            self.wall_textures.insert(WallType::EnergyConduit, texture);
            println!("Loaded energy_conduit.png");
        }
        
        // Try to load floor and ceiling textures
        if let Ok(mut texture) = load_texture("assets/textures/floor.png").await {
            texture.set_filter(FilterMode::Nearest);
            self.floor_texture = Some(texture);
            println!("Loaded floor.png with nearest filtering (sharp)");
        }
        if let Ok(mut texture) = load_texture("assets/textures/ceiling.png").await {
            texture.set_filter(FilterMode::Nearest);
            self.ceiling_texture = Some(texture);
            println!("Loaded ceiling.png");
        }
        
        println!("Texture loading complete.");
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

    /// Render ECS entities - unified approach for all renderable components
    pub fn render_ecs_entities(&mut self, world: &World) {
        // Set the 3D camera for ECS rendering
        set_camera(&self.camera);
        
        // Use atmospheric background color (could be enhanced with ECS lighting later)
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0)); // Dark blue atmosphere
        
        // Unified rendering - all entities with Transform + renderable components
        self.render_mesh_components(world);
        self.render_static_components(world);
        
        // Reset to default camera after ECS rendering
        set_default_camera();
    }

    /// Render all entities with mesh-based components - now unified too!
    fn render_mesh_components(&mut self, world: &World) {
        let mut mesh_count = 0;
        
        // Render StaticMesh components (walls, floors, ceilings)
        for (entity, transform, static_mesh) in world.query_2::<Transform, StaticMesh>() {
            if self.should_render_entity(entity, world) && static_mesh.enabled && transform.is_enabled() {
                if let Some(mesh) = &static_mesh.mesh {
                    draw_mesh(mesh);
                    mesh_count += 1;
                }
            }
        }
        
        // Log occasionally
        static mut MESH_FRAME_COUNT: u32 = 0;
        unsafe {
            MESH_FRAME_COUNT += 1;
            if MESH_FRAME_COUNT % 300 == 0 {
                println!("🏗️ ECS Unified Mesh Rendering: {} StaticMesh entities", mesh_count);
            }
        }
    }

    /// Render all entities with renderable components - truly unified!
    fn render_static_components(&mut self, world: &World) {
        let mut render_count = 0;
        let mut renderer_count = 0;
        
        // Render new Renderer components (replaces StaticRenderer)
        for (entity, transform, renderer) in world.query_2::<Transform, Renderer>() {
            renderer_count += 1;
            if self.should_render_entity(entity, world) && renderer.should_render() && transform.is_enabled() {
                self.render_with_renderer_component(entity, transform, renderer, world);
                render_count += 1;
            }
        }
        
        // Still support legacy StaticRenderer for backward compatibility during transition
        for (entity, transform, static_renderer) in world.query_2::<Transform, StaticRenderer>() {
            if self.should_render_entity(entity, world) && static_renderer.should_render() && transform.is_enabled() {
                // Check if this entity is a light source to render as sphere
                let is_light_source = world.has::<LightSource>(entity);
                
                if is_light_source {
                    // Render light sources as glowing spheres
                    self.render_light_sphere(transform, static_renderer, world.get::<LightSource>(entity));
                } else {
                    // Render normal entities
                    let render_data = static_renderer.get_render_data();
                    self.render_with_data(transform, &render_data);
                }
                render_count += 1;
            }
        }
        
        // Log occasionally
        static mut RENDER_FRAME_COUNT: u32 = 0;
        unsafe {
            RENDER_FRAME_COUNT += 1;
            if RENDER_FRAME_COUNT % 300 == 0 {
                println!("🎨 ECS Unified Rendering: {} renderable entities ({} Renderer components found)", render_count, renderer_count);
            }
        }
    }

    /// Render using generic render data - works for any Renderable component!
    fn render_with_data(&self, transform: &Transform, render_data: &RenderData) {
        match &render_data.render_type {
            RenderType::Mesh => {
                // Mesh rendering is handled directly in mesh components
            },
            RenderType::Cube { size } => {
                let texture = render_data.texture_name.as_ref()
                    .and_then(|name| self.get_wall_texture_by_name(name));
                draw_cube(transform.position, *size, texture, render_data.color);
            },
            RenderType::Custom => {
                // Handle custom rendering if needed
            },
        }
    }

    /// Render using the new Renderer component
    fn render_with_renderer_component(&self, entity: crate::ecs::Entity, transform: &Transform, renderer: &Renderer, world: &World) {
        // Check if this entity is a light source for special handling
        let is_light_source = world.has::<LightSource>(entity);
        
        match &renderer.render_mode {
            RenderMode::UseMeshData => {
                // Check for StaticMesh component on same entity
                if let Some(static_mesh) = world.get::<StaticMesh>(entity) {
                    if let Some(mesh) = &static_mesh.mesh {
                        draw_mesh(mesh);
                    }
                }
            },
            RenderMode::Cube { size } => {
                let texture = renderer.material.texture_name.as_ref()
                    .and_then(|name| self.get_wall_texture_by_name(name))
                    .or(renderer.material.texture.as_ref());
                draw_cube(transform.position, *size, texture, renderer.material.color);
            },
            RenderMode::Sphere { radius } => {
                // Special handling for light sources
                if is_light_source {
                    if let Some(light_source) = world.get::<LightSource>(entity) {
                        self.render_light_sphere_new(transform, renderer, Some(light_source));
                    } else {
                        self.render_light_sphere_new(transform, renderer, None);
                    }
                } else {
                    // Normal sphere rendering
                    draw_sphere(transform.position, *radius, renderer.material.texture.as_ref(), renderer.material.color);
                }
            },
            RenderMode::Cylinder { radius, height } => {
                // For now, render as cube until we have cylinder primitive
                let size = Vec3::new(*radius * 2.0, *height, *radius * 2.0);
                let texture = renderer.material.texture.as_ref();
                draw_cube(transform.position, size, texture, renderer.material.color);
            },
            RenderMode::Plane { width, height } => {
                // For now, render as thin cube
                let size = Vec3::new(*width, 0.01, *height);
                let texture = renderer.material.texture.as_ref();
                draw_cube(transform.position, size, texture, renderer.material.color);
            },
            RenderMode::Custom => {
                // Handle custom rendering if needed
            },
        }
    }

    /// Render a light source as a glowing sphere using new Renderer component
    fn render_light_sphere_new(&self, transform: &Transform, renderer: &Renderer, light_source: Option<&LightSource>) {
        if !renderer.material.visible {
            return;
        }
        
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

    /// Check if an entity should be rendered (unified enable/disable logic)
    fn should_render_entity(&self, entity: crate::ecs::Entity, world: &World) -> bool {
        world.is_valid(entity) && entity.enabled
    }
    
    /// Render a single static entity
    fn render_static_entity(&self, transform: &Transform, static_renderer: &StaticRenderer) {
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
                // For props, we could add a prop texture later
                (size, None)
            },
        };
        
        // Use color from StaticRenderer component (modified by lighting system)
        let color = static_renderer.color;
        
        draw_cube(transform.position, size, texture, color);
    }
    
    /// Get wall texture by texture name
    fn get_wall_texture_by_name(&self, texture_name: &str) -> Option<&Texture2D> {
        // Map texture names to wall types
        let wall_type = match texture_name {
            "tech_panel.png" => WallType::TechPanel,
            "hull_plating.png" => WallType::HullPlating,
            "control_system.png" => WallType::ControlSystem,
            "energy_conduit.png" => WallType::EnergyConduit,
            _ => return None,
        };
        
        self.wall_textures.get(&wall_type)
    }

    /// Render a light source as a glowing sphere
    fn render_light_sphere(&self, transform: &Transform, static_renderer: &StaticRenderer, light_source: Option<&LightSource>) {
        if !static_renderer.visible {
            return;
        }
        
        let sphere_radius = 0.15; // Small sphere for light visualization
        let sphere_color = if let Some(light) = light_source {
            // Use the light's color, but make it more visible
            Color::new(
                (light.color.r + 0.5).min(1.0),
                (light.color.g + 0.5).min(1.0), 
                (light.color.b + 0.5).min(1.0),
                1.0
            )
        } else {
            static_renderer.color
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