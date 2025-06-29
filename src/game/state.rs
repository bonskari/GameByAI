use macroquad::prelude::*;
use std::time::Instant;
use super::{map::Map, player::Player, input::{InputHandler, PlayerInput}};
use super::rendering::DeferredRenderer;
use super::ecs_state::EcsGameState;
use super::level_data::LevelDataHotReload;
use super::config::GameConfig;

/// Overall game state for testing and gameplay
pub struct GameState {
    pub map: Map,
    pub frame_count: u32,
    pub start_time: Instant,
    pub deferred_renderer: DeferredRenderer,
    pub view_mode_3d: bool, // Toggle between 2D and 3D view

    // ECS system
    pub ecs_state: EcsGameState,
    // Centralized input handling
    pub input_handler: InputHandler,
    // Hot-reload system for world configuration
    pub level_data_hot_reload: Option<LevelDataHotReload>,
    // Track light entities by config index for smart updates
    pub world_config_light_entities: Vec<Option<crate::ecs::Entity>>,
    // Track object entities by config index for smart updates
    pub world_config_object_entities: Vec<Option<crate::ecs::Entity>>,
    // Game configuration from config.ini
    pub config: GameConfig,
}

impl GameState {
    /// Create a new game state
    pub fn new() -> Self {
        Self::with_config(GameConfig::default())
    }

    /// Create a new game state with specific configuration
    pub fn with_config(config: GameConfig) -> Self {
        GameState {
            map: Map::new(),
            frame_count: 0,
            start_time: Instant::now(),
            deferred_renderer: DeferredRenderer::new(),
            view_mode_3d: true, // Start in 3D mode for visual tests

            // ECS system
            ecs_state: EcsGameState::new(),
            // Centralized input handling
            input_handler: InputHandler::new(),
            // Hot-reload system (initialized later)
            level_data_hot_reload: None,
            // Track light entities for smart updates
            world_config_light_entities: Vec::new(),
            // Track object entities for smart updates
            world_config_object_entities: Vec::new(),
            // Store configuration
            config,
        }
    }
    
    /// Initialize hot-reload system for world configuration
    pub fn init_hot_reload(&mut self, config_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        match LevelDataHotReload::new(config_file) {
            Ok(mut hot_reload) => {
                // Disable hardcoded geometry since we're using config system
                self.ecs_state.disable_hardcoded_geometry();
                
                // Apply the initial configuration
                if let Some(initial_config) = hot_reload.get_config() {
                    self.apply_world_config(&initial_config);
                    hot_reload.set_last_applied_config(initial_config);
                    println!("✅ Applied initial world configuration");
                }
                
                self.level_data_hot_reload = Some(hot_reload);
                println!("🔥 World config hot-reload initialized for: {}", config_file);
                Ok(())
            }
            Err(e) => {
                eprintln!("❌ Failed to initialize hot-reload system: {}", e);
                Err(e)
            }
        }
    }
    
    /// Async initialization that sets up textures and meshes
    pub async fn initialize(&mut self) {
        self.ecs_state.initialize().await;
        
        // Initialize deferred renderer
        self.deferred_renderer.load_textures().await;
        self.deferred_renderer.initialize_shaders().await;
    }
    
    /// Update the game state
    pub fn update(&mut self, delta_time: f32) {
        self.frame_count += 1;
        
        // Update ECS state first
        self.ecs_state.update(delta_time);
        
        // Check for world configuration changes
        self.update_world_config();
        
        // Legacy player sync is no longer needed - pure ECS now
        
        // Toggle between 2D and 3D view with TAB key
        if is_key_pressed(KeyCode::Tab) {
            self.view_mode_3d = !self.view_mode_3d;
        }
    }
    
    /// Update world configuration hot-reload system
    fn update_world_config(&mut self) {
        let (config_to_apply, diff_to_apply, error_message) = if let Some(hot_reload) = &mut self.level_data_hot_reload {
            // Check for file changes
            hot_reload.update();
            
            let (config, diff) = if hot_reload.has_changed() {
                println!("🔄 World configuration changed, reloading...");
                
                if let Some(config) = hot_reload.get_config() {
                    // Get the diff to see what actually changed
                    let diff = hot_reload.get_config_diff();
                    
                    // Clone the config to avoid borrowing issues
                    let config_clone = config.clone();
                    hot_reload.set_last_applied_config(config_clone.clone());
                    (Some(config_clone), diff)
                } else {
                    println!("❌ Failed to get updated configuration");
                    (None, None)
                }
            } else {
                (None, None)
            };
            
            // Get any error message while we still have mutable access
            let error = hot_reload.get_last_error();
            (config, diff, error)
        } else {
            (None, None, None)
        };
        
        // Apply the configuration selectively if we have changes
        if let (Some(config), Some(diff)) = (config_to_apply, diff_to_apply) {
            self.apply_world_config_selective(&config, &diff);
        }
        
        // Display any errors
        if let Some(error) = error_message {
            println!("❌ World config error: {}", error);
        }
    }
    
