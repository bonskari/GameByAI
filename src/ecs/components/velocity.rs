//! Velocity component for linear and angular velocity

use macroquad::prelude::*;
use crate::ecs::Component;

/// Linear and angular velocity
#[derive(Debug, Clone)]
pub struct Velocity {
    pub linear: Vec3,
    pub angular: Vec3,
    pub enabled: bool,
}

impl Velocity {
    pub fn new() -> Self {
        Self {
            linear: Vec3::ZERO,
            angular: Vec3::ZERO,
            enabled: true,
        }
    }

    pub fn linear(velocity: Vec3) -> Self {
        Self {
            linear: velocity,
            angular: Vec3::ZERO,
            enabled: true,
        }
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
}

impl Default for Velocity {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Velocity {
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