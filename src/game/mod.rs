//! Game module - Core game logic and systems
//!
//! This module contains:
//! - Player entity and movement
//! - Map system and collision detection  
//! - 3D rendering system
//! - Texture management

pub mod map;
pub mod player;
pub mod level_generator;
pub mod input;
pub mod ecs_state;
pub mod state;
pub mod rendering;
pub mod texture_generator;
pub mod textures;
pub mod level_data;
pub mod config;

// Re-export commonly used types
pub use map::Map;
pub use player::Player;
pub use level_generator::LevelMeshBuilder;
pub use input::{InputHandler, PlayerInput};
pub use state::GameState;
pub use ecs_state::EcsGameState;
pub use rendering::DeferredRenderer;
pub use level_data::{LevelData, LevelDataHotReload, PlayerConfig, LightConfig, ObjectConfig};
pub use config::GameConfig; 