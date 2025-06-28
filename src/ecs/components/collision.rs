//! Collision detection components

use macroquad::prelude::*;
use crate::ecs::{Component, World};
use super::{Transform, StaticRenderer};

#[derive(Debug, Clone)]
pub struct Collider {
    pub shape: ColliderShape,
    pub is_static: bool,        // Static vs dynamic objects
    pub is_trigger: bool,       // Trigger vs solid collision
    pub material: ColliderMaterial, // Physics material properties
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub enum ColliderShape {
    Box { size: Vec3 },
    Sphere { radius: f32 },
    Capsule { height: f32, radius: f32 },
}

#[derive(Debug, Clone)]
pub struct ColliderMaterial {
    pub friction: f32,
    pub restitution: f32,      // Bounciness (0.0 = no bounce, 1.0 = perfect bounce)
    pub density: f32,          // For dynamic objects
}

impl Collider {
    /// Create a new collider with specified properties
    pub fn new(shape: ColliderShape, is_static: bool, is_trigger: bool) -> Self {
        Self {
            shape,
            is_static,
            is_trigger,
            material: ColliderMaterial::default(),
            enabled: true,
        }
    }

    /// Create a static solid collider (for walls, obstacles)
    pub fn static_solid(shape: ColliderShape) -> Self {
        Self::new(shape, true, false)
    }

    /// Create a static trigger collider (for sensors, pickups)
    pub fn static_trigger(shape: ColliderShape) -> Self {
        Self::new(shape, true, true)
    }

    /// Create a dynamic solid collider (for physics objects)
    pub fn dynamic_solid(shape: ColliderShape) -> Self {
        Self::new(shape, false, false)
    }

    /// Create a dynamic trigger collider (for moving sensors)
    pub fn dynamic_trigger(shape: ColliderShape) -> Self {
        Self::new(shape, false, true)
    }

    /// Set the material properties
    pub fn with_material(mut self, material: ColliderMaterial) -> Self {
        self.material = material;
        self
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

    /// Check if this collider blocks movement (solid and not trigger and enabled)
    pub fn blocks_movement(&self) -> bool {
        self.enabled && !self.is_trigger
    }

    /// Check for collision at a specific position, considering the shape and size of colliders
    pub fn check_position_collision(world: &World, player_position: Vec3, player_radius: f32) -> bool {
        let player_shape = ColliderShape::Capsule { height: 1.8, radius: player_radius };
        let player_transform = Transform::new(player_position);

        for (entity, transform, collider, _) in world.query_3::<Transform, Collider, StaticRenderer>() {
            // Skip disabled entities or disabled colliders
            if !world.is_valid(entity) || !entity.enabled || !collider.is_enabled() || !transform.is_enabled() {
                continue;
            }
            
            if !collider.blocks_movement() {
                continue;
            }

            // Use proper shape-based collision detection
            if player_shape.overlaps_with(&player_transform, &collider.shape, transform) {
                return true;
            }
        }

        false
    }

    /// Legacy grid-based collision check for backward compatibility
    pub fn check_grid_collision(world: &World, x: f32, z: f32) -> bool {
        Self::check_position_collision(world, Vec3::new(x, 0.6, z), 0.25)
    }
}

impl Component for Collider {
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

impl ColliderShape {
    /// Check if a point is inside this collider shape at the given transform
    pub fn contains_point(&self, point: Vec3, transform: &Transform) -> bool {
        match self {
            ColliderShape::Box { size } => {
                let half_size = *size * 0.5;
                let local_point = point - transform.position;
                
                local_point.x.abs() <= half_size.x &&
                local_point.y.abs() <= half_size.y &&
                local_point.z.abs() <= half_size.z
            },
            ColliderShape::Sphere { radius } => {
                let distance = (point - transform.position).length();
                distance <= *radius
            },
            ColliderShape::Capsule { height, radius } => {
                let local_point = point - transform.position;
                let half_height = height * 0.5;
                
                // Check if point is within the cylindrical part
                let horizontal_distance = (local_point.x * local_point.x + local_point.z * local_point.z).sqrt();
                
                if local_point.y.abs() <= half_height {
                    // Within cylinder height, check radius
                    horizontal_distance <= *radius
                } else {
                    // Check distance to hemisphere caps
                    let cap_center_y = if local_point.y > 0.0 { half_height } else { -half_height };
                    let cap_center = Vec3::new(0.0, cap_center_y, 0.0);
                    let distance_to_cap = (local_point - cap_center).length();
                    distance_to_cap <= *radius
                }
            }
        }
    }

