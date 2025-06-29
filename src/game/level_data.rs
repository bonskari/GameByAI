//! World configuration system for loading world data from JSON

use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;
use notify::{Watcher, RecursiveMode, Event, EventKind, Result as NotifyResult};
use std::sync::mpsc::{channel, Receiver};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

/// World configuration loaded from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelData {
    #[serde(default)]
    pub player: Option<PlayerConfig>,
    pub lights: Vec<LightConfig>,
    pub objects: Vec<ObjectConfig>,
    pub settings: Option<WorldSettings>,
}

impl PartialEq for LevelData {
    fn eq(&self, other: &Self) -> bool {
        self.player == other.player &&
        self.lights == other.lights && 
        self.objects == other.objects && 
        self.settings == other.settings
    }
}

/// Player configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerConfig {
    #[serde(default)]
    pub name: Option<String>,      // Optional name for identification
    pub spawn_position: [f32; 3],  // [x, y, z] spawn position
    #[serde(default = "default_player_rotation")]
    pub spawn_rotation: [f32; 2],  // [yaw, pitch] in radians
    #[serde(default = "default_move_speed")]
    pub move_speed: f32,           // Movement speed
    #[serde(default = "default_turn_speed")]
    pub turn_speed: f32,           // Turn speed
    #[serde(default = "default_mouse_sensitivity")]
    pub mouse_sensitivity: f32,    // Mouse sensitivity
    #[serde(default = "default_player_height")]
    pub height: f32,               // Player collision height
    #[serde(default = "default_player_radius")]
    pub radius: f32,               // Player collision radius
    #[serde(default)]
    pub enabled: bool,
}

fn default_player_rotation() -> [f32; 2] {
    [0.0, 0.0]
}

fn default_move_speed() -> f32 {
    8.0
}

fn default_turn_speed() -> f32 {
    3.0
}

fn default_mouse_sensitivity() -> f32 {
    0.001
}

fn default_player_height() -> f32 {
    1.8
}

fn default_player_radius() -> f32 {
    0.25
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            name: Some("Player".to_string()),
            spawn_position: [1.5, 0.6, 1.5], // Default spawn in corner
            spawn_rotation: default_player_rotation(),
            move_speed: default_move_speed(),
            turn_speed: default_turn_speed(),
            mouse_sensitivity: default_mouse_sensitivity(),
            height: default_player_height(),
            radius: default_player_radius(),
            enabled: true,
        }
    }
}

/// Light configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LightConfig {
    #[serde(default)]
    pub name: Option<String>,      // Optional name for identification and debugging
    #[serde(rename = "type")]
    pub light_type: String,        // "omni", "directional", "spot"
    pub position: [f32; 3],        // [x, y, z]
    pub color: [f32; 4],           // [r, g, b, a]
    pub intensity: f32,
    pub radius: f32,
    #[serde(default)]
    pub enabled: bool,
}

/// Object configuration  
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ObjectConfig {
    #[serde(default)]
    pub name: Option<String>,      // Optional name for identification and debugging
    pub mesh: String,              // Mesh: primitive ("cube", "sphere") or file path ("models/chair.obj")
    #[serde(default)]
    pub texture: Option<String>,   // Optional texture file path
    #[serde(default = "default_collision_type")]
    pub collision_type: String,    // "solid", "trigger", "none"
    pub position: [f32; 3],        // [x, y, z]
    #[serde(default = "default_scale")]
    pub scale: [f32; 3],           // [x, y, z] - defaults to [1,1,1]
    #[serde(default = "default_rotation")]
    pub rotation: [f32; 3],        // [x, y, z] rotation in radians
    #[serde(default)]
    pub color: Option<[f32; 4]>,   // Optional color override
    #[serde(default)]
    pub enabled: bool,
}

/// World settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorldSettings {
    #[serde(default)]
    pub ambient_light: Option<[f32; 4]>,  // Global ambient lighting
    #[serde(default)]
    pub fog_color: Option<[f32; 4]>,      // Fog color
    #[serde(default)]
    pub fog_density: Option<f32>,         // Fog density
}

fn default_scale() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

fn default_rotation() -> [f32; 3] {
    [0.0, 0.0, 0.0]
}

fn default_collision_type() -> String {
    "solid".to_string()
}

impl Default for LightConfig {
    fn default() -> Self {
        Self {
            name: None,
            light_type: "omni".to_string(),
            position: [0.0, 1.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
            intensity: 1.0,
            radius: 5.0,
            enabled: true,
        }
    }
}

impl Default for ObjectConfig {
    fn default() -> Self {
        Self {
            name: None,
            mesh: "cube".to_string(),
            texture: None,
            collision_type: default_collision_type(),
            position: [0.0, 0.0, 0.0],
            scale: default_scale(),
            rotation: default_rotation(),
            color: None,
            enabled: true,
        }
    }
}

impl LevelData {
    /// Load world configuration from JSON file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: LevelData = serde_json::from_str(&content)?;
        Ok(config)
    }
    
