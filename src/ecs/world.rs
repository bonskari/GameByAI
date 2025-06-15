//! World - the main ECS container

use crate::ecs::{Entity, EntityManager, ComponentManager, Component};

/// The main ECS world that contains all entities, components, and systems
pub struct World {
    /// Entity management
    entities: EntityManager,
    /// Component storage
    components: ComponentManager,
}

impl World {
    /// Create a new world
    pub fn new() -> Self {
        Self {
            entities: EntityManager::new(),
            components: ComponentManager::new(),
        }
    }
    
    /// Create a new entity
    pub fn spawn(&mut self) -> EntityBuilder {
        let entity = self.entities.create();
        EntityBuilder::new(self, entity)
    }
    
    /// Destroy an entity and all its components
    pub fn despawn(&mut self, entity: Entity) -> bool {
        if self.entities.destroy(entity) {
            self.components.remove_all(entity);
            true
        } else {
            false
        }
    }
    
    /// Check if an entity is valid
    pub fn is_valid(&self, entity: Entity) -> bool {
        self.entities.is_valid(entity)
    }
    
    /// Add a component to an entity
    pub fn add<T: Component>(&mut self, entity: Entity, component: T) -> bool {
        if self.entities.is_valid(entity) {
            self.components.add(entity, component)
        } else {
            false
        }
    }
    
    /// Remove a component from an entity
    pub fn remove<T: Component>(&mut self, entity: Entity) -> bool {
        self.components.remove::<T>(entity)
    }
    
    /// Get a component for an entity
    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T> {
        if self.entities.is_valid(entity) {
            self.components.get(entity)
        } else {
            None
        }
    }
    
    /// Get a mutable component for an entity
    pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        if self.entities.is_valid(entity) {
            self.components.get_mut(entity)
        } else {
            None
        }
    }
    
    /// Check if an entity has a component
    pub fn has<T: Component>(&self, entity: Entity) -> bool {
        if self.entities.is_valid(entity) {
            self.components.has::<T>(entity)
        } else {
            false
        }
    }
    
    /// Get component storage for iteration
    pub fn storage<T: Component>(&self) -> Option<&crate::ecs::component::TypedComponentStorage<T>> {
        self.components.storage::<T>()
    }
    
    /// Get mutable component storage for iteration
    pub fn storage_mut<T: Component>(&mut self) -> Option<&mut crate::ecs::component::TypedComponentStorage<T>> {
        self.components.storage_mut::<T>()
    }
    
    /// Get entity manager
    pub fn entities(&self) -> &EntityManager {
        &self.entities
    }
    
    /// Get component manager
    pub fn components(&self) -> &ComponentManager {
        &self.components
    }
    
    /// Get mutable component manager
    pub fn components_mut(&mut self) -> &mut ComponentManager {
        &mut self.components
    }
    
    /// Clear all entities and components
    pub fn clear(&mut self) {
        self.components.clear();
        self.entities = EntityManager::new();
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating entities with components
pub struct EntityBuilder<'w> {
    world: &'w mut World,
    entity: Entity,
}

impl<'w> EntityBuilder<'w> {
    /// Create a new entity builder
    fn new(world: &'w mut World, entity: Entity) -> Self {
        Self { world, entity }
    }
    
    /// Add a component to the entity being built
    pub fn with<T: Component>(self, component: T) -> Self {
        self.world.add(self.entity, component);
        self
    }
    
    /// Finish building and return the entity
    pub fn build(self) -> Entity {
        self.entity
    }
    
    /// Get the entity being built
    pub fn entity(&self) -> Entity {
        self.entity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[derive(Debug, PartialEq)]
    struct Position {
        x: f32,
        y: f32,
    }
    
    impl Component for Position {}
    
    #[derive(Debug, PartialEq)]
    struct Velocity {
        dx: f32,
        dy: f32,
    }
    
    impl Component for Velocity {}
    
    #[test]
    fn test_entity_creation() {
        let mut world = World::new();
        
        let entity = world.spawn().build();
        assert!(world.is_valid(entity));
    }
    
    #[test]
    fn test_component_operations() {
        let mut world = World::new();
        
        let entity = world.spawn()
            .with(Position { x: 1.0, y: 2.0 })
            .with(Velocity { dx: 0.5, dy: -0.5 })
            .build();
        
        // Test component retrieval
        assert_eq!(world.get::<Position>(entity), Some(&Position { x: 1.0, y: 2.0 }));
        assert_eq!(world.get::<Velocity>(entity), Some(&Velocity { dx: 0.5, dy: -0.5 }));
        
        // Test component mutation
        if let Some(pos) = world.get_mut::<Position>(entity) {
            pos.x = 10.0;
        }
        assert_eq!(world.get::<Position>(entity), Some(&Position { x: 10.0, y: 2.0 }));
        
        // Test component removal
        assert!(world.remove::<Velocity>(entity));
        assert!(!world.has::<Velocity>(entity));
        assert!(world.has::<Position>(entity));
    }
    
    #[test]
    fn test_entity_destruction() {
        let mut world = World::new();
        
        let entity = world.spawn()
            .with(Position { x: 1.0, y: 2.0 })
            .build();
        
        assert!(world.is_valid(entity));
        assert!(world.has::<Position>(entity));
        
        assert!(world.despawn(entity));
        assert!(!world.is_valid(entity));
        assert!(!world.has::<Position>(entity));
    }
} 