use macroquad::prelude::*;
use std::time::Instant;
use super::{map::Map, player::Player, raycast::RaycastRenderer};

/// Overall game state for testing and gameplay
pub struct GameState {
    pub map: Map,
    pub player: Player,
    pub frame_count: u32,
    pub start_time: Instant,
    pub raycast_renderer: RaycastRenderer,
    pub view_mode_3d: bool, // Toggle between 2D and 3D view
}

impl GameState {
    /// Create a new game state
    pub fn new() -> Self {
        GameState {
            map: Map::new(),
            player: Player::new(5.0, 5.0),
            frame_count: 0,
            start_time: Instant::now(),
            raycast_renderer: RaycastRenderer::new(),
            view_mode_3d: true, // Start in 3D mode by default
        }
    }
    
    /// Update the game state
    pub fn update(&mut self, dt: f32) {
        self.player.update(dt, &self.map);
        self.frame_count += 1;
        
        // Toggle between 2D and 3D view with TAB key
        if is_key_pressed(KeyCode::Tab) {
            self.view_mode_3d = !self.view_mode_3d;
        }
    }
    
    /// Draw the game state
    pub fn draw(&mut self) {
        if self.view_mode_3d {
            // Draw 3D raycasted view
            self.raycast_renderer.render_3d_view(&self.map, &self.player);
            
            // Draw 3D UI overlay
            draw_text("WOLFENSTEIN BY AI - 3D Mode", 20.0, 20.0, 20.0, GREEN);
            draw_text(&format!("FPS: {:.0} | Pos: ({:.1}, {:.1}) | Rot: {:.1}°", 
                get_fps(), self.player.x, self.player.y, self.player.rotation.to_degrees()), 
                20.0, screen_height() - 40.0, 16.0, WHITE);
            draw_text("WASD: Move/Turn | TAB: 2D View | ESC: Exit", 20.0, screen_height() - 20.0, 16.0, GRAY);
        } else {
            // Draw 2D top-down view (original)
            clear_background(BLACK);
            
            let tile_size = 50.0;
            let offset_x = 50.0;
            let offset_y = 50.0;
            
            // Draw map
            for y in 0..self.map.height {
                for x in 0..self.map.width {
                    let screen_x = offset_x + x as f32 * tile_size;
                    let screen_y = offset_y + y as f32 * tile_size;
                    let color = if self.map.tiles[y][x] == 1 { WHITE } else { DARKGRAY };
                    draw_rectangle(screen_x, screen_y, tile_size, tile_size, color);
                }
            }
            
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
            draw_text("WASD: Move/Turn | TAB: 3D View | ESC: Exit", 20.0, screen_height() - 20.0, 16.0, GRAY);
        }
    }
} 