    /// Apply world configuration selectively based on what changed
    fn apply_world_config_selective(&mut self, config: &super::level_data::LevelData, diff: &super::level_data::LevelDataDiff) {
        if !diff.has_changes() {
            return;
        }
        
        println!("🌍 Applying selective world configuration changes: {}", diff.get_summary());
        
        // Only update player if player config changed
        if diff.player_changed {
            if let Some(player_config) = &config.player {
                if player_config.enabled {
                    self.apply_player_config(player_config);
                }
            }
        }
        
        // Handle light changes
        self.apply_light_changes(config, diff);
        
        // Handle object changes  
        self.apply_object_changes(config, diff);
        
        // Apply settings changes
        if diff.settings_changed {
            self.apply_settings_config(config);
        }
        
        println!("✅ Selective world configuration applied successfully!");
    }

    /// Apply world configuration to the ECS world (full reload)
    fn apply_world_config(&mut self, config: &super::level_data::LevelData) {
        println!("🌍 Applying world configuration:");
        if config.player.is_some() {
            println!("  - Player configuration");
        }
        println!("  - {} lights", config.lights.len());
        println!("  - {} objects", config.objects.len());
        
        // Remove all existing config-created entities (lights and objects)
        self.remove_all_config_entities();
        
        // Ensure tracking vectors have correct size
        self.world_config_light_entities.resize(config.lights.len(), None);
        self.world_config_object_entities.resize(config.objects.len(), None);
        
        // Apply player configuration
        if let Some(player_config) = &config.player {
            if player_config.enabled {
                self.apply_player_config(player_config);
            }
        }
        
        // Add lights from configuration
        for (i, light_config) in config.lights.iter().enumerate() {
            if !light_config.enabled {
                continue;
            }
            
            let position = Vec3::new(
                light_config.position[0],
                light_config.position[1],
                light_config.position[2]
            );
            
            let color = Color::new(
                light_config.color[0],
                light_config.color[1],
                light_config.color[2],
                light_config.color[3]
            );
            
            // Create light entity with visible sphere
            let light_entity = self.ecs_state.world.spawn()
                .with(crate::ecs::Transform::new(position))
                .with(crate::ecs::LightSource::new(
                    color,
                    light_config.intensity,
                    light_config.radius,
                    match light_config.light_type.as_str() {
                        "warning" => crate::ecs::LightSourceType::Warning { pulse_speed: 2.0 },
                        "energy" => crate::ecs::LightSourceType::Energy { flow_speed: 1.5 },
                        "control" => crate::ecs::LightSourceType::Control { flicker_speed: 0.1 },
                        _ => crate::ecs::LightSourceType::Ambient,
                    }
                ))
                .with(crate::ecs::Renderer::sphere(0.15)
                    .with_color(Color::new(
                        (color.r + 0.3).min(1.0),
                        (color.g + 0.3).min(1.0),
                        (color.b + 0.3).min(1.0),
                        1.0
                    ))
                    .with_enabled(true))
                .build();
            
            // Track the created light entity
            self.world_config_light_entities[i] = Some(light_entity);
            
            let default_name = format!("Light_{}", i);
            let light_name = light_config.name.as_deref().unwrap_or(&default_name);
            println!("  ✅ Created {} light '{}' at {:?}", light_config.light_type, light_name, position);
        }
        
        // Add objects from configuration
        for (i, object_config) in config.objects.iter().enumerate() {
            if !object_config.enabled {
                continue;
            }
            
            let position = Vec3::new(
                object_config.position[0],
                object_config.position[1],
                object_config.position[2]
            );
            
            let scale = Vec3::new(
                object_config.scale[0],
                object_config.scale[1],
                object_config.scale[2]
            );
            
            let _rotation = Vec3::new(
                object_config.rotation[0],
                object_config.rotation[1],
                object_config.rotation[2]
            );
            
            let color = object_config.color.map(|c| Color::new(c[0], c[1], c[2], c[3]));
            
            // Create the object entity with appropriate components
            let mut entity_builder = self.ecs_state.world.spawn()
                .with(crate::ecs::Transform::new(position));
            
            // Add rendering component based on mesh type
            let renderer = match object_config.mesh.as_str() {
                "cube" => crate::ecs::Renderer::cube(scale),
                "sphere" => {
                    let radius = scale.x.max(scale.y).max(scale.z) * 0.5;
                    crate::ecs::Renderer::sphere(radius)
                },
                "cylinder" => {
                    let radius = scale.x.max(scale.z) * 0.5;
                    let height = scale.y;
                    crate::ecs::Renderer::cylinder(radius, height)
                },
                "plane" => crate::ecs::Renderer::plane(scale.x, scale.z),
                // Custom mesh file
                mesh_path => {
                    // For now, fallback to cube for custom meshes
                    // TODO: Implement custom mesh loading
                    println!("⚠️ Custom mesh loading not yet implemented: {}", mesh_path);
                    crate::ecs::Renderer::cube(scale)
                }
            };
            
            // Apply texture and color
            let mut final_renderer = renderer;
            if let Some(texture_name) = &object_config.texture {
                final_renderer = final_renderer.with_texture_name(texture_name.clone());
            }
            if let Some(color) = color {
                final_renderer = final_renderer.with_color(color);
            }
            
            entity_builder = entity_builder.with(final_renderer.with_enabled(true));
            
            // Add collision component based on collision type
            match object_config.collision_type.as_str() {
                "solid" => {
                    entity_builder = entity_builder.with(crate::ecs::Collider::static_solid(
                        crate::ecs::ColliderShape::Box { size: scale }
                    ));
                },
                "trigger" => {
                    entity_builder = entity_builder.with(crate::ecs::Collider::static_trigger(
                        crate::ecs::ColliderShape::Box { size: scale }
                    ));
                },
                "none" => {
                    // No collision component
                },
                _ => {
                    println!("⚠️ Unknown collision type: {}", object_config.collision_type);
                }
            }
            
            let object_entity = entity_builder.build();
            
            // Track the created object entity
            self.world_config_object_entities[i] = Some(object_entity);
            
            let default_name = format!("Object_{}", i);
            let object_name = object_config.name.as_deref().unwrap_or(&default_name);
            println!("  ✅ Created {} object '{}' at {:?} (collision: {})", 
                    object_config.mesh, object_name, position, object_config.collision_type);
        }
        
        // Apply global settings
        if let Some(settings) = &config.settings {
            if let Some(ambient) = settings.ambient_light {
                println!("  🌅 Ambient light: {:?}", ambient);
            }
            if let Some(fog_color) = settings.fog_color {
                println!("  🌫️ Fog color: {:?}", fog_color);
            }
            if let Some(fog_density) = settings.fog_density {
                println!("  🌫️ Fog density: {:.2}", fog_density);
            }
        }
        
        println!("✅ World configuration applied successfully!");
    }
    