    /// Save world configuration to JSON file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Create a default world configuration
    pub fn default_config() -> Self {
        Self {
            player: Some(PlayerConfig {
                name: Some("Player".to_string()),
                spawn_position: [5.0, 0.6, 5.0], // Center of the map
                spawn_rotation: [0.0, 0.0],
                move_speed: 8.0,
                turn_speed: 3.0,
                mouse_sensitivity: 0.001,
                height: 1.8,
                radius: 0.25,
                enabled: true,
            }),
            lights: vec![
                LightConfig {
                    name: Some("MainLight_Center".to_string()),
                    light_type: "omni".to_string(),
                    position: [5.0, 1.5, 5.0],
                    color: [0.2, 0.4, 1.0, 1.0],  // Blue light
                    intensity: 2.0,
                    radius: 8.0,
                    enabled: true,
                }
            ],
            objects: vec![
                // Could add props, decorative objects, etc.
            ],
            settings: Some(WorldSettings {
                ambient_light: Some([0.1, 0.1, 0.2, 1.0]),  // Subtle blue ambient
                fog_color: None,
                fog_density: None,
            }),
        }
    }
    
    /// Convert to macroquad types for easy use
    pub fn get_light_position(&self, index: usize) -> Option<Vec3> {
        self.lights.get(index).map(|light| {
            Vec3::new(light.position[0], light.position[1], light.position[2])
        })
    }
    
    pub fn get_light_color(&self, index: usize) -> Option<Color> {
        self.lights.get(index).map(|light| {
            Color::new(light.color[0], light.color[1], light.color[2], light.color[3])
        })
    }
    
    pub fn get_object_position(&self, index: usize) -> Option<Vec3> {
        self.objects.get(index).map(|obj| {
            Vec3::new(obj.position[0], obj.position[1], obj.position[2])
        })
    }
    
    pub fn get_object_scale(&self, index: usize) -> Option<Vec3> {
        self.objects.get(index).map(|obj| {
            Vec3::new(obj.scale[0], obj.scale[1], obj.scale[2])
        })
    }
    
    /// Get player spawn position
    pub fn get_player_spawn_position(&self) -> Vec3 {
        if let Some(player) = &self.player {
            Vec3::new(player.spawn_position[0], player.spawn_position[1], player.spawn_position[2])
        } else {
            Vec3::new(1.5, 0.6, 1.5) // Default position
        }
    }
    
    /// Get player spawn rotation (yaw, pitch)
    pub fn get_player_spawn_rotation(&self) -> (f32, f32) {
        if let Some(player) = &self.player {
            (player.spawn_rotation[0], player.spawn_rotation[1])
        } else {
            (0.0, 0.0) // Default rotation
        }
    }
    
    /// Get player configuration or default
    pub fn get_player_config(&self) -> PlayerConfig {
        self.player.clone().unwrap_or_default()
    }
}

/// Hot-reload system for world configuration
pub struct LevelDataHotReload {
    config_path: String,
    config: Arc<Mutex<LevelData>>,
    last_applied_config: Option<LevelData>,
    receiver: Receiver<NotifyResult<Event>>,
    _watcher: notify::RecommendedWatcher, // Keep watcher alive
    pub config_changed: bool,
    pub last_error: Option<String>,
    last_reload_time: std::time::Instant,
    debounce_duration: std::time::Duration,
    last_check_time: std::time::Instant,
    check_interval: std::time::Duration,
}

impl LevelDataHotReload {
    /// Create a new hot-reload system for the given config file
    pub fn new<P: AsRef<Path>>(config_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = config_path.as_ref().to_string_lossy().to_string();
        
        // Load initial configuration
        let initial_config = if Path::new(&config_path).exists() {
            LevelData::load_from_file(&config_path)?
        } else {
            // Create default config if file doesn't exist
            let default_config = LevelData::default_config();
            default_config.save_to_file(&config_path)?;
            println!("Created default world config at: {}", config_path);
            default_config
        };
        
        let config = Arc::new(Mutex::new(initial_config));
        let (sender, receiver) = channel();
        
        // Create file watcher
        let mut watcher = notify::recommended_watcher(sender)?;
        watcher.watch(std::path::Path::new(&config_path), RecursiveMode::NonRecursive)?;
        
        println!("🔥 Hot-reload system initialized for: {}", config_path);
        
        Ok(Self {
            config_path,
            config,
            last_applied_config: None,
            receiver,
            _watcher: watcher,
            config_changed: false,
            last_error: None,
            last_reload_time: Instant::now(),
            debounce_duration: Duration::from_millis(500), // 500ms debounce
            last_check_time: Instant::now(),
            check_interval: Duration::from_secs(5), // Check every 5 seconds
        })
    }
    
