use macroquad::prelude::*;

/// Low-poly mesh generator for creating simple geometric shapes
pub struct LowPolyMeshGenerator;

impl LowPolyMeshGenerator {
    /// Create a low-poly pyramid (5 vertices, 6 triangles)
    pub fn create_pyramid() -> Mesh {
        let vertices = vec![
            // Base vertices (square)
            Vertex {
                position: Vec3::new(-0.5, 0.0, -0.5),
                uv: Vec2::new(0.0, 0.0),
                normal: Vec4::new(0.0, 1.0, 0.0, 0.0),
                color: [255, 200, 150, 255], // Warm orange
            },
            Vertex {
                position: Vec3::new(0.5, 0.0, -0.5),
                uv: Vec2::new(1.0, 0.0),
                normal: Vec4::new(0.0, 1.0, 0.0, 0.0),
                color: [255, 200, 150, 255],
            },
            Vertex {
                position: Vec3::new(0.5, 0.0, 0.5),
                uv: Vec2::new(1.0, 1.0),
                normal: Vec4::new(0.0, 1.0, 0.0, 0.0),
                color: [255, 200, 150, 255],
            },
            Vertex {
                position: Vec3::new(-0.5, 0.0, 0.5),
                uv: Vec2::new(0.0, 1.0),
                normal: Vec4::new(0.0, 1.0, 0.0, 0.0),
                color: [255, 200, 150, 255],
            },
            // Apex vertex
            Vertex {
                position: Vec3::new(0.0, 1.0, 0.0),
                uv: Vec2::new(0.5, 0.5),
                normal: Vec4::new(0.0, 1.0, 0.0, 0.0),
                color: [255, 150, 100, 255], // Darker at top
            },
        ];

        let indices = vec![
            // Base (2 triangles)
            0, 1, 2,
            0, 2, 3,
            // Sides (4 triangles)
            0, 4, 1,
            1, 4, 2,
            2, 4, 3,
            3, 4, 0,
        ];

        Mesh {
            vertices,
            indices,
            texture: None,
        }
    }

    /// Create a low-poly octahedron (6 vertices, 8 triangles)
    pub fn create_octahedron() -> Mesh {
        let vertices = vec![
            // Top vertex
            Vertex {
                position: Vec3::new(0.0, 1.0, 0.0),
                uv: Vec2::new(0.5, 1.0),
                normal: Vec4::new(0.0, 1.0, 0.0, 0.0),
                color: [100, 255, 150, 255], // Bright green at top
            },
            // Middle ring (4 vertices)
            Vertex {
                position: Vec3::new(1.0, 0.0, 0.0),
                uv: Vec2::new(1.0, 0.5),
                normal: Vec4::new(1.0, 0.0, 0.0, 0.0),
                color: [150, 255, 100, 255],
            },
            Vertex {
                position: Vec3::new(0.0, 0.0, 1.0),
                uv: Vec2::new(0.5, 0.0),
                normal: Vec4::new(0.0, 0.0, 1.0, 0.0),
                color: [150, 255, 100, 255],
            },
            Vertex {
                position: Vec3::new(-1.0, 0.0, 0.0),
                uv: Vec2::new(0.0, 0.5),
                normal: Vec4::new(-1.0, 0.0, 0.0, 0.0),
                color: [150, 255, 100, 255],
            },
            Vertex {
                position: Vec3::new(0.0, 0.0, -1.0),
                uv: Vec2::new(0.5, 1.0),
                normal: Vec4::new(0.0, 0.0, -1.0, 0.0),
                color: [150, 255, 100, 255],
            },
            // Bottom vertex
            Vertex {
                position: Vec3::new(0.0, -1.0, 0.0),
                uv: Vec2::new(0.5, 0.0),
                normal: Vec4::new(0.0, -1.0, 0.0, 0.0),
                color: [50, 200, 100, 255], // Darker green at bottom
            },
        ];

        let indices = vec![
            // Top pyramid (4 triangles)
            0, 1, 2,
            0, 2, 3,
            0, 3, 4,
            0, 4, 1,
            // Bottom pyramid (4 triangles)
            5, 2, 1,
            5, 3, 2,
            5, 4, 3,
            5, 1, 4,
        ];

        Mesh {
            vertices,
            indices,
            texture: None,
        }
    }

    /// Create a low-poly diamond/gem shape (10 vertices, 16 triangles)
    pub fn create_diamond() -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Top point
        vertices.push(Vertex {
            position: Vec3::new(0.0, 1.0, 0.0),
            uv: Vec2::new(0.5, 1.0),
            normal: Vec4::new(0.0, 1.0, 0.0, 0.0),
            color: [255, 255, 255, 255], // White at top
        });

        // Upper ring (4 vertices)
        let upper_height = 0.3;
        let upper_radius = 0.7;
        for i in 0..4 {
            let angle = (i as f32) * std::f32::consts::PI * 0.5;
            let x = angle.cos() * upper_radius;
            let z = angle.sin() * upper_radius;
            
            vertices.push(Vertex {
                position: Vec3::new(x, upper_height, z),
                uv: Vec2::new(0.5 + x * 0.5, 0.5 + z * 0.5),
                normal: Vec4::new(x, 0.5, z, 0.0).normalize(),
                color: [200, 200, 255, 255], // Light blue
            });
        }

