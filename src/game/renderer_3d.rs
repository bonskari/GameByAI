use macroquad::prelude::*;
use super::{map::{Map, WallType}, player::Player};
use std::collections::HashMap;

#[derive(Clone)]
struct WallMeshData {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

pub struct Modern3DRenderer {
    camera: Camera3D,
    wall_meshes: HashMap<WallType, Mesh>,
    floor_mesh: Option<Mesh>,
    ceiling_mesh: Option<Mesh>,
    needs_rebuild: bool,
}

impl Modern3DRenderer {
    pub fn new() -> Self {
        Modern3DRenderer {
            camera: Camera3D {
                position: vec3(0.0, 1.0, 0.0),
                up: vec3(0.0, 1.0, 0.0),
                target: vec3(1.0, 1.0, 0.0),
                ..Default::default()
            },
            wall_meshes: HashMap::new(),
            floor_mesh: None,
            ceiling_mesh: None,
            needs_rebuild: true,
        }
    }

    pub fn build_geometry(&mut self, map: &Map) {
        if !self.needs_rebuild {
            return;
        }

        self.wall_meshes.clear();
        let wall_height = 2.0;

        let mut mesh_data_map: HashMap<WallType, WallMeshData> = HashMap::new();

        for y in 0..map.height {
            for x in 0..map.width {
                if map.is_wall(x as i32, y as i32) {
                    let wall_type = map.get_wall_type(x as i32, y as i32);
                    let data = mesh_data_map.entry(wall_type).or_insert_with(|| WallMeshData {
                        vertices: Vec::new(),
                        indices: Vec::new(),
                    });
                    Self::add_wall_faces(data, x, y, wall_height, map, &wall_type);
                }
            }
        }

        for (wall_type, data) in mesh_data_map {
            self.wall_meshes.insert(wall_type, Mesh {
                vertices: data.vertices,
                indices: data.indices.iter().map(|&i| i as u16).collect(),
                texture: None,
            });
        }

        self.floor_mesh = Some(Self::create_floor_plane(map));
        self.ceiling_mesh = Some(Self::create_ceiling_plane(map));

        self.needs_rebuild = false;
        println!("3D geometry rebuilt successfully.");
    }

    fn add_wall_faces(data: &mut WallMeshData, x: usize, y: usize, height: f32, map: &Map, wall_type: &WallType) {
        let (v0, v1, v2, v3, v4, v5, v6, v7) = Self::get_wall_vertices(x, y);

        // Normals for each face
        let normals = [
            vec3(0.0, 0.0, 1.0),   // Front
            vec3(0.0, 0.0, -1.0),  // Back
            vec3(1.0, 0.0, 0.0),   // Right
            vec3(-1.0, 0.0, 0.0),  // Left
            vec3(0.0, 1.0, 0.0),   // Top
            vec3(0.0, -1.0, 0.0),  // Bottom
        ];

        let faces: &[(Vec3, Vec3, Vec3, Vec3, bool, usize)] = &[
            (v0, v1, v2, v3, true, 0), // Front face
            (v5, v4, v7, v6, true, 1), // Back face
            (v1, v5, v6, v2, true, 2), // Right face
            (v4, v0, v3, v7, true, 3), // Left face
            (v3, v2, v6, v7, false, 4), // Top face
            (v4, v5, v1, v0, false, 5), // Bottom face
        ];

        for (p0, p1, p2, p3, is_vertical, normal_idx) in faces.iter() {
            let base = data.vertices.len() as u32;
            let normal = normals[*normal_idx];
            let uvs = [
                vec2(0.0, 0.0),
                vec2(1.0, 0.0),
                vec2(1.0, 1.0),
                vec2(0.0, 1.0),
            ];
            Self::add_vertex(data, *p0, uvs[0], *is_vertical, height, map, *wall_type, vec4(normal.x, normal.y, normal.z, 0.0));
            Self::add_vertex(data, *p1, uvs[1], *is_vertical, height, map, *wall_type, vec4(normal.x, normal.y, normal.z, 0.0));
            Self::add_vertex(data, *p2, uvs[2], *is_vertical, height, map, *wall_type, vec4(normal.x, normal.y, normal.z, 0.0));
            Self::add_vertex(data, *p3, uvs[3], *is_vertical, height, map, *wall_type, vec4(normal.x, normal.y, normal.z, 0.0));
            data.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        }
    }
    
    fn get_wall_vertices(x: usize, y: usize) -> (Vec3, Vec3, Vec3, Vec3, Vec3, Vec3, Vec3, Vec3) {
        let fx = x as f32;
        let fy = y as f32;
        let h = 2.0; // wall height
        (
            vec3(fx, 0.0, fy + 1.0),      // v0
            vec3(fx + 1.0, 0.0, fy + 1.0),// v1
            vec3(fx + 1.0, h, fy + 1.0),  // v2
            vec3(fx, h, fy + 1.0),        // v3
            vec3(fx, 0.0, fy),            // v4
            vec3(fx + 1.0, 0.0, fy),      // v5
            vec3(fx + 1.0, h, fy),        // v6
            vec3(fx, h, fy),              // v7
        )
    }

    fn add_vertex(data: &mut WallMeshData, position: Vec3, uv: Vec2, is_vertical: bool, height: f32, map: &Map, wall_type: WallType, normal: Vec4) {
        let (u, v) = if is_vertical {
            (uv.x, uv.y * height)
        } else {
            (uv.x, uv.y)
        };
        let texture_color = map.get_procedural_texture_color(wall_type, is_vertical, u, v);
        let color_bytes = [
            (texture_color.r * 255.0) as u8,
            (texture_color.g * 255.0) as u8,
            (texture_color.b * 255.0) as u8,
            255
        ];
        data.vertices.push(Vertex {
            position,
            uv: vec2(u, v),
            color: color_bytes,
            normal,
        });
    }

