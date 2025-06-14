//! Clean 3D renderer with separated concerns

use macroquad::prelude::*;
use crate::game::rendering::MaterialManager;
use crate::game::{Map, Player};

/// Modern 3D renderer with clean separation of concerns
pub struct Modern3DRenderer {
    camera: Camera3D,
    material_manager: MaterialManager,
    // Mesh data will be stored here
    wall_mesh: Option<Mesh>,
    floor_mesh: Option<Mesh>,
    ceiling_mesh: Option<Mesh>,
}

impl Modern3DRenderer {
    /// Create a new renderer
    pub fn new() -> Self {
        let camera = Camera3D {
            position: vec3(1.5, 1.0, 1.5),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(2.0, 1.0, 1.5),
            ..Default::default()
        };

        Self {
            camera,
            material_manager: MaterialManager::new(),
            wall_mesh: None,
            floor_mesh: None,
            ceiling_mesh: None,
        }
    }

    /// Update camera based on player position
    pub fn update_camera(&mut self, player: &Player) {
        // Keep the existing camera logic for now
        self.camera.position = vec3(player.x, player.z, player.y);
        let yaw = player.rotation;
        let pitch = player.pitch;
        
        let look_x = yaw.cos() * pitch.cos();
        let look_y = pitch.sin();
        let look_z = yaw.sin() * pitch.cos();
        
        self.camera.target = self.camera.position + vec3(look_x, look_y, look_z);
    }

    /// Render the 3D scene
    pub fn render(&mut self, _map: &Map, player: &Player) {
        self.update_camera(player);
        
        set_camera(&self.camera);
        
        // For now, just clear the screen
        // The actual rendering will be implemented after we migrate the existing renderer
        clear_background(BLACK);
        
        // Placeholder text
        set_default_camera();
        draw_text("New Clean Renderer (Placeholder)", 10.0, 30.0, 20.0, WHITE);
    }

    /// Get mutable reference to material manager
    pub fn material_manager_mut(&mut self) -> &mut MaterialManager {
        &mut self.material_manager
    }

    /// Get reference to material manager
    pub fn material_manager(&self) -> &MaterialManager {
        &self.material_manager
    }
} 