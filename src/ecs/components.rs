//! Game-specific components for the Wolfenstein ECS

use macroquad::prelude::*;
use crate::ecs::Component;

/// 3D position and orientation
#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    /// Create a new transform at the origin
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
    
    /// Create a transform at a specific position
    pub fn at_position(position: Vec3) -> Self {
        Self {
            position,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
    
    /// Get forward direction vector
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::NEG_Z
    }
    
    /// Get right direction vector
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }
    
    /// Get up direction vector
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }
    
    /// Rotate around Y axis (yaw)
    pub fn rotate_y(&mut self, angle: f32) {
        self.rotation = Quat::from_rotation_y(angle) * self.rotation;
    }
    
    /// Rotate around X axis (pitch)
    pub fn rotate_x(&mut self, angle: f32) {
        self.rotation = self.rotation * Quat::from_rotation_x(angle);
    }
    
    /// Translate by a vector
    pub fn translate(&mut self, delta: Vec3) {
        self.position += delta;
    }
    
    /// Get transformation matrix
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
}

impl Component for Transform {}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

/// Linear and angular velocity
#[derive(Debug, Clone, PartialEq)]
pub struct Velocity {
    pub linear: Vec3,
    pub angular: Vec3,
}

impl Velocity {
    /// Create zero velocity
    pub fn new() -> Self {
        Self {
            linear: Vec3::ZERO,
            angular: Vec3::ZERO,
        }
    }
    
    /// Create with linear velocity only
    pub fn linear(velocity: Vec3) -> Self {
        Self {
            linear: velocity,
            angular: Vec3::ZERO,
        }
    }
}

impl Component for Velocity {}

impl Default for Velocity {
    fn default() -> Self {
        Self::new()
    }
}

/// Renderable mesh with material
pub struct MeshRenderer {
    pub mesh: Option<Mesh>,
    pub texture: Option<Texture2D>,
    pub color: Color,
    pub visible: bool,
}

impl MeshRenderer {
    /// Create a new mesh renderer
    pub fn new() -> Self {
        Self {
            mesh: None,
            texture: None,
            color: WHITE,
            visible: true,
        }
    }
    
    /// Create with texture
    pub fn with_texture(texture: Texture2D) -> Self {
        Self {
            mesh: None,
            texture: Some(texture),
            color: WHITE,
            visible: true,
        }
    }
    
    /// Create with color
    pub fn with_color(color: Color) -> Self {
        Self {
            mesh: None,
            texture: None,
            color,
            visible: true,
        }
    }
}

impl Component for MeshRenderer {}

impl Default for MeshRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Player-specific component
#[derive(Debug, Clone)]
pub struct Player {
    pub move_speed: f32,
    pub mouse_sensitivity: f32,
    pub jump_force: f32,
    pub is_grounded: bool,
}

impl Player {
    /// Create a new player component
    pub fn new() -> Self {
        Self {
            move_speed: 5.0,
            mouse_sensitivity: 0.002,
            jump_force: 8.0,
            is_grounded: true,
        }
    }
}

impl Component for Player {}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

/// Wall component for collision detection
#[derive(Debug, Clone)]
pub struct Wall {
    pub wall_type: WallType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WallType {
    Solid,
    Door,
    Window,
}

impl Wall {
    /// Create a solid wall
    pub fn solid() -> Self {
        Self {
            wall_type: WallType::Solid,
        }
    }
}

impl Component for Wall {}

/// Floor component
#[derive(Debug, Clone)]
pub struct Floor {
    pub height: f32,
}

impl Floor {
    /// Create a floor at specific height
    pub fn at_height(height: f32) -> Self {
        Self { height }
    }
}

impl Component for Floor {}

/// Ceiling component
#[derive(Debug, Clone)]
pub struct Ceiling {
    pub height: f32,
}

impl Ceiling {
    /// Create a ceiling at specific height
    pub fn at_height(height: f32) -> Self {
        Self { height }
    }
}

impl Component for Ceiling {}

/// Bounding box for collision detection
#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    /// Create a bounding box from min/max points
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }
    
    /// Create a unit cube bounding box
    pub fn unit_cube() -> Self {
        Self {
            min: Vec3::splat(-0.5),
            max: Vec3::splat(0.5),
        }
    }
    
    /// Create a bounding box from center and size
    pub fn from_center_size(center: Vec3, size: Vec3) -> Self {
        let half_size = size * 0.5;
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }
    
    /// Check if this box intersects with another
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }
    
    /// Get the center point
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }
    
    /// Get the size
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }
}

impl Component for BoundingBox {} 