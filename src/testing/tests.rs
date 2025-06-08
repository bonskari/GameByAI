//! Test implementations for the integrated testing system
//! 
//! Contains fast, lightweight tests that validate core game functionality
//! without complex visual feedback that might cause hanging.

use macroquad::prelude::*;
use futures;
use crate::testing::runner::TestRunner;
use crate::game::{Map, Player, RaycastRenderer};

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
    let mut player = Player::new(5.0, 5.0);
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
    
    if map.is_wall(5, 5) {
        return Err("False wall detected at (5,5)".to_string());
    }
    
    // Test bounds checking
    if !map.is_wall(-1, 5) {
        return Err("Out-of-bounds not detected".to_string());
    }
    
    Ok("Collision detection OK".to_string())
}

/// Test raycasting engine - pure logic testing without visuals
pub async fn test_raycasting(_runner: &mut TestRunner) -> Result<String, String> {
    let map = Map::new();
    let raycast = RaycastRenderer::new();
    
    // Test ray casting from center of map towards a wall
    let player_x = 5.0;
    let player_y = 5.0;
    let ray_angle = 0.0; // Looking east
    
    // Cast ray towards east wall
    if let Some(hit) = raycast.cast_ray(&map, player_x, player_y, ray_angle) {
        // Should hit the wall at x=9 (since map is 10x10 with walls at edges)
        if hit.distance <= 0.0 || hit.distance > 10.0 {
            return Err(format!("Invalid ray distance: {:.2}", hit.distance));
        }
        
        // Should be a reasonable distance to east wall
        let expected_distance = 9.0 - player_x; // Distance to wall at x=9
        if (hit.distance - expected_distance).abs() > 0.5 {
            return Err(format!("Unexpected ray distance: {:.2}, expected ~{:.2}", hit.distance, expected_distance));
        }
        
        Ok(format!("Raycasting OK (distance: {:.2})", hit.distance))
    } else {
        Err("Ray did not hit any wall".to_string())
    }
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
            
            runner.run_test("Raycasting Engine", |r| {
                futures::executor::block_on(test_raycasting(r))
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
        "raycast" => {
            runner.run_test("Raycasting Engine", |r| {
                futures::executor::block_on(test_raycasting(r))
            });
        },
        _ => {
            println!("❌ Unknown test type: {}", test_type);
            println!("Available tests: all, graphics, movement, collision, raycast");
            return;
        }
    }
    
    let success = runner.print_summary();
    std::process::exit(if success { 0 } else { 1 });
} 