    /// Check if this collider overlaps with another collider
    pub fn overlaps_with(&self, self_transform: &Transform, other: &ColliderShape, other_transform: &Transform) -> bool {
        match (self, other) {
            (ColliderShape::Box { size: size1 }, ColliderShape::Box { size: size2 }) => {
                let half_size1 = *size1 * 0.5;
                let half_size2 = *size2 * 0.5;
                
                let pos1 = self_transform.position;
                let pos2 = other_transform.position;
                
                (pos1.x - half_size1.x <= pos2.x + half_size2.x) &&
                (pos1.x + half_size1.x >= pos2.x - half_size2.x) &&
                (pos1.y - half_size1.y <= pos2.y + half_size2.y) &&
                (pos1.y + half_size1.y >= pos2.y - half_size2.y) &&
                (pos1.z - half_size1.z <= pos2.z + half_size2.z) &&
                (pos1.z + half_size1.z >= pos2.z - half_size2.z)
            },
            // Capsule vs Box collision (most common case for player vs walls)
            (ColliderShape::Capsule { height, radius }, ColliderShape::Box { size }) |
            (ColliderShape::Box { size }, ColliderShape::Capsule { height, radius }) => {
                let (capsule_pos, box_pos, capsule_height, capsule_radius, box_size) = 
                    if matches!(self, ColliderShape::Capsule { .. }) {
                        (self_transform.position, other_transform.position, *height, *radius, *size)
                    } else {
                        (other_transform.position, self_transform.position, *height, *radius, *size)
                    };

                let box_half_size = box_size * 0.5;
                
                // Find the closest point on the box to the capsule center
                let closest_x = (capsule_pos.x).clamp(box_pos.x - box_half_size.x, box_pos.x + box_half_size.x);
                let closest_y = (capsule_pos.y).clamp(box_pos.y - box_half_size.y, box_pos.y + box_half_size.y);
                let closest_z = (capsule_pos.z).clamp(box_pos.z - box_half_size.z, box_pos.z + box_half_size.z);
                
                let closest_point = Vec3::new(closest_x, closest_y, closest_z);
                
                // Check if the closest point is within the capsule's radius
                let half_height = capsule_height * 0.5;
                let capsule_center = capsule_pos;
                
                // Distance from capsule center to closest point on box
                let diff = closest_point - capsule_center;
                
                // For capsule collision, we need to check against the cylindrical body and hemispheres
                if diff.y.abs() <= half_height {
                    // Point is within the cylindrical part of the capsule
                    let horizontal_distance = (diff.x * diff.x + diff.z * diff.z).sqrt();
                    horizontal_distance <= capsule_radius
                } else {
                    // Point is above or below the cylinder, check against hemisphere caps
                    let cap_center_y = if diff.y > 0.0 { half_height } else { -half_height };
                    let cap_center = Vec3::new(0.0, cap_center_y, 0.0);
                    let distance_to_cap = (diff - cap_center).length();
                    distance_to_cap <= capsule_radius
                }
            },
            // For other shape combinations, fall back to simple approximation
            _ => {
                // Simple approximation: test if center of one shape is inside the other
                self.contains_point(other_transform.position, self_transform) ||
                other.contains_point(self_transform.position, other_transform)
            }
        }
    }

    /// Get the bounding box of this collider shape
    pub fn get_bounds(&self, transform: &Transform) -> (Vec3, Vec3) {
        match self {
            ColliderShape::Box { size } => {
                let half_size = *size * 0.5;
                let min = transform.position - half_size;
                let max = transform.position + half_size;
                (min, max)
            },
            ColliderShape::Sphere { radius } => {
                let r = Vec3::new(*radius, *radius, *radius);
                let min = transform.position - r;
                let max = transform.position + r;
                (min, max)
            },
            ColliderShape::Capsule { height, radius } => {
                let half_height = height * 0.5;
                let r = Vec3::new(*radius, half_height + radius, *radius);
                let min = transform.position - r;
                let max = transform.position + r;
                (min, max)
            }
        }
    }
}

impl ColliderMaterial {
    pub fn new(friction: f32, restitution: f32, density: f32) -> Self {
        Self {
            friction,
            restitution,
            density,
        }
    }

    /// Standard material for walls/floors
    pub fn standard() -> Self {
        Self::new(0.5, 0.0, 1.0)
    }

    /// Slippery material (ice, metal)
    pub fn slippery() -> Self {
        Self::new(0.1, 0.0, 1.0)
    }

    /// Bouncy material (rubber, springs)
    pub fn bouncy() -> Self {
        Self::new(0.7, 0.8, 1.0)
    }
}

impl Default for ColliderMaterial {
    fn default() -> Self {
        Self::standard()
    }
} 