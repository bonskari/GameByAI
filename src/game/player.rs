use macroquad::prelude::*;
use super::map::Map;

/// Player entity with movement and collision
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
    
    /// Update player position and rotation based on input
    pub fn update(&mut self, dt: f32, map: &Map) {
        let mut forward_move = 0.0;
        let mut strafe_move = 0.0;
        let mut turn_direction = 0.0;
        
        // Reset debug info
        self.last_input = "None".to_string();
        self.collision_detected = false;
        
        // WASD movement controls - modern FPS style
        if is_key_down(KeyCode::W) {
            forward_move += 1.0;
            self.last_input = "W (Forward)".to_string();
        }
        if is_key_down(KeyCode::S) {
            forward_move -= 1.0;
            self.last_input = "S (Backward)".to_string();
        }
        if is_key_down(KeyCode::A) {
            strafe_move -= 1.0;
            self.last_input = "A (Strafe Left)".to_string();
        }
        if is_key_down(KeyCode::D) {
            strafe_move += 1.0;
            self.last_input = "D (Strafe Right)".to_string();
        }
        
        // Jumping with Spacebar - only if grounded
        if is_key_pressed(KeyCode::Space) && self.is_grounded {
            self.vertical_velocity = self.jump_strength;
            self.is_grounded = false;
            self.last_input = "JUMP!".to_string();
        }
        
        // Arrow keys for turning (backup controls)
        if is_key_down(KeyCode::Left) {
            turn_direction -= 1.0;
            self.last_input = "← (Turn Left)".to_string();
        }
        if is_key_down(KeyCode::Right) {
            turn_direction += 1.0;
            self.last_input = "→ (Turn Right)".to_string();
        }
        
        // Mouse look - get mouse delta and apply rotation/pitch
        let mouse_delta = mouse_delta_position();
        
        // Apply mouse movement even for very small deltas to ensure smooth diagonal movement
        if mouse_delta.x.abs() > 0.001 || mouse_delta.y.abs() > 0.001 {
            // Horizontal mouse movement controls yaw (left/right rotation)
            self.rotation -= mouse_delta.x * self.mouse_sensitivity;
            
            // Vertical mouse movement controls pitch (up/down look)  
            self.pitch += mouse_delta.y * self.mouse_sensitivity;
            
            // Clamp pitch to prevent over-rotation (roughly -85 to +85 degrees)
            let max_pitch = std::f32::consts::PI * 0.47; // ~85 degrees
            self.pitch = self.pitch.clamp(-max_pitch, max_pitch);
            
            // Debug output for significant mouse movement
            if mouse_delta.x.abs() > 0.5 || mouse_delta.y.abs() > 0.5 {
                self.last_input = format!("Mouse Look (dx:{:.2}, dy:{:.2})", mouse_delta.x, mouse_delta.y);
            }
        }
        
        // Apply keyboard rotation (fallback)
        self.rotation += turn_direction * self.turn_speed * dt;
        
        // Mouse sensitivity is now fixed at optimal value (0.18)
        
        // Apply forward/backward movement
        if forward_move != 0.0 {
            let move_distance = forward_move * self.speed * dt;
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
        if strafe_move != 0.0 {
            let strafe_distance = strafe_move * self.speed * dt;
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