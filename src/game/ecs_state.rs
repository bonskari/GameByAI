//! ECS-based game state for gradual migration

use macroquad::prelude::*;
use crate::ecs::*;
use super::map::{Map, WallType};
use super::input::PlayerInput;

/// ECS-based game state
pub struct EcsGameState {
    pub world: World,
    pub systems: SystemManager,
    pub player_entity: Option<Entity>,
    pub map: Map,
    pub frame_count: u32,
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
            .with(Collider::dynamic_solid(ColliderShape::Capsule { height: 1.8, radius: 0.3 }))
            .build();
        
        let map = Map::new();
        
        let mut ecs_state = Self {
            world,
            systems,
            player_entity: Some(player_entity),
            map,
            frame_count: 0,
        };
        
        // Populate the world with static geometry entities
        ecs_state.populate_static_geometry();
        
        ecs_state
    }
    
    /// Populate the ECS world with wall, floor, and ceiling entities using StaticRenderer
    fn populate_static_geometry(&mut self) {
        let mut wall_count = 0;
        let mut floor_count = 0;
        let mut ceiling_count = 0;
        
        // Create floor entities for the entire map
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                // Floor entity at each position
                self.world.spawn()
                    .with(Transform::new(Vec3::new(x as f32 + 0.5, 0.0, y as f32 + 0.5)))
                    .with(StaticRenderer::floor("floor.png".to_string()))
                    .with(Collider::static_trigger(ColliderShape::Box { size: Vec3::new(1.0, 0.1, 1.0) }))
                    .with(Floor)
                    .build();
                floor_count += 1;
                
                // Ceiling entity at each position
                self.world.spawn()
                    .with(Transform::new(Vec3::new(x as f32 + 0.5, 2.0, y as f32 + 0.5)))
                    .with(StaticRenderer::ceiling("ceiling.png".to_string()))
                    .with(Collider::static_trigger(ColliderShape::Box { size: Vec3::new(1.0, 0.1, 1.0) }))
                    .with(Ceiling)
                    .build();
                ceiling_count += 1;
            }
        }
        
        // Create wall entities where walls exist in the map
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                if self.map.is_wall(x as i32, y as i32) {
                    let wall_type = self.map.get_wall_type(x as i32, y as i32);
                    let texture_name = match wall_type {
                        WallType::Empty => continue, // Skip empty tiles
                        WallType::TechPanel => "tech_panel.png",
                        WallType::HullPlating => "hull_plating.png", 
                        WallType::ControlSystem => "control_system.png",
                        WallType::EnergyConduit => "energy_conduit.png",
                    };
                    
                    // Wall entity
                    self.world.spawn()
                        .with(Transform::new(Vec3::new(x as f32 + 0.5, 1.0, y as f32 + 0.5)))
                        .with(StaticRenderer::wall(texture_name.to_string()))
                        .with(Collider::static_solid(ColliderShape::Box { size: Vec3::new(1.0, 2.0, 1.0) }))
                        .with(Wall)
                        .build();
                    wall_count += 1;
                }
            }
        }
        
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
        
        // For now, implement basic player movement directly in the ECS state
        // This avoids the complex borrowing issues in the systems
        if let Some(player_entity) = self.player_entity {
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
                        
                        // Check collision with ECS entities
                        let collision_x = self.check_ecs_collision(new_x, check_pos_z);
                        let collision_z = self.check_ecs_collision(check_pos_x, new_z);
                        
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
                            
                            // Check collision with ECS entities
                            let collision_x = self.check_ecs_collision(new_x, check_pos_z);
                            let collision_z = self.check_ecs_collision(check_pos_x, new_z);
                            
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
        }
        
        // Run systems (commented out for now due to borrowing issues)
        // self.systems.run_all(&mut self.world);
    }
    
    /// Update the ECS game state (legacy method)
    /// This method is kept for compatibility but now delegates to update_with_input
    pub fn update(&mut self, delta_time: f32) {
        // Create a temporary input handler to capture input for legacy compatibility
        let input_handler = super::input::InputHandler::new();
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