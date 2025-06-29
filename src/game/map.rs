use macroquad::prelude::*;

/// Map system - grid-based like classic first-person games
#[derive(Debug, Clone)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<u8>>, // 0 = empty, 1+ = different wall types
    // World coordinate transformation
    pub world_min_x: f32,
    pub world_min_z: f32,
    pub world_max_x: f32,
    pub world_max_z: f32,
}

/// Wall texture types for sci-fi space station
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Wall(WallType),
}

/// Represents the type of a wall, which determines its texture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WallType {
    Empty = 0,
    TechPanel = 1,     // Advanced tech panels with circuits and displays
    HullPlating = 2,   // Reinforced hull plating with battle damage
    ControlSystem = 3,   // Control panels with holographic displays
    EnergyConduit = 4,   // Glowing energy conduit systems
}

impl Map {
    /// Create a new test map with different wall textures
    pub fn new() -> Self {
        // Create a more interesting map with different wall types
        let map_data = vec![
            vec![1,1,1,1,2,2,1,1,1,1],  // Stone and brick outer walls
            vec![1,0,0,0,0,0,0,0,0,2],  
            vec![1,0,3,3,0,0,4,4,0,2],  // Metal and wood inner walls
            vec![1,0,3,0,0,0,0,4,0,2],
            vec![2,0,0,0,0,0,0,0,0,1],  // Stone corridor - removed middle pillars
            vec![2,0,0,0,0,0,0,0,0,1],  // Stone corridor - removed middle pillars
            vec![1,0,4,0,0,0,0,3,0,2],
            vec![1,0,4,4,0,0,3,3,0,2],
            vec![1,0,0,0,0,0,0,0,0,2],
            vec![1,1,1,2,2,2,2,1,1,1],  // Mixed outer walls
        ];
        
        Map {
            width: 10,
            height: 10,
            tiles: map_data,
            world_min_x: 0.0,
            world_min_z: 0.0,
            world_max_x: 10.0,
            world_max_z: 10.0,
        }
    }
    
    /// Convert world coordinates to grid coordinates
    pub fn world_to_grid(&self, world_x: f32, world_z: f32) -> (i32, i32) {
        let grid_x = ((world_x - self.world_min_x) / (self.world_max_x - self.world_min_x) * self.width as f32).floor() as i32;
        let grid_z = ((world_z - self.world_min_z) / (self.world_max_z - self.world_min_z) * self.height as f32).floor() as i32;
        (grid_x, grid_z)
    }
    
    /// Convert grid coordinates to world coordinates (center of cell)
    pub fn grid_to_world(&self, grid_x: i32, grid_z: i32) -> (f32, f32) {
        let world_x = self.world_min_x + (grid_x as f32 + 0.5) * (self.world_max_x - self.world_min_x) / self.width as f32;
        let world_z = self.world_min_z + (grid_z as f32 + 0.5) * (self.world_max_z - self.world_min_z) / self.height as f32;
        (world_x, world_z)
    }
    
    /// Check if a world position contains a wall
    pub fn is_wall_world(&self, world_x: f32, world_z: f32) -> bool {
        let (grid_x, grid_z) = self.world_to_grid(world_x, world_z);
        self.is_wall(grid_x, grid_z)
    }
    
