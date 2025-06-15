//! Resource management for global game state
//! 
//! Resources are global singletons that can be accessed by systems.
//! Examples: DeltaTime, Input, AssetManager, etc.

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Trait for resources (global singletons)
pub trait Resource: 'static + Send + Sync {}

/// Manager for global resources
#[derive(Default)]
pub struct ResourceManager {
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ResourceManager {
    /// Create a new resource manager
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }
    
    /// Insert a resource
    pub fn insert<R: Resource>(&mut self, resource: R) {
        let type_id = TypeId::of::<R>();
        self.resources.insert(type_id, Box::new(resource));
    }
    
    /// Get a resource
    pub fn get<R: Resource>(&self) -> Option<&R> {
        let type_id = TypeId::of::<R>();
        self.resources.get(&type_id)?.downcast_ref::<R>()
    }
    
    /// Get a mutable resource
    pub fn get_mut<R: Resource>(&mut self) -> Option<&mut R> {
        let type_id = TypeId::of::<R>();
        self.resources.get_mut(&type_id)?.downcast_mut::<R>()
    }
    
    /// Remove a resource
    pub fn remove<R: Resource>(&mut self) -> Option<R> {
        let type_id = TypeId::of::<R>();
        let boxed = self.resources.remove(&type_id)?;
        boxed.downcast::<R>().ok().map(|b| *b)
    }
    
    /// Check if a resource exists
    pub fn has<R: Resource>(&self) -> bool {
        let type_id = TypeId::of::<R>();
        self.resources.contains_key(&type_id)
    }
} 