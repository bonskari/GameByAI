//! Texture type definitions and configuration

use macroquad::prelude::*;
use std::path::PathBuf;
use std::collections::HashMap;

/// Texture types that can be applied to walls
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureType {
    TechPanel,
    HullPlating,
    ControlSystem,
    EnergyConduit,
}

impl TextureType {
    /// Get all available texture types
    pub fn all() -> Vec<TextureType> {
        vec![
            TextureType::TechPanel,
            TextureType::HullPlating,
            TextureType::ControlSystem,
            TextureType::EnergyConduit,
        ]
    }

    /// Get the filename for this texture type
    pub fn filename(&self) -> &'static str {
        match self {
            TextureType::TechPanel => "tech_panel.png",
            TextureType::HullPlating => "hull_plating.png",
            TextureType::ControlSystem => "control_system.png",
            TextureType::EnergyConduit => "energy_conduit.png",
        }
    }
}

/// Configuration for the texture system
#[derive(Debug, Clone)]
pub struct TextureConfig {
    /// Directory where texture files are stored
    pub texture_directory: PathBuf,
    /// Size of generated textures (width and height)
    pub texture_size: u32,
    /// Whether to use procedural generation or load from files
    pub use_procedural: bool,
    /// Mapping of texture types to file paths (for file-based textures)
    pub texture_files: HashMap<TextureType, PathBuf>,
}

impl Default for TextureConfig {
    fn default() -> Self {
        let mut texture_files = HashMap::new();
        for texture_type in TextureType::all() {
            texture_files.insert(
                texture_type,
                PathBuf::from("textures").join(texture_type.filename()),
            );
        }

        Self {
            texture_directory: PathBuf::from("textures"),
            texture_size: 256,
            use_procedural: true,
            texture_files,
        }
    }
}

/// A loaded texture with metadata
#[derive(Debug, Clone)]
pub struct LoadedTexture {
    pub texture: Texture2D,
    pub texture_type: TextureType,
    pub size: (u32, u32),
}

/// Result of texture operations
pub type TextureResult<T> = Result<T, TextureError>;

/// Errors that can occur in texture operations
#[derive(Debug, Clone)]
pub enum TextureError {
    LoadFailed(String),
    GenerationFailed(String),
    InvalidFormat(String),
    IoError(String),
} 