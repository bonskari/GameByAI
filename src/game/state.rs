use macroquad::prelude::*;
use std::time::Instant;
use super::{map::Map, player::Player, input::{InputHandler, PlayerInput}};
use super::rendering::Modern3DRenderer;
use super::ecs_state::EcsGameState;
use crate::testing::visual_tests::VisualTestBot;

/// Overall game state for testing and gameplay
pub struct GameState {
    pub map: Map,
    pub player: Player,
    pub frame_count: u32,
    pub start_time: Instant,
    pub modern_3d_renderer: Modern3DRenderer,
    pub view_mode_3d: bool, // Toggle between 2D and 3D view
    pub test_bot: Option<VisualTestBot>,
    // ECS integration
    pub ecs_state: EcsGameState,
    pub use_ecs: bool, // Toggle between legacy and ECS systems
    // Centralized input handling
    pub input_handler: InputHandler,
}

impl GameState {
    /// Create a new game state
    pub fn new() -> Self {
        GameState {
            map: Map::new(),
            player: Player::new(1.5, 1.5), // Start in open area near corner
            frame_count: 0,
            start_time: Instant::now(),
            modern_3d_renderer: Modern3DRenderer::new(),
            view_mode_3d: true, // Start in 3D mode by default
            test_bot: None,
            // ECS integration
            ecs_state: EcsGameState::new(),
            use_ecs: true, // Start with ECS enabled
            // Centralized input handling
            input_handler: InputHandler::new(),
        }
    }
    
    /// Update the game state
    pub fn update(&mut self, delta_time: f32) {
        self.frame_count += 1;
        
        // Capture input once per frame
        let player_input = self.input_handler.capture_input();
        
        // Toggle between legacy and ECS systems with E key
        if is_key_pressed(KeyCode::E) {
            self.use_ecs = !self.use_ecs;
            println!("Switched to {} player system", if self.use_ecs { "ECS" } else { "Legacy" });
            
            // Sync player state when switching systems
            if self.use_ecs {
                // Copy legacy player state to ECS
                self.sync_legacy_to_ecs();
            } else {
                // Copy ECS state to legacy player
                self.sync_ecs_to_legacy();
            }
        }
        
        if self.use_ecs {
            // Update ECS systems with centralized input
            self.ecs_state.update_with_input(delta_time, &player_input);
        } else {
            // Update legacy player system with centralized input
            self.player.update_with_input(delta_time, &self.map, &player_input);
        }
        
        // Update test bot if present (always uses legacy player for now)
        if let Some(test_bot) = &mut self.test_bot {
            let mut current_player = if self.use_ecs {
                // Convert ECS player data to legacy format for test bot
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
                    self.player.clone()
                }
            } else {
                self.player.clone()
            };
            
            test_bot.update(&mut current_player, &self.map, delta_time);
            
            // If using ECS, we need to sync the test bot changes back
            if self.use_ecs {
                self.sync_test_bot_to_ecs(&current_player);
            } else {
                self.player = current_player;
            }
        }
        
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
            // Draw modern 3D view with GPU acceleration
            self.modern_3d_renderer.render(&self.map, &current_player);
            
            // Draw minimap in top-right corner during 3D mode
            self.draw_minimap(&current_player);
            
            // Draw 3D UI overlay
            let system_name = if self.use_ecs { "ECS" } else { "Legacy" };
            draw_text(&format!("GAMEBYAI - 3D MODE ({})", system_name), 20.0, 20.0, 20.0, GREEN);
            draw_text(&format!("FPS: {:.0} | Pos: ({:.1}, {:.1}, {:.1}) | Yaw: {:.1}° | Pitch: {:.1}° | Ground: {}", 
                get_fps(), current_player.x, current_player.y, current_player.z, 
                current_player.rotation.to_degrees(), current_player.pitch.to_degrees(),
                if current_player.is_grounded { "✓" } else { "✗" }), 
                20.0, screen_height() - 80.0, 16.0, WHITE);
            draw_text("🚀 TEXTURED 3D RENDERING", 20.0, screen_height() - 60.0, 16.0, GOLD);
            draw_text(&format!("System: {} | E: Toggle System", system_name), 20.0, screen_height() - 40.0, 16.0, BLUE);
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
            let system_name = if self.use_ecs { "ECS" } else { "Legacy" };
            draw_text(&format!("GAMEBYAI - 2D Map View ({})", system_name), 20.0, 20.0, 20.0, GREEN);
            draw_text(&format!("Frame: {} | FPS: {:.0} | System: {}", self.frame_count, get_fps() as i32, system_name), 20.0, screen_height() - 60.0, 16.0, WHITE);
            draw_text("E: Toggle System | WASD: Move/Strafe | Mouse: Look | SPACE: Jump | M: Toggle Mouse | TAB: 3D View | ESC: Exit", 20.0, screen_height() - 20.0, 16.0, GRAY);
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
        
