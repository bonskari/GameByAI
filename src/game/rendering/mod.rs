//! Rendering subsystem
//! 
//! This module handles all 3D rendering concerns:
//! - Core 3D geometry rendering
//! - Material and texture management
//! - Vertex buffer management
//! - Deferred rendering pipeline

pub mod renderer_3d;
pub mod materials;
pub mod vertex_data;
pub mod deferred_renderer;

// Re-export main types
pub use renderer_3d::Modern3DRenderer;
pub use materials::MaterialManager;
pub use deferred_renderer::DeferredRenderer; 