    /// Check for file changes and reload if necessary
    pub fn update(&mut self) {
        // Only check for file changes every 5 seconds to reduce overhead
        if self.last_check_time.elapsed() < self.check_interval {
            return;
        }
        
        // Update the last check time
        self.last_check_time = Instant::now();
        
        let mut should_reload = false;
        
        // Process all pending file events
        while let Ok(event_result) = self.receiver.try_recv() {
            if let Ok(event) = event_result {
                match event.kind {
                    EventKind::Modify(_) | EventKind::Create(_) => {
                        should_reload = true;
                    }
                    _ => {}
                }
            }
        }
        
        // Only reload if we haven't reloaded recently (debounce)
        if should_reload && self.last_reload_time.elapsed() >= self.debounce_duration {
            self.reload_config();
        }
    }
    
    /// Force reload the configuration from file
    pub fn reload_config(&mut self) {
        match LevelData::load_from_file(&self.config_path) {
            Ok(new_config) => {
                if let Ok(mut config) = self.config.lock() {
                    // Only mark as changed if the content actually changed
                    if *config != new_config {
                        *config = new_config;
                        self.config_changed = true;
                        self.last_error = None;
                        self.last_reload_time = Instant::now();
                        println!("✅ World config reloaded from: {} (content changed)", self.config_path);
                    } else {
                        // File was touched but content is the same - just update timestamp to prevent spam
                        self.last_reload_time = Instant::now();
                        // Don't print anything for no-change reloads to reduce spam
                    }
                }
            }
            Err(e) => {
                let error_msg = format!("Failed to reload config: {}", e);
                self.last_error = Some(error_msg.clone());
                self.last_reload_time = Instant::now();
                println!("❌ {}", error_msg);
            }
        }
    }
    
    /// Get the current configuration (thread-safe)
    pub fn get_config(&self) -> Option<LevelData> {
        self.config.lock().ok().map(|config| config.clone())
    }
    
    /// Check if the configuration has changed since last check
    pub fn has_changed(&mut self) -> bool {
        let changed = self.config_changed;
        self.config_changed = false; // Reset flag
        changed
    }
    
    /// Get the last error message, if any
    pub fn get_last_error(&mut self) -> Option<String> {
        self.last_error.take()
    }
    
    /// Set the last applied configuration (called after successful application)
    pub fn set_last_applied_config(&mut self, config: LevelData) {
        self.last_applied_config = Some(config);
    }
    
    /// Get configuration differences for smart updates
    pub fn get_config_diff(&self) -> Option<LevelDataDiff> {
        if let (Some(current), Some(last)) = (self.get_config(), &self.last_applied_config) {
            Some(LevelDataDiff::compute(&current, last))
        } else {
            None
        }
    }
    
    /// Save current configuration to file
    pub fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(config) = self.config.lock() {
            config.save_to_file(&self.config_path)?;
            println!("💾 World config saved to: {}", self.config_path);
        }
        Ok(())
    }
    
    /// Create an example world configuration file
    pub fn create_example_config<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
        let example_config = LevelData {
            player: Some(PlayerConfig {
                name: Some("ExamplePlayer".to_string()),
                spawn_position: [5.0, 0.6, 5.0],
                spawn_rotation: [0.0, 0.0],
                move_speed: 8.0,
                turn_speed: 3.0,
                mouse_sensitivity: 0.001,
                height: 1.8,
                radius: 0.25,
                enabled: true,
            }),
            lights: vec![
                LightConfig {
                    name: Some("MainLight_Center".to_string()),
                    light_type: "omni".to_string(),
                    position: [5.0, 1.5, 5.0],
                    color: [0.2, 0.4, 1.0, 1.0],  // Blue light
                    intensity: 2.0,
                    radius: 8.0,
                    enabled: true,
                },
                LightConfig {
                    name: Some("AccentLight_Corner".to_string()),
                    light_type: "omni".to_string(),
                    position: [-5.0, 1.5, -5.0],
                    color: [1.0, 0.4, 0.2, 1.0],  // Orange light
                    intensity: 1.5,
                    radius: 6.0,
                    enabled: true,
                }
            ],
            objects: vec![
                // Example wall
                ObjectConfig {
                    name: Some("Wall_TechPanel_North".to_string()),
                    mesh: "cube".to_string(),
                    texture: Some("tech_panel.png".to_string()),
                    collision_type: "solid".to_string(),
                    position: [3.0, 1.0, 3.0],
                    scale: [1.0, 2.0, 1.0],
                    rotation: [0.0, 0.0, 0.0],
                    color: None,
                    enabled: true,
                },
                // Example decorative sphere
                ObjectConfig {
                    name: Some("Decoration_Sphere_Red".to_string()),
                    mesh: "sphere".to_string(),
                    texture: None,
                    collision_type: "none".to_string(),
                    position: [5.0, 2.0, 5.0],
                    scale: [0.3, 0.3, 0.3],
                    rotation: [0.0, 0.0, 0.0],
                    color: Some([0.8, 0.2, 0.2, 1.0]), // Red
                    enabled: true,
                },
                // Example floor section
                ObjectConfig {
                    name: Some("Floor_Section_Southwest".to_string()),
                    mesh: "plane".to_string(),
                    texture: Some("floor.png".to_string()),
                    collision_type: "none".to_string(),
                    position: [7.0, 0.0, 7.0],
                    scale: [2.0, 1.0, 2.0],
                    rotation: [0.0, 0.0, 0.0],
                    color: None,
                    enabled: true,
                }
            ],
            settings: Some(WorldSettings {
                ambient_light: Some([0.1, 0.1, 0.2, 1.0]),  // Subtle blue ambient
                fog_color: Some([0.2, 0.2, 0.3, 1.0]),
                fog_density: Some(0.02),
            }),
        };
        
        example_config.save_to_file(path)?;
        Ok(())
    }
}

