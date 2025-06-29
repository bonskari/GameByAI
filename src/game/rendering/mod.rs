//! Rendering subsystem
//! 
//! This module handles all 3D rendering concerns:
//! - Deferred rendering pipeline
//! - Material and texture management
//! - Vertex buffer management

pub mod materials;
pub mod vertex_data;
pub mod deferred_renderer;
pub mod gltf_loader;
pub mod renderer_3d;
pub mod lowpoly_meshes;

// Re-export main types
pub use deferred_renderer::DeferredRenderer;
pub use gltf_loader::GltfLoader; 