    /// Remove all config-created entities (for naive reloading)
    fn remove_all_config_entities(&mut self) {
        // Remove all tracked light entities
        for light_entity in &self.world_config_light_entities {
            if let Some(entity) = light_entity {
                self.ecs_state.world.despawn(*entity);
            }
        }
        self.world_config_light_entities.clear();
        
        // Remove all tracked object entities  
        for object_entity in &self.world_config_object_entities {
            if let Some(entity) = object_entity {
                self.ecs_state.world.despawn(*entity);
            }
        }
        self.world_config_object_entities.clear();
        
        println!("🧹 Removed all config-created entities");
    }
    
    /// Apply player configuration to the existing player entity
    fn apply_player_config(&mut self, player_config: &super::level_data::PlayerConfig) {
        if let Some(player_entity) = self.ecs_state.player_entity {
            // Update player transform (position and rotation)
            if let Some(transform) = self.ecs_state.world.get_mut::<crate::ecs::Transform>(player_entity) {
                transform.position = Vec3::new(
                    player_config.spawn_position[0],
                    player_config.spawn_position[1],
                    player_config.spawn_position[2]
                );
                transform.rotation.y = player_config.spawn_rotation[0]; // Yaw
                transform.rotation.x = player_config.spawn_rotation[1]; // Pitch
            }
            
            // Update player collider if configuration changed
            if let Some(collider) = self.ecs_state.world.get_mut::<crate::ecs::Collider>(player_entity) {
                *collider = crate::ecs::Collider::dynamic_solid(crate::ecs::ColliderShape::Capsule {
                    height: player_config.height,
                    radius: player_config.radius,
                });
            }
            
            let player_name = player_config.name.as_deref().unwrap_or("Player");
            println!("  ✅ Updated player '{}' at {:?} (height: {:.1}, radius: {:.2})", 
                    player_name, 
                    [player_config.spawn_position[0], player_config.spawn_position[1], player_config.spawn_position[2]],
                    player_config.height,
                    player_config.radius);
        } else {
            println!("  ⚠️ No player entity found to configure");
        }
    }

