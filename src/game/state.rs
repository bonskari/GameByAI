use macroquad::prelude::*;
use std::time::Instant;
use super::{map::Map, player::Player};
use super::rendering::Modern3DRenderer;
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
        }
    }
    
    /// Update the game state
    pub fn update(&mut self, delta_time: f32) {
        self.frame_count += 1;
        self.player.update(delta_time, &self.map);
        
        // Update test bot if present
        if let Some(test_bot) = &mut self.test_bot {
            test_bot.update(&mut self.player, &self.map, delta_time);
        }
        
        // Toggle between 2D and 3D view with TAB key
        if is_key_pressed(KeyCode::Tab) {
            self.view_mode_3d = !self.view_mode_3d;
        }
    }
    
    /// Draw the game state
    pub fn draw(&mut self) {
        if self.view_mode_3d {
            // Draw modern 3D view with GPU acceleration
            self.modern_3d_renderer.render(&self.map, &self.player);
            
            // Draw minimap in top-right corner during 3D mode
            self.draw_minimap();
            
            // Draw 3D UI overlay
            draw_text("WOLFENSTEIN BY AI - 3D MODE", 20.0, 20.0, 20.0, GREEN);
            draw_text(&format!("FPS: {:.0} | Pos: ({:.1}, {:.1}, {:.1}) | Yaw: {:.1}° | Pitch: {:.1}° | Ground: {}", 
                get_fps(), self.player.x, self.player.y, self.player.z, 
                self.player.rotation.to_degrees(), self.player.pitch.to_degrees(),
                if self.player.is_grounded { "✓" } else { "✗" }), 
                20.0, screen_height() - 60.0, 16.0, WHITE);
            draw_text("🚀 TEXTURED 3D RENDERING", 20.0, screen_height() - 40.0, 16.0, GOLD);
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
            let player_x = offset_x + self.player.x * tile_size;
            let player_y = offset_y + self.player.y * tile_size;
            draw_circle(player_x, player_y, 10.0, YELLOW);
            
            // Draw direction
            let dir_x = player_x + 20.0 * self.player.rotation.cos();
            let dir_y = player_y + 20.0 * self.player.rotation.sin();
            draw_line(player_x, player_y, dir_x, dir_y, 3.0, RED);
            
            // Draw 2D UI
            draw_text("WOLFENSTEIN BY AI - 2D Map View", 20.0, 20.0, 20.0, GREEN);
            draw_text(&format!("Frame: {} | FPS: {:.0}", self.frame_count, get_fps() as i32), 20.0, screen_height() - 40.0, 16.0, WHITE);
            draw_text("WASD: Move/Strafe | Mouse: Look | SPACE: Jump | M: Toggle Mouse | TAB: 3D View | ESC: Exit", 20.0, screen_height() - 20.0, 16.0, GRAY);
        }
    }
    
    /// Draw a minimap in the top-right corner during 3D mode
    pub fn draw_minimap(&self) {
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
        let player_screen_x = minimap_x + self.player.x * tile_size;
        let player_screen_y = minimap_y + self.player.y * tile_size;
        
        // Player dot
        draw_circle(player_screen_x, player_screen_y, tile_size * 0.25, GREEN);
        
        // Player direction indicator
        let dir_length = tile_size * 0.4;
        let dir_end_x = player_screen_x + dir_length * self.player.rotation.cos();
        let dir_end_y = player_screen_y + dir_length * self.player.rotation.sin();
        draw_line(player_screen_x, player_screen_y, dir_end_x, dir_end_y, 2.0, GREEN);
        
        // Minimap label
        draw_text("MINIMAP", minimap_x, minimap_y - 5.0, 12.0, WHITE);
    }
} 