//! Player component

use crate::ecs::Component;

#[derive(Debug, Clone)]
pub struct Player {
    pub health: f32,
    pub max_health: f32,
    pub is_grounded: bool,
    pub move_speed: f32,
    pub jump_strength: f32,
    pub enabled: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            is_grounded: true,
            move_speed: 5.0,
            jump_strength: 8.0,
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

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Player {
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