        // Draw pathfinding visualization if test bot is active
        if let Some(test_bot) = &self.test_bot {
            // Draw explored nodes (A* search area) in blue
            for &(x, y) in &test_bot.explored_nodes {
                let screen_x = minimap_x + x as f32 * tile_size;
                let screen_y = minimap_y + y as f32 * tile_size;
                draw_rectangle(screen_x, screen_y, tile_size, tile_size, Color::new(0.3, 0.5, 1.0, 0.4));
            }
            
            // Draw path nodes (actual route) in red
            for &(x, y) in &test_bot.path_nodes {
                let screen_x = minimap_x + x as f32 * tile_size;
                let screen_y = minimap_y + y as f32 * tile_size;
                draw_rectangle(screen_x, screen_y, tile_size, tile_size, Color::new(1.0, 0.3, 0.3, 0.7));
            }
            
            // Draw current waypoint target in yellow
            if test_bot.current_waypoint < test_bot.waypoints.len() {
                let waypoint = &test_bot.waypoints[test_bot.current_waypoint];
                let target_x = minimap_x + (waypoint.x - 0.5) * tile_size;
                let target_y = minimap_y + (waypoint.y - 0.5) * tile_size;
                draw_circle(target_x + tile_size * 0.5, target_y + tile_size * 0.5, tile_size * 0.3, YELLOW);
            }
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
    
    /// Get current player data regardless of which system is active
    fn get_current_player_data(&self) -> Player {
        if self.use_ecs {
            // Convert ECS data to legacy Player format
            if let Some(legacy_data) = self.ecs_state.get_legacy_player_data() {
                Player {
                    x: legacy_data.x,
                    y: legacy_data.y,
                    z: legacy_data.z,
                    rotation: legacy_data.rotation,
                    pitch: legacy_data.pitch,
                    speed: 2.0, // Use default values for missing fields
                    turn_speed: 3.0,
                    radius: 0.3,
                    mouse_sensitivity: 0.18,
                    vertical_velocity: 0.0, // Default value
                    jump_strength: 4.5,
                    gravity: 12.0,
                    ground_height: 0.6,
                    is_grounded: legacy_data.is_grounded,
                    last_input: "ECS System".to_string(),
                    collision_detected: false,
                }
            } else {
                // Fallback to legacy player if ECS data is not available
                self.player.clone()
            }
        } else {
            self.player.clone()
        }
    }
    
    /// Sync legacy player state to ECS
    fn sync_legacy_to_ecs(&mut self) {
        // This would update the ECS player entity with legacy player data
        // For now, we'll just print a message since the ECS system starts fresh
        println!("Syncing legacy player to ECS: pos({:.1}, {:.1}, {:.1}), rot: {:.1}°", 
                 self.player.x, self.player.y, self.player.z, self.player.rotation.to_degrees());
        // TODO: Implement actual sync when ECS supports direct entity updates
    }
    
    /// Sync ECS state to legacy player
    fn sync_ecs_to_legacy(&mut self) {
        if let Some(legacy_data) = self.ecs_state.get_legacy_player_data() {
            self.player.x = legacy_data.x;
            self.player.y = legacy_data.y;
            self.player.z = legacy_data.z;
            self.player.rotation = legacy_data.rotation;
            self.player.pitch = legacy_data.pitch;
            self.player.is_grounded = legacy_data.is_grounded;
            // Keep existing values for fields not in LegacyPlayerData
            println!("Synced ECS to legacy player: pos({:.1}, {:.1}, {:.1}), rot: {:.1}°", 
                     self.player.x, self.player.y, self.player.z, self.player.rotation.to_degrees());
        }
    }
    
    /// Sync test bot changes back to ECS (placeholder for now)
    fn sync_test_bot_to_ecs(&mut self, _updated_player: &Player) {
        // TODO: Implement syncing test bot changes back to ECS
        // This would update the ECS player entity with the test bot's modifications
    }
} 