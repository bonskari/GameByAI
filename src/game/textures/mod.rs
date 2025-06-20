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
pub mod ai_generator;
pub mod candle_generator;

// Re-export main types
pub use types::{TextureType, LoadedTexture, TextureResult, TextureError}; 