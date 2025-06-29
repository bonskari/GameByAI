//! Game module - Core game logic and systems
//!
//! This module contains:
//! - Player entity and movement
//! - Map system and collision detection  
//! - 3D rendering system
//! - Texture management

pub mod config;
pub mod map;
pub mod level_generator;
pub mod level_data;
pub mod state;
pub mod ecs_state;
pub mod input;
pub mod player;
pub mod rendering;
pub mod texture_generator;
pub mod textures;
pub mod mesh_export;

// Re-export commonly used types
pub use map::Map;
pub use player::Player;
pub use state::GameState;
pub use config::GameConfig; 