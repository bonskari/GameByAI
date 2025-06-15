//! GameByAI - Main Entry Point
//!
//! A 3D first-person game created with AI assistance using Rust and macroquad.
//! Features integrated testing system and modular architecture.

use macroquad::prelude::*;
use clap::Parser;

mod cli;
mod game;
mod testing;
mod ecs;

use cli::{Cli, Commands};
use game::GameState;

/// Window configuration for fullscreen mode with native resolution
fn window_conf() -> Conf {
    Conf {
        window_title: "GameByAI - 3D Game Engine".to_owned(),
        fullscreen: true,
        window_width: 0,  // 0 means use native resolution
        window_height: 0, // 0 means use native resolution
        window_resizable: false,
        ..Default::default()
    }
}

/// Initialize a new game state with all necessary setup
async fn initialize_game() -> GameState {
    let mut game_state = GameState::new();
    
    // Load textures for 3D rendering
    game_state.modern_3d_renderer.load_textures().await;
    
    game_state
}

/// Run the main game loop
async fn run_game_loop(mut game_state: GameState, test_duration: Option<u64>) {
    let mut frame_counter = 0;
    let start_time = std::time::Instant::now();
    
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
        
        // Check for exit conditions
        if is_key_pressed(KeyCode::Escape) {
            println!("ESC pressed - exiting game");
            break;
        }
        
        // For visual tests, check if test duration exceeded or test bot finished
        if let Some(duration) = test_duration {
            if start_time.elapsed().as_secs() >= duration || game_state.ecs_state.is_test_bot_finished() {
                println!("🤖 Visual test completed");
                break;
            }
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
    
    println!("Game loop ended");
}

/// Run the game in interactive mode
async fn run_game() {
    println!("Starting GameByAI - Interactive Mode");
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
    
    let game_state = initialize_game().await;
    run_game_loop(game_state, None).await;
    
    println!("GameByAI shutting down...");
}

/// Run visual tests (same as normal game + test bot)
pub async fn run_visual_tests(test_duration: u64, auto_close: bool) {
    println!("🤖 Starting automated visual tests...");
    println!("   Duration: {}s", test_duration);
    println!("   Auto-close: {}", auto_close);
    
    // Initialize exactly like normal game
    let mut game_state = initialize_game().await;
    
    // Add test bot on top of normal game
    game_state.ecs_state.attach_test_bot(test_duration);
    
    // Run normal game loop with test bot active
    run_game_loop(game_state, Some(test_duration)).await;
}

/// Main entry point
#[macroquad::main(window_conf)]
async fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Test { test_type, timeout, verbose }) => {
            testing::run_tests(&test_type, timeout, verbose).await;
        },
        Some(Commands::VisualTest { duration, no_auto_close }) => {
            run_visual_tests(duration, !no_auto_close).await;
        },
        None => {
            run_game().await;
        }
    }
}