    /// Apply light changes selectively
    fn apply_light_changes(&mut self, config: &super::level_data::LevelData, diff: &super::level_data::LevelDataDiff) {
        // Ensure tracking vector has correct size
        self.world_config_light_entities.resize(config.lights.len(), None);
        
        // Remove lights that were removed
        for &index in &diff.lights_removed {
            if let Some(entity) = self.world_config_light_entities.get_mut(index).and_then(|e| e.take()) {
                self.ecs_state.world.despawn(entity);
                println!("  🗑️ Removed light at index {}", index);
            }
        }
        
        // Add new lights
        for &(index, ref light_config) in &diff.lights_added {
            if light_config.enabled {
                let entity = self.create_light_entity(light_config);
                if index < self.world_config_light_entities.len() {
                    self.world_config_light_entities[index] = Some(entity);
                }
                let default_name = format!("Light_{}", index);
                let light_name = light_config.name.as_deref().unwrap_or(&default_name);
                println!("  ➕ Added {} light '{}' at {:?}", light_config.light_type, light_name, light_config.position);
            }
        }
        
        // Modify existing lights
        for &(index, ref light_config) in &diff.lights_modified {
            // Remove old light
            if let Some(entity) = self.world_config_light_entities.get_mut(index).and_then(|e| e.take()) {
                self.ecs_state.world.despawn(entity);
            }
            
            // Create new light with updated config
            if light_config.enabled {
                let entity = self.create_light_entity(light_config);
                if index < self.world_config_light_entities.len() {
                    self.world_config_light_entities[index] = Some(entity);
                }
                let default_name = format!("Light_{}", index);
                let light_name = light_config.name.as_deref().unwrap_or(&default_name);
                println!("  🔄 Updated {} light '{}' at {:?}", light_config.light_type, light_name, light_config.position);
            }
        }
    }

    /// Apply object changes selectively
    fn apply_object_changes(&mut self, config: &super::level_data::LevelData, diff: &super::level_data::LevelDataDiff) {
        // Ensure tracking vector has correct size
        self.world_config_object_entities.resize(config.objects.len(), None);
        
        // Remove objects that were removed
        for &index in &diff.objects_removed {
            if let Some(entity) = self.world_config_object_entities.get_mut(index).and_then(|e| e.take()) {
                self.ecs_state.world.despawn(entity);
                println!("  🗑️ Removed object at index {}", index);
            }
        }
        
        // Add new objects
        for &(index, ref object_config) in &diff.objects_added {
            if object_config.enabled {
                let entity = self.create_object_entity(object_config);
                if index < self.world_config_object_entities.len() {
                    self.world_config_object_entities[index] = Some(entity);
                }
                let default_name = format!("Object_{}", index);
                let object_name = object_config.name.as_deref().unwrap_or(&default_name);
                println!("  ➕ Added {} object '{}' at {:?}", object_config.mesh, object_name, object_config.position);
            }
        }
        
        // Modify existing objects
        for &(index, ref object_config) in &diff.objects_modified {
            // Remove old object
            if let Some(entity) = self.world_config_object_entities.get_mut(index).and_then(|e| e.take()) {
                self.ecs_state.world.despawn(entity);
            }
            
            // Create new object with updated config
            if object_config.enabled {
                let entity = self.create_object_entity(object_config);
                if index < self.world_config_object_entities.len() {
                    self.world_config_object_entities[index] = Some(entity);
                }
                let default_name = format!("Object_{}", index);
                let object_name = object_config.name.as_deref().unwrap_or(&default_name);
                println!("  🔄 Updated {} object '{}' at {:?}", object_config.mesh, object_name, object_config.position);
            }
        }
    }

    /// Apply global settings configuration
    fn apply_settings_config(&mut self, config: &super::level_data::LevelData) {
        if let Some(settings) = &config.settings {
            if let Some(ambient) = settings.ambient_light {
                println!("  🌅 Ambient light: {:?}", ambient);
            }
            if let Some(fog_color) = settings.fog_color {
                println!("  🌫️ Fog color: {:?}", fog_color);
            }
            if let Some(fog_density) = settings.fog_density {
                println!("  🌫️ Fog density: {}", fog_density);
            }
        }
    }

