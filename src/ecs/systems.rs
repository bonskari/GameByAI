//! Game-specific systems for the 3D game

use macroquad::prelude::*;
use crate::ecs::{System, World, Transform, Pathfinder};
use crate::ecs::pathfinding::{PathfindingAlgorithms, PathfindingResult};

/// Delta time resource for frame timing
#[derive(Debug, Clone)]
pub struct DeltaTime(pub f32);

/// Map resource for collision detection
#[derive(Debug, Clone)]
pub struct MapResource {
    pub map: crate::game::map::Map,
}

/// Player movement system - handles input and movement
pub struct PlayerMovementSystem;

impl PlayerMovementSystem {
    pub fn new() -> Self {
        Self
    }
}

impl System for PlayerMovementSystem {
    fn run(&mut self, world: &mut World) {
        // This system is currently disabled in favor of direct ECS state updates
        // to avoid complex borrowing issues. The movement logic is implemented
        // directly in EcsGameState::update_with_input()
    }

    fn name(&self) -> &'static str {
        "PlayerMovementSystem"
    }
}

/// Physics system - handles gravity, velocity, and physics updates
pub struct PhysicsSystem;

impl PhysicsSystem {
    pub fn new() -> Self {
        Self
    }
}

impl System for PhysicsSystem {
    fn run(&mut self, world: &mut World) {
        // This system is currently disabled in favor of direct ECS state updates
        // to avoid complex borrowing issues. The physics logic is implemented
        // directly in EcsGameState::update_with_input()
    }

    fn name(&self) -> &'static str {
        "PhysicsSystem"
    }
}

/// Collision system - handles collision detection and response
pub struct CollisionSystem;

impl CollisionSystem {
    pub fn new() -> Self {
        Self
    }
}

impl System for CollisionSystem {
    fn run(&mut self, world: &mut World) {
        // This system is currently disabled in favor of direct ECS state updates
        // to avoid complex borrowing issues. The collision logic is implemented
        // directly in EcsGameState::update_with_input()
    }

    fn name(&self) -> &'static str {
        "CollisionSystem"
    }
}

/// Test bot system - handles test bot behavior
pub struct TestBotSystem;

impl TestBotSystem {
    pub fn new() -> Self {
        Self
    }
}

impl System for TestBotSystem {
    fn run(&mut self, world: &mut World) {
        // This system is currently disabled in favor of direct ECS state updates
        // to avoid complex borrowing issues. The test bot logic is implemented
        // directly in EcsGameState::update_all_test_bots()
    }

    fn name(&self) -> &'static str {
        "TestBotSystem"
    }
}

/// Pathfinding system - handles A* pathfinding for all entities with Pathfinder components
pub struct PathfindingSystem {
    pub algorithms: PathfindingAlgorithms,
}

impl PathfindingSystem {
    pub fn new(map: crate::game::map::Map) -> Self {
        Self {
            algorithms: PathfindingAlgorithms::new(map),
        }
    }

    /// Update the map used for pathfinding
    pub fn update_map(&mut self, map: crate::game::map::Map) {
        self.algorithms.update_map(map);
    }

    /// Process pathfinding for a specific entity (called from ECS state)
    pub fn process_entity_pathfinding(&mut self, world: &mut World, entity: crate::ecs::Entity, delta_time: f32) {
        // Get current position
        let current_position = {
            if let Some(transform) = world.get::<Transform>(entity) {
                Vec2::new(transform.position.x, transform.position.z)
            } else {
                return; // No transform, can't pathfind
            }
        };

        // Check if pathfinder needs recalculation or path following
        let needs_recalc = {
            if let Some(pathfinder) = world.get::<Pathfinder>(entity) {
                pathfinder.needs_recalculation
            } else {
                return; // No pathfinder component
            }
        };

        // Recalculate path if needed
        if needs_recalc {
            if let Some(pathfinder) = world.get_mut::<Pathfinder>(entity) {
                if let Some(target) = pathfinder.target {
                    let result = self.algorithms.find_path(current_position, target);
                    
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
        self.follow_path(world, entity, current_position, delta_time);
    }

    /// Follow the current calculated path
    fn follow_path(&self, world: &mut World, entity: crate::ecs::Entity, current_position: Vec2, delta_time: f32) {
        let (next_target, movement_speed, rotation_speed, arrival_threshold) = {
            if let Some(pathfinder) = world.get::<Pathfinder>(entity) {
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
                if let Some(pathfinder) = world.get_mut::<Pathfinder>(entity) {
                    pathfinder.advance_path_step();
                    
                    // Check if we reached the final target
                    if pathfinder.has_reached_target(current_position) {
                        println!("🎯 Pathfinding: Reached final target at ({:.2}, {:.2})", target.x, target.y);
                        pathfinder.clear_path();
                    }
                }
            } else {
                // Move toward target
                let target_angle = direction.y.atan2(direction.x);
                
                // Update rotation and position
                if let Some(transform) = world.get_mut::<Transform>(entity) {
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
                    
                    // Move forward if facing the target (within 15 degrees)
                    let facing_threshold = 15.0_f32.to_radians();
                    if angle_diff.abs() < facing_threshold {
                        let move_distance = movement_speed * delta_time;
                        let move_x = new_rotation.cos() * move_distance;
                        let move_z = new_rotation.sin() * move_distance;
                        
                        // Check collision before moving
                        let new_x = transform.position.x + move_x;
                        let new_z = transform.position.z + move_z;
                        
                        // Simple collision check (can be improved)
                        if !self.algorithms.map.is_wall(new_x.floor() as i32, new_z.floor() as i32) {
                            transform.position.x = new_x;
                            transform.position.z = new_z;
                        }
                    }
                }
                
                // Update stuck detection
                if let Some(pathfinder) = world.get_mut::<Pathfinder>(entity) {
                    let pos_diff = (current_position - pathfinder.last_position).length();
                    
                    if pos_diff < 0.001 {
                        pathfinder.stuck_time += delta_time;
                        if pathfinder.stuck_time > 1.0 { // Stuck for 1 second
                            println!("⚠️ Pathfinding: Entity stuck, recalculating path");
                            pathfinder.needs_recalculation = true;
                            pathfinder.stuck_time = 0.0;
                        }
                    } else {
                        pathfinder.stuck_time = 0.0;
                    }
                    
                    pathfinder.last_position = current_position;
                }
            }
        }
    }
}

impl System for PathfindingSystem {
    fn run(&mut self, world: &mut World) {
        // This system is currently disabled in favor of direct processing
        // from EcsGameState to avoid borrowing issues. The pathfinding logic
        // is called directly via process_entity_pathfinding()
    }

    fn name(&self) -> &'static str {
        "PathfindingSystem"
    }
} 