use std::time::{Duration, Instant};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use macroquad::prelude::*;
use crate::game::{GameState, map::{Map, WallType}};

/// Automated visual testing bot that moves around the level
pub struct VisualTestBot {
    start_time: Instant,
    test_duration: Duration,
    pub current_waypoint: usize,
    pub waypoints: Vec<Waypoint>,
    movement_speed: f32,
    rotation_speed: f32,
    stuck_time: f32,
    last_position: (f32, f32),
    pub explored_nodes: Vec<(i32, i32)>,
    pub path_nodes: Vec<(i32, i32)>,
}

/// Waypoint for movement
#[derive(Debug, Clone)]
pub struct Waypoint {
    pub x: f32,
    pub y: f32,
    pub description: String,
}

/// Node for A* pathfinding
#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    x: i32,
    y: i32,
    cost: i32,
    heuristic: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl VisualTestBot {
    /// Create a new visual test bot
    pub fn new(test_duration_seconds: u64, _auto_close: bool) -> Self {
        // Define key points that are actually reachable in the level
        // Based on the map layout, we need to follow the corridors
        let key_points = vec![
            (1.5, 1.5, "Start"),
            (3.5, 1.5, "East corridor"),
            (8.5, 1.5, "Far east"),
            (8.5, 3.5, "Southeast corner"),
            (6.5, 3.5, "South corridor"),
            (3.5, 3.5, "Southwest area"),
            (1.5, 3.5, "West corridor"),
            (1.5, 8.5, "Far south"),
            (3.5, 8.5, "South area"),
            (8.5, 8.5, "Southeast corner"),
            (8.5, 6.5, "East corridor south"),
            (6.5, 6.5, "Central south"),
            (3.5, 6.5, "West south"),
            (1.5, 6.5, "Far west south"),
            (1.5, 1.5, "Back to start"),
        ];

        println!("🤖 Generating paths between key points:");
        
        // Generate waypoints using A*
        let mut waypoints = Vec::new();
        let mut explored_nodes = Vec::new();
        let mut path_nodes = Vec::new();
        
        for i in 0..key_points.len() - 1 {
            let (start_x, start_y, start_desc) = key_points[i];
            let (end_x, end_y, end_desc) = key_points[i + 1];
            
            println!("  Path {}: ({}, {}) -> ({}, {})", 
                    i + 1, start_x, start_y, end_x, end_y);
            
            let (path, explored) = Self::find_path_with_explored(
                start_x as i32, start_y as i32,
                end_x as i32, end_y as i32,
            );
            
            println!("  Found path with {} waypoints", path.len());
            
            // Convert path to waypoints
            for (x, y) in &path {
                waypoints.push(Waypoint {
                    x: *x as f32 + 0.5, // Center in tile
                    y: *y as f32 + 0.5,
                    description: format!("Moving from {} to {}", start_desc, end_desc),
                });
            }
            for (x, y) in &explored {
                explored_nodes.push((*x, *y));
            }
            for (x, y) in &path {
                path_nodes.push((*x, *y));
            }
        }

        println!("🤖 Generated {} total waypoints", waypoints.len());

        VisualTestBot {
            start_time: Instant::now(),
            test_duration: Duration::from_secs(test_duration_seconds),
            current_waypoint: 0,
            waypoints,
            movement_speed: 2.0, // Increased speed for better movement
            rotation_speed: std::f32::consts::PI * 2.0, // Faster rotation
            stuck_time: 0.0,
            last_position: (0.0, 0.0),
            explored_nodes,
            path_nodes,
        }
    }

    /// Check if position is a wall
    fn is_wall(x: i32, y: i32) -> bool {
        // Use the actual map data instead of hardcoded walls
        if x < 0 || y < 0 || x >= 10 || y >= 10 {
            return true; // Out of bounds = wall
        }
        
        // Get wall type from map
        let wall_type = Map::new().get_wall_type(x, y);
        wall_type != WallType::Empty
    }

    /// Find path using A* algorithm with improved heuristic
    fn find_path(start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> Vec<(i32, i32)> {
        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        let mut closed_set = HashSet::new();
        
        // Initialize start node
        let start_node = Node {
            x: start_x,
            y: start_y,
            cost: 0,
            heuristic: Self::heuristic(start_x, start_y, end_x, end_y),
        };
        
        open_set.push(start_node);
        g_score.insert((start_x, start_y), 0);
        
        while let Some(current) = open_set.pop() {
            if current.x == end_x && current.y == end_y {
                return Self::reconstruct_path(came_from, (end_x, end_y));
            }
            
            closed_set.insert((current.x, current.y));
            
            // Check neighbors (8-directional movement)
            let neighbors = [
                (0, 1),   // North
                (1, 1),   // Northeast
                (1, 0),   // East
                (1, -1),  // Southeast
                (0, -1),  // South
                (-1, -1), // Southwest
                (-1, 0),  // West
                (-1, 1),  // Northwest
            ];
            
            for &(dx, dy) in &neighbors {
                let nx = current.x + dx;
                let ny = current.y + dy;
                
                if closed_set.contains(&(nx, ny)) {
                    continue;
                }
                
                // Check if wall or invalid position
                if Self::is_wall(nx, ny) {
                    continue;
                }
                
                // For diagonal movement, check if both adjacent tiles are walkable
                if dx != 0 && dy != 0 {
                    if Self::is_wall(current.x + dx, current.y) || Self::is_wall(current.x, current.y + dy) {
                        continue;
                    }
                }
                
                // Calculate movement cost (diagonal movement costs more)
                let move_cost = if dx != 0 && dy != 0 { 14 } else { 10 }; // 14 ≈ √2 * 10
                let tentative_g = g_score.get(&(current.x, current.y)).unwrap_or(&i32::MAX) + move_cost;
                
                if tentative_g < *g_score.get(&(nx, ny)).unwrap_or(&i32::MAX) {
                    came_from.insert((nx, ny), (current.x, current.y));
                    g_score.insert((nx, ny), tentative_g);
                    
                    let neighbor = Node {
                        x: nx,
                        y: ny,
                        cost: tentative_g,
                        heuristic: Self::heuristic(nx, ny, end_x, end_y),
                    };
                    
                    open_set.push(neighbor);
                }
            }
        }
        
        println!("⚠️ No path found from ({}, {}) to ({}, {})", start_x, start_y, end_x, end_y);
        Vec::new() // No path found
    }

    /// Calculate heuristic (diagonal distance)
    fn heuristic(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
        let dx = (x1 - x2).abs();
        let dy = (y1 - y2).abs();
        // Use diagonal distance heuristic
        let min = dx.min(dy);
        let max = dx.max(dy);
        (min * 14 + (max - min) * 10) // 14 ≈ √2 * 10
    }

    /// Reconstruct path from came_from map
    fn reconstruct_path(came_from: HashMap<(i32, i32), (i32, i32)>, end: (i32, i32)) -> Vec<(i32, i32)> {
        let mut path = vec![end];
        let mut current = end;
        
        while let Some(&prev) = came_from.get(&current) {
            path.push(prev);
            current = prev;
        }
        
        path.reverse();
        path
    }

    /// Update the bot's movement and testing logic
    pub fn update(&mut self, player: &mut crate::game::Player, map: &crate::game::Map, delta_time: f32) -> bool {
        let elapsed = self.start_time.elapsed();
        
        // Check if test duration exceeded
        if elapsed >= self.test_duration {
            println!("🤖 Visual test completed after {:.1}s", elapsed.as_secs_f32());
            return false; // Signal to close game
        }

        // Get current waypoint
        let waypoint = &self.waypoints[self.current_waypoint];
        
        // Calculate direction to waypoint
        let dx = waypoint.x - player.x;
        let dy = waypoint.y - player.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let target_angle = dy.atan2(dx);
        let mut angle_diff = target_angle - player.rotation;
        // Normalize angle to [-PI, PI]
        while angle_diff > std::f32::consts::PI { angle_diff -= 2.0 * std::f32::consts::PI; }
        while angle_diff < -std::f32::consts::PI { angle_diff += 2.0 * std::f32::consts::PI; }

        // Smoothly rotate toward target
        let max_turn = self.rotation_speed * delta_time;
        if angle_diff.abs() < max_turn {
            player.rotation = target_angle;
        } else if angle_diff > 0.0 {
            player.rotation += max_turn;
        } else {
            player.rotation -= max_turn;
        }
        // Normalize rotation
        while player.rotation > std::f32::consts::PI { player.rotation -= 2.0 * std::f32::consts::PI; }
        while player.rotation < -std::f32::consts::PI { player.rotation += 2.0 * std::f32::consts::PI; }

        // Only move forward if facing the waypoint (within 15 degrees)
        let facing_threshold = 15.0_f32.to_radians();
        let mut moved = false;
        if angle_diff.abs() < facing_threshold {
            // Move forward
            let move_x = player.rotation.cos() * self.movement_speed * delta_time;
            let move_y = player.rotation.sin() * self.movement_speed * delta_time;
            let new_x = player.x + move_x;
            let new_y = player.y + move_y;
            if !map.is_wall(new_x as i32, new_y as i32) {
                player.x = new_x;
                player.y = new_y;
                moved = true;
            } else {
                println!("Blocked by wall at ({:.2}, {:.2}) while moving from ({:.2}, {:.2}) toward waypoint {} at ({:.2}, {:.2})", new_x, new_y, player.x, player.y, self.current_waypoint, waypoint.x, waypoint.y);
            }
        }

        // Check if stuck
        let current_pos = (player.x, player.y);
        let pos_diff = (current_pos.0 - self.last_position.0).abs() + (current_pos.1 - self.last_position.1).abs();
        if !moved && pos_diff < 0.001 {
            self.stuck_time += delta_time;
            if self.stuck_time > 0.5 { // Reduced stuck timeout
                println!("⚠️ Stuck at player ({:.2}, {:.2}), waypoint {} at ({:.2}, {:.2}) - skipping to next", 
                    player.x, player.y, self.current_waypoint, waypoint.x, waypoint.y);
                self.current_waypoint = (self.current_waypoint + 1) % self.waypoints.len();
                self.stuck_time = 0.0;
            }
        } else {
            self.stuck_time = 0.0;
        }
        self.last_position = current_pos;

        // Move to next waypoint if close enough
        if distance < 0.3 { // Slightly larger tolerance
            println!("✓ Reached waypoint {} at ({:.2}, {:.2})", self.current_waypoint, waypoint.x, waypoint.y);
            self.current_waypoint = (self.current_waypoint + 1) % self.waypoints.len();
            self.stuck_time = 0.0;
        }

        true // Continue running until duration expires
    }

    /// Get current test progress
    pub fn get_progress(&self) -> (usize, usize, f32) {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let total = self.test_duration.as_secs_f32();
        (self.current_waypoint, self.waypoints.len(), elapsed / total)
    }

    /// Draw test overlay information
    pub fn draw_overlay(&self) {
        let (current, total, progress) = self.get_progress();
        
        // Draw test progress
        let overlay_y = 50.0;
        draw_text(&format!("🤖 VISUAL TEST BOT"), 10.0, overlay_y, 20.0, YELLOW);
        draw_text(&format!("Waypoint {}/{} ({:.0}%)", current + 1, total, progress * 100.0), 
                 10.0, overlay_y + 25.0, 16.0, WHITE);
        
        if current < self.waypoints.len() {
            let waypoint = &self.waypoints[current];
            draw_text(&format!("Target: {}", waypoint.description), 
                     10.0, overlay_y + 45.0, 14.0, LIGHTGRAY);
        }
        
        // Draw progress bar
        let bar_width = 200.0;
        let bar_height = 10.0;
        let bar_x = 10.0;
        let bar_y = overlay_y + 65.0;
        
        draw_rectangle(bar_x, bar_y, bar_width, bar_height, DARKGRAY);
        draw_rectangle(bar_x, bar_y, bar_width * progress, bar_height, GREEN);
        draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, 1.0, WHITE);
        
        // Draw minimap A* explored nodes and path
        let tile_size = 20.0;
        let offset_x = 300.0;
        let offset_y = 50.0;
        for &(x, y) in &self.explored_nodes {
            draw_rectangle(offset_x + x as f32 * tile_size, offset_y + y as f32 * tile_size, tile_size, tile_size, Color::new(0.2, 0.4, 1.0, 0.3));
        }
        for &(x, y) in &self.path_nodes {
            draw_rectangle(offset_x + x as f32 * tile_size, offset_y + y as f32 * tile_size, tile_size, tile_size, Color::new(1.0, 0.2, 0.2, 0.5));
        }
        // Draw player position
        let player = &self.last_position;
        draw_circle(offset_x + player.0 * tile_size, offset_y + player.1 * tile_size, tile_size * 0.3, YELLOW);
    }

    fn find_path_with_explored(start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        let mut closed_set = HashSet::new();
        let mut explored = Vec::new();

        let start_node = Node {
            x: start_x,
            y: start_y,
            cost: 0,
            heuristic: Self::heuristic(start_x, start_y, end_x, end_y),
        };
        open_set.push(start_node);
        g_score.insert((start_x, start_y), 0);

        while let Some(current) = open_set.pop() {
            explored.push((current.x, current.y));
            if current.x == end_x && current.y == end_y {
                return (Self::reconstruct_path(came_from, (end_x, end_y)), explored);
            }
            closed_set.insert((current.x, current.y));
            for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = current.x + dx;
                let ny = current.y + dy;
                if closed_set.contains(&(nx, ny)) {
                    continue;
                }
                if Self::is_wall(nx, ny) {
                    continue;
                }
                let tentative_g = g_score.get(&(current.x, current.y)).unwrap_or(&i32::MAX) + 1;
                if tentative_g < *g_score.get(&(nx, ny)).unwrap_or(&i32::MAX) {
                    came_from.insert((nx, ny), (current.x, current.y));
                    g_score.insert((nx, ny), tentative_g);
                    let neighbor = Node {
                        x: nx,
                        y: ny,
                        cost: tentative_g,
                        heuristic: Self::heuristic(nx, ny, end_x, end_y),
                    };
                    open_set.push(neighbor);
                }
            }
        }
        (Vec::new(), explored)
    }
}