    /// Create a light entity from configuration
    fn create_light_entity(&mut self, light_config: &super::level_data::LightConfig) -> crate::ecs::Entity {
        let position = Vec3::new(
            light_config.position[0],
            light_config.position[1],
            light_config.position[2]
        );
        
        let color = Color::new(
            light_config.color[0],
            light_config.color[1],
            light_config.color[2],
            light_config.color[3]
        );
        
        // Create light entity with visible sphere
        self.ecs_state.world.spawn()
            .with(crate::ecs::Transform::new(position))
            .with(crate::ecs::LightSource::new(
                color,
                light_config.intensity,
                light_config.radius,
                match light_config.light_type.as_str() {
                    "warning" => crate::ecs::LightSourceType::Warning { pulse_speed: 2.0 },
                    "energy" => crate::ecs::LightSourceType::Energy { flow_speed: 1.5 },
                    "control" => crate::ecs::LightSourceType::Control { flicker_speed: 0.1 },
                    _ => crate::ecs::LightSourceType::Ambient,
                }
            ))
            .with(crate::ecs::Renderer::sphere(0.15)
                .with_color(Color::new(
                    (color.r + 0.3).min(1.0),
                    (color.g + 0.3).min(1.0),
                    (color.b + 0.3).min(1.0),
                    1.0
                ))
                .with_enabled(true))
            .build()
    }

    /// Create an object entity from configuration
    fn create_object_entity(&mut self, object_config: &super::level_data::ObjectConfig) -> crate::ecs::Entity {
        let position = Vec3::new(
            object_config.position[0],
            object_config.position[1],
            object_config.position[2]
        );
        
        let scale = Vec3::new(
            object_config.scale[0],
            object_config.scale[1],
            object_config.scale[2]
        );
        
        let color = object_config.color.map(|c| Color::new(c[0], c[1], c[2], c[3]));
        
        // Create the object entity with appropriate components
        let mut entity_builder = self.ecs_state.world.spawn()
            .with(crate::ecs::Transform::new(position));
        
        // Add rendering component based on mesh type
        let renderer = match object_config.mesh.as_str() {
            "cube" => crate::ecs::Renderer::cube(scale),
            "sphere" => {
                let radius = scale.x.max(scale.y).max(scale.z) * 0.5;
                crate::ecs::Renderer::sphere(radius)
            },
            "cylinder" => {
                let radius = scale.x.max(scale.z) * 0.5;
                let height = scale.y;
                crate::ecs::Renderer::cylinder(radius, height)
            },
            "plane" => crate::ecs::Renderer::plane(scale.x, scale.z),
            // Custom mesh file
            mesh_path => {
                // For now, fallback to cube for custom meshes
                println!("⚠️ Custom mesh loading not yet implemented: {}", mesh_path);
                crate::ecs::Renderer::cube(scale)
            }
        };
        
        // Apply texture and color
        let mut final_renderer = renderer;
        if let Some(texture_name) = &object_config.texture {
            final_renderer = final_renderer.with_texture_name(texture_name.clone());
        }
        if let Some(color) = color {
            final_renderer = final_renderer.with_color(color);
        }
        
        entity_builder = entity_builder.with(final_renderer.with_enabled(true));
        
        // Add collision component based on collision type
        match object_config.collision_type.as_str() {
            "solid" => {
                entity_builder = entity_builder.with(crate::ecs::Collider::static_solid(
                    crate::ecs::ColliderShape::Box { size: scale }
                ));
            },
            "trigger" => {
                entity_builder = entity_builder.with(crate::ecs::Collider::static_trigger(
                    crate::ecs::ColliderShape::Box { size: scale }
                ));
            },
            "none" => {
                // No collision component
            },
            _ => {
                println!("⚠️ Unknown collision type: {}", object_config.collision_type);
            }
        }
        
        entity_builder.build()
    }
    
    /// Draw the game state
    pub fn draw(&mut self) {
        // Get current player data for rendering
        let current_player = self.get_current_player_data();
        
        if self.view_mode_3d {
            // Draw 3D mode content
            self.draw_3d_mode_content(&current_player);
        } else {
            // Draw 2D top-down view with enhanced pathfinding visualization
            self.draw_2d_mode_content(&current_player);
        }
    }
    
