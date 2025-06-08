use macroquad::prelude::*;
use std::collections::HashMap;
use crate::game::player::Player;

/// Map system - grid-based like classic Wolfenstein
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<u8>>, // 0 = empty, 1+ = different wall types
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
            vec![2,0,0,0,1,1,0,0,0,1],  // Stone corridor
            vec![2,0,0,0,1,1,0,0,0,1],
            vec![1,0,4,0,0,0,0,3,0,2],
            vec![1,0,4,4,0,0,3,3,0,2],
            vec![1,0,0,0,0,0,0,0,0,2],
            vec![1,1,1,2,2,2,2,1,1,1],  // Mixed outer walls
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
                let panel_u = u * 3.0; // 3 panels horizontally
                let panel_v = v * 4.0; // 4 panels vertically
                let local_u = panel_u % 1.0;
                let local_v = panel_v % 1.0;
                
                // Panel frame with beveled edges
                let frame_width = 0.06;
                let is_frame = local_u < frame_width || local_u > 1.0 - frame_width || 
                              local_v < frame_width || local_v > 1.0 - frame_width;
                
                if is_frame {
                    // Brushed metal frame with subtle chrome effect
                    base_color.r = (base_color.r + 0.08).min(1.0);
                    base_color.g = (base_color.g + 0.08).min(1.0);
                    base_color.b = (base_color.b + 0.08).min(1.0);
                } else {
                    // Circuit patterns
                    let circuit_u = (local_u - frame_width) * 1.2;
                    let circuit_v = (local_v - frame_width) * 1.2;
                    
                    // Horizontal circuit traces
                    let h_traces = ((circuit_v * 20.0) % 1.0 < 0.08) as u8 as f32;
                    // Vertical circuit traces  
                    let v_traces = ((circuit_u * 15.0) % 1.0 < 0.06) as u8 as f32;
                    // Circuit nodes at intersections
                    let node_dist = (circuit_u * 15.0 % 1.0 - 0.5).abs() + (circuit_v * 20.0 % 1.0 - 0.5).abs();
                    let is_node = node_dist < 0.15 && ((circuit_u * 15.0).floor() as i32 + (circuit_v * 20.0).floor() as i32) % 3 == 0;
                    
                    // LED status lights
                    let led_pattern = ((local_u * 8.0).floor() as i32 + (local_v * 6.0).floor() as i32) % 7;
                    let led_brightness = if led_pattern == 0 {
                        (((u + v) * 10.0).sin() * 0.5 + 0.5) * 0.4 // Pulsing LEDs
                    } else { 0.0 };
                    
                    if is_node {
                        // Bright circuit nodes
                        base_color.r = (base_color.r + 0.3).min(1.0);
                        base_color.g = (base_color.g + 0.4).min(1.0);
                        base_color.b = (base_color.b + 0.6).min(1.0);
                    } else if h_traces > 0.0 || v_traces > 0.0 {
                        // Circuit traces - subtle cyan glow
                        base_color.r = (base_color.r + 0.05).min(1.0);
                        base_color.g = (base_color.g + 0.15).min(1.0);
                        base_color.b = (base_color.b + 0.25).min(1.0);
                    }
                    
                    // Add LED lighting
                    base_color.r = (base_color.r + led_brightness * 0.8).min(1.0);
                    base_color.g = (base_color.g + led_brightness * 1.0).min(1.0);
                    base_color.b = (base_color.b + led_brightness * 0.3).min(1.0);
                }
            },
            WallType::HullPlating => {
                // Heavy duty reinforced hull with battle damage and wear
                let plate_u = u * 2.5; // 2.5 plates horizontally
                let plate_v = v * 3.0; // 3 rows vertically
                let local_u = plate_u % 1.0;
                let local_v = plate_v % 1.0;
                
                // Welded seams between plates
                let seam_width = 0.04;
                let is_seam = local_u < seam_width || local_u > 1.0 - seam_width || 
                             local_v < seam_width || local_v > 1.0 - seam_width;
                
                // Rivet pattern
                let rivet_spacing = 0.15;
                let rivet_u = (local_u / rivet_spacing) % 1.0;
                let rivet_v = (local_v / rivet_spacing) % 1.0;
                let rivet_dist = ((rivet_u - 0.5).powi(2) + (rivet_v - 0.5).powi(2)).sqrt();
                let is_rivet = rivet_dist < 0.3;
                
                // Battle damage and scratches
                let damage_pattern = ((u * 47.0).sin() * (v * 31.0).cos() + (u * 23.0).cos() * (v * 19.0).sin()).abs();
                let scratch_pattern = ((u * 150.0 + v * 200.0).sin()).abs();
                
                if is_seam {
                    // Dark welded seams
                    base_color.r *= 0.4;
                    base_color.g *= 0.4;
                    base_color.b *= 0.4;
                } else if is_rivet {
                    // Raised rivets
                    base_color.r = (base_color.r + 0.15).min(1.0);
                    base_color.g = (base_color.g + 0.15).min(1.0);
                    base_color.b = (base_color.b + 0.15).min(1.0);
                } else {
                    // Hull surface with wear and damage
                    let wear = damage_pattern * 0.15;
                    let scratches = if scratch_pattern > 0.92 { -0.1 } else { 0.0 };
                    
                    base_color.r = (base_color.r - wear + scratches).clamp(0.0, 1.0);
                    base_color.g = (base_color.g - wear + scratches).clamp(0.0, 1.0);
                    base_color.b = (base_color.b - wear + scratches).clamp(0.0, 1.0);
                }
            },
            WallType::ControlSystem => {
                // Advanced control panels with holographic displays and interface elements
                let screen_u = u * 2.0; // 2 screens horizontally
                let screen_v = v * 3.0; // 3 screens vertically
                let local_u = screen_u % 1.0;
                let local_v = screen_v % 1.0;
                
                // Screen bezel
                let bezel_width = 0.08;
                let is_bezel = local_u < bezel_width || local_u > 1.0 - bezel_width || 
                              local_v < bezel_width || local_v > 1.0 - bezel_width;
                
                // Control buttons and interfaces
                let button_pattern = ((local_u * 6.0).floor() as i32 + (local_v * 4.0).floor() as i32) % 5;
                let button_u = (local_u * 6.0) % 1.0;
                let button_v = (local_v * 4.0) % 1.0;
                let button_dist = ((button_u - 0.5).powi(2) + (button_v - 0.5).powi(2)).sqrt();
                let is_button = button_dist < 0.3 && button_pattern < 2;
                
                if is_bezel {
                    // Dark control panel housing
                    base_color.r *= 0.6;
                    base_color.g *= 0.6;
                    base_color.b *= 0.6;
                } else if is_button {
                    // Illuminated control buttons
                    let button_brightness = (((u + v) * 8.0 + button_pattern as f32).sin() * 0.3 + 0.7);
                    base_color.r = (base_color.r + button_brightness * 0.4).min(1.0);
                    base_color.g = (base_color.g + button_brightness * 0.6).min(1.0);
                    base_color.b = (base_color.b + button_brightness * 0.8).min(1.0);
                } else {
                    // Holographic display patterns
                    let holo_lines = ((local_v * 40.0) % 1.0 < 0.05) as u8 as f32;
                    let data_stream = ((local_u * 30.0 + v * 200.0).sin() * 0.5 + 0.5);
                    
                    // Cyan holographic glow
                    base_color.r = (base_color.r + holo_lines * 0.1 + data_stream * 0.05).min(1.0);
                    base_color.g = (base_color.g + holo_lines * 0.3 + data_stream * 0.15).min(1.0);
                    base_color.b = (base_color.b + holo_lines * 0.4 + data_stream * 0.2).min(1.0);
                }
            },
            WallType::EnergyConduit => {
                // Glowing energy conduit systems with power flow patterns
                let conduit_u = u * 1.5; // 1.5 conduit sections horizontally
                let conduit_v = v * 4.0; // 4 conduit sections vertically
                let local_u = conduit_u % 1.0;
                let local_v = conduit_v % 1.0;
                
                // Conduit housing
                let housing_width = 0.1;
                let is_housing = local_u < housing_width || local_u > 1.0 - housing_width;
                
                // Energy flow patterns
                let time_factor = (u + v) * 5.0; // Simulate time-based animation
                let energy_wave = ((local_v * 15.0 - time_factor).sin() * 0.5 + 0.5);
                let energy_intensity = energy_wave * energy_wave; // Square for more dramatic effect
                
                // Connection nodes
                let node_spacing = 0.25;
                let node_v = (local_v / node_spacing) % 1.0;
                let is_node = (node_v - 0.5).abs() < 0.15;
                
                if is_housing {
                    // Dark conduit housing
                    base_color.r *= 0.5;
                    base_color.g *= 0.5;
                    base_color.b *= 0.5;
                } else if is_node {
                    // Bright connection nodes
                    base_color.r = (base_color.r + energy_intensity * 0.8).min(1.0);
                    base_color.g = (base_color.g + energy_intensity * 0.4).min(1.0);
                    base_color.b = (base_color.b + energy_intensity * 1.0).min(1.0);
                } else {
                    // Energy flow core with blue/white energy
                    let core_width = 0.6;
                    let core_dist = (local_u - 0.5).abs() / (core_width * 0.5);
                    let core_intensity = (1.0 - core_dist).max(0.0);
                    
                    base_color.r = (base_color.r + core_intensity * energy_intensity * 0.6).min(1.0);
                    base_color.g = (base_color.g + core_intensity * energy_intensity * 0.8).min(1.0);
                    base_color.b = (base_color.b + core_intensity * energy_intensity * 1.0).min(1.0);
                }
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
    
    /// Get sci-fi space station floor texture with deck plating and light strips
    pub fn get_floor_texture_color(&self, x: f32, y: f32) -> Color {
        // Create technical deck plating pattern
        let plate_size = 2.0; // Size of each deck plate
        let plate_x = (x / plate_size) % 1.0;
        let plate_y = (y / plate_size) % 1.0;
        
        // Base deck color - dark metallic
        let mut base_color = Color::new(0.25, 0.28, 0.32, 1.0); // Dark metallic blue-gray
        
        // Deck plate seams
        let seam_width = 0.03;
        let has_seam = plate_x < seam_width || plate_x > 1.0 - seam_width ||
                      plate_y < seam_width || plate_y > 1.0 - seam_width;
        
        // Grating pattern within plates
        let grating_spacing = 0.15;
        let grating_x = (plate_x / grating_spacing) % 1.0;
        let grating_y = (plate_y / grating_spacing) % 1.0;
        let has_grating = (grating_x < 0.1 && grating_y > 0.1 && grating_y < 0.9) ||
                         (grating_y < 0.1 && grating_x > 0.1 && grating_x < 0.9);
        
        // LED light strips along plate edges
        let light_strip_width = 0.015;
        let has_light_strip = (plate_x > 0.48 && plate_x < 0.48 + light_strip_width) ||
                             (plate_y > 0.48 && plate_y < 0.48 + light_strip_width);
        
        if has_light_strip {
            // Bright LED light strips - cyan/blue glow
            let light_intensity = (((x + y) * 20.0).sin() * 0.3 + 0.7); // Subtle pulsing
            base_color.r = (base_color.r + light_intensity * 0.3).min(1.0);
            base_color.g = (base_color.g + light_intensity * 0.5).min(1.0);
            base_color.b = (base_color.b + light_intensity * 0.8).min(1.0);
        } else if has_seam {
            // Dark deck seams
            base_color.r *= 0.4;
            base_color.g *= 0.4;
            base_color.b *= 0.4;
        } else if has_grating {
            // Darker grating for ventilation/drainage
            base_color.r *= 0.6;
            base_color.g *= 0.6;
            base_color.b *= 0.6;
        } else {
            // Deck surface with subtle wear patterns
            let wear_pattern = ((x * 43.0).sin() * (y * 47.0).cos()) * 0.08;
            let scuff_marks = ((x * 157.0).sin() * (y * 163.0).cos()).abs();
            let scuff_effect = if scuff_marks > 0.9 { -0.05 } else { 0.0 };
            
            base_color.r = (base_color.r + wear_pattern + scuff_effect).clamp(0.0, 1.0);
            base_color.g = (base_color.g + wear_pattern + scuff_effect).clamp(0.0, 1.0);
            base_color.b = (base_color.b + wear_pattern + scuff_effect).clamp(0.0, 1.0);
        }
        
        base_color
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
                        let pulse = (((x + y) * 15.0).sin() * 0.1 + 0.9); // Subtle pulsing
                        
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
                            let led_brightness = (((x + y) * 12.0).sin() * 0.4 + 0.6);
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
} 