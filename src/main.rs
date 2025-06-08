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
    println!("Controls: WASD to move/turn, ESC to exit");
    
    let mut game_state = GameState::new();
    let mut frame_counter = 0;
    
    loop {
        let dt = get_frame_time();
        
        game_state.update(dt);
        game_state.draw();
        
        frame_counter += 1;
        
        // Debug output every 60 frames (about 1 second)
        if frame_counter % 60 == 0 {
            println!("Game running... Frame: {}, FPS: {:.0}", frame_counter, get_fps());
        }
        
        if is_key_pressed(KeyCode::Escape) {
            println!("ESC pressed - exiting game");
            break;
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
        None => {
            run_game().await;
        }
    }
}
