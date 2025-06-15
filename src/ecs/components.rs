//! Game-specific components for the GameByAI ECS

use macroquad::prelude::*;
use crate::ecs::Component;

/// Position, rotation, and scale in 3D space
#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3, // Euler angles in radians
    pub scale: Vec3,
}

impl Transform {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
        }
    }

    pub fn with_rotation(mut self, rotation: Vec3) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self
    }

    /// Get the forward direction vector
    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            self.rotation.y.cos(),
            0.0,
            self.rotation.y.sin(),
        )
    }

    /// Get the right direction vector
    pub fn right(&self) -> Vec3 {
        Vec3::new(
            (self.rotation.y + std::f32::consts::PI / 2.0).cos(),
            0.0,
            (self.rotation.y + std::f32::consts::PI / 2.0).sin(),
        )
    }

    /// Get the up direction vector
    pub fn up(&self) -> Vec3 {
        Vec3::Y
    }

    /// Apply a translation
    pub fn translate(&mut self, delta: Vec3) {
        self.position += delta;
    }

    /// Get transformation matrix
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            self.scale,
            Quat::from_euler(EulerRot::XYZ, self.rotation.x, self.rotation.y, self.rotation.z),
            self.position,
        )
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new(Vec3::ZERO)
    }
}

impl Component for Transform {}

/// Linear and angular velocity
#[derive(Debug, Clone)]
pub struct Velocity {
    pub linear: Vec3,
    pub angular: Vec3,
}

impl Velocity {
    pub fn new() -> Self {
        Self {
            linear: Vec3::ZERO,
            angular: Vec3::ZERO,
        }
    }

    pub fn linear(velocity: Vec3) -> Self {
        Self {
            linear: velocity,
            angular: Vec3::ZERO,
        }
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Velocity {}

/// Unified renderer for all static geometry (walls, floors, ceilings, props)
pub struct StaticRenderer {
    pub mesh: Option<Mesh>,
    pub texture: Option<Texture2D>,
    pub material_type: MaterialType,
    pub color: Color,
    pub visible: bool,
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
}

// Custom Debug implementation to handle Mesh
impl std::fmt::Debug for StaticRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StaticRenderer")
            .field("mesh", &self.mesh.as_ref().map(|_| "Mesh"))
            .field("texture", &self.texture.as_ref().map(|_| "Texture2D"))
            .field("material_type", &self.material_type)
            .field("color", &self.color)
            .field("visible", &self.visible)
            .finish()
    }
}

// Custom Clone implementation to handle Mesh
impl Clone for StaticRenderer {
    fn clone(&self) -> Self {
        Self {
            mesh: None, // Can't clone Mesh, so set to None
            texture: self.texture.clone(),
            material_type: self.material_type.clone(),
            color: self.color,
            visible: self.visible,
        }
    }
}

impl Component for StaticRenderer {}

/// Collision detection and physics properties
#[derive(Debug, Clone)]
pub struct Collider {
    pub shape: ColliderShape,
    pub blocks_movement: bool,
    pub blocks_projectiles: bool,
}

#[derive(Debug, Clone)]
pub enum ColliderShape {
    Box { size: Vec3 },
    Sphere { radius: f32 },
    Capsule { height: f32, radius: f32 },
}

impl Collider {
    pub fn wall() -> Self {
        Self {
            shape: ColliderShape::Box { size: Vec3::new(1.0, 2.0, 1.0) },
            blocks_movement: true,
            blocks_projectiles: true,
        }
    }

    pub fn floor() -> Self {
        Self {
            shape: ColliderShape::Box { size: Vec3::new(1.0, 0.1, 1.0) },
            blocks_movement: false, // Floor doesn't block horizontal movement
            blocks_projectiles: false,
        }
    }

    pub fn prop() -> Self {
        Self {
            shape: ColliderShape::Box { size: Vec3::new(0.5, 1.0, 0.5) },
            blocks_movement: true,
            blocks_projectiles: true,
        }
    }

    pub fn player() -> Self {
        Self {
            shape: ColliderShape::Capsule { height: 1.8, radius: 0.3 },
            blocks_movement: false,
            blocks_projectiles: false,
        }
    }
}

impl Component for Collider {}

/// Player-specific component
#[derive(Debug, Clone)]
pub struct Player {
    pub health: f32,
    pub max_health: f32,
    pub is_grounded: bool,
    pub move_speed: f32,
    pub jump_strength: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            is_grounded: true,
            move_speed: 5.0,
            jump_strength: 4.5,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Player {}

// Tag components for easy querying
/// Marker component for wall entities
#[derive(Debug, Clone)]
pub struct Wall;

impl Component for Wall {}

/// Marker component for floor entities  
#[derive(Debug, Clone)]
pub struct Floor;

impl Component for Floor {}

/// Marker component for ceiling entities
#[derive(Debug, Clone)]
pub struct Ceiling;

impl Component for Ceiling {}

/// Marker component for prop entities
#[derive(Debug, Clone)]
pub struct Prop;

impl Component for Prop {} 