        // Lower ring (4 vertices)
        let lower_height = -0.3;
        let lower_radius = 0.5;
        for i in 0..4 {
            let angle = (i as f32) * std::f32::consts::PI * 0.5;
            let x = angle.cos() * lower_radius;
            let z = angle.sin() * lower_radius;
            
            vertices.push(Vertex {
                position: Vec3::new(x, lower_height, z),
                uv: Vec2::new(0.5 + x * 0.5, 0.5 + z * 0.5),
                normal: Vec4::new(x, -0.5, z, 0.0).normalize(),
                color: [150, 150, 200, 255], // Darker blue
            });
        }

        // Bottom point
        vertices.push(Vertex {
            position: Vec3::new(0.0, -1.0, 0.0),
            uv: Vec2::new(0.5, 0.0),
            normal: Vec4::new(0.0, -1.0, 0.0, 0.0),
            color: [100, 100, 150, 255], // Dark blue at bottom
        });

        // Top triangles (4 triangles from top point to upper ring)
        for i in 0..4 {
            let next = (i + 1) % 4;
            indices.extend_from_slice(&[0, 1 + i as u16, 1 + next as u16]);
        }

        // Middle band (8 triangles connecting upper and lower rings)
        for i in 0..4 {
            let next = (i + 1) % 4;
            let upper_curr = 1 + i as u16;
            let upper_next = 1 + next as u16;
            let lower_curr = 5 + i as u16;
            let lower_next = 5 + next as u16;
            
            // Two triangles per quad
            indices.extend_from_slice(&[upper_curr, lower_curr, upper_next]);
            indices.extend_from_slice(&[upper_next, lower_curr, lower_next]);
        }

        // Bottom triangles (4 triangles from lower ring to bottom point)
        for i in 0..4 {
            let next = (i + 1) % 4;
            indices.extend_from_slice(&[9, 5 + next as u16, 5 + i as u16]);
        }

        Mesh {
            vertices,
            indices,
            texture: None,
        }
    }

    /// Create a low-poly crystal (irregular gem shape)
    pub fn create_crystal() -> Mesh {
        let vertices = vec![
            // Irregular crystal with asymmetric points
            // Top spike
            Vertex {
                position: Vec3::new(0.1, 1.2, -0.1),
                uv: Vec2::new(0.5, 1.0),
                normal: Vec4::new(0.0, 1.0, 0.0, 0.0),
                color: [255, 100, 255, 255], // Magenta
            },
            // Upper vertices (irregular)
            Vertex {
                position: Vec3::new(0.8, 0.3, 0.2),
                uv: Vec2::new(1.0, 0.7),
                normal: Vec4::new(1.0, 0.3, 0.0, 0.0).normalize(),
                color: [200, 100, 200, 255],
            },
            Vertex {
                position: Vec3::new(-0.3, 0.4, 0.9),
                uv: Vec2::new(0.2, 0.8),
                normal: Vec4::new(0.0, 0.4, 1.0, 0.0).normalize(),
                color: [200, 100, 200, 255],
            },
            Vertex {
                position: Vec3::new(-0.7, 0.2, -0.4),
                uv: Vec2::new(0.1, 0.3),
                normal: Vec4::new(-1.0, 0.2, -0.4, 0.0).normalize(),
                color: [200, 100, 200, 255],
            },
            Vertex {
                position: Vec3::new(0.4, 0.1, -0.8),
                uv: Vec2::new(0.8, 0.1),
                normal: Vec4::new(0.4, 0.1, -1.0, 0.0).normalize(),
                color: [200, 100, 200, 255],
            },
            // Bottom point
            Vertex {
                position: Vec3::new(-0.1, -0.8, 0.1),
                uv: Vec2::new(0.4, 0.0),
                normal: Vec4::new(0.0, -1.0, 0.0, 0.0),
                color: [150, 50, 150, 255], // Dark magenta
            },
        ];

        let indices = vec![
            // Top triangles
            0, 1, 2,
            0, 2, 3,
            0, 3, 4,
            0, 4, 1,
            // Side triangles
            1, 5, 2,
            2, 5, 3,
            3, 5, 4,
            4, 5, 1,
        ];

        Mesh {
            vertices,
            indices,
            texture: None,
        }
    }

    /// Get a mesh by name
    pub fn get_mesh(name: &str) -> Option<Mesh> {
        match name {
            "pyramid" => Some(Self::create_pyramid()),
            "octahedron" => Some(Self::create_octahedron()),
            "diamond" => Some(Self::create_diamond()),
            "crystal" => Some(Self::create_crystal()),
            _ => None,
        }
    }

    /// List all available low-poly meshes
    pub fn available_meshes() -> Vec<&'static str> {
        vec!["pyramid", "octahedron", "diamond", "crystal"]
    }
} 