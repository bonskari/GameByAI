//! Game-specific systems for the 3D game

use macroquad::prelude::*;
use crate::ecs::{System, World};

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