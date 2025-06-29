//! Pathfinding and test bot components

use macroquad::prelude::*;
use crate::ecs::{Component, component::{AutoUpdatable, ComponentRegistration}};

// Auto-register TestBot component
inventory::submit! {
    ComponentRegistration {
        type_name: "TestBot",
        updater: |world, delta_time| {
            world.update_component_type::<TestBot>(delta_time);
        },
    }
}

// Auto-register Pathfinder component  
inventory::submit! {
    ComponentRegistration {
        type_name: "Pathfinder",
        updater: |world, delta_time| {
            world.update_component_type::<Pathfinder>(delta_time);
        },
    }
}

/// Test waypoint component for pathfinding demonstrations
#[derive(Debug, Clone)]
pub struct TestWaypoint {
    pub position: Vec2,
    pub radius: f32,
    pub enabled: bool,
}

impl TestWaypoint {
    pub fn new(position: Vec2, radius: f32) -> Self {
        Self {
            position,
            radius,
            enabled: true,
        }
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

impl Component for TestWaypoint {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn enable(&mut self) {
        self.enabled = true;
    }

    fn disable(&mut self) {
        self.enabled = false;
    }
}

/// Pathfinder component - can be used by any entity that needs pathfinding
#[derive(Debug, Clone)]
pub struct Pathfinder {
    pub target: Option<Vec2>,           // Current target position
    pub current_path: Vec<Vec2>,        // Calculated A* path
    pub path_index: usize,              // Current step in path
    pub movement_speed: f32,            // How fast to move
    pub rotation_speed: f32,            // How fast to rotate
    pub stuck_time: f32,                // Time spent stuck
    pub last_position: Vec2,            // Previous position for stuck detection
    pub needs_recalculation: bool,      // Whether path needs to be recalculated
    pub explored_nodes: Vec<(i32, i32)>, // A* explored nodes (for visualization)
    pub arrival_threshold: f32,         // How close to get to target
    pub enabled: bool,
}

/// Test bot component for automated testing
#[derive(Debug, Clone)]
pub struct TestBot {
    pub start_time: std::time::Instant,
    pub test_duration: std::time::Duration,
    pub current_waypoint: usize,
    pub waypoints: Vec<Vec2>, // Using Vec2 directly now
    pub enabled: bool,
}

impl TestBot {
    pub fn new(test_duration_seconds: u64) -> Self {
        // Updated waypoints for the simple rectangular room (10x10 area from 0,0 to 10,10)
        // Player starts at (5,5), so we'll navigate around the room
        let waypoints = vec![
            Vec2::new(2.0, 2.0), // Northwest area
            Vec2::new(8.0, 2.0), // Northeast area  
            Vec2::new(8.0, 8.0), // Southeast area
            Vec2::new(2.0, 8.0), // Southwest area
            Vec2::new(5.0, 5.0), // Back to center
            Vec2::new(3.0, 5.0), // West center
            Vec2::new(7.0, 5.0), // East center
            Vec2::new(5.0, 3.0), // North center
            Vec2::new(5.0, 7.0), // South center
        ];

        Self {
            start_time: std::time::Instant::now(),
            test_duration: std::time::Duration::from_secs(test_duration_seconds),
            current_waypoint: 0,
            waypoints,
            enabled: true,
        }
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Enable this component
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable this component
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if this component is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_progress(&self) -> (usize, usize, f32) {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let total = self.test_duration.as_secs_f32();
        (self.current_waypoint, self.waypoints.len(), elapsed / total)
    }

    pub fn is_finished(&self) -> bool {
        self.start_time.elapsed() >= self.test_duration
    }

    /// Get the current target waypoint
    pub fn get_current_target(&self) -> Option<Vec2> {
        if self.current_waypoint < self.waypoints.len() {
            Some(self.waypoints[self.current_waypoint])
        } else {
            None
        }
    }

    /// Move to the next waypoint
    pub fn advance_waypoint(&mut self) {
        self.current_waypoint = (self.current_waypoint + 1) % self.waypoints.len();
        let waypoint = self.waypoints[self.current_waypoint];
        println!("✓ TestBot advancing to waypoint {} at ({:.2}, {:.2})",
                 self.current_waypoint,
                 waypoint.x,
                 waypoint.y);
    }
}

impl Component for TestBot {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn enable(&mut self) {
        self.enabled = true;
    }

    fn disable(&mut self) {
        self.enabled = false;
    }
}

impl AutoUpdatable for TestBot {
    fn auto_update(&mut self, _entity: crate::ecs::Entity, _delta_time: f32) {
        // Check if test bot is finished
        if self.is_finished() {
            println!("🤖 Visual test completed after {:.1}s", self.start_time.elapsed().as_secs_f32());
            return;
        }

        // Update internal state only - we can't access other components from here
        // The actual waypoint advancement logic will need to be handled by EcsGameState
        // since it requires cross-component communication
        
        // This is a self-contained update that only modifies TestBot's internal state
        // For example, we could update timers, internal counters, etc.
        println!("🤖 TestBot self-update: waypoint {}/{}", 
                 self.current_waypoint, 
                 self.waypoints.len());
    }
}

impl Pathfinder {
    pub fn new(movement_speed: f32, rotation_speed: f32) -> Self {
        Self {
            target: None,
            current_path: Vec::new(),
            path_index: 0,
            movement_speed,
            rotation_speed,
            stuck_time: 0.0,
            last_position: Vec2::ZERO,
            needs_recalculation: false,
            explored_nodes: Vec::new(),
            arrival_threshold: 0.4,  // Increased for better corner navigation
            enabled: true,
        }
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Enable this component
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable this component
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if this component is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Set a new target and mark for path recalculation
    pub fn set_target(&mut self, target: Vec2) {
        if !self.enabled {
            return; // Don't set targets when disabled
        }

        self.target = Some(target);
        self.needs_recalculation = true;
        self.path_index = 0;
        self.stuck_time = 0.0;
    }

    /// Check if we've reached the current target
    pub fn has_reached_target(&self, current_position: Vec2) -> bool {
        if !self.enabled {
            return false;
        }

        if let Some(target) = self.target {
            current_position.distance(target) < self.arrival_threshold
        } else {
            false
        }
    }

    /// Get the next position to move toward
    pub fn get_next_position(&self) -> Option<Vec2> {
        if !self.enabled {
            return None;
        }

        if self.path_index < self.current_path.len() {
            Some(self.current_path[self.path_index])
        } else {
            self.target
        }
    }

    /// Advance to the next step in the path
    pub fn advance_path_step(&mut self) {
        if !self.enabled {
            return;
        }

        if self.path_index < self.current_path.len() {
            self.path_index += 1;
        }
    }

    /// Clear the current path and target
    pub fn clear_path(&mut self) {
        self.current_path.clear();
        self.path_index = 0;
        self.target = None;
        self.needs_recalculation = false;
        self.explored_nodes.clear();
    }
}

impl Component for Pathfinder {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn enable(&mut self) {
        self.enabled = true;
    }

    fn disable(&mut self) {
        self.enabled = false;
    }
}

impl AutoUpdatable for Pathfinder {
    fn auto_update(&mut self, _entity: crate::ecs::Entity, delta_time: f32) {
        // Update internal timers and state
        // We can't access Transform from here, so stuck detection will remain in EcsGameState
        
        // Update path following state
        if !self.current_path.is_empty() {
            // Update internal pathfinding timers
            println!("🗺️ Pathfinder self-update: {} path points remaining", self.current_path.len());
        }
        
        // Update stuck detection timer (but we can't get current position from here)
        // This will be handled by EcsGameState which has access to both Transform and Pathfinder
        self.stuck_time += delta_time;
        
        // Note: Actual pathfinding movement still needs to be handled by EcsGameState
        // because it requires access to PathfindingAlgorithms and needs to modify Transform components
    }
} 