    /// Check if a position contains a wall
    pub fn is_wall(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return true; // Out of bounds = wall
        }
        self.tiles[y as usize][x as usize] != 0 // Any non-zero value is a wall
    }
    
    /// Get the wall type at a position
    pub fn get_wall_type(&self, x: i32, y: i32) -> WallType {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return WallType::TechPanel; // Out of bounds = default tech panel
        }
        match self.tiles[y as usize][x as usize] {
            0 => WallType::Empty,
            1 => WallType::TechPanel,
            2 => WallType::HullPlating,
            3 => WallType::ControlSystem,
            4 => WallType::EnergyConduit,
            _ => WallType::TechPanel, // Default fallback
        }
    }
    
    /// Get base color for sci-fi space station wall types
    pub fn get_wall_color(&self, wall_type: WallType, is_vertical: bool) -> Color {
        let base_color = match wall_type {
            WallType::Empty => BLACK,
            WallType::TechPanel => Color::new(0.85, 0.88, 0.92, 1.0),      // Clean tech white/silver
            WallType::HullPlating => Color::new(0.6, 0.65, 0.7, 1.0),      // Industrial hull gray
            WallType::ControlSystem => Color::new(0.2, 0.25, 0.35, 1.0),   // Dark control surface
            WallType::EnergyConduit => Color::new(0.15, 0.2, 0.25, 1.0),   // Dark conduit housing
        };
        
        // Subtle lighting variation for depth
        if is_vertical {
            base_color
        } else {
            Color::new(
                base_color.r * 0.85,
                base_color.g * 0.85, 
                base_color.b * 0.85,
                base_color.a
            )
        }
    }
    
    /// Get ultra-detailed sci-fi space station texture color based on 2D coordinates
    pub fn get_procedural_texture_color(&self, wall_type: WallType, is_vertical: bool, u: f32, v: f32) -> Color {
        let mut base_color = self.get_wall_color(wall_type, is_vertical);
        
        // Create ultra-detailed sci-fi procedural textures
        match wall_type {
            WallType::TechPanel => {
                // Advanced tech panels with circuits, LED strips, and holographic displays
                let panel_u = u; // Already scaled in renderer
                let panel_v = v; // Already scaled in renderer
                let local_u = panel_u % 1.0;
                let local_v = panel_v % 1.0;
                
                // Panel frame with beveled edges
                let frame_width = 0.06;
                let is_frame = local_u < frame_width || local_u > 1.0 - frame_width || 
                             local_v < frame_width || local_v > 1.0 - frame_width;
                
                if is_frame {
                    // Darker frame color
                    base_color.r *= 0.7;
                    base_color.g *= 0.7;
                    base_color.b *= 0.7;
                } else {
                    // Panel interior with circuit patterns
                    let circuit_pattern = ((local_u * 20.0).sin() * (local_v * 20.0).cos()).abs();
                    let led_pattern = ((local_u * 10.0).floor() as i32 + (local_v * 8.0).floor() as i32) % 2;
                    
                    if led_pattern == 0 {
                        // LED strip
                        let led_brightness = (local_u * 8.0 + local_v * 4.0).sin() * 0.3 + 0.7;
                        base_color.r = (base_color.r + led_brightness * 0.2).min(1.0);
                        base_color.g = (base_color.g + led_brightness * 0.4).min(1.0);
                        base_color.b = (base_color.b + led_brightness * 0.6).min(1.0);
                    } else {
                        // Circuit pattern
                        base_color.r = (base_color.r + circuit_pattern * 0.1).min(1.0);
                        base_color.g = (base_color.g + circuit_pattern * 0.15).min(1.0);
                        base_color.b = (base_color.b + circuit_pattern * 0.2).min(1.0);
                    }
                }
            },
            WallType::HullPlating => {
                // Heavy duty reinforced hull with battle damage and wear
                let plate_u = u; // Already scaled in renderer
                let plate_v = v; // Already scaled in renderer
                let local_u = plate_u % 1.0;
                let local_v = plate_v % 1.0;
                
                // Welded seams between plates
                let seam_width = 0.04;
                let is_seam = local_u < seam_width || local_u > 1.0 - seam_width || 
                             local_v < seam_width || local_v > 1.0 - seam_width;
                
                if is_seam {
                    // Darker seam color
                    base_color.r *= 0.5;
                    base_color.g *= 0.5;
                    base_color.b *= 0.5;
                } else {
                    // Plate surface with wear and damage
                    let wear_pattern = ((local_u * 30.0).sin() * (local_v * 30.0).cos()).abs();
                    let damage_pattern = ((local_u * 47.0).sin() * (local_v * 31.0).cos()).abs();
                    
                    base_color.r = (base_color.r - wear_pattern * 0.1 - damage_pattern * 0.05).max(0.0);
                    base_color.g = (base_color.g - wear_pattern * 0.1 - damage_pattern * 0.05).max(0.0);
                    base_color.b = (base_color.b - wear_pattern * 0.1 - damage_pattern * 0.05).max(0.0);
                }
            },
            WallType::ControlSystem => {
                // Advanced control panels with holographic displays
                let screen_u = u; // Already scaled in renderer
                let screen_v = v; // Already scaled in renderer
                let local_u = screen_u % 1.0;
                let local_v = screen_v % 1.0;
                
                // Screen bezel
                let bezel_width = 0.08;
                let is_bezel = local_u < bezel_width || local_u > 1.0 - bezel_width || 
                              local_v < bezel_width || local_v > 1.0 - bezel_width;
                
                if is_bezel {
                    // Darker bezel color
                    base_color.r *= 0.6;
                    base_color.g *= 0.6;
                    base_color.b *= 0.6;
                } else {
                    // Screen content with holographic effect
                    let hologram_pattern = ((local_u * 15.0).sin() * (local_v * 15.0).cos()).abs();
                    let data_stream = (local_u * 30.0 + local_v * 20.0).sin() * 0.5 + 0.5;
                    
                    base_color.r = (base_color.r + hologram_pattern * 0.2 + data_stream * 0.1).min(1.0);
                    base_color.g = (base_color.g + hologram_pattern * 0.3 + data_stream * 0.2).min(1.0);
                    base_color.b = (base_color.b + hologram_pattern * 0.4 + data_stream * 0.3).min(1.0);
                }
            },
            WallType::EnergyConduit => {
                // Energy conduit with glowing effects
                let conduit_u = u; // Already scaled in renderer
                let conduit_v = v; // Already scaled in renderer
                let local_u = conduit_u % 1.0;
                let local_v = conduit_v % 1.0;
                
                // Energy flow pattern
                let energy_wave = (local_v * 15.0 - local_u * 10.0).sin() * 0.5 + 0.5;
                let energy_intensity = (local_u * 20.0).sin() * 0.3 + 0.7;
                
                base_color.r = (base_color.r + energy_wave * 0.2 * energy_intensity).min(1.0);
                base_color.g = (base_color.g + energy_wave * 0.3 * energy_intensity).min(1.0);
                base_color.b = (base_color.b + energy_wave * 0.4 * energy_intensity).min(1.0);
            },
            _ => {} // No pattern for empty spaces
        }
        
        base_color
    }
    
    /// Get texture color with pattern variation based on wall offset (legacy method)
    pub fn get_textured_wall_color(&self, wall_type: WallType, is_vertical: bool, wall_x_offset: f32) -> Color {
        // Use the new procedural texture system with simplified coordinates
        self.get_procedural_texture_color(wall_type, is_vertical, wall_x_offset, 0.5)
    }
    
    /// Get pixel art style floor texture with clean, flat colors
    pub fn get_floor_texture_color(&self, x: f32, y: f32) -> Color {
        // Create pixel art style floor pattern with larger, cleaner tiles
        let tile_size = 1.0; // Smaller tiles for more detail
        let tile_x = (x / tile_size).floor() % 4.0; // 4x4 pattern
        let tile_y = (y / tile_size).floor() % 4.0;
        
        // Pixel art color palette - flat colors, no gradients
        let dark_gray = Color::new(0.2, 0.22, 0.25, 1.0);    // Main floor
        let light_gray = Color::new(0.35, 0.37, 0.4, 1.0);   // Accent tiles
        let very_dark = Color::new(0.1, 0.12, 0.15, 1.0);    // Seams/borders
        let blue_accent = Color::new(0.15, 0.25, 0.4, 1.0);  // Tech accents
        
        // Create a simple pixel art pattern
        match (tile_x as i32, tile_y as i32) {
            // Corner accent tiles
            (0, 0) | (3, 0) | (0, 3) | (3, 3) => blue_accent,
            
            // Border tiles
            (0, _) | (3, _) | (_, 0) | (_, 3) => very_dark,
            
            // Center cross pattern
            (1, 1) | (2, 2) => light_gray,
            (1, 2) | (2, 1) => light_gray,
            
            // Default floor tiles
            _ => dark_gray,
        }
    }
    
    /// Get sci-fi space station ceiling with advanced lighting and ventilation systems
    pub fn get_ceiling_texture_color(&self, x: f32, y: f32) -> Color {
        // Advanced ceiling panel system
        let panel_size = 1.8; // Size of each ceiling panel
        let panel_x = (x / panel_size) % 1.0;
        let panel_y = (y / panel_size) % 1.0;
        
        // Base ceiling color - dark metallic with blue tint
        let mut base_color = Color::new(0.18, 0.22, 0.28, 1.0); // Dark blue-gray metal
        
        // Support beam grid structure
        let beam_width = 0.06;
        let has_beam = panel_x < beam_width || panel_x > 1.0 - beam_width || 
                      panel_y < beam_width || panel_y > 1.0 - beam_width;
        
        // Lighting panel system
        let light_panel_x = (panel_x - beam_width) / (1.0 - 2.0 * beam_width);
        let light_panel_y = (panel_y - beam_width) / (1.0 - 2.0 * beam_width);
        let light_center_x = (light_panel_x - 0.5).abs();
        let light_center_y = (light_panel_y - 0.5).abs();
        let light_dist = (light_center_x * light_center_x + light_center_y * light_center_y).sqrt();
        
        // Different panel types based on position
        let panel_type = ((x / panel_size).floor() as i32 + (y / panel_size).floor() as i32) % 6;
        
        if has_beam {
            // Structural support beams with mounting hardware
            let beam_detail = ((x * 200.0).sin() * (y * 200.0).cos()).abs();
            let has_mounting = beam_detail > 0.85;
            
            if has_mounting {
                // Mounting points and bolts
                base_color.r = (base_color.r + 0.1).min(1.0);
                base_color.g = (base_color.g + 0.1).min(1.0);
                base_color.b = (base_color.b + 0.1).min(1.0);
            } else {
                // Dark structural beams
                base_color.r *= 0.5;
                base_color.g *= 0.5;
                base_color.b *= 0.5;
            }
        } else {
            match panel_type {
                0 | 1 => {
                    // Standard lighting panels with LED arrays
                    if light_dist < 0.4 {
                        let light_intensity = (1.0 - light_dist * 2.5).max(0.0);
                        let pulse = ((x + y) * 15.0).sin() * 0.1 + 0.9; // Subtle pulsing
                        
                        base_color.r = (base_color.r + light_intensity * pulse * 0.6).min(1.0);
                        base_color.g = (base_color.g + light_intensity * pulse * 0.7).min(1.0);
                        base_color.b = (base_color.b + light_intensity * pulse * 0.4).min(1.0);
                    } else {
                        // LED mounting surface
                        let grid_pattern = ((light_panel_x * 20.0) % 1.0 < 0.1) || ((light_panel_y * 20.0) % 1.0 < 0.1);
                        if grid_pattern {
                            base_color.r = (base_color.r + 0.05).min(1.0);
                            base_color.g = (base_color.g + 0.05).min(1.0);
                            base_color.b = (base_color.b + 0.05).min(1.0);
                        }
                    }
                },
                2 => {
                    // Ventilation grating panels
                    let vent_spacing = 0.12;
                    let vent_u = (light_panel_x / vent_spacing) % 1.0;
                    let vent_v = (light_panel_y / vent_spacing) % 1.0;
                    let has_vent = (vent_u > 0.2 && vent_u < 0.8) || (vent_v > 0.2 && vent_v < 0.8);
                    
                    if has_vent {
                        // Darker vent openings
                        base_color.r *= 0.3;
                        base_color.g *= 0.3;
                        base_color.b *= 0.3;
                    } else {
                        // Vent frame
                        base_color.r = (base_color.r + 0.08).min(1.0);
                        base_color.g = (base_color.g + 0.08).min(1.0);
                        base_color.b = (base_color.b + 0.08).min(1.0);
                    }
                },
                3 => {
                    // Technical access panels
                    let access_frame = light_center_x > 0.35 || light_center_y > 0.35;
                    if access_frame {
                        // Access panel frame
                        base_color.r = (base_color.r + 0.06).min(1.0);
                        base_color.g = (base_color.g + 0.06).min(1.0);
                        base_color.b = (base_color.b + 0.06).min(1.0);
                    } else {
                        // Panel center with status indicators
                        let indicator_pattern = ((light_panel_x * 8.0).floor() as i32 + (light_panel_y * 8.0).floor() as i32) % 4;
                        if indicator_pattern == 0 {
                            // Status LED
                            let led_brightness = ((x + y) * 12.0).sin() * 0.4 + 0.6;
                            base_color.r = (base_color.r + led_brightness * 0.2).min(1.0);
                            base_color.g = (base_color.g + led_brightness * 0.4).min(1.0);
                            base_color.b = (base_color.b + led_brightness * 0.1).min(1.0);
                        }
                    }
                },
                _ => {
                    // Standard ceiling panels with brushed metal finish
                    let brushed_u = (x * 180.0).sin() * 0.04;
                    let brushed_v = (y * 180.0).sin() * 0.04;
                    
                    base_color.r = (base_color.r + brushed_u).clamp(0.0, 1.0);
                    base_color.g = (base_color.g + brushed_v).clamp(0.0, 1.0);
                    base_color.b = (base_color.b + brushed_u * 0.5).clamp(0.0, 1.0);
                }
            }
        }
        
        base_color
    }
    
    /// Draw the map in top-down view with texture colors
    pub fn draw_topdown(&self, offset_x: f32, offset_y: f32, tile_size: f32) {
        for y in 0..self.height {
            for x in 0..self.width {
                let screen_x = offset_x + x as f32 * tile_size;
                let screen_y = offset_y + y as f32 * tile_size;
                
                let wall_type = self.get_wall_type(x as i32, y as i32);
                let color = if wall_type == WallType::Empty {
                    DARKGRAY   // Empty space (darker for contrast)
                } else {
                    self.get_wall_color(wall_type, true) // Show base texture color
                };
                
                draw_rectangle(screen_x, screen_y, tile_size, tile_size, color);
                
                // Draw grid lines
                draw_rectangle_lines(screen_x, screen_y, tile_size, tile_size, 1.0, GRAY);
            }
        }
    }
    
    /// Enhanced minimap with colored visualization for debugging pathfinding
    pub fn draw_enhanced_minimap(&self, offset_x: f32, offset_y: f32, tile_size: f32, 
                                player_pos: Option<(f32, f32)>, 
                                target_pos: Option<(f32, f32)>,
                                path: Option<&Vec<macroquad::math::Vec2>>,
                                explored_nodes: Option<&Vec<(i32, i32)>>) {
        
        // Draw base map with distinct colors for each wall type
        for y in 0..self.height {
            for x in 0..self.width {
                let screen_x = offset_x + x as f32 * tile_size;
                let screen_y = offset_y + y as f32 * tile_size;
                
                let wall_type = self.get_wall_type(x as i32, y as i32);
                let color = match wall_type {
                    WallType::Empty => Color::new(0.1, 0.1, 0.1, 1.0),        // Dark gray for walkable areas
                    WallType::TechPanel => Color::new(0.8, 0.8, 0.9, 1.0),    // Light blue-white for tech panels
                    WallType::HullPlating => Color::new(0.6, 0.6, 0.7, 1.0),  // Gray for hull plating
                    WallType::ControlSystem => Color::new(0.2, 0.4, 0.8, 1.0), // Blue for control systems
                    WallType::EnergyConduit => Color::new(0.8, 0.4, 0.2, 1.0), // Orange for energy conduits
                };
                
                draw_rectangle(screen_x, screen_y, tile_size, tile_size, color);
                
                // Draw grid lines for clarity
                draw_rectangle_lines(screen_x, screen_y, tile_size, tile_size, 1.0, 
                                   Color::new(0.3, 0.3, 0.3, 0.5));
            }
        }
        
        // Draw explored nodes from A* pathfinding (light yellow)
        if let Some(explored) = explored_nodes {
            for &(x, y) in explored {
                if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
                    let screen_x = offset_x + x as f32 * tile_size;
                    let screen_y = offset_y + y as f32 * tile_size;
                    draw_rectangle(screen_x + 2.0, screen_y + 2.0, tile_size - 4.0, tile_size - 4.0, 
                                 Color::new(1.0, 1.0, 0.3, 0.6)); // Light yellow with transparency
                }
            }
        }
        
        // Draw pathfinding path (bright green line)
        if let Some(path_points) = path {
            for i in 0..path_points.len().saturating_sub(1) {
                let start = path_points[i];
                let end = path_points[i + 1];
                
                let start_screen_x = offset_x + start.x * tile_size;
                let start_screen_y = offset_y + start.y * tile_size;
                let end_screen_x = offset_x + end.x * tile_size;
                let end_screen_y = offset_y + end.y * tile_size;
                
                draw_line(start_screen_x, start_screen_y, end_screen_x, end_screen_y, 3.0, GREEN);
                
                // Draw path point markers
                draw_circle(start_screen_x, start_screen_y, 3.0, LIME);
            }
            
            // Draw final path point
            if let Some(last_point) = path_points.last() {
                let screen_x = offset_x + last_point.x * tile_size;
                let screen_y = offset_y + last_point.y * tile_size;
                draw_circle(screen_x, screen_y, 3.0, LIME);
            }
        }
        
        // Draw target position (bright red circle)
        if let Some((target_x, target_y)) = target_pos {
            let screen_x = offset_x + target_x * tile_size;
            let screen_y = offset_y + target_y * tile_size;
            draw_circle(screen_x, screen_y, 6.0, RED);
            draw_circle_lines(screen_x, screen_y, 8.0, 2.0, Color::new(0.5, 0.0, 0.0, 1.0)); // Dark red
            
            // Draw target label
            draw_text("TARGET", screen_x - 15.0, screen_y - 12.0, 12.0, RED);
        }
        
        // Draw player position (bright blue circle)
        if let Some((player_x, player_y)) = player_pos {
            let screen_x = offset_x + player_x * tile_size;
            let screen_y = offset_y + player_y * tile_size;
            draw_circle(screen_x, screen_y, 5.0, BLUE);
            draw_circle_lines(screen_x, screen_y, 7.0, 2.0, DARKBLUE);
            
            // Draw player label
            draw_text("PLAYER", screen_x - 18.0, screen_y - 12.0, 12.0, BLUE);
        }
        
        // Draw legend
        let legend_x = offset_x + self.width as f32 * tile_size + 20.0;
        let legend_y = offset_y;
        
        draw_text("MINIMAP LEGEND:", legend_x, legend_y + 15.0, 14.0, WHITE);
        
        // Wall type legend
        draw_rectangle(legend_x, legend_y + 25.0, 15.0, 15.0, Color::new(0.8, 0.8, 0.9, 1.0));
        draw_text("Tech Panel", legend_x + 20.0, legend_y + 37.0, 12.0, WHITE);
        
        draw_rectangle(legend_x, legend_y + 45.0, 15.0, 15.0, Color::new(0.6, 0.6, 0.7, 1.0));
        draw_text("Hull Plating", legend_x + 20.0, legend_y + 57.0, 12.0, WHITE);
        
        draw_rectangle(legend_x, legend_y + 65.0, 15.0, 15.0, Color::new(0.2, 0.4, 0.8, 1.0));
        draw_text("Control System", legend_x + 20.0, legend_y + 77.0, 12.0, WHITE);
        
        draw_rectangle(legend_x, legend_y + 85.0, 15.0, 15.0, Color::new(0.8, 0.4, 0.2, 1.0));
        draw_text("Energy Conduit", legend_x + 20.0, legend_y + 97.0, 12.0, WHITE);
        
        draw_rectangle(legend_x, legend_y + 105.0, 15.0, 15.0, Color::new(0.1, 0.1, 0.1, 1.0));
        draw_text("Walkable Area", legend_x + 20.0, legend_y + 117.0, 12.0, WHITE);
        
        // Pathfinding legend
        draw_circle(legend_x + 7.0, legend_y + 137.0, 5.0, BLUE);
        draw_text("Player", legend_x + 20.0, legend_y + 142.0, 12.0, WHITE);
        
        draw_circle(legend_x + 7.0, legend_y + 152.0, 5.0, RED);
        draw_text("Target", legend_x + 20.0, legend_y + 157.0, 12.0, WHITE);
        
        draw_line(legend_x, legend_y + 167.0, legend_x + 15.0, legend_y + 167.0, 3.0, GREEN);
        draw_text("Path", legend_x + 20.0, legend_y + 172.0, 12.0, WHITE);
        
        draw_rectangle(legend_x, legend_y + 182.0, 15.0, 15.0, Color::new(1.0, 1.0, 0.3, 0.6));
        draw_text("Explored", legend_x + 20.0, legend_y + 194.0, 12.0, WHITE);
    }
} 