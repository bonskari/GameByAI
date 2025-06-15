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

// Re-export core types
pub use entity::{Entity, EntityManager};
pub use component::{Component, ComponentManager};
pub use world::World;
pub use system::{System, SystemManager};
pub use resource::{Resource, ResourceManager};

// Re-export game components
pub use components::{
    Transform, Velocity, StaticRenderer, MaterialType, Collider, ColliderShape,
    Player, Wall, Floor, Ceiling, Prop
};

// Re-export systems
pub use systems::{DeltaTime, MapResource};

/// Type alias for component type identification
pub type ComponentTypeId = std::any::TypeId; 