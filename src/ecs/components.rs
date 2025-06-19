//! Game-specific components for the GameByAI ECS

use macroquad::prelude::*;
use crate::ecs::{Component, World, Entity};

/// Position, rotation, and scale in 3D space
#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3, // Euler angles in radians
    pub scale: Vec3,
    pub enabled: bool,
}

impl Transform {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
            enabled: true,
        }
    }

    pub fn with_rotation(mut self, rotation: Vec3) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
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

    /// Get the forward direction vector
    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            self.rotation.y.cos(),
            0.0,
            self.rotation.y.sin(),
        )
    }

    /// Get the right direction vector
    pub fn right(&self) -> Vec3 {
        Vec3::new(
            (self.rotation.y + std::f32::consts::PI / 2.0).cos(),
            0.0,
            (self.rotation.y + std::f32::consts::PI / 2.0).sin(),
        )
    }

    /// Get the up direction vector
    pub fn up(&self) -> Vec3 {
        Vec3::Y
    }

    /// Apply a translation
    pub fn translate(&mut self, delta: Vec3) {
        self.position += delta;
    }

    /// Get transformation matrix
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            self.scale,
            Quat::from_euler(EulerRot::XYZ, self.rotation.x, self.rotation.y, self.rotation.z),
            self.position,
        )
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new(Vec3::ZERO)
    }
}

impl Component for Transform {
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

/// Linear and angular velocity
#[derive(Debug, Clone)]
pub struct Velocity {
    pub linear: Vec3,
    pub angular: Vec3,
    pub enabled: bool,
}

impl Velocity {
    pub fn new() -> Self {
        Self {
            linear: Vec3::ZERO,
            angular: Vec3::ZERO,
            enabled: true,
        }
    }

    pub fn linear(velocity: Vec3) -> Self {
        Self {
            linear: velocity,
            angular: Vec3::ZERO,
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
}

impl Default for Velocity {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Velocity {
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

/// Unified renderer for all static geometry (walls, floors, ceilings, props)
pub struct StaticRenderer {
    pub mesh: Option<Mesh>,
    pub texture: Option<Texture2D>,
    pub material_type: MaterialType,
    pub color: Color,
    pub visible: bool,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub enum MaterialType {
    Wall { texture_name: String },
    Floor { texture_name: String },
    Ceiling { texture_name: String },
    Prop { texture_name: String },
}

impl StaticRenderer {
    pub fn new(material_type: MaterialType) -> Self {
        Self {
            mesh: None,
            texture: None,
            material_type,
            color: WHITE,
            visible: true,
            enabled: true,
        }
    }

    pub fn wall(texture_name: String) -> Self {
        Self::new(MaterialType::Wall { texture_name })
    }

    pub fn floor(texture_name: String) -> Self {
        Self::new(MaterialType::Floor { texture_name })
    }

    pub fn ceiling(texture_name: String) -> Self {
        Self::new(MaterialType::Ceiling { texture_name })
    }

    pub fn prop(texture_name: String) -> Self {
        Self::new(MaterialType::Prop { texture_name })
    }

    pub fn with_texture(mut self, texture: Texture2D) -> Self {
        self.texture = Some(texture);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_mesh(mut self, mesh: Mesh) -> Self {
        self.mesh = Some(mesh);
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

    /// Check if this renderer should actually render (both enabled and visible)
    pub fn should_render(&self) -> bool {
        self.enabled && self.visible
    }

    /// Enable/disable rendering (affects visible flag)
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Component for StaticRenderer {
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

// Custom Debug implementation to handle Mesh
impl std::fmt::Debug for StaticRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StaticRenderer")
            .field("mesh", &self.mesh.as_ref().map(|_| "Mesh"))
            .field("texture", &self.texture.as_ref().map(|_| "Texture2D"))
            .field("material_type", &self.material_type)
            .field("color", &self.color)
            .field("visible", &self.visible)
            .field("enabled", &self.enabled)
            .finish()
    }
}

// Custom Clone implementation to handle Mesh
impl Clone for StaticRenderer {
    fn clone(&self) -> Self {
        Self {
            mesh: None, // Can't clone Mesh, so set to None
            texture: self.texture.clone(),
            material_type: self.material_type.clone(),
            color: self.color,
            visible: self.visible,
            enabled: self.enabled,
        }
    }
}

/// Collision detection and physics properties
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

/// Player-specific component
#[derive(Debug, Clone)]
pub struct Player {
    pub health: f32,
    pub max_health: f32,
    pub is_grounded: bool,
    pub move_speed: f32,
    pub jump_strength: f32,
    pub enabled: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            is_grounded: true,
            move_speed: 5.0,
            jump_strength: 4.5,
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
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Player {
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

// Tag components for easy querying
/// Marker component for wall entities
#[derive(Debug, Clone)]
pub struct Wall {
    pub enabled: bool,
}

impl Wall {
    pub fn new() -> Self {
        Self { enabled: true }
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
}

impl Default for Wall {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Wall {
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

/// Marker component for floor entities  
#[derive(Debug, Clone)]
pub struct Floor {
    pub enabled: bool,
}

impl Floor {
    pub fn new() -> Self {
        Self { enabled: true }
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
}

impl Default for Floor {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Floor {
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

/// Marker component for ceiling entities
#[derive(Debug, Clone)]
pub struct Ceiling {
    pub enabled: bool,
}

impl Ceiling {
    pub fn new() -> Self {
        Self { enabled: true }
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
}

impl Default for Ceiling {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Ceiling {
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

/// Marker component for prop entities
#[derive(Debug, Clone)]
pub struct Prop {
    pub enabled: bool,
}

impl Prop {
    pub fn new() -> Self {
        Self { enabled: true }
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
}

impl Default for Prop {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Prop {
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
        let waypoints = vec![
            Vec2::new(2.5, 1.5), // First target - East corridor
            Vec2::new(3.5, 1.5), // Continue east
            Vec2::new(4.5, 1.5), // Far east
            Vec2::new(5.5, 1.5), // Eastern corridor
            Vec2::new(6.5, 1.5), // Near east wall
            Vec2::new(7.5, 1.5), // East end
            Vec2::new(8.5, 1.5), // Turn point
            Vec2::new(8.5, 6.5), // Go south to safe row 6
            Vec2::new(6.5, 6.5), // Move west in safe row 6
            Vec2::new(3.5, 6.5), // Continue west in safe row 6
            Vec2::new(1.5, 6.5), // West end in safe row 6
            Vec2::new(1.5, 3.5), // Move north to middle
            Vec2::new(1.5, 1.5), // Return to start
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

/// Wall mesh component that contains all walls as a single mesh with proper UV mapping
pub struct WallMesh {
    pub mesh: Option<Mesh>,
    pub enabled: bool,
}

impl WallMesh {
    pub fn new() -> Self {
        Self {
            mesh: None,
            enabled: true,
        }
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_mesh(mut self, mesh: Mesh) -> Self {
        self.mesh = Some(mesh);
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
}

impl Default for WallMesh {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for WallMesh {
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

/// Single mesh component that holds all floor geometry for efficient rendering
pub struct FloorMesh {
    pub mesh: Option<Mesh>,
    pub enabled: bool,
}

impl FloorMesh {
    /// Create a new floor mesh component
    pub fn new(mesh: Mesh) -> Self {
        Self {
            mesh: Some(mesh),
            enabled: true,
        }
    }

    /// Create an empty floor mesh component
    pub fn empty() -> Self {
        Self {
            mesh: None,
            enabled: true,
        }
    }

    /// Check if the floor mesh is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable the floor mesh
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the floor mesh
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Set enabled state
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl Component for FloorMesh {} 