    /// Draw 3D mode content
    fn draw_3d_mode_content(&mut self, current_player: &Player) {
        // Deferred rendering
        self.deferred_renderer.update_camera(&current_player);
        let time = self.start_time.elapsed().as_secs_f32();
        self.deferred_renderer.render(&self.ecs_state.world, time);
        
        // Draw minimap in top-right corner during 3D mode
        self.draw_minimap(&current_player);
        
        // Show performance stats overlay if test is active
        if self.ecs_state.has_test_bot() {
            self.draw_performance_analysis_overlay();
        }
        
        // Show lighting test overlay if lighting tests are active
        if self.ecs_state.has_lighting_test() {
            self.draw_lighting_test_overlay();
        }
        
        // Draw 3D UI overlay (moved down to avoid overlap with performance stats)
        let ui_y_offset = if self.ecs_state.has_test_bot() { 120.0 } else { 20.0 };
        draw_text("GAMEBYAI - 3D MODE (DEFERRED RENDERING)", 20.0, ui_y_offset, 20.0, GREEN);
        
        // Show test status (positioned below the main title)
        if self.ecs_state.has_test_bot() {
            if let Some((current, total, progress)) = self.ecs_state.get_test_bot_progress() {
                draw_text(&format!("🤖 TEST ACTIVE: {}/{} waypoints ({:.1}%)", current, total, progress * 100.0), 
                         20.0, ui_y_offset + 25.0, 16.0, YELLOW);
                draw_text("USER INPUT DISABLED", 20.0, ui_y_offset + 45.0, 16.0, RED);
            } else {
                draw_text("🤖 TEST BOT ACTIVE", 20.0, ui_y_offset + 25.0, 16.0, YELLOW);
                draw_text("USER INPUT DISABLED", 20.0, ui_y_offset + 45.0, 16.0, RED);
            }
        } else {
            // Show pillar toggle status
            let (pillars_enabled, pillar_count) = self.ecs_state.get_pillar_status();
            let pillar_status = if pillars_enabled { "ENABLED" } else { "DISABLED" };
            let pillar_color = if pillars_enabled { GREEN } else { RED };
            draw_text(&format!("🏛️ Middle Pillars: {} ({} pillars)", pillar_status, pillar_count), 
                     20.0, ui_y_offset + 25.0, 16.0, pillar_color);
        }
        
        draw_text(&format!("FPS: {:.0} | Pos: ({:.1}, {:.1}, {:.1}) | Yaw: {:.1}° | Pitch: {:.1}° | Ground: {}", 
            get_fps(), current_player.x, current_player.y, current_player.z, 
            current_player.rotation.to_degrees(), current_player.pitch.to_degrees(),
            if current_player.is_grounded { "✓" } else { "✗" }), 
            20.0, screen_height() - 100.0, 16.0, WHITE);
        draw_text("🚀 DEFERRED RENDERING", 20.0, screen_height() - 80.0, 16.0, GOLD);
        draw_text("System: ECS", 20.0, screen_height() - 60.0, 16.0, BLUE);
        draw_text("WASD: Move/Strafe | Mouse: Look | SPACE: Jump | T: Toggle Pillars | TAB: 2D View | ESC: Exit", 20.0, screen_height() - 40.0, 16.0, GRAY);
        draw_text("M: Toggle Mouse | TAB: 2D View | ESC: Exit", 20.0, screen_height() - 20.0, 16.0, GRAY);
    }
    
