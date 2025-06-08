use std::time::{Duration, Instant};
use macroquad::prelude::*;
use crate::game::GameState;
use super::screenshot_validator::{ScreenshotValidator, ComparisonResult};

/// Automated visual testing bot that moves around and validates textures
pub struct VisualTestBot {
    start_time: Instant,
    test_duration: Duration,
    current_test: usize,
    test_positions: Vec<TestPosition>,
    movement_speed: f32,
    auto_close: bool,
    screenshot_validator: ScreenshotValidator,
    screenshot_results: Vec<ComparisonResult>,
    last_screenshot_time: f32,
}

/// Test position with expected visual validation
#[derive(Debug, Clone)]
pub struct TestPosition {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub description: String,
    pub expected_wall_types: Vec<String>,
    pub hold_time: f32, // How long to stay at this position
}

impl VisualTestBot {
    /// Create a new visual test bot
    pub fn new(test_duration_seconds: u64, auto_close: bool) -> Self {
        let test_positions = vec![
            TestPosition {
                x: 1.5, y: 1.5, rotation: 0.0,
                description: "Starting position - TechPanel walls".to_string(),
                expected_wall_types: vec!["TechPanel".to_string()],
                hold_time: 2.0,
            },
            TestPosition {
                x: 3.0, y: 1.5, rotation: std::f32::consts::PI / 2.0,
                description: "East corridor - HullPlating walls".to_string(),
                expected_wall_types: vec!["HullPlating".to_string()],
                hold_time: 2.0,
            },
            TestPosition {
                x: 5.0, y: 3.0, rotation: std::f32::consts::PI,
                description: "Center area - ControlSystem walls".to_string(),
                expected_wall_types: vec!["ControlSystem".to_string()],
                hold_time: 2.0,
            },
            TestPosition {
                x: 7.0, y: 4.0, rotation: -std::f32::consts::PI / 2.0,
                description: "North area - EnergyConduit walls".to_string(),
                expected_wall_types: vec!["EnergyConduit".to_string()],
                hold_time: 2.0,
            },
            TestPosition {
                x: 8.5, y: 2.5, rotation: std::f32::consts::PI / 4.0,
                description: "Corner view - Mixed wall types".to_string(),
                expected_wall_types: vec!["TechPanel".to_string(), "HullPlating".to_string()],
                hold_time: 3.0,
            },
        ];

        VisualTestBot {
            start_time: Instant::now(),
            test_duration: Duration::from_secs(test_duration_seconds),
            current_test: 0,
            test_positions,
            movement_speed: 2.0,
            auto_close,
            screenshot_validator: ScreenshotValidator::new("test_screenshots", 0.05), // 5% tolerance
            screenshot_results: Vec::new(),
            last_screenshot_time: 0.0,
        }
    }

    /// Update the bot's movement and testing logic
    pub async fn update(&mut self, game_state: &mut GameState, delta_time: f32) -> bool {
        let elapsed = self.start_time.elapsed();
        
        // Check if test duration exceeded
        if elapsed >= self.test_duration {
            if self.auto_close {
                println!("🤖 Visual test completed after {:.1}s - Auto-closing", elapsed.as_secs_f32());
                return false; // Signal to close game
            }
        }

        // Move to current test position
        if self.current_test < self.test_positions.len() {
            let target = self.test_positions[self.current_test].clone();
            let moved = self.move_towards_target(game_state, &target, delta_time);
            
            if moved {
                // We've reached the target, hold position and validate
                self.validate_position(game_state, &target).await;
                
                // Move to next test after hold time
                if elapsed.as_secs_f32() > (self.current_test as f32 + 1.0) * target.hold_time {
                    self.current_test += 1;
                    if self.current_test >= self.test_positions.len() {
                        println!("🤖 All test positions completed!");
                        
                        // Generate final screenshot report
                        self.generate_final_report();
                        
                        if self.auto_close {
                            return false;
                        }
                    }
                }
            }
        }

        true // Continue running
    }

