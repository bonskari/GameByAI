//! A* Pathfinding algorithms for ECS entities

use macroquad::prelude::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use crate::game::map::Map;

/// A* pathfinding service that can be shared across systems
#[derive(Debug)]
pub struct PathfindingAlgorithms {
    pub map: Map,
}

/// Node used in A* pathfinding
#[derive(Debug, Clone, PartialEq)]
struct AStarNode {
    position: (i32, i32),
    g_cost: f32,  // Distance from start
    h_cost: f32,  // Heuristic distance to goal
    f_cost: f32,  // Total cost (g + h)
    parent: Option<(i32, i32)>,
}

impl Eq for AStarNode {}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap (BinaryHeap is max-heap by default)
        other.f_cost.partial_cmp(&self.f_cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Result of A* pathfinding including explored nodes for visualization
#[derive(Debug, Clone)]
pub struct PathfindingResult {
    pub path: Vec<Vec2>,
    pub explored_nodes: Vec<(i32, i32)>,
    pub found: bool,
}

impl PathfindingAlgorithms {
    pub fn new(map: Map) -> Self {
        Self { map }
    }

    /// Find path using A* algorithm
    pub fn find_path(&self, start: Vec2, goal: Vec2) -> PathfindingResult {
        let start_grid = (start.x.floor() as i32, start.y.floor() as i32);
        let goal_grid = (goal.x.floor() as i32, goal.y.floor() as i32);

        // Check if start or goal are in walls
        if self.map.is_wall(start_grid.0, start_grid.1) || self.map.is_wall(goal_grid.0, goal_grid.1) {
            return PathfindingResult {
                path: Vec::new(),
                explored_nodes: Vec::new(),
                found: false,
            };
        }

        let mut open_set = BinaryHeap::new();
        let mut closed_set = HashSet::new();
        let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut g_score: HashMap<(i32, i32), f32> = HashMap::new();
        let mut explored_nodes = Vec::new();

        // Initialize start node
        let start_node = AStarNode {
            position: start_grid,
            g_cost: 0.0,
            h_cost: self.heuristic(start_grid, goal_grid),
            f_cost: self.heuristic(start_grid, goal_grid),
            parent: None,
        };

        open_set.push(start_node);
        g_score.insert(start_grid, 0.0);

        while let Some(current) = open_set.pop() {
            let current_pos = current.position;

            // Add to explored nodes for visualization
            explored_nodes.push(current_pos);

            // Check if we reached the goal
            if current_pos == goal_grid {
                let path = self.reconstruct_path(&came_from, current_pos, start, goal);
                return PathfindingResult {
                    path,
                    explored_nodes,
                    found: true,
                };
            }

            closed_set.insert(current_pos);

            // Check all neighbors (8-directional movement)
            for neighbor_pos in self.get_neighbors(current_pos) {
                if closed_set.contains(&neighbor_pos) {
                    continue;
                }

                // Skip if neighbor is a wall
                if self.map.is_wall(neighbor_pos.0, neighbor_pos.1) {
                    continue;
                }

                // Calculate movement cost (diagonal movement costs more)
                let movement_cost = if (neighbor_pos.0 - current_pos.0).abs() + (neighbor_pos.1 - current_pos.1).abs() == 2 {
                    1.414 // Diagonal movement (sqrt(2))
                } else {
                    1.0   // Straight movement
                };

                let tentative_g_score = current.g_cost + movement_cost;

                // Check if this path to neighbor is better
                if let Some(&existing_g) = g_score.get(&neighbor_pos) {
                    if tentative_g_score >= existing_g {
                        continue;
                    }
                }

                // This path is the best so far
                came_from.insert(neighbor_pos, current_pos);
                g_score.insert(neighbor_pos, tentative_g_score);

                let neighbor_node = AStarNode {
                    position: neighbor_pos,
                    g_cost: tentative_g_score,
                    h_cost: self.heuristic(neighbor_pos, goal_grid),
                    f_cost: tentative_g_score + self.heuristic(neighbor_pos, goal_grid),
                    parent: Some(current_pos),
                };

                open_set.push(neighbor_node);
            }
        }

        // No path found
        PathfindingResult {
            path: Vec::new(),
            explored_nodes,
            found: false,
        }
    }

    /// Get all valid neighbors for a position (8-directional)
    fn get_neighbors(&self, pos: (i32, i32)) -> Vec<(i32, i32)> {
        let mut neighbors = Vec::new();
        let (x, y) = pos;

        // 8-directional movement
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue; // Skip current position
                }

                let new_x = x + dx;
                let new_y = y + dy;

                // Check bounds
                if new_x >= 0 && new_y >= 0 && 
                   new_x < self.map.width as i32 && new_y < self.map.height as i32 {
                    neighbors.push((new_x, new_y));
                }
            }
        }

        neighbors
    }

    /// Heuristic function (Manhattan distance with diagonal adjustment)
    fn heuristic(&self, a: (i32, i32), b: (i32, i32)) -> f32 {
        let dx = (a.0 - b.0).abs() as f32;
        let dy = (a.1 - b.1).abs() as f32;
        
        // Octile distance (better for 8-directional movement)
        let min = dx.min(dy);
        let max = dx.max(dy);
        min * 1.414 + (max - min)
    }

    /// Reconstruct the path from the came_from map
    fn reconstruct_path(&self, came_from: &HashMap<(i32, i32), (i32, i32)>, 
                       mut current: (i32, i32), start: Vec2, goal: Vec2) -> Vec<Vec2> {
        let mut path = Vec::new();
        
        // Add the goal position first
        path.push(goal);

        // Trace back through the path
        while let Some(&parent) = came_from.get(&current) {
            // Convert grid position to world position (center of grid cell)
            let world_pos = Vec2::new(current.0 as f32 + 0.5, current.1 as f32 + 0.5);
            path.push(world_pos);
            current = parent;
        }

        // Add start position
        path.push(start);

        // Reverse to get path from start to goal
        path.reverse();
        
        // Remove the first element (start position) since we're already there
        if !path.is_empty() {
            path.remove(0);
        }

        path
    }

    /// Update the internal map reference
    pub fn update_map(&mut self, map: Map) {
        self.map = map;
    }
} 