//! Game-specific systems for the 3D game

use macroquad::prelude::*;
use crate::ecs::{System, World, Transform, Pathfinder, LightSource, LightReceiver, Wall, StaticRenderer, Floor, Ceiling};
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
        // Check if entity is valid
        if !world.is_valid(entity) {
            return; // Skip invalid entities
        }

        // Get current position
        let current_position = {
            if let Some(transform) = world.get::<Transform>(entity) {
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
            if let Some(pathfinder) = world.get::<Pathfinder>(entity) {
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
                if let Some(pathfinder) = world.get::<Pathfinder>(entity) {
                    pathfinder.target
                } else {
                    return;
                }
            };
            
            if let Some(target) = target {
                // Use ECS-aware pathfinding that respects disabled entities
                let result = self.algorithms.find_path_with_ecs(current_position, target, world);
                
                // Update the pathfinder with results
                if let Some(pathfinder) = world.get_mut::<Pathfinder>(entity) {
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
                        if !crate::ecs::Collider::check_grid_collision(world, new_x, new_z) {
                            // Re-acquire transform to update position
                            if let Some(transform) = world.get_mut::<Transform>(entity) {
                                transform.position.x = new_x;
                                transform.position.z = new_z;
                            }
                        }
                    }
                }
                
                // Update stuck detection with more aggressive unsticking
                let (pos_diff, stuck_time, needs_unstick) = {
                    if let Some(pathfinder) = world.get::<Pathfinder>(entity) {
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
                        if let Some(transform) = world.get::<Transform>(entity) {
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
                    if !crate::ecs::Collider::check_grid_collision(world, unstick_x, unstick_z) {
                        if let Some(transform) = world.get_mut::<Transform>(entity) {
                            transform.position.x = unstick_x;
                            transform.position.z = unstick_z;
                            println!("🔄 Unstick movement applied");
                        }
                    }
                    
                    // Update pathfinder state
                    if let Some(pathfinder) = world.get_mut::<Pathfinder>(entity) {
                        pathfinder.needs_recalculation = true;
                        pathfinder.stuck_time = 0.0;
                        pathfinder.last_position = current_position;
                    }
                } else {
                    // Update pathfinder state normally
                    if let Some(pathfinder) = world.get_mut::<Pathfinder>(entity) {
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

/// Lighting system that calculates lighting for all entities with LightReceiver components
pub fn lighting_system(world: &mut World, time: f32) {
    let mut light_sources = Vec::new();
    
    // Collect all active light sources with their positions
    for (entity, transform, light_source) in world.query_2::<Transform, LightSource>() {
        if world.is_valid(entity) && entity.enabled && light_source.is_enabled() && transform.is_enabled() {
            light_sources.push((transform.position, light_source.clone()));
        }
    }
    
    // Collect light receiver entities and calculate lighting for each
    let mut lighting_updates = Vec::new();
    
    for (entity, transform, light_receiver) in world.query_2::<Transform, LightReceiver>() {
        if !world.is_valid(entity) || !entity.enabled || !light_receiver.is_enabled() || !transform.is_enabled() {
            continue;
        }
        
        // Calculate lighting at this position
        let mut final_color = light_receiver.ambient_color;
        
        for (light_pos, light_source) in &light_sources {
            let distance = (*light_pos - transform.position).length();
            
            if distance > light_source.radius {
                continue;
            }
            
            // Calculate attenuation (quadratic falloff)
            let attenuation = (1.0f32 - (distance / light_source.radius)).max(0.0);
            let attenuation = attenuation * attenuation;
            
            // Get animated intensity
            let animated_intensity = light_source.get_animated_intensity(time);
            
            // Apply light contribution
            let contribution = attenuation * animated_intensity;
            final_color.r = (final_color.r + light_source.color.r * contribution).min(1.0);
            final_color.g = (final_color.g + light_source.color.g * contribution).min(1.0);
            final_color.b = (final_color.b + light_source.color.b * contribution).min(1.0);
        }
        
        lighting_updates.push((entity, final_color));
    }
    
    // Debug output every few seconds
    static mut LAST_DEBUG_TIME: f32 = 0.0;
    unsafe {
        if (time - LAST_DEBUG_TIME) > 3.0 { // Debug every 3 seconds
            println!("💡 Lighting System: {} light sources, {} light receivers", light_sources.len(), lighting_updates.len());
            LAST_DEBUG_TIME = time;
        }
    }
    
    // Apply lighting updates after the query
    for (entity, final_color) in lighting_updates {
        if let Some(mut receiver) = world.get_mut::<LightReceiver>(entity) {
            receiver.update_lighting(final_color);
            
            // Also apply lighting to the StaticRenderer color if entity has one
            if let Some(mut renderer) = world.get_mut::<StaticRenderer>(entity) {
                renderer.color = final_color;
            }
        }
    }
}

/// System that creates light sources for wall entities based on their wall type
/// This runs only once to set up lights, then stops
pub fn wall_lighting_setup_system(world: &mut World) {
    // Check if we already have light sources
    let existing_lights = {
        let mut count = 0;
        for (_, _) in world.query_1::<LightSource>() {
            count += 1;
            if count > 0 { break; } // Exit early if we have any lights
        }
        count
    };
    
    // Skip if we already have lights
    if existing_lights > 0 {
        return;
    }
    
    let mut walls_needing_lights = Vec::new();
    
    // Find all walls that don't have light sources yet
    for (entity, transform, wall) in world.query_2::<Transform, Wall>() {
        if world.is_valid(entity) && entity.enabled && wall.is_enabled() && transform.is_enabled() {
            walls_needing_lights.push((entity, transform.position, wall.wall_type));
        }
    }
    
    // Create new light source entities near walls
    let lights_created = walls_needing_lights.len();
    for (_wall_entity, position, wall_type) in walls_needing_lights {
        let light_source = match wall_type {
            crate::game::map::WallType::TechPanel => {
                LightSource::warning(3.0, 5.0)  // Much brighter and larger range
            },
            crate::game::map::WallType::EnergyConduit => {
                LightSource::energy(3.0, 5.0)  // Much brighter and larger range
            },
            crate::game::map::WallType::ControlSystem => {
                LightSource::control(3.0, 5.0)  // Much brighter and larger range
            },
            crate::game::map::WallType::HullPlating => {
                LightSource::ambient(1.0, 3.0)  // Much brighter and larger range
            },
            _ => continue, // Skip empty spaces
        };
        
        // Create a new light entity near the wall
        let light_position = position + Vec3::new(0.0, 1.0, 0.0); // Slightly above wall
        let _light_entity = world.spawn()
            .with(Transform::new(light_position))
            .with(light_source)
            .build();
    }
    
    // Add LightReceiver to individual Wall entities (since we're back to individual walls)
    let wall_entities_for_lighting: Vec<_> = world.query_2::<Transform, StaticRenderer>()
        .into_iter()
        .filter_map(|(entity, _transform, static_renderer)| {
            // Only add to entities that are walls (have Wall component) and don't already have LightReceiver
            if world.is_valid(entity) && entity.enabled && world.get::<Wall>(entity).is_some() && world.get::<LightReceiver>(entity).is_none() {
                Some(entity)
            } else {
                None
            }
        })
        .collect();
    
    for entity in wall_entities_for_lighting {
        world.add(entity, LightReceiver::new());
    }
    
    // Also add LightReceiver to floor and ceiling entities
    let floor_entities: Vec<_> = world.query_2::<Transform, Floor>()
        .into_iter()
        .filter_map(|(entity, _transform, _floor)| {
            if world.is_valid(entity) && entity.enabled {
                Some(entity)
            } else {
                None
            }
        })
        .collect();
    
    for entity in floor_entities {
        world.add(entity, LightReceiver::new());
    }
    
    let ceiling_entities: Vec<_> = world.query_2::<Transform, Ceiling>()
        .into_iter()
        .filter_map(|(entity, _transform, _ceiling)| {
            if world.is_valid(entity) && entity.enabled {
                Some(entity)
            } else {
                None
            }
        })
        .collect();
    
    for entity in ceiling_entities {
        world.add(entity, LightReceiver::new());
    }
    
    if lights_created > 0 {
        println!("💡 Wall Lighting Setup: Created {} light sources and added light receivers", lights_created);
    }
} 