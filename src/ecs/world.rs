//! World - the main ECS container

use crate::ecs::{Entity, EntityManager, ComponentManager, Component};

/// Read-only view of the world for component updates
pub struct WorldView<'a> {
    entities: &'a EntityManager,
    components: &'a ComponentManager,
}

impl<'a> WorldView<'a> {
    pub fn new(entities: &'a EntityManager, components: &'a ComponentManager) -> Self {
        Self { entities, components }
    }

    /// Get a component for an entity (read-only)
    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T> {
        if self.entities.is_valid(entity) {
            self.components.get(entity)
        } else {
            None
        }
    }

    /// Check if an entity is valid
    pub fn is_valid(&self, entity: Entity) -> bool {
        self.entities.is_valid(entity)
    }

    /// Check if an entity has a component
    pub fn has<T: Component>(&self, entity: Entity) -> bool {
        if self.entities.is_valid(entity) {
            self.components.has::<T>(entity)
        } else {
            false
        }
    }

    /// Get all entities
    pub fn all_entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entities.all_entities().into_iter()
    }
}

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
    
    /// Simple query for entities with 3 components (immutable)
    pub fn query_3<T1: Component, T2: Component, T3: Component>(&self) -> Vec<(Entity, &T1, &T2, &T3)> {
        let mut results = Vec::new();
        
        // Get all entities that have all three component types
        for entity in self.entities.all_entities() {
            if let (Some(comp1), Some(comp2), Some(comp3)) = (
                self.get::<T1>(entity),
                self.get::<T2>(entity), 
                self.get::<T3>(entity)
            ) {
                results.push((entity, comp1, comp2, comp3));
            }
        }
        
        results
    }
    
    /// Simple query for entities with 2 components (immutable)
    pub fn query_2<T1: Component, T2: Component>(&self) -> Vec<(Entity, &T1, &T2)> {
        let mut results = Vec::new();
        
        // Get all entities that have both component types
        for entity in self.entities.all_entities() {
            if let (Some(comp1), Some(comp2)) = (
                self.get::<T1>(entity),
                self.get::<T2>(entity)
            ) {
                results.push((entity, comp1, comp2));
            }
        }
        
        results
    }
    
    /// Simple query for entities with 1 component (immutable)
    pub fn query_1<T: Component>(&self) -> Vec<(Entity, &T)> {
        let mut results = Vec::new();
        
        // Get all entities that have the component type
        for entity in self.entities.all_entities() {
            if let Some(comp) = self.get::<T>(entity) {
                results.push((entity, comp));
            }
        }
        
        results
    }
    
    /// Clear all entities and components
    pub fn clear(&mut self) {
        self.components.clear();
        self.entities = EntityManager::new();
    }
    
    /// Check if a component should be processed by systems
    /// Returns true only if the entity is valid AND the component exists AND the component is enabled
    pub fn should_process_component<T: crate::ecs::Component>(&self, entity: Entity) -> bool {
        if !self.is_valid(entity) {
            return false;
        }
        
        // Check if component exists and is enabled
        if let Some(component) = self.get::<T>(entity) {
            component.is_enabled()
        } else {
            false // Component doesn't exist
        }
    }
    
    /// Get multiple mutable component references for the same entity
    /// This is safe because we're getting different component types for the same entity
    pub fn get_mut_pair<T1: Component + 'static, T2: Component + 'static>(&mut self, entity: Entity) -> (Option<&mut T1>, Option<&mut T2>) {
        // Get the first component
        let comp1 = self.components.get_mut::<T1>(entity);
        let comp2 = self.components.get_mut::<T2>(entity);
        
        // This is a simplified version - in a real ECS we'd need more sophisticated handling
        // For now, we'll just return None for both to avoid borrowing issues
        // TODO: Implement proper multi-component access
        (None, None)
    }

    /// Update all registered components using the inventory system
    pub fn update_all_components(&mut self, delta_time: f32) {
        // Use inventory to discover and update all registered components
        for registration in inventory::iter::<crate::ecs::component::ComponentRegistration> {
            (registration.updater)(self, delta_time);
        }
    }
    
    /// Update all components of a specific type that implement AutoUpdatable
    pub fn update_component_type<T: crate::ecs::component::AutoUpdatable>(&mut self, delta_time: f32) {
        // Collect entities with this component type to avoid borrowing issues
        let entities_with_component: Vec<Entity> = {
            if let Some(storage) = self.storage::<T>() {
                storage.entities().to_vec()
            } else {
                return; // No storage for this component type
            }
        };
        
        // Update each component
        for entity in entities_with_component {
            if !self.is_valid(entity) {
                continue;
            }

            // Check if component should be updated
            let should_update = {
                if let Some(component) = self.get::<T>(entity) {
                    entity.enabled && component.is_enabled()
                } else {
                    false
                }
            };

            if should_update {
                // Update the component (self-contained only)
                if let Some(component) = self.get_mut::<T>(entity) {
                    component.auto_update(entity, delta_time);
                }
            }
        }
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