//! Component modules

pub mod transform;
pub mod velocity;
pub mod player;
pub mod collision;
pub mod entities;
pub mod pathfinding;
pub mod meshes;
pub mod lighting;
pub mod rendering;

// Re-export all components
pub use transform::Transform;
pub use velocity::Velocity;
pub use player::Player;
pub use collision::{Collider, ColliderShape, ColliderMaterial};
pub use entities::{Wall, Floor, Ceiling, Prop};
pub use pathfinding::{TestWaypoint, Pathfinder, TestBot};
pub use meshes::{StaticMesh, StaticMeshType, Renderer, RenderMode, RenderMaterial};
pub use lighting::{LightSource, LightSourceType, LightReceiver, LightingTest};
pub use rendering::{StaticRenderer, MaterialType, Renderable, RenderData, RenderType}; 