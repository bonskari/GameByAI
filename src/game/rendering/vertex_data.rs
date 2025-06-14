//! Vertex data structures and mesh management

use macroquad::prelude::*;

/// Vertex structure for 3D rendering
#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vec3,
    pub uv: Vec2,
    pub color: [u8; 4],
    pub normal: Vec4,
}

/// Wall mesh data container
#[derive(Debug, Default)]
pub struct WallMeshData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl WallMeshData {
    /// Create a new empty wall mesh
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Clear all mesh data
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    /// Get the number of triangles in this mesh
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    /// Get the number of vertices in this mesh
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Check if the mesh is empty
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty() || self.indices.is_empty()
    }
}

/// Floor/ceiling mesh data container
#[derive(Debug, Default)]
pub struct PlaneMeshData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl PlaneMeshData {
    /// Create a new empty plane mesh
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Clear all mesh data
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }
} 