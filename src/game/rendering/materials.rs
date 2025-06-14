//! Material and texture management for rendering

use macroquad::prelude::*;
use crate::game::textures::{TextureType, LoadedTexture};
use std::collections::HashMap;

/// Manages materials and their association with textures
#[derive(Debug)]
pub struct MaterialManager {
    /// Loaded textures by type
    textures: HashMap<TextureType, LoadedTexture>,
    /// Whether textures are currently enabled
    textures_enabled: bool,
}

impl MaterialManager {
    /// Create a new material manager
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            textures_enabled: false,
        }
    }

    /// Add a loaded texture to the manager
    pub fn add_texture(&mut self, texture: LoadedTexture) {
        self.textures.insert(texture.texture_type, texture);
    }

    /// Get a texture by type
    pub fn get_texture(&self, texture_type: TextureType) -> Option<&LoadedTexture> {
        self.textures.get(&texture_type)
    }

    /// Enable or disable texture rendering
    pub fn set_textures_enabled(&mut self, enabled: bool) {
        self.textures_enabled = enabled;
    }

    /// Check if textures are enabled
    pub fn textures_enabled(&self) -> bool {
        self.textures_enabled
    }

    /// Get the color for a texture at a specific UV coordinate
    /// This is used for vertex coloring when textures aren't directly supported
    pub fn get_texture_color(&self, texture_type: TextureType, uv: Vec2) -> Color {
        // For now, return procedural colors based on texture type
        // This will be replaced with actual texture sampling later
        match texture_type {
            TextureType::TechPanel => {
                let base = Color::new(0.6, 0.62, 0.65, 1.0);
                let pattern = ((uv.x * 8.0).sin() * (uv.y * 8.0).cos()).abs();
                Color::new(
                    base.r + pattern * 0.3,
                    base.g + pattern * 0.3,
                    base.b + pattern * 0.3,
                    1.0,
                )
            }
            TextureType::HullPlating => {
                let base = Color::new(0.3, 0.325, 0.35, 1.0);
                let rivet_pattern = if (uv.x * 4.0) % 1.0 < 0.1 || (uv.y * 4.0) % 1.0 < 0.1 {
                    0.5
                } else {
                    0.0
                };
                Color::new(
                    base.r + rivet_pattern * 0.2,
                    base.g + rivet_pattern * 0.2,
                    base.b + rivet_pattern * 0.2,
                    1.0,
                )
            }
            TextureType::ControlSystem => {
                let base = Color::new(0.12, 0.15, 0.21, 1.0);
                let circuit = ((uv.x * 16.0).sin() + (uv.y * 16.0).cos()).abs() * 0.5;
                Color::new(
                    base.r + circuit * 0.4,
                    base.g + circuit * 0.6,
                    base.b + circuit * 0.8,
                    1.0,
                )
            }
            TextureType::EnergyConduit => {
                let base = Color::new(0.22, 0.305, 0.39, 1.0);
                let energy = ((uv.x + uv.y) * 6.0).sin().abs();
                Color::new(
                    base.r + energy * 0.1,
                    base.g + energy * 0.15,
                    base.b + energy * 0.2,
                    1.0,
                )
            }
        }
    }

    /// Get all loaded texture types
    pub fn loaded_texture_types(&self) -> Vec<TextureType> {
        self.textures.keys().copied().collect()
    }

    /// Check if a specific texture type is loaded
    pub fn has_texture(&self, texture_type: TextureType) -> bool {
        self.textures.contains_key(&texture_type)
    }

    /// Clear all loaded textures
    pub fn clear(&mut self) {
        self.textures.clear();
    }
} 