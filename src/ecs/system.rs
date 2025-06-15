//! System management and execution
//! 
//! This module will contain the system trait and system manager.
//! Systems operate on components and implement game logic.

use crate::ecs::World;

/// Trait that all systems must implement
pub trait System: Send + Sync {
    /// Run the system with access to the world
    fn run(&mut self, world: &mut World);
    
    /// Get the system name for debugging
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

/// Manages and executes systems
#[derive(Default)]
pub struct SystemManager {
    systems: Vec<Box<dyn System>>,
}

impl SystemManager {
    /// Create a new system manager
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }
    
    /// Add a system
    pub fn add_system<S: System + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }
    
    /// Run all systems
    pub fn run_all(&mut self, world: &mut World) {
        for system in &mut self.systems {
            system.run(world);
        }
    }
    
    /// Get number of systems
    pub fn len(&self) -> usize {
        self.systems.len()
    }
    
    /// Check if no systems are registered
    pub fn is_empty(&self) -> bool {
        self.systems.is_empty()
    }
} 