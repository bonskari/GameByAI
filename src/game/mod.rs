//! Game logic modules
//! 
//! This module contains all game-related components:
//! - Map system and collision detection
//! - Player entity and movement
//! - Game state management
//! - Modern 3D rendering with texture support

pub mod map;
pub mod player;
pub mod state;
pub mod renderer_3d;
pub mod rendering;
pub mod textures;

// Re-export main types for convenience
pub use map::Map;
pub use player::Player;
pub use state::GameState;
pub use rendering::Modern3DRenderer; 