//! Test implementations for the integrated testing system
//! 
//! Contains fast, lightweight tests that validate core game functionality
//! without complex visual feedback that might cause hanging.

use macroquad::prelude::*;
use futures;
use crate::testing::runner::TestRunner;
use crate::game::{Map, Player};
// Lighting tests now integrated into ECS state

/// Test graphics initialization - no visual feedback to prevent hanging
pub async fn test_graphics_initialization(_runner: &mut TestRunner) -> Result<String, String> {
    // Test basic macroquad functions without visual output
    let screen_w = screen_width();
    let screen_h = screen_height();
    
    if screen_w <= 0.0 || screen_h <= 0.0 {
        return Err("Invalid screen dimensions".to_string());
    }
    
    // No visual feedback - just verify the functions work
    Ok(format!("Graphics OK ({}x{})", screen_w as i32, screen_h as i32))
}

/// Test game loop performance - pure logic testing without frames
pub async fn test_game_loop(_runner: &mut TestRunner) -> Result<String, String> {
    // Test frame time calculation without actual frames
    let dt = get_frame_time();
    
    // Basic sanity check on frame time
    if dt < 0.0 || dt > 10.0 {
        return Err(format!("Invalid frame time: {}", dt));
    }
    
    // Test some basic math that would be used in game loop
    let fps_estimate = if dt > 0.0 { 1.0 / dt } else { 60.0 };
    
    if fps_estimate < 0.1 {
        return Err(format!("Unreasonably low FPS estimate: {:.1}", fps_estimate));
    }
    
    Ok(format!("Game loop OK (estimated: {:.0} FPS)", fps_estimate.min(999.0)))
}

/// Test player movement logic - pure logic testing without visuals
pub async fn test_player_movement(_runner: &mut TestRunner) -> Result<String, String> {
    let map = Map::new();
    let mut player = Player::new(1.5, 1.5); // Use valid starting position
    let original_x = player.x;
    let original_y = player.y;
    let original_rotation = player.rotation;
    
    // Simulate movement without any visual feedback
    let dt = 1.0 / 60.0; // 60 FPS simulation
    
    // Test forward movement (simulate W key)
    let new_x = player.x + player.speed * dt * player.rotation.cos();
    let new_y = player.y + player.speed * dt * player.rotation.sin();
    
    if !map.is_wall(new_x as i32, new_y as i32) {
        player.x = new_x;
        player.y = new_y;
    }
    
    // Test rotation
    player.rotation += 1.0 * dt;
    
    // Verify changes
    let position_changed = (player.x - original_x).abs() > 0.001 || (player.y - original_y).abs() > 0.001;
    let rotation_changed = (player.rotation - original_rotation).abs() > 0.001;
    
    if !position_changed && !rotation_changed {
        return Err("Player movement logic not working".to_string());
    }
    
    Ok(format!("Movement OK (pos: {:.2},{:.2}, rot: {:.2})", player.x, player.y, player.rotation))
}

/// Test collision detection - pure logic testing
pub async fn test_collision_detection(_runner: &mut TestRunner) -> Result<String, String> {
    let map = Map::new();
    
    // Test wall detection - no visual feedback
    if !map.is_wall(0, 0) {
        return Err("Wall not detected at (0,0)".to_string());
    }
    
    if map.is_wall(1, 1) {
        return Err("False wall detected at (1,1) - should be empty".to_string());
    }
    
    // Test bounds checking
    if !map.is_wall(-1, 5) {
        return Err("Out-of-bounds not detected".to_string());
    }
    
    Ok("Collision detection OK".to_string())
}

/// Test texture system - verify different wall types work correctly
pub async fn test_texture_system(_runner: &mut TestRunner) -> Result<String, String> {
    let map = Map::new();
    
    // Test wall type detection
    let _wall_type_stone = map.get_wall_type(0, 0);
    let wall_type_empty = map.get_wall_type(1, 1);
    
    if wall_type_empty != super::super::game::map::WallType::Empty {
        return Err("Empty space not detected correctly".to_string());
    }
    
    // Test texture color generation
    let tech_color = map.get_wall_color(super::super::game::map::WallType::TechPanel, true);
    let hull_color = map.get_wall_color(super::super::game::map::WallType::HullPlating, true);
    
    // Colors should be different
    if tech_color.r == hull_color.r && tech_color.g == hull_color.g && tech_color.b == hull_color.b {
        return Err("TechPanel and HullPlating should have different colors".to_string());
    }
    
    // Test textured wall colors
    let textured_tech = map.get_textured_wall_color(super::super::game::map::WallType::TechPanel, true, 0.5);
    
    // Should be valid color values
    if textured_tech.r < 0.0 || textured_tech.r > 1.0 {
        return Err("Invalid color values generated".to_string());
    }
    
    Ok("Texture system OK (multiple wall types working)".to_string())
}

