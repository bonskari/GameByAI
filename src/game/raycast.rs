//! 3D Raycasting Engine
//! 
//! Implements the classic Wolfenstein-style raycasting algorithm to render
//! a 3D view from the 2D map data using DDA (Digital Differential Analyzer).

use macroquad::prelude::*;
use super::{Map, Player};

/// Raycasting configuration and camera settings
pub struct RaycastCamera {
    pub fov: f32,           // Field of view in radians (e.g., PI/3 = 60 degrees)
    pub render_distance: f32, // Maximum render distance
    pub screen_width: f32,  // Screen width for ray calculations
    pub screen_height: f32, // Screen height for wall height calculations
}

impl RaycastCamera {
    /// Create a new raycast camera with sensible defaults
    pub fn new() -> Self {
        RaycastCamera {
            fov: std::f32::consts::PI / 3.0, // 60 degrees
            render_distance: 20.0,           // Can see 20 units ahead
            screen_width: screen_width(),
            screen_height: screen_height(),
        }
    }
    
    /// Update screen dimensions (call when window resizes)
    pub fn update_screen_size(&mut self) {
        self.screen_width = screen_width();
        self.screen_height = screen_height();
    }
}

/// Result of a single raycast
#[derive(Debug, Clone)]
pub struct RayHit {
    pub distance: f32,      // Distance from player to wall
    pub wall_hit_x: f32,    // X coordinate where ray hit wall
    pub wall_hit_y: f32,    // Y coordinate where ray hit wall
    pub is_vertical_wall: bool, // True if we hit a vertical wall (NS), false if horizontal (EW)
    pub wall_x_offset: f32, // Position along the wall (0.0 to 1.0) for texturing
}

/// 3D Raycasting renderer
pub struct RaycastRenderer {
    pub camera: RaycastCamera,
}

impl RaycastRenderer {
    /// Create a new raycast renderer
    pub fn new() -> Self {
        RaycastRenderer {
            camera: RaycastCamera::new(),
        }
    }
    
    /// Cast a single ray and return hit information
    /// Uses DDA (Digital Differential Analyzer) algorithm for efficiency
    pub fn cast_ray(&self, map: &Map, start_x: f32, start_y: f32, ray_angle: f32) -> Option<RayHit> {
        // Ray direction
        let ray_dir_x = ray_angle.cos();
        let ray_dir_y = ray_angle.sin();
        
        // Current position
        let mut map_x = start_x as i32;
        let mut map_y = start_y as i32;
        
        // Length of ray from current pos to next x or y axis
        let delta_dist_x = (1.0 / ray_dir_x).abs();
        let delta_dist_y = (1.0 / ray_dir_y).abs();
        
        // Calculate step and initial side_dist
        let (step_x, mut side_dist_x) = if ray_dir_x < 0.0 {
            (-1, (start_x - map_x as f32) * delta_dist_x)
        } else {
            (1, (map_x as f32 + 1.0 - start_x) * delta_dist_x)
        };
        
        let (step_y, mut side_dist_y) = if ray_dir_y < 0.0 {
            (-1, (start_y - map_y as f32) * delta_dist_y)
        } else {
            (1, (map_y as f32 + 1.0 - start_y) * delta_dist_y)
        };
        
        // Perform DDA
        let mut hit = false;
        let mut is_vertical_wall = false;
        let mut step_count = 0;
        let max_steps = (self.camera.render_distance * 2.0) as i32; // Prevent infinite loops
        
        while !hit && step_count < max_steps {
            // Jump to next map square, either in x-direction, or in y-direction
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_x += step_x;
                is_vertical_wall = true;
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                is_vertical_wall = false;
            }
            
            // Check if ray has hit a wall
            if map.is_wall(map_x, map_y) {
                hit = true;
            }
            
            step_count += 1;
        }
        
        if !hit {
            return None; // Ray went out of bounds or hit max distance
        }
        
        // Calculate distance
        let perp_wall_dist = if is_vertical_wall {
            (map_x as f32 - start_x + (1.0 - step_x as f32) / 2.0) / ray_dir_x
        } else {
            (map_y as f32 - start_y + (1.0 - step_y as f32) / 2.0) / ray_dir_y
        };
        
        // Calculate exact hit position
        let wall_hit_x = start_x + perp_wall_dist * ray_dir_x;
        let wall_hit_y = start_y + perp_wall_dist * ray_dir_y;
        
        // Calculate wall offset for texturing (position along the wall)
        let wall_x_offset = if is_vertical_wall {
            wall_hit_y - wall_hit_y.floor()
        } else {
            wall_hit_x - wall_hit_x.floor()
        };
        
        Some(RayHit {
            distance: perp_wall_dist,
            wall_hit_x,
            wall_hit_y,
            is_vertical_wall,
            wall_x_offset,
        })
    }
    
    /// Render the 3D view using raycasting
    pub fn render_3d_view(&mut self, map: &Map, player: &Player) {
        // Update camera screen size in case window was resized
        self.camera.update_screen_size();
        
        let screen_w = self.camera.screen_width;
        let screen_h = self.camera.screen_height;
        
        // Clear screen with floor/ceiling colors
        clear_background(Color::new(0.2, 0.1, 0.0, 1.0)); // Dark brown floor base
        
        // Draw ceiling (top half of screen)
        draw_rectangle(0.0, 0.0, screen_w, screen_h / 2.0, Color::new(0.1, 0.1, 0.2, 1.0)); // Dark blue ceiling
        
        // Draw floor (bottom half of screen) 
        draw_rectangle(0.0, screen_h / 2.0, screen_w, screen_h / 2.0, Color::new(0.3, 0.2, 0.1, 1.0)); // Brown floor
        
        // Calculate number of rays to cast (one per screen column for smooth rendering)
        let num_rays = screen_w as i32;
        
        for screen_x in 0..num_rays {
            // Calculate ray angle
            let camera_x = 2.0 * screen_x as f32 / screen_w - 1.0; // x coordinate in camera space
            let ray_angle = player.rotation + camera_x * (self.camera.fov / 2.0);
            
            // Cast the ray
            if let Some(hit) = self.cast_ray(map, player.x, player.y, ray_angle) {
                // Skip if hit is too far away
                if hit.distance > self.camera.render_distance {
                    continue;
                }
                
                // Calculate wall height on screen based on distance
                let wall_height = screen_h / hit.distance;
                
                // Calculate top and bottom of wall on screen
                let wall_top = (screen_h - wall_height) / 2.0;
                let wall_bottom = wall_top + wall_height;
                
                // Choose wall color based on orientation (add depth perception)
                let base_color = if hit.is_vertical_wall {
                    Color::new(0.8, 0.8, 0.8, 1.0) // Lighter for vertical walls
                } else {
                    Color::new(0.6, 0.6, 0.6, 1.0) // Darker for horizontal walls  
                };
                
                // Add distance-based shading (further = darker)
                let brightness = (1.0 - (hit.distance / self.camera.render_distance)).max(0.2);
                let wall_color = Color::new(
                    base_color.r * brightness,
                    base_color.g * brightness, 
                    base_color.b * brightness,
                    1.0
                );
                
                // Draw the wall strip
                draw_line(
                    screen_x as f32, 
                    wall_top.max(0.0), 
                    screen_x as f32, 
                    wall_bottom.min(screen_h),
                    1.0,
                    wall_color
                );
            }
        }
    }
} 