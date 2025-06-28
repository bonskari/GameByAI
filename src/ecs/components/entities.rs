//! Entity marker components for different game objects

use crate::ecs::Component;

/// Marker component for wall entities
#[derive(Debug, Clone)]
pub struct Wall {
    pub wall_type: crate::game::map::WallType,
    pub enabled: bool,
}

impl Wall {
    pub fn new() -> Self {
        Self {
            wall_type: crate::game::map::WallType::TechPanel, // Default type
            enabled: true,
        }
    }

    pub fn new_with_type(wall_type: crate::game::map::WallType) -> Self {
        Self {
            wall_type,
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

impl Default for Wall {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Wall {
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

/// Marker component for floor entities
#[derive(Debug, Clone)]
pub struct Floor {
    pub enabled: bool,
}

impl Floor {
    pub fn new() -> Self {
        Self { enabled: true }
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

impl Default for Floor {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Floor {
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

/// Marker component for ceiling entities
#[derive(Debug, Clone)]
pub struct Ceiling {
    pub enabled: bool,
}

impl Ceiling {
    pub fn new() -> Self {
        Self { enabled: true }
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

impl Default for Ceiling {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Ceiling {
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

/// Marker component for prop entities
#[derive(Debug, Clone)]
pub struct Prop {
    pub enabled: bool,
}

impl Prop {
    pub fn new() -> Self {
        Self { enabled: true }
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

impl Default for Prop {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Prop {
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