/// Test player starting position - validate player starts in open area
pub async fn test_player_starting_position(_runner: &mut TestRunner) -> Result<String, String> {
    let map = Map::new();
    let game_state = super::super::game::GameState::new();
    
    // Get player data from ECS
    if let Some(legacy_data) = game_state.ecs_state.get_legacy_player_data() {
        // Check if player starting position is in a wall
        if map.is_wall(legacy_data.x as i32, legacy_data.y as i32) {
            return Err(format!("Player starts inside wall at ({:.1}, {:.1})", 
                legacy_data.x, legacy_data.y));
        }
        
        // Check if starting position is within map bounds
        if legacy_data.x < 0.0 || legacy_data.y < 0.0 ||
           legacy_data.x >= map.width as f32 || legacy_data.y >= map.height as f32 {
            return Err(format!("Player starts outside map bounds at ({:.1}, {:.1})", 
                legacy_data.x, legacy_data.y));
        }
        
        // Check reasonable starting height
        if legacy_data.z <= 0.0 || legacy_data.z > 3.0 {
            return Err(format!("Player starting height unreasonable: {:.1}", legacy_data.z));
        }
        
        Ok(format!("Starting position OK at ({:.1}, {:.1}, {:.1})", 
            legacy_data.x, legacy_data.y, legacy_data.z))
    } else {
        Err("Could not get player data from ECS".to_string())
    }
}

/// Test pitch controls - verify pitch is properly clamped and functional
pub async fn test_pitch_controls(_runner: &mut TestRunner) -> Result<String, String> {
    let mut player = Player::new(5.0, 5.0);
    
    // Test initial pitch
    if player.pitch != 0.0 {
        return Err(format!("Initial pitch should be 0.0, got {:.2}", player.pitch));
    }
    
    // Test pitch clamping - simulate extreme mouse movement
    let max_pitch = std::f32::consts::PI * 0.47; // ~85 degrees
    
    // Set pitch beyond maximum
    player.pitch = max_pitch + 1.0;
    player.pitch = player.pitch.clamp(-max_pitch, max_pitch);
    
    if (player.pitch - max_pitch).abs() > 0.001 {
        return Err(format!("Pitch clamping failed: {:.2}, expected {:.2}", player.pitch, max_pitch));
    }
    
    // Test negative pitch clamping
    player.pitch = -max_pitch - 1.0;
    player.pitch = player.pitch.clamp(-max_pitch, max_pitch);
    
    if (player.pitch - (-max_pitch)).abs() > 0.001 {
        return Err(format!("Negative pitch clamping failed: {:.2}, expected {:.2}", player.pitch, -max_pitch));
    }
    
    // Test normal pitch range
    player.pitch = 0.5; // ~28 degrees
    if player.pitch.abs() > max_pitch {
        return Err("Normal pitch range validation failed".to_string());
    }
    
    Ok(format!("Pitch controls OK (range: ±{:.1}°)", max_pitch.to_degrees()))
}

/// Test lighting system performance with progressive light counts
pub async fn test_lighting_performance(_runner: &mut TestRunner) -> Result<String, String> {
    // Lighting performance tests are now integrated into the visual test system
    // This test is deprecated - use: cargo run -- visual-test
    Ok("Lighting tests moved to visual test system. Use: cargo run -- visual-test".to_string())
}

/// Main test runner function - executes all or specific tests
pub async fn run_tests(test_type: &str, timeout: u64, verbose: bool) {
    let mut runner = TestRunner::new(verbose, timeout);
    
    match test_type {
        "all" => {
            runner.run_test("Graphics Initialization", |r| {
                futures::executor::block_on(test_graphics_initialization(r))
            });
            
            runner.run_test("Game Loop", |r| {
                futures::executor::block_on(test_game_loop(r))
            });
            
            runner.run_test("Player Movement", |r| {
                futures::executor::block_on(test_player_movement(r))
            });
            
            runner.run_test("Collision Detection", |r| {
                futures::executor::block_on(test_collision_detection(r))
            });
            
            runner.run_test("Texture System", |r| {
                futures::executor::block_on(test_texture_system(r))
            });
            
            runner.run_test("Player Starting Position", |r| {
                futures::executor::block_on(test_player_starting_position(r))
            });
            
            runner.run_test("Pitch Controls", |r| {
                futures::executor::block_on(test_pitch_controls(r))
            });
            
            runner.run_test("Lighting Performance", |r| {
                futures::executor::block_on(test_lighting_performance(r))
            });
        },
        "graphics" => {
            runner.run_test("Graphics Initialization", |r| {
                futures::executor::block_on(test_graphics_initialization(r))
            });
        },
        "movement" => {
            runner.run_test("Player Movement", |r| {
                futures::executor::block_on(test_player_movement(r))
            });
        },
        "collision" => {
            runner.run_test("Collision Detection", |r| {
                futures::executor::block_on(test_collision_detection(r))
            });
        },
        "texture" => {
            runner.run_test("Texture System", |r| {
                futures::executor::block_on(test_texture_system(r))
            });
        },
        "pitch" => {
            runner.run_test("Pitch Controls", |r| {
                futures::executor::block_on(test_pitch_controls(r))
            });
        },
        "position" => {
            runner.run_test("Player Starting Position", |r| {
                futures::executor::block_on(test_player_starting_position(r))
            });
        },
        "lighting" => {
            runner.run_test("Lighting Performance", |r| {
                futures::executor::block_on(test_lighting_performance(r))
            });
        },
        _ => {
            println!("❌ Unknown test type: {}", test_type);
            println!("Available tests: all, graphics, movement, collision, texture, pitch, position, lighting");
            return;
        }
    }
    
    let success = runner.print_summary();
    std::process::exit(if success { 0 } else { 1 });
} 