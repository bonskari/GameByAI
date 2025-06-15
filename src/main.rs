//! Wolfenstein by AI - Main Entry Point
//! 
//! A Wolfenstein-style game created with AI assistance using Rust and macroquad.
//! Features integrated testing system and modular architecture.

use macroquad::prelude::*;
use clap::Parser;

mod cli;
mod game;
mod testing;

use cli::{Cli, Commands};
use game::GameState;

/// Run the game in interactive mode
async fn run_game() {
    println!("Starting Wolfenstein by AI - Interactive Mode");
    println!("Controls: WASD to move/strafe, Mouse to look, SPACE to jump, ESC to exit");
    
    // Enable mouse capture for FPS-style mouse look
    set_cursor_grab(true);
    show_mouse(false);
    
    // Try to disable V-sync for higher framerates
    #[cfg(not(target_arch = "wasm32"))]
    {
        use macroquad::window::*;
        // Request uncapped framerate if possible
        request_new_screen_size(screen_width(), screen_height());
    }
    
    let mut game_state = GameState::new();
    
    // Load textures (Step 1: Just load them, don't use them yet)
    game_state.modern_3d_renderer.load_textures().await;
    
    let mut frame_counter = 0;
    
    loop {
        let dt = get_frame_time();
        
        game_state.update(dt);
        game_state.draw();
        
        frame_counter += 1;
        
        // Enhanced performance monitoring every 60 frames
        if frame_counter % 60 == 0 {
            let fps = get_fps() as f32;
            let performance_rating = if fps > 200.0 { "🚀 BLAZING" } 
                                   else if fps > 150.0 { "🔥 EXCELLENT" }
                                   else if fps > 120.0 { "✅ GOOD" }
                                   else if fps > 60.0 { "⚠️ OKAY" }
                                   else { "❌ SLOW" };
            
            println!("Game running... Frame: {}, FPS: {:.0} {}", frame_counter, fps, performance_rating);
        }
        
        if is_key_pressed(KeyCode::Escape) {
            println!("ESC pressed - exiting game");
            break;
        }
        
        // Toggle mouse capture with M key
        if is_key_pressed(KeyCode::M) {
            // Toggle between mouse capture states
            use std::sync::atomic::{AtomicBool, Ordering};
            static MOUSE_CAPTURED: AtomicBool = AtomicBool::new(true);
            
            let current = MOUSE_CAPTURED.load(Ordering::Relaxed);
            let new_state = !current;
            MOUSE_CAPTURED.store(new_state, Ordering::Relaxed);
            
            set_cursor_grab(new_state);
            show_mouse(!new_state);
            println!("Mouse capture toggled: {}", new_state);
        }
        
        next_frame().await;
    }
    
    println!("Wolfenstein by AI shutting down...");
}

/// Main entry point
#[macroquad::main("Wolfenstein by AI")]
async fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Test { test_type, timeout, verbose }) => {
            testing::run_tests(&test_type, timeout, verbose).await;
        },
        Some(Commands::VisualTest { duration, no_auto_close }) => {
            testing::run_visual_tests(duration, !no_auto_close).await;
        },
        None => {
            run_game().await;
        }
    }
}
