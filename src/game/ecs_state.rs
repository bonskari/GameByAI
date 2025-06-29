//! ECS-based game state for gradual migration

use macroquad::prelude::*;
use crate::ecs::*;
use crate::ecs::pathfinding::PathfindingAlgorithms;
use crate::testing::performance_test::PerformanceTest;
use super::map::{Map, WallType};
use super::input::PlayerInput;
use crate::game::level_generator::LevelMeshBuilder;
use std::time::Instant;
use std::collections::HashMap;

/// ECS-based game state that manages all entities and components
pub struct EcsGameState {
    pub world: World,
    pub pathfinding_algorithms: PathfindingAlgorithms,
    pub player_entity: Option<Entity>,
    pub map: Map,
    pub frame_count: u32,
    pub middle_pillar_entities: Vec<Entity>,  // Track middle pillars for toggling
    pub pillars_enabled: bool,                // Current state of middle pillars
    pub last_pillar_toggle_time: std::time::Instant, // Track when pillars were last toggled
    pub use_hardcoded_geometry: bool,         // Whether to create hardcoded walls/floors/ceilings
}

impl EcsGameState {
    /// Create a new ECS game state
    pub fn new() -> Self {
        let mut world = World::new();
        
        // Create player entity with new component design
        let player_entity = world.spawn()
            .with(Transform::new(Vec3::new(1.5, 0.6, 1.5)))
            .with(Velocity::new())
            .with(Player::new())
            .with(Collider::dynamic_solid(ColliderShape::Capsule { height: 1.8, radius: 0.25 }))
            .build();
        
        let map = Map::new();
        
        Self {
            world,
            pathfinding_algorithms: PathfindingAlgorithms::new(map.clone()),
            player_entity: Some(player_entity),
            map,
            frame_count: 0,
            middle_pillar_entities: Vec::new(),
            pillars_enabled: true,
            last_pillar_toggle_time: std::time::Instant::now(),
            use_hardcoded_geometry: true, // Default to true for backward compatibility
        }
    }
    
    /// Async initialization that sets up the static geometry with textures
    pub async fn initialize(&mut self) {
        // Populate the world with static geometry entities
        self.populate_static_geometry().await;
    }
    
    /// Disable hardcoded geometry creation (use when world config system is active)  
    pub fn disable_hardcoded_geometry(&mut self) {
        self.use_hardcoded_geometry = false;
        println!("🔧 Hardcoded geometry disabled - using world config system");
    }
    