/// Represents changes between two world configurations for smart updates
#[derive(Debug, Clone)]
pub struct LevelDataDiff {
    pub player_changed: bool,
    pub lights_added: Vec<(usize, LightConfig)>,      // index, config
    pub lights_removed: Vec<usize>,                   // indices to remove
    pub lights_modified: Vec<(usize, LightConfig)>,   // index, new config
    pub objects_added: Vec<(usize, ObjectConfig)>,    // index, config
    pub objects_removed: Vec<usize>,                  // indices to remove
    pub objects_modified: Vec<(usize, ObjectConfig)>, // index, new config
    pub settings_changed: bool,
}

impl LevelDataDiff {
    /// Compute differences between current and previous configurations
    pub fn compute(current: &LevelData, previous: &LevelData) -> Self {
        let mut diff = LevelDataDiff {
            player_changed: current.player != previous.player,
            lights_added: Vec::new(),
            lights_removed: Vec::new(),
            lights_modified: Vec::new(),
            objects_added: Vec::new(),
            objects_removed: Vec::new(),
            objects_modified: Vec::new(),
            settings_changed: current.settings != previous.settings,
        };
        
        // Compare lights
        for (i, current_light) in current.lights.iter().enumerate() {
            if let Some(previous_light) = previous.lights.get(i) {
                if current_light != previous_light {
                    diff.lights_modified.push((i, current_light.clone()));
                }
            } else {
                diff.lights_added.push((i, current_light.clone()));
            }
        }
        
        // Find removed lights
        for i in current.lights.len()..previous.lights.len() {
            diff.lights_removed.push(i);
        }
        
        // Compare objects (similar logic)
        for (i, current_object) in current.objects.iter().enumerate() {
            if let Some(previous_object) = previous.objects.get(i) {
                if current_object != previous_object {
                    diff.objects_modified.push((i, current_object.clone()));
                }
            } else {
                diff.objects_added.push((i, current_object.clone()));
            }
        }
        
        // Find removed objects
        for i in current.objects.len()..previous.objects.len() {
            diff.objects_removed.push(i);
        }
        
        diff
    }
    
    /// Check if there are any changes
    pub fn has_changes(&self) -> bool {
        self.player_changed ||
        !self.lights_added.is_empty() || 
        !self.lights_removed.is_empty() || 
        !self.lights_modified.is_empty() ||
        !self.objects_added.is_empty() || 
        !self.objects_removed.is_empty() || 
        !self.objects_modified.is_empty() ||
        self.settings_changed
    }
    
    /// Get a summary of changes for logging
    pub fn get_summary(&self) -> String {
        let mut changes = Vec::new();
        
        if self.player_changed {
            changes.push("player changed".to_string());
        }
        if !self.lights_added.is_empty() {
            changes.push(format!("{} lights added", self.lights_added.len()));
        }
        if !self.lights_removed.is_empty() {
            changes.push(format!("{} lights removed", self.lights_removed.len()));
        }
        if !self.lights_modified.is_empty() {
            changes.push(format!("{} lights modified", self.lights_modified.len()));
        }
        if !self.objects_added.is_empty() {
            changes.push(format!("{} objects added", self.objects_added.len()));
        }
        if !self.objects_removed.is_empty() {
            changes.push(format!("{} objects removed", self.objects_removed.len()));
        }
        if !self.objects_modified.is_empty() {
            changes.push(format!("{} objects modified", self.objects_modified.len()));
        }
        if self.settings_changed {
            changes.push("settings changed".to_string());
        }
        
        if changes.is_empty() {
            "no changes".to_string()
        } else {
            changes.join(", ")
        }
    }
}