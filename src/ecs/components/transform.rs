//! Transform component for position, rotation, and scale in 3D space

use macroquad::prelude::*;
use crate::ecs::Component;

/// Position, rotation, and scale in 3D space
#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3, // Euler angles in radians
    pub scale: Vec3,
    pub enabled: bool,
}

impl Transform {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
            enabled: true,
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

impl Component for Transform {
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