//! Component storage and management

use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::ecs::{Entity, ComponentTypeId};
use crate::ecs::entity::EntityId;

/// Trait that all components must implement
pub trait Component: 'static + Send + Sync {
    /// Get the type ID for this component
    fn type_id() -> ComponentTypeId where Self: Sized {
        TypeId::of::<Self>()
    }
}

/// Type-erased component storage
pub trait ComponentStorage: Any + Send + Sync {
    /// Insert a component for an entity
    fn insert(&mut self, entity: Entity, component: Box<dyn Any + Send + Sync>) -> bool;
    
    /// Remove a component for an entity
    fn remove(&mut self, entity: Entity) -> bool;
    
    /// Check if an entity has this component
    fn has(&self, entity: Entity) -> bool;
    
    /// Get the component type ID
    fn type_id(&self) -> ComponentTypeId;
    
    /// Clear all components
    fn clear(&mut self);
    
    /// Get number of components stored
    fn len(&self) -> usize;
    
    /// Check if storage is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Downcast to concrete type (immutable)
    fn as_any(&self) -> &dyn Any;
    
    /// Downcast to concrete type (mutable)
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Concrete storage for a specific component type
#[derive(Debug)]
pub struct TypedComponentStorage<T: Component> {
    /// Sparse array mapping entity ID to dense array index
    sparse: Vec<Option<usize>>,
    /// Dense array of components
    dense: Vec<T>,
    /// Dense array of entity handles
    entities: Vec<Entity>,
    /// Map from entity ID to dense index for faster lookup
    entity_to_dense: HashMap<EntityId, usize>,
}

impl<T: Component> TypedComponentStorage<T> {
    /// Create a new typed component storage
    pub fn new() -> Self {
        Self {
            sparse: Vec::new(),
            dense: Vec::new(),
            entities: Vec::new(),
            entity_to_dense: HashMap::new(),
        }
    }
    
    /// Insert a component for an entity
    pub fn insert(&mut self, entity: Entity, component: T) -> bool {
        // Ensure sparse array is large enough
        if entity.id as usize >= self.sparse.len() {
            self.sparse.resize(entity.id as usize + 1, None);
        }
        
        // Check if entity already has this component
        if let Some(dense_index) = self.sparse[entity.id as usize] {
            // Update existing component
            self.dense[dense_index] = component;
            self.entities[dense_index] = entity; // Update generation
            return false; // Didn't add new component
        }
        
        // Add new component
        let dense_index = self.dense.len();
        self.dense.push(component);
        self.entities.push(entity);
        self.sparse[entity.id as usize] = Some(dense_index);
        self.entity_to_dense.insert(entity.id, dense_index);
        
        true // Added new component
    }
    
    /// Remove a component for an entity
    pub fn remove(&mut self, entity: Entity) -> bool {
        if entity.id as usize >= self.sparse.len() {
            return false;
        }
        
        if let Some(dense_index) = self.sparse[entity.id as usize] {
            // Verify generation matches
            if self.entities[dense_index] != entity {
                return false;
            }
            
            // Swap-remove from dense arrays
            let last_index = self.dense.len() - 1;
            if dense_index != last_index {
                // Move last element to removed position
                self.dense.swap(dense_index, last_index);
                self.entities.swap(dense_index, last_index);
                
                // Update sparse array for moved entity
                let moved_entity = self.entities[dense_index];
                self.sparse[moved_entity.id as usize] = Some(dense_index);
                self.entity_to_dense.insert(moved_entity.id, dense_index);
            }
            
            // Remove last element
            self.dense.pop();
            self.entities.pop();
            self.sparse[entity.id as usize] = None;
            self.entity_to_dense.remove(&entity.id);
            
            true
        } else {
            false
        }
    }
    
