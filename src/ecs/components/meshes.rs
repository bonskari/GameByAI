//! Mesh components for rendering

use macroquad::prelude::*;
use crate::ecs::Component;
// Forward declaration - will be resolved by mod.rs re-exports

/// Unified mesh component for all static geometry (walls, floors, ceilings)
pub struct StaticMesh {
    pub mesh: Option<Mesh>,
    pub mesh_type: StaticMeshType,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StaticMeshType {
    Walls,
    Floor,
    Ceiling,
    Props,
}

impl StaticMesh {
    /// Create a new static mesh component
    pub fn new(mesh: Mesh, mesh_type: StaticMeshType) -> Self {
        Self {
            mesh: Some(mesh),
            mesh_type,
            enabled: true,
        }
    }

    /// Create an empty static mesh component
    pub fn empty(mesh_type: StaticMeshType) -> Self {
        Self {
            mesh: None,
            mesh_type,
            enabled: true,
        }
    }

    /// Create a walls mesh
    pub fn walls(mesh: Mesh) -> Self {
        Self::new(mesh, StaticMeshType::Walls)
    }

    /// Create a floor mesh
    pub fn floor(mesh: Mesh) -> Self {
        Self::new(mesh, StaticMeshType::Floor)
    }

    /// Create a ceiling mesh
    pub fn ceiling(mesh: Mesh) -> Self {
        Self::new(mesh, StaticMeshType::Ceiling)
    }

    /// Create a props mesh
    pub fn props(mesh: Mesh) -> Self {
        Self::new(mesh, StaticMeshType::Props)
    }

    /// Set the mesh
    pub fn with_mesh(mut self, mesh: Mesh) -> Self {
        self.mesh = Some(mesh);
        self
    }

    /// Check if the mesh is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable the mesh
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the mesh
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Set enabled state
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl Component for StaticMesh {}

impl super::rendering::Renderable for StaticMesh {
    fn get_render_data(&self) -> super::rendering::RenderData {
        super::rendering::RenderData {
            render_type: if self.mesh.is_some() {
                super::rendering::RenderType::Mesh
            } else {
                super::rendering::RenderType::Custom
            },
            color: WHITE, // Meshes typically use their own colors
            texture_name: None, // Meshes have textures baked in
            size: Vec3::ONE, // Not used for meshes
        }
    }
}