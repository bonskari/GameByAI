use macroquad::prelude::*;

/// Map system - grid-based like classic Wolfenstein
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<u8>>, // 0 = empty, 1 = wall
}

impl Map {
    /// Create a new test map
    pub fn new() -> Self {
        // Create a simple test map for debugging
        let map_data = vec![
            vec![1,1,1,1,1,1,1,1,1,1],
            vec![1,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,0,0,0,0,0,0,1],
            vec![1,0,0,0,0,0,0,0,0,1],
            vec![1,1,1,1,1,1,1,1,1,1],
        ];
        
        Map {
            width: 10,
            height: 10,
            tiles: map_data,
        }
    }
    
    /// Check if a position contains a wall
    pub fn is_wall(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return true; // Out of bounds = wall
        }
        self.tiles[y as usize][x as usize] == 1
    }
    
    /// Draw the map in top-down view
    pub fn draw_topdown(&self, offset_x: f32, offset_y: f32, tile_size: f32) {
        for y in 0..self.height {
            for x in 0..self.width {
                let screen_x = offset_x + x as f32 * tile_size;
                let screen_y = offset_y + y as f32 * tile_size;
                
                let color = if self.tiles[y][x] == 1 {
                    WHITE  // Wall
                } else {
                    DARKGRAY   // Empty space (darker for contrast)
                };
                
                draw_rectangle(screen_x, screen_y, tile_size, tile_size, color);
                
                // Draw grid lines
                draw_rectangle_lines(screen_x, screen_y, tile_size, tile_size, 1.0, GRAY);
            }
        }
    }
} 