    /// Draw 2D mode content
    fn draw_2d_mode_content(&mut self, current_player: &Player) {
        clear_background(BLACK);
        
        // Get player position for enhanced minimap
        let player_pos = if let Some(transform) = self.ecs_state.get_player_transform() {
            Some((transform.position.x, transform.position.z))
        } else {
            None
        };
        
        // Get pathfinding information if test is active
        let (target_pos, path, explored_nodes) = if self.ecs_state.has_test_bot() {
            let target = self.ecs_state.get_test_bot_target();
            let path_info = self.ecs_state.get_pathfinding_debug_info();
            (target, path_info.0, path_info.1)
        } else {
            (None, None, None)
        };
        
        // Draw enhanced minimap with pathfinding visualization
        self.map.draw_enhanced_minimap(
            50.0, 50.0, 40.0,  // offset_x, offset_y, tile_size (larger for 2D view)
            player_pos,
            target_pos,
            path.as_ref(),
            explored_nodes.as_ref()
        );
        
        // Show automatic performance analysis if test is active
        if self.ecs_state.has_test_bot() {
            self.draw_performance_analysis_overlay();
            
            // Show test status
            if let Some((current, total, progress)) = self.ecs_state.get_test_bot_progress() {
                draw_text(&format!("🤖 TEST ACTIVE: {}/{} waypoints ({:.1}%)", current, total, progress * 100.0), 
                         50.0, 500.0, 18.0, YELLOW);
                draw_text("USER INPUT DISABLED", 50.0, 520.0, 16.0, RED);
            } else {
                draw_text("🤖 TEST RUNNING - Initializing...", 50.0, 500.0, 18.0, YELLOW);
            }
        }
        
        // Show lighting test overlay if lighting tests are active  
        if self.ecs_state.has_lighting_test() {
            self.draw_lighting_test_overlay();
        }
        
        if !self.ecs_state.has_test_bot() && !self.ecs_state.has_lighting_test() {
            // Show pillar toggle status when not testing
            let (pillars_enabled, pillar_count) = self.ecs_state.get_pillar_status();
            let pillar_status = if pillars_enabled { "ENABLED" } else { "DISABLED" };
            let pillar_color = if pillars_enabled { GREEN } else { RED };
            draw_text(&format!("🏛️ Middle Pillars: {} ({} pillars)", pillar_status, pillar_count), 
                     50.0, 500.0, 16.0, pillar_color);
        }
        
        // Draw 2D UI
        draw_text("GAMEBYAI - 2D Enhanced Map View (ECS)", 20.0, 20.0, 20.0, GREEN);
        // Show FPS if enabled in config
        let fps_text = if self.config.should_show_fps() {
            format!("Frame: {} | FPS: {:.0} | System: ECS", self.frame_count, get_fps() as i32)
        } else {
            format!("Frame: {} | System: ECS", self.frame_count)
        };
        draw_text(&fps_text, 20.0, screen_height() - 60.0, 16.0, WHITE);
        draw_text("WASD: Move/Strafe | Mouse: Look | SPACE: Jump | M: Toggle Mouse | TAB: 3D View | ESC: Exit", 20.0, screen_height() - 20.0, 16.0, GRAY);
    }
    