    /// Populate the ECS world with wall, floor, and ceiling entities using StaticRenderer
    async fn populate_static_geometry(&mut self) {
        if !self.use_hardcoded_geometry {
            println!("🔧 Skipping hardcoded geometry - using world config system");
            return;
        }
        
        let mut wall_count = 0;
        let mut floor_count = 0;
        let mut ceiling_count = 0;
        
        // Create separate wall meshes for each texture type
        let mesh_builder = LevelMeshBuilder::new(self.map.clone());
        
        // Generate separate meshes for each wall type
        for wall_type in [WallType::TechPanel, WallType::HullPlating, WallType::ControlSystem, WallType::EnergyConduit] {
            let wall_mesh = mesh_builder.generate_wall_mesh_for_type(wall_type).await;
            
            // Only create entity if mesh has geometry
            if !wall_mesh.vertices.is_empty() {
                let _wall_mesh_entity = self.world.spawn()
                    .with(Transform::new(Vec3::ZERO))  // World origin since mesh contains world coordinates
                    .with(StaticMesh::walls(wall_mesh))
                    .build();
                wall_count += 1;
            }
        }
        
        // Generate and create the single floor mesh entity
        let floor_mesh = mesh_builder.generate_floor_mesh_with_texture().await;
        let _floor_mesh_entity = self.world.spawn()
            .with(Transform::new(Vec3::ZERO))
            .with(StaticMesh::floor(floor_mesh))
            .build();
        floor_count = 1; // One floor mesh entity
        
        // Generate and create the single ceiling mesh entity
        let ceiling_mesh = mesh_builder.generate_ceiling_mesh_with_texture().await;
        let _ceiling_mesh_entity = self.world.spawn()
            .with(Transform::new(Vec3::ZERO))
            .with(StaticMesh::ceiling(ceiling_mesh))
            .build();
        ceiling_count = 1; // One ceiling mesh entity
        
        // Create collision entities for walls (separate from rendering)
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                if self.map.is_wall(x as i32, y as i32) {
                    let _wall_type = self.map.get_wall_type(x as i32, y as i32);
                    
                    // Check if this is a middle pillar (roughly center of map)
                    let is_middle_pillar = (x == 4 || x == 5) && (y == 4 || y == 5);
                    
                    // Create collision entity for this wall position
                    let collision_entity = self.world.spawn()
                        .with(Transform::new(Vec3::new(x as f32 + 0.5, 1.0, y as f32 + 0.5)))
                        .with(Collider::static_solid(ColliderShape::Box { size: Vec3::new(1.0, 2.0, 1.0) }))
                        .with(StaticRenderer::wall("invisible".to_string()).with_enabled(false)) // Invisible renderer for collision query
                        .with(Wall::new())
                        .build();
                    
                    // Track middle pillar entities for toggling
                    if is_middle_pillar {
                        self.middle_pillar_entities.push(collision_entity);
                    }
                    
                    wall_count += 1;
                }
            }
        }
        
        println!("ECS: Populated world with {} walls, {} floors, {} ceilings ({} total entities)", 
                 wall_count, floor_count, ceiling_count, wall_count + floor_count + ceiling_count + 1); // +1 for player
        
        // Note: Default light creation is now handled by world config system
        // Only create default light if no config system is being used
        // This will be overridden by hot-reload system if active
    }
    
    /// Check collision with ECS entities that have colliders at the given position
    fn check_ecs_collision(&self, x: f32, z: f32) -> bool {
        // Use the centralized collision detection from the Collider component
        Collider::check_grid_collision(&self.world, x, z)
    }
    
    /// Update the ECS game state with centralized input
    pub fn update_with_input(&mut self, delta_time: f32, input: &PlayerInput) {
        self.frame_count += 1;
        
        // Handle manual pillar toggling (T key)
        if input.toggle_pillars_pressed {
            self.toggle_middle_pillars();
        }
        
        // Automatic pillar toggling during tests (every 3 seconds: hidden for 1s, visible for 2s)
        if self.has_test_bot() {
            let elapsed = self.last_pillar_toggle_time.elapsed().as_secs_f32();
            
            if self.pillars_enabled && elapsed >= 2.0 {
                // Pillars have been visible for 2 seconds, hide them for 1 second
                self.toggle_middle_pillars();
                println!("🤖 Auto-hiding pillars for pathfinding test (1 second)");
            } else if !self.pillars_enabled && elapsed >= 1.0 {
                // Pillars have been hidden for 1 second, show them for 2 seconds
                self.toggle_middle_pillars();
                println!("🤖 Auto-showing pillars for pathfinding test (2 seconds)");
            }
        }
        
        // Handle performance test
        if input.performance_test_pressed {
            Self::run_comprehensive_performance_tests();
        }
        
        // Handle visual test with integrated performance testing (F1 key)
        if input.debug_info_pressed {
            println!("🤖 F1 pressed - Starting visual test with integrated performance testing...");
            self.attach_test_bot(30); // 30 second test
        }
        
        // Disable user input when test bot is active
        let test_active = self.has_test_bot();
        
        // Debug: Print test status (reduced frequency to avoid console I/O)
        if test_active && self.frame_count % 900 == 0 { // Every 15 seconds at 60fps
            println!("🔍 DEBUG: Test active, user input disabled");
            // Print detailed debug info every 15 seconds during tests
            self.print_debug_info();
        }
        
        // For now, implement basic player movement directly in the ECS state
        // This avoids the complex borrowing issues in the systems
        // But only if no test is running (tests control the player)
        if let Some(player_entity) = self.player_entity {
            if !test_active {
            // Update player transform based on input
            if let Some(transform) = self.world.get_mut::<Transform>(player_entity) {
                // Apply mouse look
                if input.has_look_input() {
                    transform.rotation.y -= input.mouse_delta.x * input.mouse_sensitivity;
                    transform.rotation.x += input.mouse_delta.y * input.mouse_sensitivity;
                    
                    // Clamp pitch
                    let max_pitch = std::f32::consts::PI * 0.47;
                    transform.rotation.x = transform.rotation.x.clamp(-max_pitch, max_pitch);
                }
                
                // Apply keyboard rotation (fallback)
                transform.rotation.y += input.turn_delta * input.turn_speed * delta_time;
                
                // Apply movement with ECS collision detection
                if input.has_movement() {
                    let yaw = transform.rotation.y;
                    let current_pos_x = transform.position.x;
                    let current_pos_z = transform.position.z;
                    
                    // Forward/backward movement with ECS collision detection
                    if input.forward_move != 0.0 {
                        let move_distance = input.forward_move * input.move_speed * delta_time;
                        let new_x = current_pos_x + move_distance * yaw.cos();
                        let new_z = current_pos_z + move_distance * yaw.sin();
                        
                        // Store current position values to avoid borrowing conflicts
                        let check_pos_z = transform.position.z;
                        let check_pos_x = transform.position.x;
                        
                        // Release the mutable borrow before calling collision detection
                        drop(transform);
                        
                        // Check collision with ECS entities using proper shape-based collision
                        let test_position = Vec3::new(new_x, 0.6, check_pos_z);
                        let collision_x = Collider::check_position_collision(&self.world, test_position, 0.25);
                        let test_position = Vec3::new(check_pos_x, 0.6, new_z);
                        let collision_z = Collider::check_position_collision(&self.world, test_position, 0.25);
                        
                        // Re-acquire mutable borrow to update position
                        if let Some(transform) = self.world.get_mut::<Transform>(player_entity) {
                            if !collision_x {
                                transform.position.x = new_x;
                            }
                            
                            if !collision_z {
                                transform.position.z = new_z;
                            }
                        }
                    }
                    
                    // Strafe movement with ECS collision detection
                    if input.strafe_move != 0.0 {
                        if let Some(transform) = self.world.get_mut::<Transform>(player_entity) {
                            let strafe_distance = input.strafe_move * input.move_speed * delta_time;
                            let strafe_angle = yaw + std::f32::consts::PI / 2.0;
                            let new_x = transform.position.x + strafe_distance * strafe_angle.cos();
                            let new_z = transform.position.z + strafe_distance * strafe_angle.sin();
                            
                            // Store current position values to avoid borrowing conflicts
                            let check_pos_z = transform.position.z;
                            let check_pos_x = transform.position.x;
                            
                            // Release the mutable borrow before calling collision detection
                            drop(transform);
                            
                            // Check collision with ECS entities using proper shape-based collision
                            let test_position = Vec3::new(new_x, 0.6, check_pos_z);
                            let collision_x = Collider::check_position_collision(&self.world, test_position, 0.25);
                            let test_position = Vec3::new(check_pos_x, 0.6, new_z);
                            let collision_z = Collider::check_position_collision(&self.world, test_position, 0.25);
                            
                            // Re-acquire mutable borrow to update position
                            if let Some(transform) = self.world.get_mut::<Transform>(player_entity) {
                                if !collision_x {
                                    transform.position.x = new_x;
                                }
                                
                                if !collision_z {
                                    transform.position.z = new_z;
                                }
                            }
                        }
                    }
                }
            }
            
            // Handle jumping and gravity with proper physics using Velocity component
            let mut player_grounded = true;
            let mut should_jump = false;
            
            // First, check if player is grounded and handle jump input
            if let Some(player) = self.world.get_mut::<Player>(player_entity) {
                player_grounded = player.is_grounded;
                
                // Jumping with proper physics - only if grounded
                if input.jump_pressed && player.is_grounded {
                    should_jump = true;
                    player.is_grounded = false;
                }
            }
            
            // Set jump velocity if needed
            if should_jump {
                if let Some(velocity) = self.world.get_mut::<Velocity>(player_entity) {
                    velocity.linear.y = 4.5; // Same jump strength as legacy system
                }
            }
            
            // Apply gravity to velocity
            if !player_grounded {
                if let Some(velocity) = self.world.get_mut::<Velocity>(player_entity) {
                    let gravity = 12.0;
                    velocity.linear.y -= gravity * delta_time;
                }
            }
            
            // Apply vertical velocity to position and check for landing
            let mut vertical_velocity = 0.0;
            if let Some(velocity) = self.world.get::<Velocity>(player_entity) {
                vertical_velocity = velocity.linear.y;
            }
            
            if let Some(transform) = self.world.get_mut::<Transform>(player_entity) {
                transform.position.y += vertical_velocity * delta_time;
                
                // Check if we've landed (same ground height as legacy)
                let ground_height = 0.6;
                if transform.position.y <= ground_height {
                    transform.position.y = ground_height;
                    
                    // Stop vertical movement and set grounded
                    if let Some(velocity) = self.world.get_mut::<Velocity>(player_entity) {
                        velocity.linear.y = 0.0;
                    }
                    
                    if let Some(player) = self.world.get_mut::<Player>(player_entity) {
                        player.is_grounded = true;
                    }
                }
            }
            } // Close the !test_active condition
        } // Close the player_entity check
        
        // Update all ECS components/systems
        self.update_ecs_systems(delta_time);
        
        // Continue with visual feedback for tests
        if self.has_test_bot() {
            
            // Visual feedback when test is running
            if self.frame_count % 120 == 0 { // Every 2 seconds at 60fps
                if let Some((current, total, progress)) = self.get_test_bot_progress() {
                    println!("🤖 Test Bot Progress: {}/{} waypoints ({:.1}%)", current, total, progress * 100.0);
                }
            }
        }
        
        // Run lighting systems (DISABLED for performance - was causing massive FPS drop)
        // TODO: Optimize lighting system to run less frequently or use spatial partitioning
        // crate::ecs::systems::wall_lighting_setup_system(&mut self.world);
        // crate::ecs::systems::lighting_system(&mut self.world, self.frame_count as f32 * 0.016);
        
        // Run systems (commented out for now due to borrowing issues)
        // self.systems.run_all(&mut self.world);
    }
    
    /// Update the ECS game state (legacy method)
    /// This method is kept for compatibility but now delegates to update_with_input
    pub fn update(&mut self, delta_time: f32) {
        // Create a temporary input handler to capture input for legacy compatibility
        let mut input_handler = super::input::InputHandler::new();
        let input = input_handler.capture_input();
        self.update_with_input(delta_time, &input);
    }
    
    /// Get player transform for rendering
    pub fn get_player_transform(&self) -> Option<&Transform> {
        if let Some(player_entity) = self.player_entity {
            self.world.get::<Transform>(player_entity)
        } else {
            None
        }
    }
    
    /// Get player component for debugging
    pub fn get_player(&self) -> Option<&Player> {
        if let Some(player_entity) = self.player_entity {
            self.world.get::<Player>(player_entity)
        } else {
            None
        }
    }
    
    /// Convert ECS transform to legacy player format for compatibility
    pub fn get_legacy_player_data(&self) -> Option<LegacyPlayerData> {
        if let Some(player_entity) = self.player_entity {
            let transform = self.world.get::<Transform>(player_entity)?;
            let player = self.world.get::<Player>(player_entity)?;
            
            // Use the rotation values directly (they're stored as Euler angles)
            let yaw = transform.rotation.y;
            let pitch = transform.rotation.x;
            
            Some(LegacyPlayerData {
                x: transform.position.x,
                y: transform.position.z, // Note: Z in 3D becomes Y in 2D
                z: transform.position.y, // Note: Y in 3D is height
                rotation: yaw,
                pitch,
                is_grounded: player.is_grounded,
            })
        } else {
            None
        }
    }
    
    /// Attach a test bot to automatically navigate through waypoints for testing
    pub fn attach_test_bot(&mut self, test_duration_seconds: u64) {
        // Print initial debug state
        println!("🔍 ECS Debug Info - Test Start:");
        self.print_debug_info();
        
        // Create test bot entity with all necessary components
        let entity = self.world.spawn()
            .with(Transform::new(Vec3::new(1.5, 0.6, 1.5)))  // Start position
            .with(Player::new())
            .with(TestBot::new(test_duration_seconds))
            .with(Pathfinder::new(2.0, 5.0))  // movement_speed, rotation_speed
            .with(Collider::dynamic_solid(ColliderShape::Box { size: Vec3::new(0.5, 1.8, 0.5) }))
            .with(Velocity::new())
            .entity();
        
        self.player_entity = Some(entity);
        
        println!("🤖 Test bot attached: Entity {:?} with {} second duration", entity, test_duration_seconds);
    }
    
    /// Check if any entity has test bot component
    pub fn has_test_bot(&self) -> bool {
        for (_, _) in self.world.query_1::<TestBot>() {
            return true;
        }
        false
    }
    
    /// Get test bot progress for UI display (from any test bot entity)
    pub fn get_test_bot_progress(&self) -> Option<(usize, usize, f32)> {
        for (_, test_bot) in self.world.query_1::<TestBot>() {
            return Some(test_bot.get_progress());
        }
        None
    }
    
    /// Check if any test bot is finished
    pub fn is_test_bot_finished(&self) -> bool {
        for (_, test_bot) in self.world.query_1::<TestBot>() {
            if test_bot.is_finished() {
                return true;
            }
        }
        false
    }
    
    /// Update all ECS systems and components - unified approach
    fn update_ecs_systems(&mut self, delta_time: f32) {
        // Auto-update all registered components using inventory system
        // This handles self-contained component updates (TestBot, Pathfinder, etc.)
        self.world.update_all_components(delta_time);
        
        // Process cross-component systems that need access to external resources
        self.process_pathfinding_systems(delta_time);  // Needs PathfindingAlgorithms
        self.process_lighting_systems(delta_time);     // Needs cross-component queries
    }

    /// Process pathfinding systems that need access to PathfindingAlgorithms
    fn process_pathfinding_systems(&mut self, delta_time: f32) {
        // Collect entities with pathfinders to avoid borrowing conflicts
        let pathfinder_entities: Vec<crate::ecs::Entity> = {
            let mut entities = Vec::new();
            for (entity, _pathfinder) in self.world.query_1::<Pathfinder>() {
                entities.push(entity);
            }
            entities
        };
        
        // Process pathfinding for all entities that have pathfinder components
        for entity in pathfinder_entities {
            // Handle pathfinding logic that needs PathfindingAlgorithms
            self.process_entity_pathfinding(entity, delta_time);
            
            // Handle TestBot waypoint progression if this entity has a TestBot
            if self.world.has::<TestBot>(entity) {
                self.update_test_bot_waypoints(entity);
            }
        }
    }
    
    /// Process pathfinding for a specific entity (moved from PathfindingSystem)
    fn process_entity_pathfinding(&mut self, entity: Entity, delta_time: f32) {
        // Check if entity is valid
        if !self.world.is_valid(entity) {
            return; // Skip invalid entities
        }

        // Get current position
        let current_position = {
            if let Some(transform) = self.world.get::<Transform>(entity) {
                if !entity.enabled || !transform.is_enabled() {
                    return; // Skip if entity or transform is disabled
                }
                Vec2::new(transform.position.x, transform.position.z)
            } else {
                return; // No transform, can't pathfind
            }
        };

        // Check if pathfinder needs recalculation or path following
        let needs_recalc = {
            if let Some(pathfinder) = self.world.get::<Pathfinder>(entity) {
                if !entity.enabled || !pathfinder.is_enabled() {
                    return; // Skip if entity or pathfinder is disabled
                }
                pathfinder.needs_recalculation
            } else {
                return; // No pathfinder component
            }
        };

        // Recalculate path if needed
        if needs_recalc {
            let target = {
                if let Some(pathfinder) = self.world.get::<Pathfinder>(entity) {
                    pathfinder.target
                } else {
                    return;
                }
            };
            
            if let Some(target) = target {
                // Use ECS-aware pathfinding that respects disabled entities
                let result = self.pathfinding_algorithms.find_path_with_ecs(current_position, target, &self.world);
                
                // Update the pathfinder with results
                if let Some(pathfinder) = self.world.get_mut::<Pathfinder>(entity) {
                    if result.found {
                        pathfinder.current_path = result.path;
                        pathfinder.explored_nodes = result.explored_nodes;
                        pathfinder.path_index = 0;
                        pathfinder.needs_recalculation = false;
                        
                        println!("🗺️ A* pathfinding: Found path with {} steps, explored {} nodes", 
                                 pathfinder.current_path.len(), pathfinder.explored_nodes.len());
                    } else {
                        println!("❌ A* pathfinding: No path found from ({:.1}, {:.1}) to ({:.1}, {:.1})", 
                                 current_position.x, current_position.y, target.x, target.y);
                        pathfinder.clear_path();
                    }
                }
            }
        }

        // Follow the current path
        self.follow_path(entity, current_position, delta_time);
    }

    /// Follow the current calculated path (moved from PathfindingSystem)
    fn follow_path(&mut self, entity: Entity, current_position: Vec2, delta_time: f32) {
        let (next_target, movement_speed, rotation_speed, arrival_threshold) = {
            if let Some(pathfinder) = self.world.get::<Pathfinder>(entity) {
                let next_target = pathfinder.get_next_position();
                (next_target, pathfinder.movement_speed, pathfinder.rotation_speed, pathfinder.arrival_threshold)
            } else {
                return;
            }
        };

        if let Some(target) = next_target {
            // Calculate movement toward target
            let direction = target - current_position;
            let distance = direction.length();

            if distance < arrival_threshold {
                // Reached current path step, advance to next
                if let Some(pathfinder) = self.world.get_mut::<Pathfinder>(entity) {
                    pathfinder.advance_path_step();
                    println!("✅ Pathfinding: Reached waypoint at ({:.2}, {:.2}), advancing to next", target.x, target.y);
                    
                    // Check if we reached the final target
                    if pathfinder.has_reached_target(current_position) {
                        println!("🎯 Pathfinding: Reached final target at ({:.2}, {:.2})", target.x, target.y);
                        // Don't clear the path here - let the waypoint system handle target changes
                        // pathfinder.clear_path();
                    }
                }
            } else {
                // Move toward target
                let target_angle = direction.y.atan2(direction.x);
                
                // Update rotation and position
                if let Some(transform) = self.world.get_mut::<Transform>(entity) {
                    let current_rotation = transform.rotation.y;
                    let mut angle_diff = target_angle - current_rotation;
                    
                    // Normalize angle to [-PI, PI]
                    while angle_diff > std::f32::consts::PI { angle_diff -= 2.0 * std::f32::consts::PI; }
                    while angle_diff < -std::f32::consts::PI { angle_diff += 2.0 * std::f32::consts::PI; }
                    
                    // Update rotation
                    let max_turn = rotation_speed * delta_time;
                    let new_rotation = if angle_diff.abs() < max_turn {
                        target_angle
                    } else if angle_diff > 0.0 {
                        current_rotation + max_turn
                    } else {
                        current_rotation - max_turn
                    };
                    
                    transform.rotation.y = new_rotation;
                    
                    // Move forward if facing the target (within 30 degrees for better corner navigation)
                    let facing_threshold = 30.0_f32.to_radians();
                    if angle_diff.abs() < facing_threshold {
                        let move_distance = movement_speed * delta_time;
                        let move_x = new_rotation.cos() * move_distance;
                        let move_z = new_rotation.sin() * move_distance;
                        
                        // Store current position values to avoid borrowing conflicts
                        let current_x = transform.position.x;
                        let current_z = transform.position.z;
                        
                        // Calculate new position
                        let new_x = current_x + move_x;
                        let new_z = current_z + move_z;
                        
                        // Release the transform borrow before collision check
                        drop(transform);
                        
                        // Use ECS-aware collision check that respects disabled entities
                        if !crate::ecs::Collider::check_grid_collision(&self.world, new_x, new_z) {
                            // Re-acquire transform to update position
                            if let Some(transform) = self.world.get_mut::<Transform>(entity) {
                                transform.position.x = new_x;
                                transform.position.z = new_z;
                            }
                        }
                    }
                }
                
                // Update stuck detection with more aggressive unsticking
                let (pos_diff, stuck_time, needs_unstick) = {
                    if let Some(pathfinder) = self.world.get::<Pathfinder>(entity) {
                        let pos_diff = (current_position - pathfinder.last_position).length();
                        let stuck_time = pathfinder.stuck_time + delta_time;
                        let needs_unstick = pos_diff < 0.01 && stuck_time > 0.5;
                        (pos_diff, stuck_time, needs_unstick)
                    } else {
                        return;
                    }
                };
                
                if needs_unstick {
                    println!("⚠️ Pathfinding: Entity stuck at ({:.2}, {:.2}), trying alternative movement", 
                            current_position.x, current_position.y);
                    
                    // Try to move slightly in a different direction to unstick
                    let (unstick_x, unstick_z) = {
                        if let Some(transform) = self.world.get::<Transform>(entity) {
                            let unstick_angle = transform.rotation.y + std::f32::consts::PI / 4.0; // 45 degrees
                            let unstick_distance = 0.1; // Small movement
                            let unstick_x = transform.position.x + unstick_angle.cos() * unstick_distance;
                            let unstick_z = transform.position.z + unstick_angle.sin() * unstick_distance;
                            (unstick_x, unstick_z)
                        } else {
                            return;
                        }
                    };
                    
                    // Check if unstick position is valid
                    if !crate::ecs::Collider::check_grid_collision(&self.world, unstick_x, unstick_z) {
                        if let Some(transform) = self.world.get_mut::<Transform>(entity) {
                            transform.position.x = unstick_x;
                            transform.position.z = unstick_z;
                            println!("🔄 Unstick movement applied");
                        }
                    }
                    
                    // Update pathfinder state
                    if let Some(pathfinder) = self.world.get_mut::<Pathfinder>(entity) {
                        pathfinder.needs_recalculation = true;
                        pathfinder.stuck_time = 0.0;
                        pathfinder.last_position = current_position;
                    }
                } else {
                    // Update pathfinder state normally
                    if let Some(pathfinder) = self.world.get_mut::<Pathfinder>(entity) {
                        if pos_diff < 0.01 {
                            pathfinder.stuck_time = stuck_time;
                        } else {
                            pathfinder.stuck_time = 0.0;
                        }
                        pathfinder.last_position = current_position;
                    }
                }
            }
        }
    }
    
    /// Update TestBot waypoint management (pathfinding handles movement)
    fn update_test_bot_waypoints(&mut self, entity: Entity) {
        // Check if test bot is finished
        if let Some(test_bot) = self.world.get::<TestBot>(entity) {
            if test_bot.is_finished() {
                println!("🤖 Visual test completed after {:.1}s", test_bot.start_time.elapsed().as_secs_f32());
                return;
            }
        } else {
            return;
        }

        // Get current position and add debug output
        let current_position = {
            if let Some(transform) = self.world.get::<Transform>(entity) {
                let pos = Vec2::new(transform.position.x, transform.position.z);
                // Debug: Print position every few frames to track movement
                if self.frame_count % 300 == 0 { // Every 5 seconds at 60fps
                    println!("🔍 DEBUG: Player position: ({:.2}, {:.2})", pos.x, pos.y);
                }
                pos
            } else {
                return;
            }
        };

        // Check pathfinder status
        let (has_target, has_reached_target, path_is_empty) = {
            if let Some(pathfinder) = self.world.get::<Pathfinder>(entity) {
                let has_target = pathfinder.target.is_some();
                let has_reached = pathfinder.has_reached_target(current_position);
                let path_empty = pathfinder.current_path.is_empty();
                (has_target, has_reached, path_empty)
            } else {
                (false, false, true)
            }
        };

        // If we've reached the target OR the path is empty (indicating completion), advance waypoint
        if (has_target && has_reached_target) || (has_target && path_is_empty) {
            // First, advance the waypoint and get the new target
            let new_target_info = {
                if let Some(test_bot) = self.world.get_mut::<TestBot>(entity) {
                    test_bot.advance_waypoint();
                    let current_waypoint = test_bot.current_waypoint;
                    let new_target = test_bot.get_current_target();
                    Some((current_waypoint, new_target))
                } else {
                    None
                }
            };
            
            // Then set the new target for pathfinder
            if let Some((current_waypoint, Some(new_target))) = new_target_info {
                if let Some(pathfinder) = self.world.get_mut::<Pathfinder>(entity) {
                    pathfinder.set_target(new_target);
                    println!("🎯 TestBot advancing to waypoint {} at ({:.2}, {:.2})", 
                            current_waypoint, new_target.x, new_target.y);
                }
            }
        } else if !has_target {
            // No target set, set initial target
            if let Some(test_bot) = self.world.get::<TestBot>(entity) {
                if let Some(target) = test_bot.get_current_target() {
                    if let Some(pathfinder) = self.world.get_mut::<Pathfinder>(entity) {
                        pathfinder.set_target(target);
                        println!("🎯 TestBot set initial pathfinding target: ({:.2}, {:.2})", target.x, target.y);
                    }
                }
            }
        }
    }
    
    /// Toggle the enabled state of middle pillar entities (component-level)
    pub fn toggle_middle_pillars(&mut self) {
        self.pillars_enabled = !self.pillars_enabled;
        self.last_pillar_toggle_time = std::time::Instant::now(); // Reset timer
        
        let action = if self.pillars_enabled { "enabled" } else { "disabled" };
        println!("🏛️ Middle pillars {} (Total: {} pillars)", action, self.middle_pillar_entities.len());
        
        // Toggle each middle pillar entity's components
        for &entity in &self.middle_pillar_entities {
            // Toggle StaticRenderer component (affects rendering)
            if let Some(static_renderer) = self.world.get_mut::<StaticRenderer>(entity) {
                if self.pillars_enabled {
                    static_renderer.enable();
                } else {
                    static_renderer.disable();
                }
            }
            
            // Toggle Collider component (affects physics/collision)
            if let Some(collider) = self.world.get_mut::<Collider>(entity) {
                if self.pillars_enabled {
                    collider.enable();
                } else {
                    collider.disable();
                }
            }
        }
    }
    
    /// Get pillar toggle status for UI display
    pub fn get_pillar_status(&self) -> (bool, usize) {
        (self.pillars_enabled, self.middle_pillar_entities.len())
    }
    
    /// Get current test bot target position for minimap visualization
    pub fn get_test_bot_target(&self) -> Option<(f32, f32)> {
        if let Some(player_entity) = self.player_entity {
            if let Some(pathfinder) = self.world.get::<Pathfinder>(player_entity) {
                if let Some(target) = pathfinder.target {
                    return Some((target.x, target.y));
                }
            }
        }
        None
    }
    
    /// Get pathfinding debug information (path and explored nodes) for minimap visualization
    pub fn get_pathfinding_debug_info(&self) -> (Option<Vec<macroquad::math::Vec2>>, Option<Vec<(i32, i32)>>) {
        if let Some(player_entity) = self.player_entity {
            if let Some(pathfinder) = self.world.get::<Pathfinder>(player_entity) {
                let path = if !pathfinder.current_path.is_empty() {
                    Some(pathfinder.current_path.clone())
                } else {
                    None
                };
                
                let explored = if !pathfinder.explored_nodes.is_empty() {
                    Some(pathfinder.explored_nodes.clone())
                } else {
                    None
                };
                
                return (path, explored);
            }
        }
        (None, None)
    }
    
    /// Run integrated performance test automatically during visual tests
    pub fn run_integrated_performance_test() {
        println!("\n🔥 INTEGRATED PERFORMANCE TEST (Running alongside visual test)");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🎯 Running Quick Performance Check (100 iterations, 253 entities)...");
        
        let result = crate::testing::performance_test::PerformanceTest::run_game_realistic_test();
        
        println!("\n📊 PERFORMANCE RESULTS:");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Current ECS System Performance:");
        if result.performance_difference_percent > 20.0 {
            println!("✅ Entity.enabled approach is {:.2}% faster", result.performance_difference_percent);
            println!("💡 Significant performance difference - current approach is well-optimized");
        } else {
            println!("⚠️ Performance difference: {:.2}%", result.performance_difference_percent);
            println!("💡 Minimal performance difference - architecture choice is flexible");
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🤖 Visual test will now begin with performance-validated ECS system!");
    }
    
    /// Run comprehensive performance tests (P key - for detailed analysis)
    pub fn run_comprehensive_performance_tests() {
        println!("\n🔥 COMPREHENSIVE PERFORMANCE ANALYSIS!");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        // Run multiple test scenarios
        println!("\n🎯 Running Game-Realistic Test (1000 iterations, 253 entities)...");
        let realistic_result = PerformanceTest::run_game_realistic_test();
        
        println!("\n🚀 Running Stress Test (100 iterations, 10,000 entities)...");
        let stress_result = PerformanceTest::run_stress_test();
        
        println!("\n📈 COMPREHENSIVE ANALYSIS:");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Game-realistic scenario (253 entities):");
        println!("  • Performance difference: {:.2}%", realistic_result.performance_difference_percent);
        println!("Stress test scenario (10,000 entities):");
        println!("  • Performance difference: {:.2}%", stress_result.performance_difference_percent);
        
        if realistic_result.performance_difference_percent < 5.0 && stress_result.performance_difference_percent < 20.0 {
            println!("\n💡 RECOMMENDATION: Use entity.enabled approach for code simplicity");
            println!("   The Vec<bool> optimization provides negligible benefits at this scale");
        } else {
            println!("\n💡 RECOMMENDATION: Current Vec<bool> approach shows meaningful performance benefits");
            println!("   The optimization is justified by the performance gains");
        }
        
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
    
    /// Start lighting performance tests by adding LightingTest component
    pub fn start_lighting_tests(&mut self) {
        // Find or create a test entity and add LightingTest component
        let test_entity = self.world.spawn()
            .with(crate::ecs::LightingTest::new())
            .build();
        
        println!("🔆 Started lighting performance tests with entity: {:?}", test_entity);
    }
    
    /// Process all entities with LightingTest components
    fn process_lighting_systems(&mut self, _delta_time: f32) {
        // Collect test phase changes first to avoid borrowing conflicts
        let mut phase_changes = Vec::new();
        let mut finished_tests = Vec::new();

        // Find any lighting test entities
        let test_entities: Vec<_> = self.world
            .query_1::<crate::ecs::LightingTest>()
            .into_iter()
            .map(|(entity, _)| entity)
            .collect();

        for entity in test_entities {
            if let Some(lighting_test) = self.world.get_mut::<crate::ecs::LightingTest>(entity) {
                // Check if we should advance to the next phase
                if lighting_test.should_advance_phase() {
                    lighting_test.advance_phase();
                    
                    // Store phase change info
                    if let Some(phase) = lighting_test.get_current_phase() {
                        phase_changes.push((phase.name.clone(), phase.light_count));
                    } else {
                        // Test finished
                        finished_tests.push(entity);
                    }
                }
            }
        }
        
        // Apply phase changes after releasing the borrow
        for (phase_name, light_count) in phase_changes {
            println!("🔆 Lighting test phase: {} ({} lights)", phase_name, light_count);
            self.set_light_count(light_count);
        }
        
        // Clean up finished tests
        for entity in finished_tests {
            println!("✅ Lighting tests completed!");
            self.world.despawn(entity);
            self.remove_all_lights();
        }
    }

    /// Check if lighting tests are active
    pub fn has_lighting_test(&self) -> bool {
        self.world.query_1::<crate::ecs::LightingTest>().into_iter().next().is_some()
    }

    /// Get current lighting test info for display
    pub fn get_lighting_test_info(&self) -> Option<(String, usize, f32, f32, Color)> {
        if let Some((_, lighting_test)) = self.world.query_1::<crate::ecs::LightingTest>().into_iter().next() {
            if let Some(phase) = lighting_test.get_current_phase() {
                let elapsed = lighting_test.get_phase_elapsed_time();
                return Some((
                    phase.name.clone(),
                    phase.light_count,
                    elapsed,
                    phase.duration_seconds,
                    phase.background_color,
                ));
            }
        }
        None
    }
    
    /// Remove all lights from the ECS world
    pub fn remove_all_lights(&mut self) {
        let light_entities: Vec<_> = self.world
            .query_1::<LightSource>()
            .into_iter()
            .map(|(entity, _)| entity)
            .collect();

        for entity in light_entities {
            self.world.despawn(entity);
        }
    }
    
    /// Create a single omni light with visible sphere mesh in the center of the scene
    pub fn create_single_omni_light(&mut self) {
        self.create_single_omni_light_with_color(Color::new(1.0, 1.0, 1.0, 1.0));
    }
    
    /// Create a single omni light with custom color
    pub fn create_single_omni_light_with_color(&mut self, light_color: Color) {
        self.create_single_omni_light_with_logging_and_color(true, light_color);
    }
    
    /// Create a red omni light
    pub fn create_red_omni_light(&mut self) {
        self.create_single_omni_light_with_color(Color::new(1.0, 0.2, 0.2, 1.0));
    }
    
    /// Create a blue omni light  
    pub fn create_blue_omni_light(&mut self) {
        self.create_single_omni_light_with_color(Color::new(0.2, 0.4, 1.0, 1.0));
    }
    
    /// Create a green omni light
    pub fn create_green_omni_light(&mut self) {
        self.create_single_omni_light_with_color(Color::new(0.2, 1.0, 0.3, 1.0));
    }
    
    /// Create a warm orange omni light
    pub fn create_orange_omni_light(&mut self) {
        self.create_single_omni_light_with_color(Color::new(1.0, 0.6, 0.2, 1.0));
    }
    
    /// Create a purple omni light
    pub fn create_purple_omni_light(&mut self) {
        self.create_single_omni_light_with_color(Color::new(0.8, 0.2, 1.0, 1.0));
    }
    
    /// Create a cyan omni light
    pub fn create_cyan_omni_light(&mut self) {
        self.create_single_omni_light_with_color(Color::new(0.2, 0.9, 0.9, 1.0));
    }
    
    /// Create a single omni light with visible sphere mesh in the center of the scene
    fn create_single_omni_light_with_logging(&mut self, verbose: bool) {
        self.create_single_omni_light_with_logging_and_color(verbose, Color::new(1.0, 1.0, 1.0, 1.0));
    }
    
    /// Create a single omni light with visible sphere mesh in the center of the scene
    fn create_single_omni_light_with_logging_and_color(&mut self, verbose: bool, light_color: Color) {
        // Remove all existing lights first
        self.remove_all_lights();
        
        // Calculate center position of the map
        let center_x = self.map.width as f32 / 2.0;
        let center_z = self.map.height as f32 / 2.0;
        let center_position = Vec3::new(center_x, 1.5, center_z); // Slightly elevated
        
        // Create an omni light with the specified color
        let omni_light = LightSource::new(
            light_color,
            2.0,    // High intensity
            8.0,    // Large radius to cover most of the scene
            LightSourceType::Ambient // Static, no animation
        );
        
        // Make the sphere mesh color match the light color but slightly brighter for visibility
        let sphere_color = Color::new(
            (light_color.r + 0.3).min(1.0),
            (light_color.g + 0.3).min(1.0),
            (light_color.b + 0.3).min(1.0),
            1.0
        );
        
        // Create the light entity with both lighting and visual components
        let light_entity = self.world.spawn()
            .with(Transform::new(center_position))
            .with(omni_light)
            .with(Renderer::sphere(0.15)
                .with_color(sphere_color)
                .with_enabled(true))
            .build();
        
        // Debug: Check if entity was created successfully
        println!("🔆 DEBUG: Light entity created with ID: {:?}", light_entity);
        println!("🔆 DEBUG: Light position: {:?}", center_position);
        println!("🔆 DEBUG: Light color: {:?}", light_color);
        println!("🔆 DEBUG: Sphere color: {:?}", sphere_color);
        
        // Debug: Check if components were added
        if let Some(transform) = self.world.get::<Transform>(light_entity) {
            println!("🔆 DEBUG: Transform component found at position: {:?}", transform.position);
        } else {
            println!("❌ ERROR: Transform component not found!");
        }
        
        if self.world.has::<LightSource>(light_entity) {
            println!("🔆 DEBUG: LightSource component found");
        } else {
            println!("❌ ERROR: LightSource component not found!");
        }
        
        if self.world.has::<Renderer>(light_entity) {
            println!("🔆 DEBUG: Renderer component found");
            if let Some(renderer) = self.world.get::<Renderer>(light_entity) {
                println!("🔆 DEBUG: Renderer mode: {:?}", renderer.get_mode_name());
                println!("🔆 DEBUG: Renderer enabled: {}", renderer.enabled);
                println!("🔆 DEBUG: Renderer should_render: {}", renderer.should_render());
            }
        } else {
            println!("❌ ERROR: Renderer component not found!");
        }
        
        if verbose {
            println!("🔆 Created single omni light at center position: {:?} with color: {:?}", center_position, light_color);
        }
    }
    
    /// Set the number of lights in the world (for testing)
    pub fn set_light_count(&mut self, count: usize) {
        // Remove all existing lights
        self.remove_all_lights();
        
        // Add the requested number of lights
        if count == 0 {
            return; // No lights needed
        }
        
        // Add lights at strategic positions
        let positions = vec![
            Vec3::new(5.0, 1.0, 5.0),   // Center
            Vec3::new(2.0, 1.0, 2.0),   // Corner 1
            Vec3::new(8.0, 1.0, 2.0),   // Corner 2  
            Vec3::new(2.0, 1.0, 8.0),   // Corner 3
            Vec3::new(8.0, 1.0, 8.0),   // Corner 4
            Vec3::new(3.0, 1.0, 5.0),   // Mid-left
            Vec3::new(7.0, 1.0, 5.0),   // Mid-right
            Vec3::new(5.0, 1.0, 3.0),   // Mid-top
        ];

        // Add strategic lights first
        let strategic_count = count.min(positions.len());
        for i in 0..strategic_count {
            let light_type = match i % 4 {
                0 => LightSource::energy(1.5, 4.0),
                1 => LightSource::warning(1.5, 4.0),
                2 => LightSource::control(1.5, 4.0),
                _ => LightSource::ambient(1.0, 3.0),
            };

            self.world.spawn()
                .with(Transform::new(positions[i]))
                .with(light_type)
                .build();
        }
        
        // Add random lights if more are needed
        if count > strategic_count {
            use ::rand::Rng;
            let mut rng = ::rand::thread_rng();
            
            for i in strategic_count..count {
                let x = rng.gen_range(1.0..9.0);
                let y = 1.0;
                let z = rng.gen_range(1.0..9.0);
                
                let position = Vec3::new(x, y, z);
                
                let light_type = match i % 4 {
                    0 => LightSource::warning(rng.gen_range(0.5..2.0), rng.gen_range(2.0..6.0)),
                    1 => LightSource::energy(rng.gen_range(0.5..2.0), rng.gen_range(2.0..6.0)),
                    2 => LightSource::control(rng.gen_range(0.5..2.0), rng.gen_range(2.0..6.0)),
                    _ => LightSource::ambient(rng.gen_range(0.3..1.5), rng.gen_range(1.0..4.0)),
                };

                self.world.spawn()
                    .with(Transform::new(position))
                    .with(light_type)
                    .build();
            }
        }
    }

    /// Update the map for pathfinding (moved from PathfindingSystem)
    pub fn update_pathfinding_map(&mut self, map: Map) {
        self.pathfinding_algorithms.update_map(map);
    }

    /// Get detailed debug information about ECS world state
    pub fn get_debug_info(&self) -> String {
        let mut debug_info = String::new();
        
        debug_info.push_str("🔍 === ECS DEBUG INFO ===\n");
        
        // Count entities by type
        let mut entity_counts = HashMap::new();
        let mut total_entities = 0;
        
        // Count different entity types
        for (entity, _) in self.world.query_1::<Transform>() {
            if !self.world.is_valid(entity) || !entity.enabled {
                continue;
            }
            
            total_entities += 1;
            
            // Determine entity type based on components
            let entity_type = if self.world.has::<Player>(entity) {
                "Player"
            } else if self.world.has::<TestBot>(entity) {
                "TestBot"
            } else if self.world.has::<StaticMesh>(entity) {
                // Determine StaticMesh type
                if let Some(static_mesh) = self.world.get::<StaticMesh>(entity) {
                    match static_mesh.mesh_type {
                        StaticMeshType::Walls => "WallMesh",
                        StaticMeshType::Floor => "FloorMesh", 
                        StaticMeshType::Ceiling => "CeilingMesh",
                        StaticMeshType::Props => "PropMesh",
                    }
                } else {
                    "StaticMesh"
                }
            } else if self.world.has::<Wall>(entity) {
                "Wall"
            } else if self.world.has::<Floor>(entity) {
                "Floor"
            } else if self.world.has::<Ceiling>(entity) {
                "Ceiling"
            } else if self.world.has::<Prop>(entity) {
                "Prop"
            } else if self.world.has::<LightSource>(entity) {
                "LightSource"
            } else if self.world.has::<StaticRenderer>(entity) {
                "StaticRenderer"
            } else {
                "Unknown"
            };
            
            *entity_counts.entry(entity_type).or_insert(0) += 1;
        }
        
        debug_info.push_str(&format!("📊 Total Active Entities: {}\n", total_entities));
        debug_info.push_str("📋 Entity Types:\n");
        
        for (entity_type, count) in &entity_counts {
            debug_info.push_str(&format!("   • {}: {}\n", entity_type, count));
        }
        
        // Count components with specific functionality
        let mut pathfinder_count = 0;
        let mut collider_count = 0;
        let mut velocity_count = 0;
        let mut light_receiver_count = 0;
        
        for (entity, _) in self.world.query_1::<Transform>() {
            if !self.world.is_valid(entity) || !entity.enabled {
                continue;
            }
            
            if self.world.has::<Pathfinder>(entity) {
                pathfinder_count += 1;
            }
            if self.world.has::<Collider>(entity) {
                collider_count += 1;
            }
            if self.world.has::<Velocity>(entity) {
                velocity_count += 1;
            }
            if self.world.has::<LightReceiver>(entity) {
                light_receiver_count += 1;
            }
        }
        
        debug_info.push_str("🔧 Component Usage:\n");
        debug_info.push_str(&format!("   • Pathfinder: {}\n", pathfinder_count));
        debug_info.push_str(&format!("   • Collider: {}\n", collider_count));
        debug_info.push_str(&format!("   • Velocity: {}\n", velocity_count));
        debug_info.push_str(&format!("   • LightReceiver: {}\n", light_receiver_count));
        
        // Special entities status
        debug_info.push_str("🎯 Special Entities:\n");
        debug_info.push_str(&format!("   • Player Entity: {:?}\n", self.player_entity));
        debug_info.push_str(&format!("   • Middle Pillars: {} ({})\n", 
            self.middle_pillar_entities.len(),
            if self.pillars_enabled { "ENABLED" } else { "DISABLED" }
        ));
        
        // Test status
        if self.has_test_bot() {
            if let Some((current, total, progress)) = self.get_test_bot_progress() {
                debug_info.push_str(&format!("🤖 Test Bot: {}/{} waypoints ({:.1}%)\n", current, total, progress * 100.0));
            } else {
                debug_info.push_str("🤖 Test Bot: Active (initializing)\n");
            }
        } else {
            debug_info.push_str("🤖 Test Bot: Inactive\n");
        }
        
        if self.has_lighting_test() {
            if let Some((test_name, light_count, elapsed, duration, _)) = self.get_lighting_test_info() {
                debug_info.push_str(&format!("💡 Lighting Test: {} ({} lights, {:.1}s/{:.1}s)\n", 
                    test_name, light_count, elapsed, duration));
            }
        } else {
            debug_info.push_str("💡 Lighting Test: Inactive\n");
        }
        
        debug_info.push_str("🔍 === END DEBUG INFO ===\n");
        debug_info
    }
    
    /// Print comprehensive debug information
    pub fn print_debug_info(&self) {
        println!("{}", self.get_debug_info());
    }
}

/// Temporary structure to bridge ECS and legacy systems
#[derive(Debug, Clone)]
pub struct LegacyPlayerData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation: f32,
    pub pitch: f32,
    pub is_grounded: bool,
} 