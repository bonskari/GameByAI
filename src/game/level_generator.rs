//! Level generator that creates optimized wall meshes with proper UV mapping

use macroquad::prelude::*;
use crate::game::map::{Map, WallType};

/// Level mesh generator that creates optimized wall geometry
pub struct LevelMeshBuilder {
    map: Map,
}

impl LevelMeshBuilder {
    pub fn new(map: Map) -> Self {
        Self { map }
    }

    /// Generate a single mesh containing all wall geometry with proper UV mapping
    pub async fn generate_wall_mesh_with_texture(&self) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut vertex_count = 0u16;

        for y in 0..self.map.height {
            for x in 0..self.map.width {
                if self.map.is_wall(x as i32, y as i32) {
                    let wall_type = self.map.get_wall_type(x as i32, y as i32);
                    let pos = Vec3::new(x as f32 + 0.5, 1.0, y as f32 + 0.5);
                    
                    // Only create faces that are exposed (adjacent to empty space)
                    
                    // North face (positive Z)
                    if !self.map.is_wall(x as i32, (y + 1) as i32) {
                        self.add_wall_face(
                            &mut vertices,
                            &mut indices,
                            &mut vertex_count,
                            pos,
                            [
                                vec3(-0.5, -1.0, 0.5),  // Bottom left
                                vec3(0.5, -1.0, 0.5),   // Bottom right
                                vec3(0.5, 1.0, 0.5),    // Top right  
                                vec3(-0.5, 1.0, 0.5),   // Top left
                            ],
                            wall_type,
                        );
                    }
                    
                    // South face (negative Z)
                    if !self.map.is_wall(x as i32, (y as i32) - 1) {
                        self.add_wall_face(
                            &mut vertices,
                            &mut indices,
                            &mut vertex_count,
                            pos,
                            [
                                vec3(0.5, -1.0, -0.5),   // Bottom left
                                vec3(-0.5, -1.0, -0.5),  // Bottom right
                                vec3(-0.5, 1.0, -0.5),   // Top right
                                vec3(0.5, 1.0, -0.5),    // Top left
                            ],
                            wall_type,
                        );
                    }
                    
                    // East face (positive X)
                    if !self.map.is_wall((x + 1) as i32, y as i32) {
                        self.add_wall_face(
                            &mut vertices,
                            &mut indices,
                            &mut vertex_count,
                            pos,
                            [
                                vec3(0.5, -1.0, 0.5),    // Bottom left
                                vec3(0.5, -1.0, -0.5),   // Bottom right
                                vec3(0.5, 1.0, -0.5),    // Top right
                                vec3(0.5, 1.0, 0.5),     // Top left
                            ],
                            wall_type,
                        );
                    }
                    
                    // West face (negative X)
                    if !self.map.is_wall((x as i32) - 1, y as i32) {
                        self.add_wall_face(
                            &mut vertices,
                            &mut indices,
                            &mut vertex_count,
                            pos,
                            [
                                vec3(-0.5, -1.0, -0.5),  // Bottom left
                                vec3(-0.5, -1.0, 0.5),   // Bottom right
                                vec3(-0.5, 1.0, 0.5),    // Top right
                                vec3(-0.5, 1.0, -0.5),   // Top left
                            ],
                            wall_type,
                        );
                    }
                }
            }
        }

        // Load a texture for the walls
        let texture = if let Ok(mut tex) = load_texture("assets/textures/tech_panel.png").await {
            tex.set_filter(FilterMode::Nearest);
            Some(tex)
        } else {
            None
        };

        println!("Generated wall mesh with {} vertices and {} faces", vertices.len(), indices.len() / 3);

