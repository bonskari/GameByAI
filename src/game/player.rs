use macroquad::prelude::*;
use super::map::Map;
use super::input::PlayerInput;

/// Player entity with movement and collision
#[derive(Clone)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub z: f32,           // Height/vertical position
    pub rotation: f32,    // Yaw (left/right look)
    pub pitch: f32,       // Pitch (up/down look)
    pub speed: f32,
    pub turn_speed: f32,
    pub radius: f32,
    pub mouse_sensitivity: f32, // Adjustable mouse sensitivity
    // Jumping physics
    pub vertical_velocity: f32, // Current vertical velocity
    pub jump_strength: f32,     // How strong the jump is
    pub gravity: f32,           // Downward acceleration
    pub ground_height: f32,     // Height of the ground
    pub is_grounded: bool,      // Whether player is on ground
    // Debug info
    pub last_input: String,
    pub collision_detected: bool,
}

impl Player {
    /// Create a new player at the given position
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            z: 0.6,           // Eye height above ground (typical FPS height)
            rotation: 0.0,
            pitch: 0.0,       // Start looking straight ahead
            speed: 2.0,
            turn_speed: 3.0,
            radius: 0.3,
            mouse_sensitivity: 0.18, // Original responsive sensitivity restored  
            // Jumping physics
            vertical_velocity: 0.0,
            jump_strength: 4.5,    // Strong enough jump for responsive feel
            gravity: 12.0,         // Realistic gravity
            ground_height: 0.6,    // Eye height when standing
            is_grounded: true,     // Start on ground
            last_input: "None".to_string(),
            collision_detected: false,
        }
    }
    
    /// Update player position and rotation based on centralized input
    pub fn update_with_input(&mut self, dt: f32, map: &Map, input: &PlayerInput) {
        // Reset debug info
        self.last_input = input.debug_string();
        self.collision_detected = false;
        
        // Apply mouse look
        if input.has_look_input() {
            // Horizontal mouse movement controls yaw (left/right rotation)
            self.rotation -= input.mouse_delta.x * input.mouse_sensitivity;
            
            // Vertical mouse movement controls pitch (up/down look)  
            self.pitch += input.mouse_delta.y * input.mouse_sensitivity;
            
            // Clamp pitch to prevent over-rotation (roughly -85 to +85 degrees)
            let max_pitch = std::f32::consts::PI * 0.47; // ~85 degrees
            self.pitch = self.pitch.clamp(-max_pitch, max_pitch);
        }
        
        // Apply keyboard rotation (fallback)
        self.rotation += input.turn_delta * input.turn_speed * dt;
        
        // Apply forward/backward movement
        if input.forward_move != 0.0 {
            let move_distance = input.forward_move * input.move_speed * dt;
            let new_x = self.x + move_distance * self.rotation.cos();
            let new_y = self.y + move_distance * self.rotation.sin();
            
            // Collision check for forward/backward movement
            let collision_x = map.is_wall(new_x as i32, self.y as i32);
            let collision_y = map.is_wall(self.x as i32, new_y as i32);
            
            if !collision_x {
                self.x = new_x;
            } else {
                self.collision_detected = true;
            }
            
            if !collision_y {
                self.y = new_y;
            } else {
                self.collision_detected = true;
            }
        }
        
        // Apply strafe movement (perpendicular to facing direction)
        if input.strafe_move != 0.0 {
            let strafe_distance = input.strafe_move * input.move_speed * dt;
            // Strafe direction is 90 degrees from facing direction
            let strafe_angle = self.rotation + std::f32::consts::PI / 2.0;
            let new_x = self.x + strafe_distance * strafe_angle.cos();
            let new_y = self.y + strafe_distance * strafe_angle.sin();
            
            // Collision check for strafe movement
            let collision_x = map.is_wall(new_x as i32, self.y as i32);
            let collision_y = map.is_wall(self.x as i32, new_y as i32);
            
            if !collision_x {
                self.x = new_x;
            } else {
                self.collision_detected = true;
            }
            
            if !collision_y {
                self.y = new_y;
            } else {
                self.collision_detected = true;
            }
        }
        
        // Jumping with Spacebar - only if grounded
        if input.jump_pressed && self.is_grounded {
            self.vertical_velocity = self.jump_strength;
            self.is_grounded = false;
        }
        
        // Apply gravity and vertical movement
        if !self.is_grounded {
            // Apply gravity (downward acceleration)
            self.vertical_velocity -= self.gravity * dt;
            
            // Update vertical position
            self.z += self.vertical_velocity * dt;
            
            // Check if we've landed
            if self.z <= self.ground_height {
                self.z = self.ground_height;
                self.vertical_velocity = 0.0;
                self.is_grounded = true;
            }
        }
    }
    
    /// Update player position and rotation based on input (legacy method)
    /// This method is kept for compatibility but now delegates to update_with_input
    pub fn update(&mut self, dt: f32, map: &Map) {
        // Create a temporary input handler to capture input for legacy compatibility
        let input_handler = super::input::InputHandler::new();
        let input = input_handler.capture_input();
        self.update_with_input(dt, map, &input);
    }
    
    /// Draw the player in top-down view
    pub fn draw_topdown(&self, offset_x: f32, offset_y: f32, tile_size: f32) {
        let screen_x = offset_x + self.x * tile_size;
        let screen_y = offset_y + self.y * tile_size;
        
        // Draw player as yellow circle (larger for visibility)
        draw_circle(screen_x, screen_y, 6.0, YELLOW);
        
        // Draw direction indicator (longer line)
        let direction_length = 12.0;
        let end_x = screen_x + direction_length * self.rotation.cos();
        let end_y = screen_y + direction_length * self.rotation.sin();
        draw_line(screen_x, screen_y, end_x, end_y, 3.0, RED);
        
        // Draw collision radius for debugging
        draw_circle_lines(screen_x, screen_y, self.radius * tile_size, 1.0, 
                         if self.collision_detected { RED } else { GREEN });
    }
} 