use macroquad::prelude::*;
use super::map::Map;

/// Player entity with movement and collision
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub speed: f32,
    pub turn_speed: f32,
    pub radius: f32,
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
            rotation: 0.0,
            speed: 2.0,
            turn_speed: 3.0,
            radius: 0.3,
            last_input: "None".to_string(),
            collision_detected: false,
        }
    }
    
    /// Update player position and rotation based on input
    pub fn update(&mut self, dt: f32, map: &Map) {
        let mut move_direction = 0.0;
        let mut turn_direction = 0.0;
        
        // Reset debug info
        self.last_input = "None".to_string();
        self.collision_detected = false;
        
        // WASD movement controls with debug tracking
        if is_key_down(KeyCode::W) {
            move_direction += 1.0;
            self.last_input = "W (Forward)".to_string();
        }
        if is_key_down(KeyCode::S) {
            move_direction -= 1.0;
            self.last_input = "S (Backward)".to_string();
        }
        if is_key_down(KeyCode::A) {
            turn_direction -= 1.0;
            self.last_input = "A (Turn Left)".to_string();
        }
        if is_key_down(KeyCode::D) {
            turn_direction += 1.0;
            self.last_input = "D (Turn Right)".to_string();
        }
        
        // Apply rotation
        self.rotation += turn_direction * self.turn_speed * dt;
        
        // Apply movement with collision detection
        if move_direction != 0.0 {
            let move_distance = move_direction * self.speed * dt;
            let new_x = self.x + move_distance * self.rotation.cos();
            let new_y = self.y + move_distance * self.rotation.sin();
            
            // Simple collision check - just check the center point for now
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