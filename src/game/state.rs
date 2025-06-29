use macroquad::prelude::*;
use std::time::Instant;
use super::{map::Map, player::Player, input::InputHandler};
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
    // Loading progress display
    pub loading_progress: Option<LoadingProgress>,
}

/// Loading progress information
#[derive(Debug, Clone)]
pub struct LoadingProgress {
    pub stage: String,
    pub detail: String,
    pub current: usize,
    pub total: usize,
    pub start_time: Instant,
}

impl LoadingProgress {
    pub fn new(stage: &str) -> Self {
        Self {
            stage: stage.to_string(),
            detail: String::new(),
            current: 0,
            total: 0,
            start_time: Instant::now(),
        }
    }
    
    pub fn update(&mut self, detail: &str, current: usize, total: usize) {
        self.detail = detail.to_string();
        self.current = current;
        self.total = total;
    }
    
    pub fn elapsed(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }
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
            // Loading progress display
            loading_progress: None,
        }
    }
    
    /// Initialize hot-reload system for world configuration
    pub async fn init_hot_reload(&mut self, config_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        match LevelDataHotReload::new(config_file) {
            Ok(mut hot_reload) => {
                println!("🔧 Using JSON map system - no procedural geometry");
                
                // Apply the initial configuration
                if let Some(initial_config) = hot_reload.get_config() {
                    self.apply_world_config(&initial_config).await;
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
        self.deferred_renderer.initialize_shaders().await;
        // Note: Texture preloading now happens in apply_world_config() after entities are created
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
        
        // Regenerate pathfinding map if objects changed
        if diff.objects_added.len() > 0 || diff.objects_modified.len() > 0 || diff.objects_removed.len() > 0 {
            println!("🗺️ Objects changed - regenerating pathfinding map");
            let pathfinding_map = self.generate_pathfinding_map_from_level(config);
            self.map = pathfinding_map.clone();
            self.ecs_state.update_pathfinding_map(pathfinding_map);
        }
        
        // TODO: GLTF meshes need to be preloaded for hot-reload changes
        // Currently only initial loading supports GLTF preloading due to async constraints
        if diff.objects_added.len() > 0 || diff.objects_modified.len() > 0 {
            println!("⚠️ Objects changed - GLTF meshes may need to be reloaded manually");
        }
        
        println!("✅ Selective world configuration applied successfully!");
    }

    /// Generate a pathfinding map from the current level configuration
    fn generate_pathfinding_map_from_level(&self, config: &super::level_data::LevelData) -> Map {
        // Determine map bounds from objects
        let mut min_x: f32 = 0.0;
        let mut max_x: f32 = 10.0;
        let mut min_z: f32 = 0.0;
        let mut max_z: f32 = 10.0;
        
        // Find the actual bounds of the level
        for obj in &config.objects {
            if !obj.enabled {
                continue;
            }
            
            let pos_x = obj.position[0];
            let pos_z = obj.position[2];
            let scale_x = obj.scale[0];
            let scale_z = obj.scale[2];
            
            // Calculate object bounds
            let obj_min_x = pos_x - scale_x / 2.0;
            let obj_max_x = pos_x + scale_x / 2.0;
            let obj_min_z = pos_z - scale_z / 2.0;
            let obj_max_z = pos_z + scale_z / 2.0;
            
            min_x = min_x.min(obj_min_x);
            max_x = max_x.max(obj_max_x);
            min_z = min_z.min(obj_min_z);
            max_z = max_z.max(obj_max_z);
        }
        
        // Add some padding
        min_x -= 1.0;
        max_x += 1.0;
        min_z -= 1.0;
        max_z += 1.0;
        
        // Create grid (1 unit per cell)
        let width = (max_x - min_x).ceil() as usize;
        let height = (max_z - min_z).ceil() as usize;
        
        // Initialize as all walkable
        let mut tiles = vec![vec![0u8; width]; height];
        
        // Mark solid objects as walls
        for obj in &config.objects {
            if !obj.enabled || obj.collision_type != "solid" {
                continue;
            }
            
            // Skip floors and ceilings - they don't block horizontal movement
            let obj_name = obj.name.as_ref().map(|s| s.to_lowercase()).unwrap_or_default();
            if obj_name.contains("floor") || obj_name.contains("ceiling") {
                continue;
            }
            
            let pos_x = obj.position[0];
            let pos_z = obj.position[2];
            let scale_x = obj.scale[0];
            let scale_z = obj.scale[2];
            
            // Calculate object bounds
            let obj_min_x = pos_x - scale_x / 2.0;
            let obj_max_x = pos_x + scale_x / 2.0;
            let obj_min_z = pos_z - scale_z / 2.0;
            let obj_max_z = pos_z + scale_z / 2.0;
            
            // Mark grid cells that overlap with this object as walls
            for z in 0..height {
                for x in 0..width {
                    let cell_min_x = min_x + (x as f32 / width as f32) * (max_x - min_x);
                    let cell_max_x = min_x + ((x + 1) as f32 / width as f32) * (max_x - min_x);
                    let cell_min_z = min_z + (z as f32 / height as f32) * (max_z - min_z);
                    let cell_max_z = min_z + ((z + 1) as f32 / height as f32) * (max_z - min_z);
                    
                    // Check if object overlaps with this grid cell
                    if obj_max_x > cell_min_x && obj_min_x < cell_max_x &&
                       obj_max_z > cell_min_z && obj_min_z < cell_max_z {
                        tiles[z][x] = 1; // Mark as wall
                    }
                }
            }
        }
        
        println!("🗺️ Generated pathfinding map: {}x{} (bounds: {:.1},{:.1} to {:.1},{:.1})", 
                 width, height, min_x, min_z, max_x, max_z);
        
        // Debug: Print the generated map
        println!("🔍 DEBUG: Generated pathfinding map layout:");
        for z in 0..height {
            let mut row = String::new();
            for x in 0..width {
                if tiles[z][x] == 0 {
                    row.push('.');  // Walkable
                } else {
                    row.push('#');  // Wall
                }
            }
            println!("🔍 Row {}: {}", z, row);
        }
        
        // Debug: Check specific positions
        let test_positions = vec![(1.5, 1.5), (2.0, 2.0), (5.0, 5.0), (8.0, 8.0)];
        for (world_x, world_z) in test_positions {
            let grid_x = ((world_x - min_x) / (max_x - min_x) * width as f32).floor() as usize;
            let grid_z = ((world_z - min_z) / (max_z - min_z) * height as f32).floor() as usize;
            let is_blocked = if grid_z < height && grid_x < width {
                tiles[grid_z][grid_x] != 0
            } else {
                true
            };
            println!("🔍 Position ({:.1}, {:.1}) -> grid ({}, {}) -> {}", 
                     world_x, world_z, grid_x, grid_z, 
                     if is_blocked { "BLOCKED" } else { "WALKABLE" });
        }
        
        Map {
            width,
            height,
            tiles,
            world_min_x: min_x,
            world_min_z: min_z,
            world_max_x: max_x,
            world_max_z: max_z,
        }
    }

    /// Apply world configuration to the ECS world (full reload)
    async fn apply_world_config(&mut self, config: &super::level_data::LevelData) {
        println!("🌍 Applying world configuration:");
        if config.player.is_some() {
            println!("  - Player configuration");
        }
        println!("  - {} lights", config.lights.len());
        println!("  - {} objects", config.objects.len());
        
        // Generate pathfinding map from level data
        let pathfinding_map = self.generate_pathfinding_map_from_level(config);
        self.map = pathfinding_map.clone();
        self.ecs_state.update_pathfinding_map(pathfinding_map);
        
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
            
            let rotation = Vec3::new(
                object_config.rotation[0],
                object_config.rotation[1],
                object_config.rotation[2]
            );
            
            let color = object_config.color.map(|c| Color::new(c[0], c[1], c[2], c[3]));
            
            // Create the object entity with appropriate components
            let mut entity_builder = self.ecs_state.world.spawn()
                .with(crate::ecs::Transform::new(position)
                    .with_scale(scale)
                    .with_rotation(rotation));
            
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
                // Check if it's a GLTF file
                if crate::game::rendering::GltfLoader::is_gltf_file(mesh_path) {
                    println!("🔧 Attempting to load GLTF mesh: {}", mesh_path);
                    // Use Custom render mode with mesh path for GLTF loading
                    crate::ecs::Renderer {
                        render_mode: crate::ecs::RenderMode::Custom,
                        material: crate::ecs::RenderMaterial::default(),
                        custom_mesh_path: Some(mesh_path.to_string()),
                        enabled: true,
                    }
                } else {
                    println!("⚠️ Unsupported mesh format: {}", mesh_path);
                    crate::ecs::Renderer::cube(scale)
                }
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
        
        // Show loading progress for texture loading
        self.loading_progress = Some(LoadingProgress::new("Loading Textures"));
        
        // Preload textures from actual world configuration (with progress updates)
        println!("🖼️ Preloading textures from world configuration...");
        let texture_count = self.deferred_renderer.get_required_texture_count(&self.ecs_state.world);
        for i in 0..texture_count {
            if let Some(progress) = &mut self.loading_progress {
                progress.update(&format!("Loading texture {}/{}", i + 1, texture_count), i + 1, texture_count);
            }
            self.draw_loading_screen();
            next_frame().await;
        }
        self.deferred_renderer.preload_textures_from_world(&self.ecs_state.world).await;
        
        // Update progress for GLTF loading
        if let Some(progress) = &mut self.loading_progress {
            progress.stage = "Loading 3D Models".to_string();
            progress.detail = "Scanning for GLTF files...".to_string();
            progress.current = 0;
            progress.total = 0;
        }
        
        // Draw loading screen
        self.draw_loading_screen();
        next_frame().await;
        
        // Preload GLTF meshes now that entities are created
        println!("🖼️ Preloading GLTF meshes...");
        let gltf_count = self.deferred_renderer.get_required_gltf_count(&self.ecs_state.world);
        for i in 0..gltf_count {
            if let Some(progress) = &mut self.loading_progress {
                progress.update(&format!("Loading 3D model {}/{}", i + 1, gltf_count), i + 1, gltf_count);
            }
            self.draw_loading_screen();
            next_frame().await;
        }
        self.deferred_renderer.preload_gltf_meshes(&self.ecs_state.world).await;
        
        // Clear loading progress
        self.loading_progress = None;
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
        
        let rotation = Vec3::new(
            object_config.rotation[0],
            object_config.rotation[1],
            object_config.rotation[2]
        );
        
        let color = object_config.color.map(|c| Color::new(c[0], c[1], c[2], c[3]));
        
        // Create the object entity with appropriate components
        let mut entity_builder = self.ecs_state.world.spawn()
            .with(crate::ecs::Transform::new(position)
                .with_scale(scale)
                .with_rotation(rotation));
        
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
                if mesh_path.ends_with(".gltf") || mesh_path.ends_with(".glb") {
                    println!("🔧 Attempting to load GLTF mesh: {}", mesh_path);
                    crate::ecs::Renderer::custom().with_custom_mesh_path(mesh_path.to_string())
                } else {
                    // For non-GLTF files, fallback to cube
                    println!("⚠️ Unsupported mesh format, using cube: {}", mesh_path);
                    crate::ecs::Renderer::cube(scale)
                }
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
    pub async fn draw(&mut self) {
        // Get current player data for rendering
        let current_player = self.get_current_player_data();
        
        if self.view_mode_3d {
            // Draw 3D mode content
            self.draw_3d_mode_content(&current_player).await;
        } else {
            // Draw 2D top-down view with enhanced pathfinding visualization
            self.draw_2d_mode_content(&current_player);
        }
    }
    
    /// Draw 3D mode content
    async fn draw_3d_mode_content(&mut self, current_player: &Player) {
        // Deferred rendering
        self.deferred_renderer.update_camera(&current_player);
        let time = self.start_time.elapsed().as_secs_f32();
        self.deferred_renderer.render(&self.ecs_state.world, time).await;
        
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

    /// Draw performance analysis overlay
    fn draw_performance_analysis_overlay(&self) {
        // Performance analysis overlay with dark background for better readability
        let overlay_x = 50.0;
        let overlay_y = 50.0;
        let overlay_width = 400.0;
        let overlay_height = 300.0;
        
        // Semi-transparent dark background
        draw_rectangle(overlay_x - 10.0, overlay_y - 10.0, overlay_width + 20.0, overlay_height + 20.0, Color::new(0.0, 0.0, 0.0, 0.8));
        
        let mut y_offset = overlay_y;
        let line_height = 20.0;
        
        // Title
        draw_text("🔬 PERFORMANCE ANALYSIS", overlay_x, y_offset, 18.0, GOLD);
        y_offset += line_height * 1.5;
        
        // Current FPS
        let fps = get_fps() as f32;
        let fps_color = if fps >= 60.0 { GREEN } else if fps >= 30.0 { YELLOW } else { RED };
        draw_text(&format!("Current FPS: {:.0}", fps), overlay_x, y_offset, 16.0, fps_color);
        y_offset += line_height;
        
        // Frame time
        let frame_time = 1000.0 / fps.max(1.0);
        let frame_color = if frame_time <= 16.7 { GREEN } else if frame_time <= 33.3 { YELLOW } else { RED };
        draw_text(&format!("Frame Time: {:.1}ms", frame_time), overlay_x, y_offset, 16.0, frame_color);
        y_offset += line_height;
        
        // Total entities
        let entity_count = self.ecs_state.world.entities().active_count();
        draw_text(&format!("Entities: {}", entity_count), overlay_x, y_offset, 16.0, WHITE);
        y_offset += line_height;
        
        // Memory usage estimate (rough)
        let memory_kb = entity_count * 1024 / 1024; // Very rough estimate
        draw_text(&format!("Est. Memory: {}KB", memory_kb), overlay_x, y_offset, 16.0, WHITE);
        y_offset += line_height;
        
        // Performance recommendations
        y_offset += line_height * 0.5;
        draw_text("💡 Recommendations:", overlay_x, y_offset, 14.0, YELLOW);
        y_offset += line_height;
        
        if fps < 30.0 {
            draw_text("• Reduce entity count", overlay_x + 10.0, y_offset, 12.0, RED);
            y_offset += line_height * 0.8;
            draw_text("• Disable lighting system", overlay_x + 10.0, y_offset, 12.0, RED);
            y_offset += line_height * 0.8;
        } else if fps < 60.0 {
            draw_text("• Consider optimizing rendering", overlay_x + 10.0, y_offset, 12.0, YELLOW);
            y_offset += line_height * 0.8;
        } else {
            draw_text("• Performance is good!", overlay_x + 10.0, y_offset, 12.0, GREEN);
            y_offset += line_height * 0.8;
        }
        
        // Test information
        if self.ecs_state.has_test_bot() {
            y_offset += line_height * 0.5;
            draw_text("🤖 Test Mode Active", overlay_x, y_offset, 14.0, Color::new(0.0, 1.0, 1.0, 1.0)); // CYAN equivalent
            y_offset += line_height;
            
            if let Some((current, total, progress)) = self.ecs_state.get_test_bot_progress() {
                draw_text(&format!("Progress: {}/{} ({:.1}%)", current, total, progress * 100.0), overlay_x + 10.0, y_offset, 12.0, Color::new(0.0, 1.0, 1.0, 1.0)); // CYAN equivalent
            }
        }
    }

    /// Display loading screen with progress information
    pub fn draw_loading_screen(&self) {
        if let Some(progress) = &self.loading_progress {
            clear_background(BLACK);
            
            let screen_width = screen_width();
            let screen_height = screen_height();
            let center_x = screen_width / 2.0;
            let center_y = screen_height / 2.0;
            
            // Main title
            let title = "GAMEBYAI - LOADING";
            let title_size = 32.0;
            let title_width = measure_text(title, None, title_size as u16, 1.0).width;
            draw_text(title, center_x - title_width / 2.0, center_y - 100.0, title_size, WHITE);
            
            // Loading stage
            let stage_text = &progress.stage;
            let stage_size = 24.0;
            let stage_width = measure_text(stage_text, None, stage_size as u16, 1.0).width;
            draw_text(stage_text, center_x - stage_width / 2.0, center_y - 50.0, stage_size, YELLOW);
            
            // Detail information
            if !progress.detail.is_empty() {
                let detail_text = &progress.detail;
                let detail_size = 18.0;
                let detail_width = measure_text(detail_text, None, detail_size as u16, 1.0).width;
                draw_text(detail_text, center_x - detail_width / 2.0, center_y - 20.0, detail_size, LIGHTGRAY);
            }
            
            // Progress bar
            if progress.total > 0 {
                let bar_width = 400.0;
                let bar_height = 20.0;
                let bar_x = center_x - bar_width / 2.0;
                let bar_y = center_y + 20.0;
                
                // Background
                draw_rectangle(bar_x, bar_y, bar_width, bar_height, DARKGRAY);
                
                // Progress fill
                let progress_ratio = progress.current as f32 / progress.total as f32;
                let fill_width = bar_width * progress_ratio;
                draw_rectangle(bar_x, bar_y, fill_width, bar_height, GREEN);
                
                // Progress text
                let progress_text = format!("{}/{}", progress.current, progress.total);
                let progress_text_size = 16.0;
                let progress_text_width = measure_text(&progress_text, None, progress_text_size as u16, 1.0).width;
                draw_text(&progress_text, center_x - progress_text_width / 2.0, bar_y + bar_height + 25.0, progress_text_size, WHITE);
            }
            
            // Elapsed time
            let elapsed = progress.elapsed();
            let time_text = format!("Elapsed: {:.1}s", elapsed);
            let time_size = 16.0;
            let time_width = measure_text(&time_text, None, time_size as u16, 1.0).width;
            draw_text(&time_text, center_x - time_width / 2.0, center_y + 80.0, time_size, GRAY);
            
            // Loading animation (spinning dots)
            let dots = (elapsed * 2.0) as usize % 4;
            let loading_text = format!("Loading{}", ".".repeat(dots));
            let loading_size = 14.0;
            let loading_width = measure_text(&loading_text, None, loading_size as u16, 1.0).width;
            draw_text(&loading_text, center_x - loading_width / 2.0, center_y + 110.0, loading_size, BLUE);
        }
    }
} 