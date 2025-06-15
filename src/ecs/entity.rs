//! Entity management with safe handles and generation counters

use std::collections::VecDeque;

/// Unique identifier for entities
pub type EntityId = u32;

/// Entity handle with generation for safe access
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    pub id: EntityId,
    pub generation: u32,
}

impl Entity {
    /// Create a new entity with given id and generation
    pub fn new(id: EntityId, generation: u32) -> Self {
        Self { id, generation }
    }
    
    /// Check if this entity handle is valid
    pub fn is_valid(&self) -> bool {
        self.id != EntityId::MAX && self.generation > 0
    }
}

/// Manages entity creation, destruction, and validation
#[derive(Debug)]
pub struct EntityManager {
    /// Current generation for each entity slot
    generations: Vec<u32>,
    /// Free entity IDs that can be reused
    free_entities: VecDeque<EntityId>,
    /// Next entity ID to allocate if no free ones available
    next_id: EntityId,
}

impl EntityManager {
    /// Create a new entity manager
    pub fn new() -> Self {
        Self {
            generations: Vec::new(),
            free_entities: VecDeque::new(),
            next_id: 0,
        }
    }
    
    /// Create a new entity
    pub fn create(&mut self) -> Entity {
        if let Some(id) = self.free_entities.pop_front() {
            // Reuse a free entity ID
            let generation = self.generations[id as usize];
            Entity::new(id, generation)
        } else {
            // Allocate a new entity ID
            let id = self.next_id;
            self.next_id += 1;
            
            // Ensure we have space in generations array
            if id as usize >= self.generations.len() {
                self.generations.resize(id as usize + 1, 0);
            }
            
            self.generations[id as usize] = 1; // Start at generation 1
            Entity::new(id, 1)
        }
    }
    
    /// Destroy an entity (marks it for reuse)
    pub fn destroy(&mut self, entity: Entity) -> bool {
        if !self.is_valid(entity) {
            return false;
        }
        
        // Increment generation to invalidate existing handles
        self.generations[entity.id as usize] += 1;
        
        // Add to free list for reuse
        self.free_entities.push_back(entity.id);
        
        true
    }
    
    /// Check if an entity handle is still valid
    pub fn is_valid(&self, entity: Entity) -> bool {
        if entity.id as usize >= self.generations.len() {
            return false;
        }
        
        self.generations[entity.id as usize] == entity.generation
    }
    
    /// Get the current generation for an entity ID
    pub fn generation(&self, id: EntityId) -> Option<u32> {
        self.generations.get(id as usize).copied()
    }
    
    /// Get total number of entities ever created
    pub fn total_created(&self) -> u32 {
        self.next_id
    }
    
    /// Get number of currently active entities
    pub fn active_count(&self) -> usize {
        self.next_id as usize - self.free_entities.len()
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_entity_creation() {
        let mut manager = EntityManager::new();
        
        let entity1 = manager.create();
        let entity2 = manager.create();
        
        assert_ne!(entity1.id, entity2.id);
        assert!(manager.is_valid(entity1));
        assert!(manager.is_valid(entity2));
    }
    
    #[test]
    fn test_entity_destruction() {
        let mut manager = EntityManager::new();
        
        let entity = manager.create();
        assert!(manager.is_valid(entity));
        
        assert!(manager.destroy(entity));
        assert!(!manager.is_valid(entity));
    }
    
    #[test]
    fn test_entity_reuse() {
        let mut manager = EntityManager::new();
        
        let entity1 = manager.create();
        let id1 = entity1.id;
        
        manager.destroy(entity1);
        
        let entity2 = manager.create();
        
        // Should reuse the same ID but with different generation
        assert_eq!(entity2.id, id1);
        assert_ne!(entity2.generation, entity1.generation);
        assert!(!manager.is_valid(entity1)); // Old handle invalid
        assert!(manager.is_valid(entity2));  // New handle valid
    }
} 