//! Query system for efficient component access
//! 
//! This module will contain the query system for type-safe,
//! efficient iteration over entities with specific components.

use crate::ecs::{World, Component};

/// Query trait for accessing components
pub trait Query<'w> {
    type Item;
    
    /// Execute the query and return an iterator
    fn query(world: &'w World) -> impl Iterator<Item = Self::Item>;
}

// TODO: Implement query macros and types for:
// - Query<(&Transform, &Velocity)>
// - Query<(&mut Transform, &Velocity)>
// - Query<(Entity, &Transform, &mut Velocity)>
// - With/Without filters 