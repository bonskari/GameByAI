//! ECS-based game state for gradual migration

use macroquad::prelude::*;
use crate::ecs::*;
use crate::ecs::systems::*;
use crate::testing::performance_test::PerformanceTest;
use super::map::{Map, WallType};
use super::input::PlayerInput;
use crate::game::level_generator::LevelMeshBuilder;
use std::time::Instant;

/// ECS-based game state that manages all entities and systems
pub struct EcsGameState {
    pub world: World,
    pub systems: SystemManager,
    pub pathfinding_system: PathfindingSystem,
    pub player_entity: Option<Entity>,
    pub map: Map,
    pub frame_count: u32,
    pub middle_pillar_entities: Vec<Entity>,  // Track middle pillars for toggling
    pub pillars_enabled: bool,                // Current state of middle pillars
    pub last_pillar_toggle_time: std::time::Instant, // Track when pillars were last toggled
}

impl EcsGameState {
    /// Create a new ECS game state
    pub fn new() -> Self {
        let mut world = World::new();
        let mut systems = SystemManager::new();
        
        // Add systems in the correct order (commented out for now)
        // systems.add_system(PlayerMovementSystem::new());
        // systems.add_system(PhysicsSystem::new());
        // systems.add_system(CollisionSystem::new());
        
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
            systems,
            pathfinding_system: PathfindingSystem::new(map.clone()),
            player_entity: Some(player_entity),
            map,
            frame_count: 0,
            middle_pillar_entities: Vec::new(),
            pillars_enabled: true,
            last_pillar_toggle_time: std::time::Instant::now(),
        }
    }
    
    /// Async initialization that sets up the static geometry with textures
    pub async fn initialize(&mut self) {
        // Populate the world with static geometry entities
        self.populate_static_geometry().await;
    }
    
    /// Populate the ECS world with wall, floor, and ceiling entities using StaticRenderer
    async fn populate_static_geometry(&mut self) {
        let mut wall_count = 0;
        let mut floor_count = 0;
        let mut ceiling_count = 0;
        
        // Create ceiling entities for the entire map (keeping individual entities for now)
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                // Ceiling entity at each position
                self.world.spawn()
                    .with(Transform::new(Vec3::new(x as f32 + 0.5, 2.0, y as f32 + 0.5)))
                    .with(StaticRenderer::ceiling("ceiling.png".to_string()))
                    .with(Collider::static_trigger(ColliderShape::Box { size: Vec3::new(1.0, 0.1, 1.0) }))
                    .with(Ceiling::new())
                    .build();
                ceiling_count += 1;
            }
        }
        
        // Create separate wall meshes for each texture type
        let mesh_builder = LevelMeshBuilder::new(self.map.clone());
        
        // Generate separate meshes for each wall type
        for wall_type in [WallType::TechPanel, WallType::HullPlating, WallType::ControlSystem, WallType::EnergyConduit] {
            let wall_mesh = mesh_builder.generate_wall_mesh_for_type(wall_type).await;
            
            // Only create entity if mesh has geometry
            if !wall_mesh.vertices.is_empty() {
                let _wall_mesh_entity = self.world.spawn()
                    .with(Transform::new(Vec3::ZERO))  // World origin since mesh contains world coordinates
                    .with(WallMesh::new().with_mesh(wall_mesh))
                    .build();
            }
        }
        
        // Generate and create the single floor mesh entity
        let floor_mesh = mesh_builder.generate_floor_mesh_with_texture().await;
        let _floor_mesh_entity = self.world.spawn()
            .with(Transform::new(Vec3::ZERO))
            .with(FloorMesh::new(floor_mesh))
            .build();
        
        floor_count = 1; // One floor mesh entity
        
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
        
        wall_count += 1; // One additional entity for the rendering mesh
        
        println!("ECS: Populated world with {} walls, {} floors, {} ceilings ({} total entities)", 
                 wall_count, floor_count, ceiling_count, wall_count + floor_count + ceiling_count + 1); // +1 for player
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
        
        // Debug: Print test status
        if test_active && self.frame_count % 300 == 0 { // Every 5 seconds at 60fps
            println!("🔍 DEBUG: Test is active, user input should be disabled");
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
        
        // Update all test bot entities
        if self.has_test_bot() {
            self.update_all_test_bots(delta_time);
            
            // Visual feedback when test is running
            if self.frame_count % 120 == 0 { // Every 2 seconds at 60fps
                if let Some((current, total, progress)) = self.get_test_bot_progress() {
                    println!("🤖 Test Bot Progress: {}/{} waypoints ({:.1}%)", current, total, progress * 100.0);
                }
            }
        }
        
        // Run lighting systems
        crate::ecs::systems::wall_lighting_setup_system(&mut self.world);
        crate::ecs::systems::lighting_system(&mut self.world, self.frame_count as f32 * 0.016); // Approximate time
        
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
    
    /// Attach a test bot component to the player entity with integrated performance testing
    pub fn attach_test_bot(&mut self, test_duration_seconds: u64) {
        if let Some(player_entity) = self.player_entity {
            let test_bot = TestBot::new(test_duration_seconds);
            let pathfinder = Pathfinder::new(2.0, 5.0); // Slower movement for better precision, faster rotation
            
            self.world.add(player_entity, test_bot);
            self.world.add(player_entity, pathfinder);
            
            // Run integrated performance test
            Self::run_integrated_performance_test();
            
            println!("🤖 TestBot attached with A* pathfinding capabilities");
        }
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
    
    /// Update all entities with TestBot components (proper ECS approach)
    fn update_all_test_bots(&mut self, delta_time: f32) {
        // Collect entities first to avoid borrowing conflicts
        let test_bot_entities: Vec<crate::ecs::Entity> = {
            let mut entities = Vec::new();
            for (entity, _test_bot) in self.world.query_1::<TestBot>() {
                entities.push(entity);
            }
            entities
        };
        
        // Process pathfinding for all entities with TestBot components
        for entity in test_bot_entities {
            // Update pathfinding for this test bot entity
            self.pathfinding_system.process_entity_pathfinding(&mut self.world, entity, delta_time);
            
            // Update the test bot's waypoint progression
            self.update_test_bot_waypoints(entity);
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