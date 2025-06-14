//! Texture caching and management

use crate::game::textures::{TextureType, LoadedTexture};
use std::collections::HashMap;

/// Manages texture caching and lifecycle
#[derive(Debug)]
pub struct TextureCache {
    textures: HashMap<TextureType, LoadedTexture>,
}

impl TextureCache {
    /// Create a new texture cache
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    /// Add a texture to the cache
    pub fn insert(&mut self, texture: LoadedTexture) {
        self.textures.insert(texture.texture_type, texture);
    }

    /// Get a texture from the cache
    pub fn get(&self, texture_type: TextureType) -> Option<&LoadedTexture> {
        self.textures.get(&texture_type)
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.textures.clear();
    }
} 