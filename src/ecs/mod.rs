//! Entity Component System (ECS) for GameByAI
//!
//! A high-performance, cache-friendly ECS designed for the 3D game.
//! Features:
//! - Type-safe component storage
//! - Efficient batch processing
//! - Integrated rendering system
//! - Easy parallelization

pub mod entity;
pub mod component;
pub mod world;
pub mod system;
pub mod query;
pub mod resource;
pub mod components;
pub mod systems;
pub mod pathfinding;

/// Type alias for component type identification
pub type ComponentTypeId = std::any::TypeId;

// Re-export all ECS types for convenience
pub use entity::{Entity, EntityManager};
pub use component::{Component, ComponentStorage, ComponentManager, TypedComponentStorage};
pub use world::{World, EntityBuilder};
pub use system::{System, SystemManager};
pub use query::Query;
pub use resource::{Resource, ResourceManager};
pub use pathfinding::{PathfindingAlgorithms, PathfindingResult};

// Re-export game-specific components
pub use components::{
    Transform, Velocity, StaticRenderer, MaterialType, Collider, ColliderShape, 
    ColliderMaterial, Player, Wall, Floor, Ceiling, Prop, TestBot, TestWaypoint, Pathfinder,
    WallMesh
};

// Re-export systems
pub use systems::{DeltaTime, MapResource}; 