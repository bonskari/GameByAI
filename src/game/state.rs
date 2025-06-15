use macroquad::prelude::*;
use std::time::Instant;
use super::{map::Map, player::Player, input::{InputHandler, PlayerInput}};
use super::rendering::Modern3DRenderer;
use super::ecs_state::EcsGameState;

/// Overall game state for testing and gameplay
pub struct GameState {
    pub map: Map,
    pub frame_count: u32,
    pub start_time: Instant,
    pub modern_3d_renderer: Modern3DRenderer,
    pub view_mode_3d: bool, // Toggle between 2D and 3D view

    // ECS system
    pub ecs_state: EcsGameState,
    // Centralized input handling
    pub input_handler: InputHandler,
}

impl GameState {
    /// Create a new game state
    pub fn new() -> Self {
        GameState {
            map: Map::new(),
            frame_count: 0,
            start_time: Instant::now(),
            modern_3d_renderer: Modern3DRenderer::new(),
            view_mode_3d: true, // Start in 3D mode by default

            // ECS system
            ecs_state: EcsGameState::new(),
            // Centralized input handling
            input_handler: InputHandler::new(),
        }
    }
    
    /// Update the game state
    pub fn update(&mut self, delta_time: f32) {
        self.frame_count += 1;
        
        // Update ECS state first
        self.ecs_state.update(delta_time);
        
        // Legacy player sync is no longer needed - pure ECS now
        
        // Toggle between 2D and 3D view with TAB key
        if is_key_pressed(KeyCode::Tab) {
            self.view_mode_3d = !self.view_mode_3d;
        }
    }
    
    /// Draw the game state
    pub fn draw(&mut self) {
        // Get current player data for rendering
        let current_player = self.get_current_player_data();
        
        if self.view_mode_3d {
            // Update camera and render ECS entities
            self.modern_3d_renderer.update_camera(&current_player);
            self.modern_3d_renderer.render_ecs_entities(&self.ecs_state.world);
            
            // Draw minimap in top-right corner during 3D mode
            self.draw_minimap(&current_player);
            
            // Draw 3D UI overlay
            draw_text("GAMEBYAI - 3D MODE (ECS)", 20.0, 20.0, 20.0, GREEN);
            draw_text(&format!("FPS: {:.0} | Pos: ({:.1}, {:.1}, {:.1}) | Yaw: {:.1}° | Pitch: {:.1}° | Ground: {}", 
                get_fps(), current_player.x, current_player.y, current_player.z, 
                current_player.rotation.to_degrees(), current_player.pitch.to_degrees(),
                if current_player.is_grounded { "✓" } else { "✗" }), 
                20.0, screen_height() - 80.0, 16.0, WHITE);
            draw_text("🚀 TEXTURED 3D RENDERING", 20.0, screen_height() - 60.0, 16.0, GOLD);
            draw_text("System: ECS", 20.0, screen_height() - 40.0, 16.0, BLUE);
            draw_text("WASD: Move/Strafe | Mouse: Look | SPACE: Jump | M: Toggle Mouse | TAB: 2D View | ESC: Exit", 20.0, screen_height() - 20.0, 16.0, GRAY);
        } else {
            // Draw 2D top-down view (original)
            clear_background(BLACK);
            
            let tile_size = 50.0;
            let offset_x = 50.0;
            let offset_y = 50.0;
            
            // Draw map with texture colors
            self.map.draw_topdown(offset_x, offset_y, tile_size);
            
            // Draw player
            let player_x = offset_x + current_player.x * tile_size;
            let player_y = offset_y + current_player.y * tile_size;
            draw_circle(player_x, player_y, 10.0, YELLOW);
            
            // Draw direction
            let dir_x = player_x + 20.0 * current_player.rotation.cos();
            let dir_y = player_y + 20.0 * current_player.rotation.sin();
            draw_line(player_x, player_y, dir_x, dir_y, 3.0, RED);
            
            // Draw 2D UI
            draw_text("GAMEBYAI - 2D Map View (ECS)", 20.0, 20.0, 20.0, GREEN);
            draw_text(&format!("Frame: {} | FPS: {:.0} | System: ECS", self.frame_count, get_fps() as i32), 20.0, screen_height() - 60.0, 16.0, WHITE);
            draw_text("WASD: Move/Strafe | Mouse: Look | SPACE: Jump | M: Toggle Mouse | TAB: 3D View | ESC: Exit", 20.0, screen_height() - 20.0, 16.0, GRAY);
        }
    }
    
