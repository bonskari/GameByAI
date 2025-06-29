use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

/// Game configuration loaded from config.ini
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub display: DisplayConfig,
    pub graphics: GraphicsConfig,
    pub audio: AudioConfig,
    pub controls: ControlsConfig,
    pub gameplay: GameplayConfig,
    pub development: DevelopmentConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsConfig {
    pub quality: String,  // low/medium/high/ultra
    pub max_fps: u32,     // 0 = unlimited
    pub anti_aliasing: bool,
    pub shadows: String,  // off/low/medium/high
    pub textures: String, // low/medium/high
    pub render_distance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub audio_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlsConfig {
    pub mouse_sensitivity: f32,
    pub invert_mouse_y: bool,
    pub move_speed: f32,
    pub jump_strength: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameplayConfig {
    pub fov: f32,
    pub show_fps: bool,
    pub show_debug: bool,
    pub auto_save_interval: u32, // seconds, 0 = disabled
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentConfig {
    pub hot_reload: bool,
    pub default_level: String,
    pub performance_monitoring: bool,
    pub log_level: String,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            display: DisplayConfig {
                width: 1920,
                height: 1080,
                fullscreen: false,
                vsync: true,
                title: "GameByAI - 3D Engine".to_string(),
            },
            graphics: GraphicsConfig {
                quality: "medium".to_string(),
                max_fps: 144,
                anti_aliasing: true,
                shadows: "medium".to_string(),
                textures: "high".to_string(),
                render_distance: 1.0,
            },
            audio: AudioConfig {
                master_volume: 0.8,
                music_volume: 0.6,
                sfx_volume: 0.8,
                audio_enabled: true,
            },
            controls: ControlsConfig {
                mouse_sensitivity: 1.0,
                invert_mouse_y: false,
                move_speed: 1.0,
                jump_strength: 1.0,
            },
            gameplay: GameplayConfig {
                fov: 90.0,
                show_fps: true,
                show_debug: false,
                auto_save_interval: 300,
            },
            development: DevelopmentConfig {
                hot_reload: true,
                default_level: "maps/default_level.json".to_string(),
                performance_monitoring: true,
                log_level: "info".to_string(),
            },
        }
    }
}

impl GameConfig {
    /// Load configuration from INI file
    pub fn load_from_ini<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        Self::parse_ini(&content)
    }

    /// Parse INI content into GameConfig
    fn parse_ini(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = Self::default();
        let mut current_section = String::new();

        for line in content.lines() {
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse section headers
            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len()-1].to_lowercase();
                continue;
            }

            // Parse key-value pairs
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim();
                let value = line[eq_pos + 1..].trim();

                match current_section.as_str() {
                    "display" => config.parse_display_setting(key, value)?,
                    "graphics" => config.parse_graphics_setting(key, value)?,
                    "audio" => config.parse_audio_setting(key, value)?,
                    "controls" => config.parse_controls_setting(key, value)?,
                    "gameplay" => config.parse_gameplay_setting(key, value)?,
                    "development" => config.parse_development_setting(key, value)?,
                    _ => {
                        println!("⚠️ Unknown config section: {}", current_section);
                    }
                }
            }
        }

        Ok(config)
    }

    fn parse_display_setting(&mut self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        match key {
            "width" => self.display.width = value.parse()?,
            "height" => self.display.height = value.parse()?,
            "fullscreen" => self.display.fullscreen = value.parse()?,
            "vsync" => self.display.vsync = value.parse()?,
            "title" => self.display.title = value.to_string(),
            _ => println!("⚠️ Unknown display setting: {}", key),
        }
        Ok(())
    }

    fn parse_graphics_setting(&mut self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        match key {
            "quality" => self.graphics.quality = value.to_string(),
            "max_fps" => self.graphics.max_fps = value.parse()?,
            "anti_aliasing" => self.graphics.anti_aliasing = value.parse()?,
            "shadows" => self.graphics.shadows = value.to_string(),
            "textures" => self.graphics.textures = value.to_string(),
            "render_distance" => self.graphics.render_distance = value.parse()?,
            _ => println!("⚠️ Unknown graphics setting: {}", key),
        }
        Ok(())
    }

    fn parse_audio_setting(&mut self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        match key {
            "master_volume" => self.audio.master_volume = value.parse()?,
            "music_volume" => self.audio.music_volume = value.parse()?,
            "sfx_volume" => self.audio.sfx_volume = value.parse()?,
            "audio_enabled" => self.audio.audio_enabled = value.parse()?,
            _ => println!("⚠️ Unknown audio setting: {}", key),
        }
        Ok(())
    }

    fn parse_controls_setting(&mut self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        match key {
            "mouse_sensitivity" => self.controls.mouse_sensitivity = value.parse()?,
            "invert_mouse_y" => self.controls.invert_mouse_y = value.parse()?,
            "move_speed" => self.controls.move_speed = value.parse()?,
            "jump_strength" => self.controls.jump_strength = value.parse()?,
            _ => println!("⚠️ Unknown controls setting: {}", key),
        }
        Ok(())
    }

    fn parse_gameplay_setting(&mut self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        match key {
            "fov" => self.gameplay.fov = value.parse()?,
            "show_fps" => self.gameplay.show_fps = value.parse()?,
            "show_debug" => self.gameplay.show_debug = value.parse()?,
            "auto_save_interval" => self.gameplay.auto_save_interval = value.parse()?,
            _ => println!("⚠️ Unknown gameplay setting: {}", key),
        }
        Ok(())
    }

    fn parse_development_setting(&mut self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        match key {
            "hot_reload" => self.development.hot_reload = value.parse()?,
            "default_level" => self.development.default_level = value.to_string(),
            "performance_monitoring" => self.development.performance_monitoring = value.parse()?,
            "log_level" => self.development.log_level = value.to_string(),
            _ => println!("⚠️ Unknown development setting: {}", key),
        }
        Ok(())
    }

    /// Save configuration to INI file
    pub fn save_to_ini<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let ini_content = self.to_ini_string();
        fs::write(path, ini_content)?;
        Ok(())
    }

    /// Convert configuration to INI format string
    fn to_ini_string(&self) -> String {
        format!(
r#"[Display]
# Window resolution (width x height)
width = {}
height = {}
# Fullscreen mode (true/false)
fullscreen = {}
# VSync enabled (true/false)
vsync = {}
# Window title
title = {}

[Graphics]
# Graphics quality preset (low/medium/high/ultra)
quality = {}
# Maximum FPS (0 = unlimited)
max_fps = {}
# Enable anti-aliasing (true/false)
anti_aliasing = {}
# Shadow quality (off/low/medium/high)
shadows = {}
# Texture quality (low/medium/high)
textures = {}
# Render distance multiplier (0.5 - 2.0)
render_distance = {}

[Audio]
# Master volume (0.0 - 1.0)
master_volume = {}
# Music volume (0.0 - 1.0)
music_volume = {}
# Sound effects volume (0.0 - 1.0)
sfx_volume = {}
# Enable audio (true/false)
audio_enabled = {}

[Controls]
# Mouse sensitivity (0.1 - 5.0)
mouse_sensitivity = {}
# Invert mouse Y axis (true/false)
invert_mouse_y = {}
# Movement speed multiplier (0.5 - 2.0)
move_speed = {}
# Jump strength multiplier (0.5 - 2.0)
jump_strength = {}

[Gameplay]
# Field of view in degrees (60 - 120)
fov = {}
# Show FPS counter (true/false)
show_fps = {}
# Show debug info (true/false)
show_debug = {}
# Auto-save interval in seconds (0 = disabled)
auto_save_interval = {}

[Development]
# Enable hot-reload for level data (true/false)
hot_reload = {}
# Default level file
default_level = {}
# Enable performance monitoring (true/false)
performance_monitoring = {}
# Log level (trace/debug/info/warn/error)
log_level = {}
"#,
            self.display.width, self.display.height, self.display.fullscreen, self.display.vsync, self.display.title,
            self.graphics.quality, self.graphics.max_fps, self.graphics.anti_aliasing, self.graphics.shadows, self.graphics.textures, self.graphics.render_distance,
            self.audio.master_volume, self.audio.music_volume, self.audio.sfx_volume, self.audio.audio_enabled,
            self.controls.mouse_sensitivity, self.controls.invert_mouse_y, self.controls.move_speed, self.controls.jump_strength,
            self.gameplay.fov, self.gameplay.show_fps, self.gameplay.show_debug, self.gameplay.auto_save_interval,
            self.development.hot_reload, self.development.default_level, self.development.performance_monitoring, self.development.log_level
        )
    }

    /// Apply configuration settings to macroquad window
    pub fn apply_window_settings(&self) {
        // Note: Some settings like resolution and fullscreen need to be applied during window creation
        // These utility methods help access the config values
    }

    /// Get window configuration for macroquad
    pub fn get_window_conf(&self) -> macroquad::window::Conf {
        macroquad::window::Conf {
            window_title: self.display.title.clone(),
            window_width: self.display.width as i32,
            window_height: self.display.height as i32,
            fullscreen: self.display.fullscreen,
            ..Default::default()
        }
    }

    /// Validate configuration values and fix any invalid ones
    pub fn validate_and_fix(&mut self) {
        // Clamp values to valid ranges
        self.graphics.render_distance = self.graphics.render_distance.clamp(0.5, 2.0);
        self.controls.mouse_sensitivity = self.controls.mouse_sensitivity.clamp(0.1, 5.0);
        self.controls.move_speed = self.controls.move_speed.clamp(0.5, 2.0);
        self.controls.jump_strength = self.controls.jump_strength.clamp(0.5, 2.0);
        self.gameplay.fov = self.gameplay.fov.clamp(60.0, 120.0);
        self.audio.master_volume = self.audio.master_volume.clamp(0.0, 1.0);
        self.audio.music_volume = self.audio.music_volume.clamp(0.0, 1.0);
        self.audio.sfx_volume = self.audio.sfx_volume.clamp(0.0, 1.0);

        // Validate string values
        if !["low", "medium", "high", "ultra"].contains(&self.graphics.quality.as_str()) {
            self.graphics.quality = "medium".to_string();
        }
        if !["off", "low", "medium", "high"].contains(&self.graphics.shadows.as_str()) {
            self.graphics.shadows = "medium".to_string();
        }
        if !["low", "medium", "high"].contains(&self.graphics.textures.as_str()) {
            self.graphics.textures = "high".to_string();
        }
        if !["trace", "debug", "info", "warn", "error"].contains(&self.development.log_level.as_str()) {
            self.development.log_level = "info".to_string();
        }
    }
}

/// Helper functions for easy configuration access
impl GameConfig {
    pub fn get_default_level_path(&self) -> &str {
        &self.development.default_level
    }

    pub fn is_hot_reload_enabled(&self) -> bool {
        self.development.hot_reload
    }

    pub fn should_show_fps(&self) -> bool {
        self.gameplay.show_fps
    }

    pub fn should_show_debug(&self) -> bool {
        self.gameplay.show_debug
    }

    pub fn get_mouse_sensitivity(&self) -> f32 {
        self.controls.mouse_sensitivity
    }

    pub fn get_move_speed_multiplier(&self) -> f32 {
        self.controls.move_speed
    }
} 