    /// Get a component for an entity
    pub fn get(&self, entity: Entity) -> Option<&T> {
        if entity.id as usize >= self.sparse.len() {
            return None;
        }
        
        if let Some(dense_index) = self.sparse[entity.id as usize] {
            // Verify generation matches
            if self.entities[dense_index] == entity {
                Some(&self.dense[dense_index])
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Get a mutable component for an entity
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        if entity.id as usize >= self.sparse.len() {
            return None;
        }
        
        if let Some(dense_index) = self.sparse[entity.id as usize] {
            // Verify generation matches
            if self.entities[dense_index] == entity {
                Some(&mut self.dense[dense_index])
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Check if an entity has this component
    pub fn has(&self, entity: Entity) -> bool {
        self.get(entity).is_some()
    }
    
    /// Iterate over all components and their entities
    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.entities.iter().zip(self.dense.iter()).map(|(&entity, component)| (entity, component))
    }
    
    /// Iterate over all components and their entities (mutable)
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        self.entities.iter().zip(self.dense.iter_mut()).map(|(&entity, component)| (entity, component))
    }
    
    /// Get all entities that have this component
    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }
    
    /// Get all components (dense array)
    pub fn components(&self) -> &[T] {
        &self.dense
    }
    
    /// Get all components (dense array, mutable)
    pub fn components_mut(&mut self) -> &mut [T] {
        &mut self.dense
    }
}

impl<T: Component> ComponentStorage for TypedComponentStorage<T> {
    fn insert(&mut self, entity: Entity, component: Box<dyn Any + Send + Sync>) -> bool {
        if let Ok(component) = component.downcast::<T>() {
            self.insert(entity, *component)
        } else {
            false
        }
    }
    
    fn remove(&mut self, entity: Entity) -> bool {
        self.remove(entity)
    }
    
    fn has(&self, entity: Entity) -> bool {
        self.has(entity)
    }
    
    fn type_id(&self) -> ComponentTypeId {
        TypeId::of::<T>()
    }
    
    fn clear(&mut self) {
        self.sparse.clear();
        self.dense.clear();
        self.entities.clear();
        self.entity_to_dense.clear();
    }
    
    fn len(&self) -> usize {
        self.dense.len()
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<T: Component> Default for TypedComponentStorage<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Manager for all component storages
#[derive(Default)]
pub struct ComponentManager {
    /// Map from component type ID to storage
    storages: HashMap<ComponentTypeId, Box<dyn ComponentStorage>>,
}

impl ComponentManager {
    /// Create a new component manager
    pub fn new() -> Self {
        Self {
            storages: HashMap::new(),
        }
    }
    
    /// Register a component type
    pub fn register<T: Component>(&mut self) {
        let type_id = TypeId::of::<T>();
        if !self.storages.contains_key(&type_id) {
            self.storages.insert(type_id, Box::new(TypedComponentStorage::<T>::new()));
        }
    }
    
    /// Add a component to an entity
    pub fn add<T: Component>(&mut self, entity: Entity, component: T) -> bool {
        self.register::<T>();
        let type_id = TypeId::of::<T>();
        
        if let Some(storage) = self.storages.get_mut(&type_id) {
            storage.insert(entity, Box::new(component))
        } else {
            false
        }
    }
    
    /// Remove a component from an entity
    pub fn remove<T: Component>(&mut self, entity: Entity) -> bool {
        let type_id = TypeId::of::<T>();
        
        if let Some(storage) = self.storages.get_mut(&type_id) {
            storage.remove(entity)
        } else {
            false
        }
    }
    
    /// Get a component for an entity
    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        
        if let Some(storage) = self.storages.get(&type_id) {
            // Downcast to typed storage
            if let Some(typed_storage) = storage.as_any().downcast_ref::<TypedComponentStorage<T>>() {
                typed_storage.get(entity)
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Get a mutable component for an entity
    pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        
        if let Some(storage) = self.storages.get_mut(&type_id) {
            // Downcast to typed storage
            if let Some(typed_storage) = storage.as_any_mut().downcast_mut::<TypedComponentStorage<T>>() {
                typed_storage.get_mut(entity)
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Check if an entity has a component
    pub fn has<T: Component>(&self, entity: Entity) -> bool {
        let type_id = TypeId::of::<T>();
        
        if let Some(storage) = self.storages.get(&type_id) {
            storage.has(entity)
        } else {
            false
        }
    }
    
    /// Get typed storage for a component type
    pub fn storage<T: Component>(&self) -> Option<&TypedComponentStorage<T>> {
        let type_id = TypeId::of::<T>();
        
        if let Some(storage) = self.storages.get(&type_id) {
            storage.as_any().downcast_ref::<TypedComponentStorage<T>>()
        } else {
            None
        }
    }
    
    /// Get mutable typed storage for a component type
    pub fn storage_mut<T: Component>(&mut self) -> Option<&mut TypedComponentStorage<T>> {
        self.register::<T>();
        let type_id = TypeId::of::<T>();
        
        if let Some(storage) = self.storages.get_mut(&type_id) {
            storage.as_any_mut().downcast_mut::<TypedComponentStorage<T>>()
        } else {
            None
        }
    }
    
    /// Remove all components for an entity
    pub fn remove_all(&mut self, entity: Entity) {
        for storage in self.storages.values_mut() {
            storage.remove(entity);
        }
    }
    
    /// Clear all components
    pub fn clear(&mut self) {
        for storage in self.storages.values_mut() {
            storage.clear();
        }
    }
} 