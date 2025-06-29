//! Rendering components and systems

use macroquad::prelude::*;
use crate::ecs::{Component, World, Entity, Transform, component::{ComponentRegistration}};

/// Unified renderer for all static geometry (walls, floors, ceilings, props)
pub struct StaticRenderer {
    pub mesh: Option<Mesh>,
    pub texture: Option<Texture2D>,
    pub material_type: MaterialType,
    pub color: Color,
    pub visible: bool,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub enum MaterialType {
    Wall { texture_name: String },
    Floor { texture_name: String },
    Ceiling { texture_name: String },
    Prop { texture_name: String },
}

impl StaticRenderer {
    pub fn new(material_type: MaterialType) -> Self {
        Self {
            mesh: None,
            texture: None,
            material_type,
            color: WHITE,
            visible: true,
            enabled: true,
        }
    }

    pub fn wall(texture_name: String) -> Self {
        Self::new(MaterialType::Wall { texture_name })
    }

    pub fn floor(texture_name: String) -> Self {
        Self::new(MaterialType::Floor { texture_name })
    }

    pub fn ceiling(texture_name: String) -> Self {
        Self::new(MaterialType::Ceiling { texture_name })
    }

    pub fn prop(texture_name: String) -> Self {
        Self::new(MaterialType::Prop { texture_name })
    }

    pub fn with_texture(mut self, texture: Texture2D) -> Self {
        self.texture = Some(texture);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_mesh(mut self, mesh: Mesh) -> Self {
        self.mesh = Some(mesh);
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
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

    /// Check if this component should be rendered
    pub fn should_render(&self) -> bool {
        self.enabled && self.visible
    }

    /// Set visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Component for StaticRenderer {
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

impl Renderable for StaticRenderer {
    fn get_render_data(&self) -> RenderData {
        let texture_name = match &self.material_type {
            MaterialType::Wall { texture_name } => Some(texture_name.clone()),
            MaterialType::Floor { texture_name } => Some(texture_name.clone()),
            MaterialType::Ceiling { texture_name } => Some(texture_name.clone()),
            MaterialType::Prop { texture_name } => Some(texture_name.clone()),
        };

        RenderData {
            render_type: if self.mesh.is_some() {
                RenderType::Mesh
            } else {
                RenderType::Custom
            },
            color: self.color,
            texture_name,
            size: Vec3::ONE,
        }
    }

    fn should_render(&self) -> bool {
        self.enabled && self.visible
    }
}

impl std::fmt::Debug for StaticRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StaticRenderer")
            .field("material_type", &self.material_type)
            .field("color", &self.color)
            .field("visible", &self.visible)
            .field("enabled", &self.enabled)
            .field("has_mesh", &self.mesh.is_some())
            .field("has_texture", &self.texture.is_some())
            .finish()
    }
}

impl Clone for StaticRenderer {
    fn clone(&self) -> Self {
        Self {
            mesh: None, // Meshes don't implement Clone, so we skip them
            texture: self.texture.clone(),
            material_type: self.material_type.clone(),
            color: self.color,
            visible: self.visible,
            enabled: self.enabled,
        }
    }
}

/// Trait for components that can be rendered
pub trait Renderable: Component {
    /// Get the render data for this component
    fn get_render_data(&self) -> RenderData;
    
    /// Check if this component should be rendered
    fn should_render(&self) -> bool {
        true
    }
    
    /// Get the render priority (lower = rendered first)
    fn render_priority(&self) -> u32 {
        100 // Default priority
    }
}

/// Data needed to render any component
pub struct RenderData {
    pub render_type: RenderType,
    pub color: Color,
    pub texture_name: Option<String>,
    pub size: Vec3,
}

/// Types of rendering
pub enum RenderType {
    Mesh, // Will use mesh from component directly
    Cube { size: Vec3 },
    Custom,
} 