    fn create_floor_plane(map: &Map) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        for y in 0..map.height {
            for x in 0..map.width {
                let fx = x as f32;
                let fz = y as f32;
                let base = vertices.len() as u16;
                let c1 = map.get_floor_texture_color(fx, fz);
                let c2 = map.get_floor_texture_color(fx + 1.0, fz);
                let c3 = map.get_floor_texture_color(fx + 1.0, fz + 1.0);
                let c4 = map.get_floor_texture_color(fx, fz + 1.0);
                vertices.extend_from_slice(&[
                    Vertex {
                        position: vec3(fx, 0.0, fz),
                        uv: vec2(0.0, 0.0),
                        color: [
                            (c1.r * 255.0) as u8,
                            (c1.g * 255.0) as u8,
                            (c1.b * 255.0) as u8,
                            255
                        ],
                        normal: vec4(0.0, 1.0, 0.0, 0.0),
                    },
                    Vertex {
                        position: vec3(fx + 1.0, 0.0, fz),
                        uv: vec2(1.0, 0.0),
                        color: [
                            (c2.r * 255.0) as u8,
                            (c2.g * 255.0) as u8,
                            (c2.b * 255.0) as u8,
                            255
                        ],
                        normal: vec4(0.0, 1.0, 0.0, 0.0),
                    },
                    Vertex {
                        position: vec3(fx + 1.0, 0.0, fz + 1.0),
                        uv: vec2(1.0, 1.0),
                        color: [
                            (c3.r * 255.0) as u8,
                            (c3.g * 255.0) as u8,
                            (c3.b * 255.0) as u8,
                            255
                        ],
                        normal: vec4(0.0, 1.0, 0.0, 0.0),
                    },
                    Vertex {
                        position: vec3(fx, 0.0, fz + 1.0),
                        uv: vec2(0.0, 1.0),
                        color: [
                            (c4.r * 255.0) as u8,
                            (c4.g * 255.0) as u8,
                            (c4.b * 255.0) as u8,
                            255
                        ],
                        normal: vec4(0.0, 1.0, 0.0, 0.0),
                    },
                ]);
                indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
            }
        }
        Mesh { vertices, indices, texture: None }
    }

    fn create_ceiling_plane(map: &Map) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let h = 2.0;
        for y in 0..map.height {
            for x in 0..map.width {
                let fx = x as f32;
                let fz = y as f32;
                let base = vertices.len() as u16;
                let c1 = map.get_ceiling_texture_color(fx, fz);
                let c2 = map.get_ceiling_texture_color(fx + 1.0, fz);
                let c3 = map.get_ceiling_texture_color(fx + 1.0, fz + 1.0);
                let c4 = map.get_ceiling_texture_color(fx, fz + 1.0);
                vertices.extend_from_slice(&[
                    Vertex {
                        position: vec3(fx, h, fz),
                        uv: vec2(0.0, 0.0),
                        color: [
                            (c1.r * 255.0) as u8,
                            (c1.g * 255.0) as u8,
                            (c1.b * 255.0) as u8,
                            255
                        ],
                        normal: vec4(0.0, -1.0, 0.0, 0.0),
                    },
                    Vertex {
                        position: vec3(fx + 1.0, h, fz),
                        uv: vec2(1.0, 0.0),
                        color: [
                            (c2.r * 255.0) as u8,
                            (c2.g * 255.0) as u8,
                            (c2.b * 255.0) as u8,
                            255
                        ],
                        normal: vec4(0.0, -1.0, 0.0, 0.0),
                    },
                    Vertex {
                        position: vec3(fx + 1.0, h, fz + 1.0),
                        uv: vec2(1.0, 1.0),
                        color: [
                            (c3.r * 255.0) as u8,
                            (c3.g * 255.0) as u8,
                            (c3.b * 255.0) as u8,
                            255
                        ],
                        normal: vec4(0.0, -1.0, 0.0, 0.0),
                    },
                    Vertex {
                        position: vec3(fx, h, fz + 1.0),
                        uv: vec2(0.0, 1.0),
                        color: [
                            (c4.r * 255.0) as u8,
                            (c4.g * 255.0) as u8,
                            (c4.b * 255.0) as u8,
                            255
                        ],
                        normal: vec4(0.0, -1.0, 0.0, 0.0),
                    },
                ]);
                indices.extend_from_slice(&[base, base + 2, base + 1, base, base + 3, base + 2]);
            }
        }
        Mesh { vertices, indices, texture: None }
    }

    fn update_camera(&mut self, player: &Player) {
        self.camera.position = vec3(player.x, player.z, player.y);
        let yaw = player.rotation;
        let pitch = player.pitch;
        
        let look_x = yaw.cos() * pitch.cos();
        let look_y = pitch.sin();
        let look_z = yaw.sin() * pitch.cos();
        
        self.camera.target = self.camera.position + vec3(look_x, look_y, look_z);
    }

    pub fn render(&mut self, map: &Map, player: &Player) {
        self.build_geometry(map);
        self.update_camera(player);
        set_camera(&self.camera);
        gl_use_default_material();
        clear_background(DARKGRAY);

        for mesh in self.wall_meshes.values() {
            draw_mesh(mesh);
        }
        if let Some(ref mesh) = self.floor_mesh {
            draw_mesh(mesh);
        }
        if let Some(ref mesh) = self.ceiling_mesh {
            draw_mesh(mesh);
        }
        set_default_camera();
    }

    pub fn mark_dirty(&mut self) {
        self.needs_rebuild = true;
    }
}