    /// Draw a minimap in the top-right corner during 3D mode
    pub fn draw_minimap(&self, current_player: &Player) {
        let minimap_size = 150.0;
        let minimap_x = screen_width() - minimap_size - 10.0;
        let minimap_y = 10.0;
        let tile_size = minimap_size / self.map.width.max(self.map.height) as f32;
        
        // Draw minimap background with border
        draw_rectangle(minimap_x - 2.0, minimap_y - 2.0, minimap_size + 4.0, minimap_size + 4.0, WHITE);
        draw_rectangle(minimap_x, minimap_y, minimap_size, minimap_size, BLACK);
        
        // Draw map tiles with texture colors
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                let screen_x = minimap_x + x as f32 * tile_size;
                let screen_y = minimap_y + y as f32 * tile_size;
                
                let wall_type = self.map.get_wall_type(x as i32, y as i32);
                let color = if wall_type == super::map::WallType::Empty {
                    Color::new(0.2, 0.2, 0.2, 1.0)  // Dark gray empty space
                } else {
                    // Use texture color but dimmed for minimap
                    let texture_color = self.map.get_wall_color(wall_type, true);
                    Color::new(
                        texture_color.r * 0.8,
                        texture_color.g * 0.8,
                        texture_color.b * 0.8,
                        1.0
                    )
                };
                
                draw_rectangle(screen_x, screen_y, tile_size, tile_size, color);
            }
        }
        
        // Draw pathfinding visualization if ECS test bot is active
        if self.ecs_state.has_test_bot() {
            // For now, just draw a simple indicator that test bot is active
            // TODO: Add pathfinding visualization to ECS test bot
            draw_text("🤖 TEST BOT ACTIVE", minimap_x, minimap_y + minimap_size + 15.0, 12.0, YELLOW);
        }
        
        // Draw player position and direction
        let player_screen_x = minimap_x + current_player.x * tile_size;
        let player_screen_y = minimap_y + current_player.y * tile_size;
        
        // Player dot
        draw_circle(player_screen_x, player_screen_y, tile_size * 0.25, GREEN);
        
        // Player direction indicator
        let dir_length = tile_size * 0.4;
        let dir_end_x = player_screen_x + dir_length * current_player.rotation.cos();
        let dir_end_y = player_screen_y + dir_length * current_player.rotation.sin();
        draw_line(player_screen_x, player_screen_y, dir_end_x, dir_end_y, 2.0, GREEN);
        
        // Minimap label
        draw_text("MINIMAP", minimap_x, minimap_y - 5.0, 12.0, WHITE);
    }
    
    /// Get current player data from ECS
    fn get_current_player_data(&self) -> Player {
        // Convert ECS data to Player format for rendering compatibility
        if let Some(legacy_data) = self.ecs_state.get_legacy_player_data() {
            Player {
                x: legacy_data.x,
                y: legacy_data.y,
                z: legacy_data.z,
                rotation: legacy_data.rotation,
                pitch: legacy_data.pitch,
                speed: 2.0,
                turn_speed: 3.0,
                radius: 0.3,
                mouse_sensitivity: 1.0,  // Will be dynamically calculated by InputHandler
                vertical_velocity: 0.0,
                jump_strength: 4.5,
                gravity: 12.0,
                ground_height: 0.6,
                is_grounded: legacy_data.is_grounded,
                last_input: "ECS System".to_string(),
                collision_detected: false,
            }
        } else {
            // Fallback default player if ECS data is not available
            Player::new(1.5, 1.5)
        }
    }

    /// Show lighting test overlay
    fn draw_lighting_test_overlay(&self) {
        if let Some((test_name, light_count, elapsed, duration, bg_color)) = self.ecs_state.get_lighting_test_info() {
            // Don't override the normal game background, just draw UI overlay
            
            // Draw lighting test information in top-center
            let center_x = screen_width() / 2.0 - 150.0;
            let test_y = 20.0;
            
            // Semi-transparent background
            draw_rectangle(center_x - 10.0, test_y - 10.0, 300.0, 120.0, 
                          Color::new(bg_color.r * 0.5, bg_color.g * 0.5, bg_color.b * 0.5, 0.9));
            
            draw_text("🔆 LIGHTING TEST", center_x, test_y + 20.0, 20.0, WHITE);
            draw_text(&format!("Phase: {}", test_name), center_x, test_y + 45.0, 16.0, WHITE);
            // Show FPS in lighting test if enabled in config
            let lights_text = if self.config.should_show_fps() {
                format!("Lights: {} | FPS: {:.0}", light_count, get_fps())
            } else {
                format!("Lights: {} | Performance Monitoring", light_count)
            };
            draw_text(&lights_text, center_x, test_y + 65.0, 14.0, YELLOW);
            draw_text(&format!("Time: {:.1}s / {:.1}s", elapsed, duration), center_x, test_y + 85.0, 14.0, GREEN);
            
            // Progress bar
            let progress = elapsed / duration;
            let bar_width = 280.0;
            let bar_height = 8.0;
            let bar_x = center_x;
            let bar_y = test_y + 100.0;
            
            // Background
            draw_rectangle(bar_x, bar_y, bar_width, bar_height, GRAY);
            // Progress
            draw_rectangle(bar_x, bar_y, bar_width * progress, bar_height, GREEN);
            // Border
            draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, 1.0, WHITE);
        }
    }

    /// Draw compact stats overlay for 3D mode
    fn draw_performance_analysis_overlay(&self) {
        let overlay_x = 10.0;   // Top-left corner
        let overlay_y = 10.0;   
        let overlay_width = 300.0;
        let overlay_height = 100.0;
        
        // Semi-transparent background
        draw_rectangle(overlay_x - 5.0, overlay_y - 5.0, overlay_width + 10.0, overlay_height + 10.0, 
                      Color::new(0.0, 0.0, 0.0, 0.8));
        
        // Performance stats title
        draw_text("📊 PERFORMANCE STATS", overlay_x, overlay_y + 15.0, 14.0, YELLOW);
        
        // Real-time FPS (only if enabled in config)
        if self.config.should_show_fps() {
            let fps = macroquad::time::get_fps();
            let fps_color = if fps >= 120 { GREEN } else if fps >= 60 { YELLOW } else { RED };
            let fps_status = if fps >= 120 { "🚀 BLAZING" } else if fps >= 60 { "✅ GOOD" } else { "⚠️ LOW" };
            draw_text(&format!("FPS: {} {}", fps, fps_status), overlay_x, overlay_y + 35.0, 12.0, fps_color);
        } else {
            draw_text("FPS: Hidden (config.ini)", overlay_x, overlay_y + 35.0, 12.0, GRAY);
        }
        
        // Pathfinding status
        draw_text("🗺️ A* Pathfinding: 4-directional", overlay_x, overlay_y + 55.0, 12.0, Color::new(0.0, 1.0, 1.0, 1.0));
        
        // Test progress if active
        if let Some((current, total, progress)) = self.ecs_state.get_test_bot_progress() {
            draw_text(&format!("🤖 Test: {}/{} waypoints ({:.0}%)", current, total, progress * 100.0), 
                     overlay_x, overlay_y + 75.0, 12.0, ORANGE);
        } else {
            draw_text("💡 Component.enabled approach active", overlay_x, overlay_y + 75.0, 12.0, LIME);
        }
    }
} 