//! Entity Component System (ECS) for GameByAI
//!
//! A high-performance, cache-friendly ECS designed for the 3D game.
//! Features:
//! - Type-safe component storage
//! - Efficient batch processing
//! - Integrated rendering system
//! - Easy parallelization

pub mod world;
pub mod entity;
pub mod component;
pub mod system;
pub mod query;
pub mod resource;
pub mod components;
pub mod systems;

// Re-export core types
pub use world::World;
pub use entity::{Entity, EntityId, EntityManager};
pub use component::{Component, ComponentStorage, ComponentManager};
pub use system::{System, SystemManager};
pub use query::Query;
pub use resource::{Resource, ResourceManager};

// Re-export common components
pub use components::*;

// Re-export game systems
pub use systems::*;

/// Type alias for component type identification
pub type ComponentTypeId = std::any::TypeId; 