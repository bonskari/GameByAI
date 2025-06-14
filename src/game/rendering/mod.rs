//! Rendering subsystem
//! 
//! This module handles all 3D rendering concerns:
//! - Core 3D geometry rendering
//! - Material and texture management
//! - Vertex buffer management

pub mod renderer_3d;
pub mod materials;
pub mod vertex_data;

// Re-export main types
pub use renderer_3d::Modern3DRenderer;
pub use materials::MaterialManager;
pub use vertex_data::{Vertex, WallMeshData}; 