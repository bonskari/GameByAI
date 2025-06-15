//! Game-specific components for the GameByAI ECS

use macroquad::prelude::*;
use crate::ecs::{Component, World};

/// Position, rotation, and scale in 3D space
#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3, // Euler angles in radians
    pub scale: Vec3,
}

impl Transform {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
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

impl Component for Transform {}

/// Linear and angular velocity
#[derive(Debug, Clone)]
pub struct Velocity {
    pub linear: Vec3,
    pub angular: Vec3,
}

impl Velocity {
    pub fn new() -> Self {
        Self {
            linear: Vec3::ZERO,
            angular: Vec3::ZERO,
        }
    }

    pub fn linear(velocity: Vec3) -> Self {
        Self {
            linear: velocity,
            angular: Vec3::ZERO,
        }
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Velocity {}

/// Unified renderer for all static geometry (walls, floors, ceilings, props)
pub struct StaticRenderer {
    pub mesh: Option<Mesh>,
    pub texture: Option<Texture2D>,
    pub material_type: MaterialType,
    pub color: Color,
    pub visible: bool,
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
        }
    }
}

impl Component for StaticRenderer {}

/// Collision detection and physics properties
#[derive(Debug, Clone)]
pub struct Collider {
    pub shape: ColliderShape,
    pub is_static: bool,        // Static vs dynamic objects
    pub is_trigger: bool,       // Trigger vs solid collision
    pub material: ColliderMaterial, // Physics material properties
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

    /// Check if this collider blocks movement (solid and not trigger)
    pub fn blocks_movement(&self) -> bool {
        !self.is_trigger
    }

    /// Check if this collider would collide at a given position
    pub fn would_collide_at(&self, position: Vec3, world: &World) -> bool {
        if self.is_trigger {
            return false; // Triggers don't block movement
        }

        // Create a temporary transform at the test position
        let test_transform = Transform::new(position);

        // Check against all other static solid colliders
        for (_entity, other_transform, other_collider, _) in world.query_3::<Transform, Collider, StaticRenderer>() {
            if other_collider.is_trigger {
                continue; // Skip triggers
            }

            if self.shape.overlaps_with(&test_transform, &other_collider.shape, other_transform) {
                return true;
            }
        }

        false
    }

    /// Grid-based collision check for performance (legacy compatibility)
    pub fn check_grid_collision(world: &World, x: f32, z: f32) -> bool {
        let grid_x = x.floor() as i32;
        let grid_z = z.floor() as i32;

        for (_entity, transform, collider, _) in world.query_3::<Transform, Collider, StaticRenderer>() {
            if !collider.blocks_movement() {
                continue;
            }

            let entity_grid_x = (transform.position.x - 0.5).floor() as i32;
            let entity_grid_z = (transform.position.z - 0.5).floor() as i32;

            if entity_grid_x == grid_x && entity_grid_z == grid_z {
                return true;
            }
        }

        false
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
        // For now, implement simple AABB vs AABB collision
        // This can be expanded to handle all shape combinations
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
            // For other shape combinations, fall back to point-in-shape tests
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

impl Component for Collider {}

/// Player-specific component
#[derive(Debug, Clone)]
pub struct Player {
    pub health: f32,
    pub max_health: f32,
    pub is_grounded: bool,
    pub move_speed: f32,
    pub jump_strength: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            is_grounded: true,
            move_speed: 5.0,
            jump_strength: 4.5,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Player {}

// Tag components for easy querying
/// Marker component for wall entities
#[derive(Debug, Clone)]
pub struct Wall;

impl Component for Wall {}

/// Marker component for floor entities  
#[derive(Debug, Clone)]
pub struct Floor;

impl Component for Floor {}

/// Marker component for ceiling entities
#[derive(Debug, Clone)]
pub struct Ceiling;

impl Component for Ceiling {}

/// Marker component for prop entities
#[derive(Debug, Clone)]
pub struct Prop;

impl Component for Prop {}

/// Test bot component for automated testing
#[derive(Debug, Clone)]
pub struct TestBot {
    pub start_time: std::time::Instant,
    pub test_duration: std::time::Duration,
    pub current_waypoint: usize,
    pub waypoints: Vec<TestWaypoint>,
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub stuck_time: f32,
    pub last_position: (f32, f32),
    pub explored_nodes: Vec<(i32, i32)>,
    pub path_nodes: Vec<(i32, i32)>,
}

/// Waypoint for test bot navigation
#[derive(Debug, Clone)]
pub struct TestWaypoint {
    pub x: f32,
    pub y: f32,
    pub description: String,
}

impl TestBot {
    pub fn new(test_duration_seconds: u64) -> Self {
        let waypoints = vec![
            TestWaypoint { x: 1.5, y: 1.5, description: "Start".to_string() },
            TestWaypoint { x: 2.5, y: 1.5, description: "East corridor".to_string() },
            TestWaypoint { x: 3.5, y: 1.5, description: "Continue east".to_string() },
            TestWaypoint { x: 3.5, y: 1.5, description: "Turn point".to_string() },
            TestWaypoint { x: 4.5, y: 1.5, description: "Far east".to_string() },
            TestWaypoint { x: 5.5, y: 1.5, description: "Eastern wall".to_string() },
            TestWaypoint { x: 6.5, y: 1.5, description: "Corner approach".to_string() },
            TestWaypoint { x: 7.5, y: 1.5, description: "Near corner".to_string() },
            TestWaypoint { x: 8.5, y: 1.5, description: "Corner".to_string() },
            TestWaypoint { x: 8.5, y: 1.5, description: "Turn south".to_string() },
        ];

        Self {
            start_time: std::time::Instant::now(),
            test_duration: std::time::Duration::from_secs(test_duration_seconds),
            current_waypoint: 0,
            waypoints,
            movement_speed: 2.0,
            rotation_speed: 3.0,
            stuck_time: 0.0,
            last_position: (1.5, 1.5),
            explored_nodes: Vec::new(),
            path_nodes: Vec::new(),
        }
    }

    pub fn get_progress(&self) -> (usize, usize, f32) {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let total = self.test_duration.as_secs_f32();
        (self.current_waypoint, self.waypoints.len(), elapsed / total)
    }

    pub fn is_finished(&self) -> bool {
        self.start_time.elapsed() >= self.test_duration
    }
}

impl Component for TestBot {} 