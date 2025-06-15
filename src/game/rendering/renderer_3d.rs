//! Clean 3D renderer with ECS-only rendering

use macroquad::prelude::*;
use crate::game::{Player};
use crate::game::map::WallType;
use crate::ecs::{World, Transform, StaticRenderer, Wall, Floor, Ceiling};
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

    /// Render ECS entities with StaticRenderer components
    pub fn render_ecs_entities(&mut self, world: &World) {
        // Set the 3D camera for ECS rendering
        set_camera(&self.camera);
        clear_background(DARKGRAY);
        
        let mut wall_count = 0;
        let mut floor_count = 0;
        let mut ceiling_count = 0;
        
        // Render walls (only if entity is enabled and components are enabled)
        for (entity, transform, static_renderer, _wall) in world.query_3::<Transform, StaticRenderer, Wall>() {
            if world.is_valid(entity) && entity.enabled && static_renderer.is_enabled() && transform.is_enabled() {
                self.render_static_entity(transform, static_renderer);
                wall_count += 1;
            }
        }
        
        // Render floors (only if entity is enabled and components are enabled)
        for (entity, transform, static_renderer, _floor) in world.query_3::<Transform, StaticRenderer, Floor>() {
            if world.is_valid(entity) && entity.enabled && static_renderer.is_enabled() && transform.is_enabled() {
                self.render_static_entity(transform, static_renderer);
                floor_count += 1;
            }
        }
        
        // Render ceilings (only if entity is enabled and components are enabled)
        for (entity, transform, static_renderer, _ceiling) in world.query_3::<Transform, StaticRenderer, Ceiling>() {
            if world.is_valid(entity) && entity.enabled && static_renderer.is_enabled() && transform.is_enabled() {
                self.render_static_entity(transform, static_renderer);
                ceiling_count += 1;
            }
        }
        
        // Reset to default camera after ECS rendering
        set_default_camera();
        
        // Print occasionally to avoid spam
        static mut FRAME_COUNT: u32 = 0;
        unsafe {
            FRAME_COUNT += 1;
            if FRAME_COUNT % 300 == 0 { // Print every 5 seconds at 60 FPS
                println!("ECS Rendering: {} walls, {} floors, {} ceilings", wall_count, floor_count, ceiling_count);
            }
        }
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
        
        // Use white color to let texture show through properly
        let color = WHITE;
        
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
} 