/// Run automated visual tests
pub async fn run_visual_tests(test_duration: u64, auto_close: bool) {
    println!("🤖 Starting automated visual tests...");
    println!("   Duration: {}s", test_duration);
    println!("   Auto-close: {}", auto_close);
    
    let mut game_state = GameState::new();
    let test_bot = VisualTestBot::new(test_duration, auto_close);
    
    // Assign the test bot to the game state
    game_state.test_bot = Some(test_bot);
    
    loop {
        let delta_time = get_frame_time();
        
        // Update player and other game state (but not test bot yet)
        game_state.frame_count += 1;
        game_state.player.update(delta_time, &game_state.map);
        
        // Toggle between 2D and 3D view with TAB key
        if is_key_pressed(KeyCode::Tab) {
            game_state.view_mode_3d = !game_state.view_mode_3d;
        }
        
        // Update test bot separately to avoid borrowing issues
        let should_exit = if let Some(test_bot) = &mut game_state.test_bot {
            !test_bot.update(&mut game_state.player, &game_state.map, delta_time)
        } else {
            false
        };
        
        if should_exit {
            break;
        }
        
        // Draw game (full 3D rendering + minimap)
        game_state.draw();
        
        // Draw test overlay
        if let Some(test_bot) = &game_state.test_bot {
            test_bot.draw_overlay();
        }
        
        next_frame().await;
    }
} 