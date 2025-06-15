//! Game module - Core game logic and systems
//!
//! This module contains:
//! - Player entity and movement
//! - Map system and collision detection  
//! - 3D rendering system
//! - Texture management

pub mod input;
pub mod player;
pub mod map;
pub mod state;
pub mod ecs_state;
pub mod rendering;
pub mod textures;

// Re-export commonly used types
pub use input::{InputHandler, PlayerInput};
pub use player::Player;
pub use map::Map;
pub use state::GameState;
pub use ecs_state::EcsGameState;
pub use rendering::Modern3DRenderer; 