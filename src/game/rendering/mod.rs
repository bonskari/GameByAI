//! Rendering subsystem
//! 
//! This module handles all 3D rendering concerns:
//! - Deferred rendering pipeline
//! - Material and texture management
//! - Vertex buffer management

pub mod materials;
pub mod vertex_data;
pub mod deferred_renderer;

// Re-export main types
pub use materials::MaterialManager;
pub use deferred_renderer::DeferredRenderer; 