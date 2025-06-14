//! Texture subsystem
//! 
//! This module handles all texture-related functionality:
//! - Texture loading from files
//! - Procedural texture generation
//! - Texture caching and management
//! - Texture type definitions

pub mod types;
pub mod loader;
pub mod generator;
pub mod cache;

// Re-export main types
pub use types::{TextureType, TextureConfig, LoadedTexture, TextureResult, TextureError};
pub use loader::TextureLoader;
pub use generator::ProceduralGenerator;
pub use cache::TextureCache; 