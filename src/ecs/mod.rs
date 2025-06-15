//! Entity Component System (ECS) Architecture
//! 
//! A high-performance, cache-friendly ECS designed for the Wolfenstein game.
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

// Re-export core types
pub use world::World;
pub use entity::{Entity, EntityId, EntityManager};
pub use component::{Component, ComponentStorage, ComponentManager};
pub use system::{System, SystemManager};
pub use query::Query;
pub use resource::{Resource, ResourceManager};

// Re-export common components
pub use components::*;

/// Type alias for component type identification
pub type ComponentTypeId = std::any::TypeId; 