    /// Move player towards target position
    fn move_towards_target(&self, game_state: &mut GameState, target: &TestPosition, delta_time: f32) -> bool {
        let player = &mut game_state.player;
        
        // Calculate distance to target
        let dx = target.x - player.x;
        let dy = target.y - player.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        // Move towards target if not close enough
        if distance > 0.1 {
            let move_x = (dx / distance) * self.movement_speed * delta_time;
            let move_y = (dy / distance) * self.movement_speed * delta_time;
            
            // Check collision before moving
            let new_x = player.x + move_x;
            let new_y = player.y + move_y;
            
            if !game_state.map.is_wall(new_x as i32, new_y as i32) {
                player.x = new_x;
                player.y = new_y;
            }
            
            false // Still moving
        } else {
            // Rotate towards target rotation
            let rotation_diff = target.rotation - player.rotation;
            let rotation_diff = ((rotation_diff + std::f32::consts::PI) % (2.0 * std::f32::consts::PI)) - std::f32::consts::PI;
            
            if rotation_diff.abs() > 0.1 {
                player.rotation += rotation_diff.signum() * 2.0 * delta_time;
                false // Still rotating
            } else {
                player.rotation = target.rotation;
                true // Reached target
            }
        }
    }

    /// Validate visual elements at current position
    async fn validate_position(&mut self, game_state: &GameState, target: &TestPosition) {
        println!("🤖 Testing position: {}", target.description);
        println!("   Player at ({:.1}, {:.1}) rotation {:.1}°", 
            game_state.player.x, game_state.player.y, game_state.player.rotation.to_degrees());
        
        // Check surrounding wall types
        let px = game_state.player.x as i32;
        let py = game_state.player.y as i32;
        
        let surrounding_walls = vec![
            (px - 1, py, "West"),
            (px + 1, py, "East"), 
            (px, py - 1, "North"),
            (px, py + 1, "South"),
        ];
        
        for (wx, wy, direction) in surrounding_walls {
            if game_state.map.is_wall(wx, wy) {
                let wall_type = game_state.map.get_wall_type(wx, wy);
                println!("   {} wall: {:?}", direction, wall_type);
            }
        }
        
        // Capture screenshot for validation
        let test_name = format!("position_{}", self.current_test);
        let result = self.screenshot_validator.capture_and_validate(&test_name, &target.description).await;
        self.screenshot_results.push(result);
        self.last_screenshot_time = self.start_time.elapsed().as_secs_f32();
    }

    /// Get current test progress
    pub fn get_progress(&self) -> (usize, usize, f32) {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let total = self.test_duration.as_secs_f32();
        (self.current_test, self.test_positions.len(), elapsed / total)
    }

    /// Draw test overlay information
    pub fn draw_overlay(&self) {
        let (current, total, progress) = self.get_progress();
        
        // Draw test progress
        let overlay_y = 50.0;
        draw_text(&format!("🤖 VISUAL TEST BOT"), 10.0, overlay_y, 20.0, YELLOW);
        draw_text(&format!("Test {}/{} ({:.0}%)", current + 1, total, progress * 100.0), 
                 10.0, overlay_y + 25.0, 16.0, WHITE);
        
        if current < self.test_positions.len() {
            let target = &self.test_positions[current];
            draw_text(&format!("Target: {}", target.description), 
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
        
        // Draw screenshot info
        if !self.screenshot_results.is_empty() {
            let passed = self.screenshot_results.iter().filter(|r| r.matches).count();
            let total = self.screenshot_results.len();
            draw_text(&format!("📸 Screenshots: {}/{} passed", passed, total), 
                     10.0, overlay_y + 85.0, 14.0, 
                     if passed == total { GREEN } else { RED });
        }
    }
    
    /// Generate final test report
    fn generate_final_report(&self) {
        let report = self.screenshot_validator.generate_report(&self.screenshot_results);
        println!("\n{}", report);
        
        // Save report to file
        if let Err(e) = std::fs::write("test_screenshots/report.txt", &report) {
            println!("⚠️ Failed to save report: {}", e);
        } else {
            println!("📄 Report saved to: test_screenshots/report.txt");
        }
    }
}

/// Run automated visual tests
pub async fn run_visual_tests(test_duration: u64, auto_close: bool) {
    println!("🤖 Starting automated visual tests...");
    println!("   Duration: {}s", test_duration);
    println!("   Auto-close: {}", auto_close);
    
    let mut game_state = GameState::new();
    let mut bot = VisualTestBot::new(test_duration, auto_close);
    
    loop {
        clear_background(Color::new(0.4, 0.6, 0.9, 1.0));
        
        let delta_time = get_frame_time();
        
        // Update bot (returns false when test should end)
        if !bot.update(&mut game_state, delta_time).await {
            break;
        }
        
        // Render game
        game_state.update(delta_time);
        game_state.draw();
        
        // Draw bot overlay
        bot.draw_overlay();
        
        // Check for manual exit
        if is_key_pressed(KeyCode::Escape) {
            println!("🤖 Visual test manually interrupted");
            break;
        }
        
        next_frame().await;
    }
    
    println!("🤖 Visual test completed!");
} 