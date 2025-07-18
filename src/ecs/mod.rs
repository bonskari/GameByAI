//! Entity Component System (ECS) Implementation
//!
//! This module provides a simple but effective ECS architecture for the game.
//! The ECS separates data (Components) from behavior (Systems) and provides
//! efficient querying and iteration over entities with specific component combinations.
//!
//! Key features:
//! - Type-safe component storage
//! - Efficient queries for entities with specific component combinations
//! - Easy parallelization

pub mod component;
pub mod components;
pub mod entity;
pub mod pathfinding;
pub mod query;
pub mod resource;
pub mod world;

/// Type alias for component type identification
pub type ComponentTypeId = std::any::TypeId;

// Re-export core ECS types
pub use entity::{Entity, EntityManager};
pub use component::{Component, ComponentStorage, ComponentManager};
pub use world::World;

// Re-export all components from the new module structure
pub use components::*;

// Re-export game-specific components
pub use components::{
    Transform, Velocity, StaticRenderer, MaterialType, Collider, ColliderShape, Player, Wall, Floor, Ceiling, Prop, TestBot, Pathfinder,
    StaticMesh, StaticMeshType, LightSource, LightSourceType, LightReceiver, LightingTest,
    Renderable, RenderData, RenderType,
};

// Re-export commonly used types
 