        Mesh {
            vertices,
            indices,
            texture,
        }
    }

    /// Generate a single mesh containing all wall geometry with proper UV mapping (sync version)
    pub fn generate_wall_mesh(&self) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut vertex_count = 0u16;

        for y in 0..self.map.height {
            for x in 0..self.map.width {
                if self.map.is_wall(x as i32, y as i32) {
                    let wall_type = self.map.get_wall_type(x as i32, y as i32);
                    let pos = Vec3::new(x as f32 + 0.5, 1.0, y as f32 + 0.5);
                    
                    // Only create faces that are exposed (adjacent to empty space)
                    
                    // North face (positive Z)
                    if !self.map.is_wall(x as i32, (y + 1) as i32) {
                        self.add_wall_face(
                            &mut vertices,
                            &mut indices,
                            &mut vertex_count,
                            pos,
                            [
                                vec3(-0.5, -1.0, 0.5),  // Bottom left
                                vec3(0.5, -1.0, 0.5),   // Bottom right
                                vec3(0.5, 1.0, 0.5),    // Top right  
                                vec3(-0.5, 1.0, 0.5),   // Top left
                            ],
                            wall_type,
                        );
                    }
                    
                    // South face (negative Z)
                    if !self.map.is_wall(x as i32, (y as i32) - 1) {
                        self.add_wall_face(
                            &mut vertices,
                            &mut indices,
                            &mut vertex_count,
                            pos,
                            [
                                vec3(0.5, -1.0, -0.5),   // Bottom left
                                vec3(-0.5, -1.0, -0.5),  // Bottom right
                                vec3(-0.5, 1.0, -0.5),   // Top right
                                vec3(0.5, 1.0, -0.5),    // Top left
                            ],
                            wall_type,
                        );
                    }
                    
                    // East face (positive X)
                    if !self.map.is_wall((x + 1) as i32, y as i32) {
                        self.add_wall_face(
                            &mut vertices,
                            &mut indices,
                            &mut vertex_count,
                            pos,
                            [
                                vec3(0.5, -1.0, 0.5),    // Bottom left
                                vec3(0.5, -1.0, -0.5),   // Bottom right
                                vec3(0.5, 1.0, -0.5),    // Top right
                                vec3(0.5, 1.0, 0.5),     // Top left
                            ],
                            wall_type,
                        );
                    }
                    
                    // West face (negative X)
                    if !self.map.is_wall((x as i32) - 1, y as i32) {
                        self.add_wall_face(
                            &mut vertices,
                            &mut indices,
                            &mut vertex_count,
                            pos,
                            [
                                vec3(-0.5, -1.0, -0.5),  // Bottom left
                                vec3(-0.5, -1.0, 0.5),   // Bottom right
                                vec3(-0.5, 1.0, 0.5),    // Top right
                                vec3(-0.5, 1.0, -0.5),   // Top left
                            ],
                            wall_type,
                        );
                    }
                }
            }
        }

        println!("Generated wall mesh with {} vertices and {} faces", vertices.len(), indices.len() / 3);

        Mesh {
            vertices,
            indices,
            texture: None, // No texture in sync version - will use vertex colors
        }
    }

    /// Add a wall face with proper UV coordinates
    fn add_wall_face(
        &self,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u16>,
        vertex_count: &mut u16,
        center_pos: Vec3,
        local_positions: [Vec3; 4],
        wall_type: WallType,
    ) {
        // Convert wall type to color for now (later we can use textures)
        let color = self.wall_type_to_color(wall_type);
        
        // Create vertices with proper UV mapping
        for (i, local_pos) in local_positions.iter().enumerate() {
            let world_pos = center_pos + *local_pos;
            
            // UV coordinates for proper texture tiling
            let uv = match i {
                0 => vec2(0.0, 1.0),  // Bottom left
                1 => vec2(1.0, 1.0),  // Bottom right
                2 => vec2(1.0, 0.0),  // Top right
                3 => vec2(0.0, 0.0),  // Top left
                _ => vec2(0.0, 0.0),
            };

            vertices.push(Vertex {
                position: world_pos,
                uv,
                color: [
                    (color.r * 255.0) as u8,
                    (color.g * 255.0) as u8,
                    (color.b * 255.0) as u8,
                    (color.a * 255.0) as u8,
                ],
                normal: Vec4::new(0.0, 0.0, 0.0, 0.0), // We'll calculate proper normals later if needed
            });
        }

        // Add indices for two triangles (making a quad)
        let base = *vertex_count;
        indices.extend_from_slice(&[
            base, base + 1, base + 2,      // First triangle
            base, base + 2, base + 3,      // Second triangle
        ]);

        *vertex_count += 4;
    }

    /// Convert wall type to color for visualization
    fn wall_type_to_color(&self, wall_type: WallType) -> Color {
        match wall_type {
            WallType::TechPanel => Color::new(0.85, 0.88, 0.92, 1.0),
            WallType::HullPlating => Color::new(0.6, 0.65, 0.7, 1.0),
            WallType::ControlSystem => Color::new(0.2, 0.25, 0.35, 1.0),
            WallType::EnergyConduit => Color::new(0.15, 0.2, 0.25, 1.0),
            WallType::Empty => WHITE,
        }
    }
} 