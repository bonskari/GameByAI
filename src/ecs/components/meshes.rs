//! Mesh-related components for ECS entities

use macroquad::prelude::*;
use crate::ecs::{Component, component::{AutoUpdatable, ComponentRegistration}};
// Forward declaration - will be resolved by mod.rs re-exports

/// Auto-register Renderer component
inventory::submit! {
    ComponentRegistration {
        type_name: "Renderer",
        updater: |world, delta_time| {
            world.update_component_type::<Renderer>(delta_time);
        },
    }
}

/// Auto-register StaticMesh component
inventory::submit! {
    ComponentRegistration {
        type_name: "StaticMesh",
        updater: |world, delta_time| {
            world.update_component_type::<StaticMesh>(delta_time);
        },
    }
}

/// Pure data component that holds mesh geometry data
pub struct StaticMesh {
    pub mesh: Option<Mesh>,
    pub mesh_type: StaticMeshType,
    pub enabled: bool,
}

/// Renderer component that handles the actual rendering behavior
pub struct Renderer {
    pub render_mode: RenderMode,
    pub material: RenderMaterial,
    pub custom_mesh_path: Option<String>,
    pub enabled: bool,
}

/// Different rendering modes
#[derive(Debug)]
pub enum RenderMode {
    /// Render as a cube primitive
    Cube { size: Vec3 },
    /// Render as a sphere primitive  
    Sphere { radius: f32 },
    /// Render as a cylinder primitive
    Cylinder { radius: f32, height: f32 },
    /// Render as a plane primitive
    Plane { width: f32, height: f32 },
    /// Use mesh data from StaticMesh component on same entity
    UseMeshData,
    /// Custom rendering (can be extended)
    Custom,
}

/// Material properties for rendering
#[derive(Debug)]
pub struct RenderMaterial {
    pub color: Color,
    pub texture: Option<Texture2D>,
    pub texture_name: Option<String>,
    pub visible: bool,
}

impl Renderer {
    /// Create renderer that uses mesh data from StaticMesh component
    pub fn from_mesh() -> Self {
        Self {
            render_mode: RenderMode::UseMeshData,
            material: RenderMaterial::default(),
            custom_mesh_path: None,
            enabled: true,
        }
    }

    /// Create renderer with cube primitive
    pub fn cube(size: Vec3) -> Self {
        Self {
            render_mode: RenderMode::Cube { size },
            material: RenderMaterial::default(),
            custom_mesh_path: None,
            enabled: true,
        }
    }

    /// Create renderer with sphere primitive
    pub fn sphere(radius: f32) -> Self {
        Self {
            render_mode: RenderMode::Sphere { radius },
            material: RenderMaterial::default(),
            custom_mesh_path: None,
            enabled: true,
        }
    }

    /// Create renderer with cylinder primitive
    pub fn cylinder(radius: f32, height: f32) -> Self {
        Self {
            render_mode: RenderMode::Cylinder { radius, height },
            material: RenderMaterial::default(),
            custom_mesh_path: None,
            enabled: true,
        }
    }

    /// Create renderer with plane primitive
    pub fn plane(width: f32, height: f32) -> Self {
        Self {
            render_mode: RenderMode::Plane { width, height },
            material: RenderMaterial::default(),
            custom_mesh_path: None,
            enabled: true,
        }
    }

    /// Create a custom renderer
    pub fn custom() -> Self {
        Self {
            render_mode: RenderMode::Custom,
            material: RenderMaterial::default(),
            custom_mesh_path: None,
            enabled: true,
        }
    }

    /// Set the color
    pub fn with_color(mut self, color: Color) -> Self {
        self.material.color = color;
        self
    }

    /// Set the texture
    pub fn with_texture(mut self, texture: Texture2D) -> Self {
        self.material.texture = Some(texture);
        self
    }

    /// Set the texture name for deferred loading
    pub fn with_texture_name(mut self, texture_name: String) -> Self {
        self.material.texture_name = Some(texture_name);
        self
    }

    /// Set visibility
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.material.visible = visible;
        self
    }

    /// Set enabled state
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set custom mesh path for GLTF loading
    pub fn with_custom_mesh_path(mut self, path: String) -> Self {
        self.custom_mesh_path = Some(path);
        self
    }

    /// Check if this should be rendered
    pub fn should_render(&self) -> bool {
        self.enabled && self.material.visible
    }

    /// Get render mode name for debugging
    pub fn get_mode_name(&self) -> &'static str {
        match &self.render_mode {
            RenderMode::Cube { .. } => "Cube",
            RenderMode::Sphere { .. } => "Sphere", 
            RenderMode::Cylinder { .. } => "Cylinder",
            RenderMode::Plane { .. } => "Plane",
            RenderMode::UseMeshData => "MeshData",
            RenderMode::Custom => "Custom",
        }
    }
}

impl Default for RenderMaterial {
    fn default() -> Self {
        Self {
            color: WHITE,
            texture: None,
            texture_name: None,
            visible: true,
        }
    }
}

impl Component for Renderer {
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

impl AutoUpdatable for Renderer {
    fn auto_update(&mut self, _entity: crate::ecs::Entity, _delta_time: f32) {
        // Renderer components don't need per-frame updates
        // Rendering happens in the rendering system
    }
}

/// Types of static meshes for identification
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

impl Component for StaticMesh {
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

impl AutoUpdatable for StaticMesh {
    fn auto_update(&mut self, _entity: crate::ecs::Entity, _delta_time: f32) {
        // StaticMesh components are pure data storage
        // They don't need per-frame updates
    }
}

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