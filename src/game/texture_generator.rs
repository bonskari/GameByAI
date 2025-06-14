use macroquad::prelude::*;
use crate::game::map::{Map, WallType};
use std::collections::HashMap;

/// Enhanced texture generation and management system with advanced tiling
pub struct TextureGenerator {
    pub textures: HashMap<String, Texture2D>,
    pub texture_size: usize,
}

impl TextureGenerator {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            texture_size: 256, // Good balance of quality and performance
        }
    }

    /// Generate and save texture files to disk, then load them
    pub async fn generate_and_load_textures(&mut self) -> Result<(), String> {
        println!("🎨 Generating texture files...");
        
        // Create textures directory if it doesn't exist
        std::fs::create_dir_all("textures").map_err(|e| format!("Failed to create textures directory: {}", e))?;
        
        // Generate each wall type texture
        self.generate_texture_file("tech_panel", Self::generate_tech_panel_pattern).await?;
        self.generate_texture_file("hull_plating", Self::generate_hull_plating_pattern).await?;
        self.generate_texture_file("control_system", Self::generate_control_system_pattern).await?;
        self.generate_texture_file("energy_conduit", Self::generate_energy_conduit_pattern).await?;
        
        println!("✅ All texture files generated and loaded!");
        Ok(())
    }

    /// Generate a single texture file and load it
    async fn generate_texture_file<F>(&mut self, name: &str, pattern_fn: F) -> Result<(), String>
    where
        F: Fn(usize, usize, usize) -> (u8, u8, u8),
    {
        let file_path = format!("textures/{}.png", name);
        
        // Check if file already exists
        if std::path::Path::new(&file_path).exists() {
            println!("   Loading existing {} texture...", name);
        } else {
            println!("   Generating {} texture...", name);
            
            // Generate texture data
            let mut image_data = Vec::with_capacity(self.texture_size * self.texture_size * 4);
            
            for y in 0..self.texture_size {
                for x in 0..self.texture_size {
                    let (r, g, b) = pattern_fn(x, y, self.texture_size);
                    image_data.push(r);
                    image_data.push(g);
                    image_data.push(b);
                    image_data.push(255); // Alpha
                }
            }
            
            // Save to file using image crate
            let img = image::RgbaImage::from_raw(self.texture_size as u32, self.texture_size as u32, image_data)
                .ok_or("Failed to create image")?;
            
            img.save(&file_path).map_err(|e| format!("Failed to save {}: {}", file_path, e))?;
        }
        
        // Load texture from file
        let texture = load_texture(&file_path).await.map_err(|e| format!("Failed to load {}: {}", file_path, e))?;
        texture.set_filter(FilterMode::Linear);
        
        self.textures.insert(name.to_string(), texture);
        Ok(())
    }

    /// Get a texture by name
    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }

    /// Tech panel pattern - clean metallic with circuit lines
    fn generate_tech_panel_pattern(x: usize, y: usize, size: usize) -> (u8, u8, u8) {
        let fx = x as f32 / size as f32;
        let fy = y as f32 / size as f32;
        
        // Base metallic color
        let base_r = 0.6;
        let base_g = 0.65;
        let base_b = 0.7;
        
        // Add circuit lines
        let circuit_x = ((fx * 8.0).sin() * 0.5 + 0.5) * 0.3;
        let circuit_y = ((fy * 6.0).sin() * 0.5 + 0.5) * 0.3;
        let circuit = (circuit_x + circuit_y).min(1.0);
        
        // Add panel seams
        let seam_x = if (fx * 4.0).fract() < 0.05 { 0.2 } else { 0.0 };
        let seam_y = if (fy * 4.0).fract() < 0.05 { 0.2 } else { 0.0 };
        
        let final_r = (base_r + circuit + seam_x).min(1.0);
        let final_g = (base_g + circuit + seam_y).min(1.0);
        let final_b = (base_b + circuit * 0.5).min(1.0);
        
        ((final_r * 255.0) as u8, (final_g * 255.0) as u8, (final_b * 255.0) as u8)
    }

    /// Hull plating pattern - darker metal with rivets
    fn generate_hull_plating_pattern(x: usize, y: usize, size: usize) -> (u8, u8, u8) {
        let fx = x as f32 / size as f32;
        let fy = y as f32 / size as f32;
        
        // Base dark metal
        let base_r = 0.3;
        let base_g = 0.35;
        let base_b = 0.4;
        
        // Add rivets
        let rivet_spacing = 0.25;
        let rivet_x = (fx / rivet_spacing).fract();
        let rivet_y = (fy / rivet_spacing).fract();
        
        let rivet_dist = ((rivet_x - 0.5).powi(2) + (rivet_y - 0.5).powi(2)).sqrt();
        let rivet = if rivet_dist < 0.1 { 0.3 } else { 0.0 };
        
        // Add scratches
        let scratch = ((fx * 16.0 + fy * 2.0).sin() * 0.5 + 0.5) * 0.1;
        
        let final_r = (base_r + rivet + scratch).min(1.0);
        let final_g = (base_g + rivet + scratch).min(1.0);
        let final_b = (base_b + rivet + scratch).min(1.0);
        
        ((final_r * 255.0) as u8, (final_g * 255.0) as u8, (final_b * 255.0) as u8)
    }

    /// Control system pattern - dark with glowing elements
    fn generate_control_system_pattern(x: usize, y: usize, size: usize) -> (u8, u8, u8) {
        let fx = x as f32 / size as f32;
        let fy = y as f32 / size as f32;
        
        // Base dark color
        let base_r = 0.1;
        let base_g = 0.15;
        let base_b = 0.2;
        
        // Add glowing control elements
        let control_x = ((fx * 6.0).sin() * 0.5 + 0.5);
        let control_y = ((fy * 4.0).cos() * 0.5 + 0.5);
        let glow = (control_x * control_y) * 0.5;
        
        // Add status lights
        let light_spacing = 0.2;
        let light_x = (fx / light_spacing).fract();
        let light_y = (fy / light_spacing).fract();
        
        let light_dist = ((light_x - 0.5).powi(2) + (light_y - 0.5).powi(2)).sqrt();
        let light = if light_dist < 0.05 { 0.6 } else { 0.0 };
        
        let final_r = (base_r + glow * 0.3 + light * 0.2).min(1.0);
        let final_g = (base_g + glow * 0.6 + light * 0.4).min(1.0);
        let final_b = (base_b + glow * 0.8 + light * 0.8).min(1.0);
        
        ((final_r * 255.0) as u8, (final_g * 255.0) as u8, (final_b * 255.0) as u8)
    }

    /// Energy conduit pattern - pipes with energy flow
    fn generate_energy_conduit_pattern(x: usize, y: usize, size: usize) -> (u8, u8, u8) {
        let fx = x as f32 / size as f32;
        let fy = y as f32 / size as f32;
        
        // Base conduit color
        let base_r = 0.2;
        let base_g = 0.3;
        let base_b = 0.4;
        
        // Add pipe structure
        let pipe_x = ((fx * 3.0).sin() * 0.5 + 0.5) * 0.2;
        let pipe_y = ((fy * 2.0).cos() * 0.5 + 0.5) * 0.2;
        
        // Add energy flow
        let flow = ((fx * 8.0 + fy * 4.0).sin() * 0.5 + 0.5) * 0.3;
        
        let final_r = (base_r + pipe_x + flow * 0.1).min(1.0);
        let final_g = (base_g + pipe_y + flow * 0.3).min(1.0);
        let final_b = (base_b + flow * 0.5).min(1.0);
        
        ((final_r * 255.0) as u8, (final_g * 255.0) as u8, (final_b * 255.0) as u8)
    }
} 