    /// Draw a minimap in the top-right corner during 3D mode
    pub fn draw_minimap(&self, current_player: &Player) {
        let minimap_size = 150.0;
        let minimap_x = screen_width() - minimap_size - 10.0;
        let minimap_y = 10.0;
        let tile_size = minimap_size / self.map.width.max(self.map.height) as f32;
        
        // Draw minimap background with border
        draw_rectangle(minimap_x - 2.0, minimap_y - 2.0, minimap_size + 4.0, minimap_size + 4.0, WHITE);
        draw_rectangle(minimap_x, minimap_y, minimap_size, minimap_size, BLACK);
        
        // Draw map tiles with texture colors
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                let screen_x = minimap_x + x as f32 * tile_size;
                let screen_y = minimap_y + y as f32 * tile_size;
                
                let wall_type = self.map.get_wall_type(x as i32, y as i32);
                let color = if wall_type == super::map::WallType::Empty {
                    Color::new(0.2, 0.2, 0.2, 1.0)  // Dark gray empty space
                } else {
                    // Use texture color but dimmed for minimap
                    let texture_color = self.map.get_wall_color(wall_type, true);
                    Color::new(
                        texture_color.r * 0.8,
                        texture_color.g * 0.8,
                        texture_color.b * 0.8,
                        1.0
                    )
                };
                
                draw_rectangle(screen_x, screen_y, tile_size, tile_size, color);
            }
        }
        
        // Draw pathfinding visualization if ECS test bot is active
        if self.ecs_state.has_test_bot() {
            // For now, just draw a simple indicator that test bot is active
            // TODO: Add pathfinding visualization to ECS test bot
            draw_text("🤖 TEST BOT ACTIVE", minimap_x, minimap_y + minimap_size + 15.0, 12.0, YELLOW);
        }
        
        // Draw player position and direction
        let player_screen_x = minimap_x + current_player.x * tile_size;
        let player_screen_y = minimap_y + current_player.y * tile_size;
        
        // Player dot
        draw_circle(player_screen_x, player_screen_y, tile_size * 0.25, GREEN);
        
        // Player direction indicator
        let dir_length = tile_size * 0.4;
        let dir_end_x = player_screen_x + dir_length * current_player.rotation.cos();
        let dir_end_y = player_screen_y + dir_length * current_player.rotation.sin();
        draw_line(player_screen_x, player_screen_y, dir_end_x, dir_end_y, 2.0, GREEN);
        
        // Minimap label
        draw_text("MINIMAP", minimap_x, minimap_y - 5.0, 12.0, WHITE);
    }
    
    /// Get current player data from ECS
    fn get_current_player_data(&self) -> Player {
        // Convert ECS data to Player format for rendering compatibility
        if let Some(legacy_data) = self.ecs_state.get_legacy_player_data() {
            Player {
                x: legacy_data.x,
                y: legacy_data.y,
                z: legacy_data.z,
                rotation: legacy_data.rotation,
                pitch: legacy_data.pitch,
                speed: 2.0,
                turn_speed: 3.0,
                radius: 0.3,
                mouse_sensitivity: 0.18,
                vertical_velocity: 0.0,
                jump_strength: 4.5,
                gravity: 12.0,
                ground_height: 0.6,
                is_grounded: legacy_data.is_grounded,
                last_input: "ECS System".to_string(),
                collision_detected: false,
            }
        } else {
            // Fallback default player if ECS data is not available
            Player::new(1.5, 1.5)
        }
    }
} 