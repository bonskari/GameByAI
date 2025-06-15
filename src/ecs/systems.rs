//! Game-specific systems for the 3D game

use macroquad::prelude::*;
use crate::ecs::{System, World, Component};
use crate::ecs::components::*;
use crate::game::map::Map;

/// Resource for delta time
#[derive(Debug, Clone)]
pub struct DeltaTime(pub f32);

impl crate::ecs::Resource for DeltaTime {}

/// Resource for game map
#[derive(Debug, Clone)]
pub struct MapResource {
    pub map: Map,
}

impl crate::ecs::Resource for MapResource {}

/// System that handles player input and movement
pub struct PlayerMovementSystem;

impl System for PlayerMovementSystem {
    fn run(&mut self, world: &mut World) {
        let dt = 1.0 / 60.0; // Default delta time, should be passed as resource
        
        // Query for entities with Player, Transform, and Velocity components
        // We need to collect entity IDs first to avoid borrowing conflicts
        let mut player_entities = Vec::new();
        
        if let Some(player_storage) = world.storage::<Player>() {
            for (entity, _) in player_storage.iter() {
                player_entities.push(entity);
            }
        }
        
        // Now process each player entity
        for entity in player_entities {
            // Get player data first (immutable borrow)
            let player_data = if let Some(player) = world.get::<Player>(entity) {
                (player.move_speed, player.mouse_sensitivity, player.jump_force, player.is_grounded)
            } else {
                continue;
            };
            
            // Now get mutable references using the safe pair method
            let (transform_opt, velocity_opt) = world.get_mut_pair::<Transform, Velocity>(entity);
            if let (Some(transform), Some(velocity)) = (transform_opt, velocity_opt) {
                self.update_player_movement_with_data(player_data, transform, velocity, dt);
            }
        }
    }
}

impl PlayerMovementSystem {
    pub fn new() -> Self {
        Self
    }
    
    fn update_player_movement_with_data(&self, player_data: (f32, f32, f32, bool), transform: &mut Transform, velocity: &mut Velocity, dt: f32) {
        let (move_speed, mouse_sensitivity, jump_force, is_grounded) = player_data;
        
        let mut forward_move = 0.0;
        let mut strafe_move = 0.0;
        let mut turn_direction = 0.0;
        
        // WASD movement controls - modern FPS style
        if is_key_down(KeyCode::W) {
            forward_move += 1.0;
        }
        if is_key_down(KeyCode::S) {
            forward_move -= 1.0;
        }
        if is_key_down(KeyCode::A) {
            strafe_move -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            strafe_move += 1.0;
        }
        
        // Jumping with Spacebar - only if grounded
        if is_key_pressed(KeyCode::Space) && is_grounded {
            velocity.linear.y = jump_force;
        }
        
        // Arrow keys for turning (backup controls)
        if is_key_down(KeyCode::Left) {
            turn_direction -= 1.0;
        }
        if is_key_down(KeyCode::Right) {
            turn_direction += 1.0;
        }
        
        // Mouse look - get mouse delta and apply rotation
        let mouse_delta = mouse_delta_position();
        
        // Apply mouse movement for smooth rotation
        if mouse_delta.x.abs() > 0.001 || mouse_delta.y.abs() > 0.001 {
            // Horizontal mouse movement controls yaw (Y-axis rotation)
            let yaw_delta = -mouse_delta.x * mouse_sensitivity;
            transform.rotate_y(yaw_delta);
            
            // Vertical mouse movement controls pitch (X-axis rotation)
            let pitch_delta = mouse_delta.y * mouse_sensitivity;
            transform.rotate_x(pitch_delta);
            
            // Clamp pitch to prevent over-rotation
            let (_, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            let max_pitch = std::f32::consts::PI * 0.47; // ~85 degrees
            let clamped_pitch = pitch.clamp(-max_pitch, max_pitch);
            
            // Reconstruct rotation with clamped pitch
            let (yaw, _, roll) = transform.rotation.to_euler(EulerRot::YXZ);
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, clamped_pitch, roll);
        }
        
        // Apply keyboard rotation (fallback)
        if turn_direction != 0.0 {
            transform.rotate_y(turn_direction * 3.0 * dt); // turn_speed = 3.0
        }
        
        // Calculate movement direction based on current rotation
        let forward_dir = transform.forward();
        let right_dir = transform.right();
        
        // Apply forward/backward movement
        if forward_move != 0.0 {
            let move_velocity = forward_dir * forward_move * move_speed;
            velocity.linear.x = move_velocity.x;
            velocity.linear.z = move_velocity.z;
        } else {
            velocity.linear.x = 0.0;
            velocity.linear.z = 0.0;
        }
        
        // Apply strafe movement (add to existing velocity)
        if strafe_move != 0.0 {
            let strafe_velocity = right_dir * strafe_move * move_speed;
            velocity.linear.x += strafe_velocity.x;
            velocity.linear.z += strafe_velocity.z;
        }
        
        // Apply gravity if not grounded
        if !is_grounded {
            velocity.linear.y -= 12.0 * dt; // gravity = 12.0
        }
    }
}

/// System that applies velocity to transform (physics integration)
pub struct PhysicsSystem;

impl System for PhysicsSystem {
    fn run(&mut self, world: &mut World) {
        let dt = 1.0 / 60.0; // Default delta time
        
        // Collect entities with both Transform and Velocity components
        let mut physics_entities = Vec::new();
        
        if let Some(velocity_storage) = world.storage::<Velocity>() {
            for (entity, _) in velocity_storage.iter() {
                // Check if this entity also has a Transform component
                if world.get::<Transform>(entity).is_some() {
                    physics_entities.push(entity);
                }
            }
        }
        
        // Apply physics to each entity
        for entity in physics_entities {
            // Get velocity data first (immutable borrow)
            let velocity_data = if let Some(velocity) = world.get::<Velocity>(entity) {
                (velocity.linear, velocity.angular)
            } else {
                continue;
            };
            
            // Now get mutable transform reference
            if let Some(transform) = world.get_mut::<Transform>(entity) {
                let (linear_velocity, angular_velocity) = velocity_data;
                
                // Apply velocity to position
                transform.position.x += linear_velocity.x * dt;
                transform.position.y += linear_velocity.y * dt;
                transform.position.z += linear_velocity.z * dt;
                
                // Apply angular velocity to rotation
                transform.rotation.x += angular_velocity.x * dt;
                transform.rotation.y += angular_velocity.y * dt;
                transform.rotation.z += angular_velocity.z * dt;
            }
        }
    }
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self
    }
}

/// System that handles collision detection and response
pub struct CollisionSystem;

impl System for CollisionSystem {
    fn run(&mut self, _world: &mut World) {
        // TODO: Implement collision detection
        // This would check for collisions between entities with BoundingBox components
        // and update their positions/velocities accordingly
    }
}

impl CollisionSystem {
    pub fn new() -> Self {
        Self
    }
} 