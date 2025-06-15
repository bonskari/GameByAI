//! Centralized input handling for player movement
//! This ensures consistent input behavior across legacy and ECS systems

use macroquad::prelude::*;

/// Player input state captured from keyboard and mouse
#[derive(Debug, Clone, Default)]
pub struct PlayerInput {
    // Movement inputs
    pub forward_move: f32,    // -1.0 to 1.0 (S to W)
    pub strafe_move: f32,     // -1.0 to 1.0 (A to D)
    pub jump_pressed: bool,   // Spacebar pressed this frame
    
    // Look inputs
    pub mouse_delta: Vec2,    // Mouse movement delta
    pub turn_delta: f32,      // Arrow key turning (fallback)
    
    // Settings
    pub mouse_sensitivity: f32,
    pub move_speed: f32,
    pub turn_speed: f32,
}

/// Input handler that captures and processes all player inputs
pub struct InputHandler {
    mouse_sensitivity: f32,
    move_speed: f32,
    turn_speed: f32,
}

impl InputHandler {
    /// Create a new input handler with default settings
    pub fn new() -> Self {
        Self {
            mouse_sensitivity: 0.18,
            move_speed: 2.0,
            turn_speed: 3.0,
        }
    }
    
    /// Capture current frame's input state
    pub fn capture_input(&self) -> PlayerInput {
        let mut input = PlayerInput {
            mouse_sensitivity: self.mouse_sensitivity,
            move_speed: self.move_speed,
            turn_speed: self.turn_speed,
            ..Default::default()
        };
        
        // Capture movement keys (WASD)
        if is_key_down(KeyCode::W) { input.forward_move += 1.0; }
        if is_key_down(KeyCode::S) { input.forward_move -= 1.0; }
        if is_key_down(KeyCode::A) { input.strafe_move -= 1.0; }
        if is_key_down(KeyCode::D) { input.strafe_move += 1.0; }
        
        // Capture jump input
        input.jump_pressed = is_key_pressed(KeyCode::Space);
        
        // Capture mouse look
        input.mouse_delta = mouse_delta_position();
        
        // Capture arrow key turning (fallback)
        if is_key_down(KeyCode::Left) { input.turn_delta -= 1.0; }
        if is_key_down(KeyCode::Right) { input.turn_delta += 1.0; }
        
        input
    }
    
    /// Update input settings
    pub fn set_mouse_sensitivity(&mut self, sensitivity: f32) {
        self.mouse_sensitivity = sensitivity;
    }
    
    pub fn set_move_speed(&mut self, speed: f32) {
        self.move_speed = speed;
    }
    
    pub fn set_turn_speed(&mut self, speed: f32) {
        self.turn_speed = speed;
    }
}

impl PlayerInput {
    /// Check if any movement input is active
    pub fn has_movement(&self) -> bool {
        self.forward_move.abs() > 0.001 || self.strafe_move.abs() > 0.001
    }
    
    /// Check if any look input is active
    pub fn has_look_input(&self) -> bool {
        self.mouse_delta.x.abs() > 0.001 || 
        self.mouse_delta.y.abs() > 0.001 || 
        self.turn_delta.abs() > 0.001
    }
    
    /// Get debug string for current input
    pub fn debug_string(&self) -> String {
        if self.jump_pressed {
            "JUMP!".to_string()
        } else if self.forward_move > 0.0 {
            "W (Forward)".to_string()
        } else if self.forward_move < 0.0 {
            "S (Backward)".to_string()
        } else if self.strafe_move < 0.0 {
            "A (Strafe Left)".to_string()
        } else if self.strafe_move > 0.0 {
            "D (Strafe Right)".to_string()
        } else if self.has_look_input() {
            format!("Mouse Look (dx:{:.2}, dy:{:.2})", self.mouse_delta.x, self.mouse_delta.y)
        } else if self.turn_delta < 0.0 {
            "← (Turn Left)".to_string()
        } else if self.turn_delta > 0.0 {
            "→ (Turn Right)".to_string()
        } else {
            "None".to_string()
        }
    }
} 