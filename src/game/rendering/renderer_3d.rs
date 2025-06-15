//! Clean 3D renderer with separated concerns

use macroquad::prelude::*;
use crate::game::rendering::MaterialManager;
use crate::game::{Map, Player};
use crate::game::map::WallType;
use crate::ecs::{World, Transform, StaticRenderer, Wall, Floor, Ceiling};
use std::collections::HashMap;

/// Modern 3D renderer with clean separation of concerns
pub struct Modern3DRenderer {
    camera: Camera3D,
    material_manager: MaterialManager,
    // Mesh data storage
    wall_meshes: HashMap<WallType, Mesh>,
    floor_mesh: Option<Mesh>,
    ceiling_mesh: Option<Mesh>,
    needs_rebuild: bool,
    // Texture storage
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
            ..Default::default()
        };

        Self {
            camera,
            material_manager: MaterialManager::new(),
            wall_meshes: HashMap::new(),
            floor_mesh: None,
            ceiling_mesh: None,
            needs_rebuild: true,
            wall_textures: HashMap::new(),
            floor_texture: None,
            ceiling_texture: None,
        }
    }

    /// Load textures from disk
    pub async fn load_textures(&mut self) {
        println!("Loading textures from disk...");
        
        // Try to load wall textures
        if let Ok(texture) = load_texture("assets/textures/tech_panel.png").await {
            self.wall_textures.insert(WallType::TechPanel, texture);
            println!("Loaded tech_panel.png");
        }
        if let Ok(texture) = load_texture("assets/textures/hull_plating.png").await {
            self.wall_textures.insert(WallType::HullPlating, texture);
            println!("Loaded hull_plating.png");
        }
        if let Ok(texture) = load_texture("assets/textures/control_system.png").await {
            self.wall_textures.insert(WallType::ControlSystem, texture);
            println!("Loaded control_system.png");
        }
        if let Ok(texture) = load_texture("assets/textures/energy_conduit.png").await {
            self.wall_textures.insert(WallType::EnergyConduit, texture);
            println!("Loaded energy_conduit.png");
        }
        
        // Try to load floor and ceiling textures
        if let Ok(mut texture) = load_texture("assets/textures/floor.png").await {
            texture.set_filter(FilterMode::Linear);
            self.floor_texture = Some(texture);
            println!("Loaded floor.png with linear filtering");
        }
        if let Ok(texture) = load_texture("assets/textures/ceiling.png").await {
            self.ceiling_texture = Some(texture);
            println!("Loaded ceiling.png");
        }
        
        println!("Texture loading complete.");
    }

    /// Build geometry from map data
    pub fn build_geometry(&mut self, map: &Map) {
        if !self.needs_rebuild {
            return;
        }

        self.wall_meshes.clear();
        let wall_height = 2.0;

        let mut mesh_data_map: HashMap<WallType, (Vec<Vertex>, Vec<u16>)> = HashMap::new();

        // Build wall meshes
        for y in 0..map.height {
            for x in 0..map.width {
                if map.is_wall(x as i32, y as i32) {
                    let wall_type = map.get_wall_type(x as i32, y as i32);
                    let (vertices, indices) = mesh_data_map.entry(wall_type).or_insert_with(|| (Vec::new(), Vec::new()));
                    Self::add_wall_cube(vertices, indices, x, y, wall_height);
                }
            }
        }

        // Convert mesh data to actual meshes
        for (wall_type, (vertices, indices)) in mesh_data_map {
            let texture = self.wall_textures.get(&wall_type).cloned();
            self.wall_meshes.insert(wall_type, Mesh {
                vertices,
                indices,
                texture,
            });
        }

        // Build floor and ceiling meshes
        self.floor_mesh = Some(self.create_floor_mesh(map));
        self.ceiling_mesh = Some(self.create_ceiling_mesh(map));

        self.needs_rebuild = false;
        println!("3D geometry rebuilt successfully with mesh system.");
    }

    /// Add a simple wall cube to the mesh
    fn add_wall_cube(vertices: &mut Vec<Vertex>, indices: &mut Vec<u16>, x: usize, y: usize, height: f32) {
        let fx = x as f32;
        let fy = y as f32;
        let base = vertices.len() as u16;
        
        // Simple cube vertices (8 corners)
        let cube_vertices = [
            // Bottom face (y=0)
            vec3(fx, 0.0, fy),         // 0
            vec3(fx + 1.0, 0.0, fy),   // 1
            vec3(fx + 1.0, 0.0, fy + 1.0), // 2
            vec3(fx, 0.0, fy + 1.0),   // 3
            // Top face (y=height)
            vec3(fx, height, fy),      // 4
            vec3(fx + 1.0, height, fy), // 5
            vec3(fx + 1.0, height, fy + 1.0), // 6
            vec3(fx, height, fy + 1.0), // 7
        ];
        
        // Add vertices with simple UV mapping
        for (i, pos) in cube_vertices.iter().enumerate() {
            vertices.push(Vertex {
                position: *pos,
                uv: vec2((i % 2) as f32, (i / 4) as f32), // Simple UV
                color: [255, 255, 255, 255],
                normal: vec4(0.0, 1.0, 0.0, 0.0), // Simple normal
            });
        }
        
        // Cube face indices (12 triangles, 6 faces)
        let cube_indices = [
            // Bottom face
            0, 2, 1, 0, 3, 2,
            // Top face  
            4, 5, 6, 4, 6, 7,
            // Front face
            0, 1, 5, 0, 5, 4,
            // Back face
            2, 3, 7, 2, 7, 6,
            // Left face
            3, 0, 4, 3, 4, 7,
            // Right face
            1, 2, 6, 1, 6, 5,
        ];
        
        // Add indices with base offset
        for &idx in &cube_indices {
            indices.push(base + idx);
        }
    }
    


    /// Create floor mesh
    fn create_floor_mesh(&self, map: &Map) -> Mesh {
        let width = map.width as f32;
        let height = map.height as f32;
        
        // Create floor slightly below ground level to avoid Z-fighting
        let floor_y = -0.01;
        
        let vertices = vec![
            Vertex {
                position: vec3(0.0, floor_y, 0.0),
                uv: vec2(0.0, 0.0),
                color: [255, 255, 255, 255],
                normal: vec4(0.0, 1.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(width, floor_y, 0.0),
                uv: vec2(width, 0.0), // Use same approach as walls - direct mapping
                color: [255, 255, 255, 255],
                normal: vec4(0.0, 1.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(width, floor_y, height),
                uv: vec2(width, height), // Use same approach as walls - direct mapping
                color: [255, 255, 255, 255],
                normal: vec4(0.0, 1.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(0.0, floor_y, height),
                uv: vec2(0.0, height), // Use same approach as walls - direct mapping
                color: [255, 255, 255, 255],
                normal: vec4(0.0, 1.0, 0.0, 0.0),
            },
        ];
        // Ensure correct triangle winding for upward-facing floor
        let indices = vec![0, 1, 2, 0, 2, 3];
        
        let mesh = Mesh { 
            vertices, 
            indices, 
            texture: self.floor_texture.clone() 
        };
        
        if self.floor_texture.is_some() {
            println!("Floor mesh created with texture applied");
        } else {
            println!("Floor mesh created WITHOUT texture - texture not loaded");
        }
        
        mesh
    }

    /// Create ceiling mesh
    fn create_ceiling_mesh(&self, map: &Map) -> Mesh {
        let ceiling_height = 2.0;
        let width = map.width as f32;
        let height = map.height as f32;
        
        let vertices = vec![
            Vertex {
                position: vec3(0.0, ceiling_height, 0.0),
                uv: vec2(0.0, 0.0),
                color: [255, 255, 255, 255],
                normal: vec4(0.0, -1.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(width, ceiling_height, 0.0),
                uv: vec2(width, 0.0),
                color: [255, 255, 255, 255],
                normal: vec4(0.0, -1.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(width, ceiling_height, height),
                uv: vec2(width, height),
                color: [255, 255, 255, 255],
                normal: vec4(0.0, -1.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(0.0, ceiling_height, height),
                uv: vec2(0.0, height),
                color: [255, 255, 255, 255],
                normal: vec4(0.0, -1.0, 0.0, 0.0),
            },
        ];
        let indices = vec![0, 2, 1, 0, 3, 2]; // Reverse winding for ceiling
        
        Mesh { 
            vertices, 
            indices, 
            texture: self.ceiling_texture.clone() 
        }
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

    /// Render the 3D scene using mesh system
    pub fn render(&mut self, map: &Map, player: &Player, render_legacy: bool) {
        // Build geometry if needed (only for legacy rendering)
        if render_legacy {
            self.build_geometry(map);
        }
        
        self.update_camera(player);
        set_camera(&self.camera);
        clear_background(DARKGRAY);

        if render_legacy {
            // Render floor mesh
            if let Some(floor_mesh) = &self.floor_mesh {
                draw_mesh(floor_mesh);
            }

            // Render wall meshes
            for (_wall_type, mesh) in &self.wall_meshes {
                draw_mesh(mesh);
            }

            // Render ceiling mesh
            if let Some(ceiling_mesh) = &self.ceiling_mesh {
                draw_mesh(ceiling_mesh);
            }
        }
        
        set_default_camera();
    }

    /// Get mutable reference to material manager
    pub fn material_manager_mut(&mut self) -> &mut MaterialManager {
        &mut self.material_manager
    }

    /// Get reference to material manager
    pub fn material_manager(&self) -> &MaterialManager {
        &self.material_manager
    }

    /// Mark geometry as needing rebuild
    pub fn mark_dirty(&mut self) {
        self.needs_rebuild = true;
    }

    /// Render ECS entities with StaticRenderer components
    pub fn render_ecs_entities(&mut self, world: &World) {
        // Set the 3D camera for ECS rendering
        set_camera(&self.camera);
        
        let mut wall_count = 0;
        let mut floor_count = 0;
        let mut ceiling_count = 0;
        
        // Render walls
        for (_entity, transform, static_renderer, _wall) in world.query_3::<Transform, StaticRenderer, Wall>() {
            self.render_static_entity(transform, static_renderer);
            wall_count += 1;
        }
        
        // Render floors
        for (_entity, transform, static_renderer, _floor) in world.query_3::<Transform, StaticRenderer, Floor>() {
            self.render_static_entity(transform, static_renderer);
            floor_count += 1;
        }
        
        // Render ceilings
        for (_entity, transform, static_renderer, _ceiling) in world.query_3::<Transform, StaticRenderer, Ceiling>() {
            self.render_static_entity(transform, static_renderer);
            ceiling_count += 1;
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
        
        // Use proper scaling to match legacy system (1x1 grid units) with muted colors
        let (size, color) = match &static_renderer.material_type {
            crate::ecs::MaterialType::Wall { .. } => (Vec3::new(1.0, 2.0, 1.0), Color::new(0.6, 0.2, 0.6, 1.0)), // Muted purple walls
            crate::ecs::MaterialType::Floor { .. } => (Vec3::new(1.0, 0.05, 1.0), Color::new(0.2, 0.6, 0.2, 1.0)), // Muted green floors
            crate::ecs::MaterialType::Ceiling { .. } => (Vec3::new(1.0, 0.05, 1.0), Color::new(0.6, 0.4, 0.2, 1.0)), // Muted orange ceilings
            crate::ecs::MaterialType::Prop { .. } => (Vec3::new(0.5, 1.0, 0.5), Color::new(0.6, 0.6, 0.2, 1.0)),
        };
        
        // Debug: Print first few entity positions to see if this method is called
        static mut DEBUG_COUNT: u32 = 0;
        unsafe {
            DEBUG_COUNT += 1;
            if DEBUG_COUNT <= 3 {
                println!("Drawing cube at: ({:.1}, {:.1}, {:.1}) size: ({:.1}, {:.1}, {:.1}) color: ({:.1}, {:.1}, {:.1})", 
                         transform.position.x, transform.position.y, transform.position.z,
                         size.x, size.y, size.z, color.r, color.g, color.b);
            }
        }
        
        draw_cube(transform.position, size, static_renderer.texture.as_ref(), color);
    }
} 