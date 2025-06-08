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

// Re-export main types for convenience
pub use map::Map;
pub use player::Player;
pub use state::GameState;
// Modern3DRenderer re-export removed - not currently used 