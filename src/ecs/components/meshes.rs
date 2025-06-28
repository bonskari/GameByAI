//! Mesh components for rendering

use macroquad::prelude::*;
use crate::ecs::Component;
// Forward declaration - will be resolved by mod.rs re-exports

/// Wall mesh component that contains all walls as a single mesh with proper UV mapping
pub struct WallMesh {
    pub mesh: Option<Mesh>,
    pub wall_type: crate::game::map::WallType,
    pub enabled: bool,
}

impl WallMesh {
    pub fn new() -> Self {
        Self {
            mesh: None,
            wall_type: crate::game::map::WallType::TechPanel, // Default type
            enabled: true,
        }
    }

    pub fn new_with_type(wall_type: crate::game::map::WallType) -> Self {
        Self {
            mesh: None,
            wall_type,
            enabled: true,
        }
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_mesh(mut self, mesh: Mesh) -> Self {
        self.mesh = Some(mesh);
        self
    }

    /// Enable this component
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable this component
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if this component is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for WallMesh {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for WallMesh {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn enable(&mut self) {
        self.enabled = true;
    }

    fn disable(&mut self) {
        self.enabled = false;
    }
}

impl super::rendering::Renderable for WallMesh {
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

/// Single mesh component that holds all floor geometry for efficient rendering
pub struct FloorMesh {
    pub mesh: Option<Mesh>,
    pub enabled: bool,
}

impl FloorMesh {
    /// Create a new floor mesh component
    pub fn new(mesh: Mesh) -> Self {
        Self {
            mesh: Some(mesh),
            enabled: true,
        }
    }

    /// Create an empty floor mesh component
    pub fn empty() -> Self {
        Self {
            mesh: None,
            enabled: true,
        }
    }

    /// Check if the floor mesh is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable the floor mesh
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the floor mesh
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Set enabled state
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl Component for FloorMesh {}

impl super::rendering::Renderable for FloorMesh {
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