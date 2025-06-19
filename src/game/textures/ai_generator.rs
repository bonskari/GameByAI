//! AI-powered texture generation using Stable Diffusion (API and Local)

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::collections::HashMap;
use crate::game::map::WallType;

use image;

/// Configuration for AI texture generation
#[derive(Debug, Clone)]
pub struct AITextureConfig {
    pub api_token: Option<String>,
    pub model: String,
    pub base_url: String,
    pub use_local: bool,
    pub local_model_path: Option<String>,
}

impl Default for AITextureConfig {
    fn default() -> Self {
        Self {
            api_token: None,
            model: "runwayml/stable-diffusion-v1-5".to_string(),
            base_url: "https://api-inference.huggingface.co/models".to_string(),
            use_local: true, // Prefer local generation
            local_model_path: None,
        }
    }
}

/// Request payload for Hugging Face Inference API
#[derive(Debug, Serialize)]
struct GenerationRequest {
    inputs: String,
    parameters: GenerationParameters,
}

#[derive(Debug, Serialize)]
struct GenerationParameters {
    width: u32,
    height: u32,
    num_inference_steps: u32,
    guidance_scale: f32,
    negative_prompt: Option<String>,
}

impl Default for GenerationParameters {
    fn default() -> Self {
        Self {
            width: 512,
            height: 512,
            num_inference_steps: 20,
            guidance_scale: 7.5,
            negative_prompt: Some("blurry, low quality, distorted, artifacts".to_string()),
        }
    }
}

/// AI texture generator using Stable Diffusion (Local and API)
pub struct AITextureGenerator {
    config: AITextureConfig,
    client: reqwest::Client,
    prompt_templates: HashMap<WallType, String>,
}

impl AITextureGenerator {
    /// Create a new AI texture generator
    pub fn new(config: AITextureConfig) -> Self {
        let client = reqwest::Client::new();
        let mut prompt_templates = HashMap::new();
        
        // Define sci-fi texture prompts optimized for seamless tiling
        prompt_templates.insert(
            WallType::TechPanel,
            "tech panel surface texture, metallic blue industrial design, geometric control interface pattern, electronic components layout, seamless repeating pattern".to_string()
        );
        
        prompt_templates.insert(
            WallType::HullPlating,
            "metal hull plating texture, riveted steel panels pattern, industrial weathered surface, geometric panel grid, seamless repeating design".to_string()
        );
        
        prompt_templates.insert(
            WallType::ControlSystem,
            "control system interface texture, circuit board pattern, green holographic display elements, tech grid layout, seamless repeating pattern".to_string()
        );
        
        prompt_templates.insert(
            WallType::EnergyConduit,
            "energy conduit surface texture, power line channels pattern, blue glowing energy flow, electrical circuit layout, seamless repeating design".to_string()
        );

        // Add floor texture
        // Note: Using TechPanel as placeholder since WallType doesn't have Floor
        // This will be expanded when we add proper FloorType enum

        Self {
            config,
            client,
            prompt_templates,
        }
    }

    /// Generate texture for a specific wall type
    pub async fn generate_texture(&self, wall_type: WallType) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let prompt = self.prompt_templates.get(&wall_type)
            .ok_or("No prompt template found for wall type")?;

        self.generate_from_prompt(prompt).await
    }

    /// Generate floor texture using optimized prompt
    pub async fn generate_floor_texture(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let floor_prompt = "industrial floor texture, metal grating pattern, sci-fi corridor flooring, geometric grid design, dark metallic surface";
        self.generate_from_prompt(floor_prompt).await
    }

    /// Generate ceiling texture using optimized prompt
    pub async fn generate_ceiling_texture(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let ceiling_prompt = "industrial ceiling texture, ventilation panels pattern, sci-fi overhead surface, geometric vent grid, metallic ceiling tiles";
        self.generate_from_prompt(ceiling_prompt).await
    }

    /// Generate texture from a custom prompt
    pub async fn generate_from_prompt(&self, prompt: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        println!("🎨 Generating texture with prompt: {}", prompt);

        if self.config.use_local {
            println!("🦀 Attempting local Stable Diffusion generation...");
            return self.try_local_generation(prompt).await;
        }

        if let Some(_token) = &self.config.api_token {
            println!("🌐 Using API with model: {}", self.config.model);
            
            // Try text-to-image endpoint first
            match self.try_text_to_image_api(prompt).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    println!("📡 Text-to-image API failed: {}", e);
                    // Try simple generation as backup
                    return self.try_simple_generation_api(prompt).await;
                }
            }
        }

        Err("No generation method configured. Set up either local Python environment (--local-only) or API token.".into())
    }

    async fn try_text_to_image_api(&self, prompt: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let request = GenerationRequest {
            inputs: prompt.to_string(),
            parameters: GenerationParameters::default(),
        };

        let url = format!("{}/{}", self.config.base_url, self.config.model);
        
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        
        if let Some(token) = &self.config.api_token {
            headers.insert("Authorization", format!("Bearer {}", token).parse()?);
        }

        let response = self.client
            .post(&url)
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status_code = response.status();
            let error_text = response.text().await?;
            return Err(format!("API failed: {} {}", status_code, error_text).into());
        }

        let image_bytes = response.bytes().await?;
        println!("✅ Generated texture: {} bytes", image_bytes.len());
        Ok(image_bytes.to_vec())
    }

    async fn try_simple_generation_api(&self, prompt: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        // Try simpler payload format
        let simple_request = serde_json::json!({
            "inputs": prompt,
            "options": {
                "wait_for_model": true
            }
        });

        let url = format!("{}/{}", self.config.base_url, self.config.model);
        
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        
        if let Some(token) = &self.config.api_token {
            headers.insert("Authorization", format!("Bearer {}", token).parse()?);
        }

        let response = self.client
            .post(&url)
            .headers(headers)
            .json(&simple_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status_code = response.status();
            let error_text = response.text().await?;
            return Err(format!("Simple API failed: {} {}", status_code, error_text).into());
        }

        let image_bytes = response.bytes().await?;
        println!("✅ Generated texture: {} bytes", image_bytes.len());
        Ok(image_bytes.to_vec())
    }

    /// Try local Stable Diffusion generation using Python subprocess only
    async fn try_local_generation(&self, prompt: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        println!("🐍 Using Python SDXL generation...");
        // Skip Candle (which has dependency issues) and go straight to working Python
        return self.try_python_subprocess(prompt).await;
    }



    /// Try to generate using a Python subprocess
    async fn try_python_subprocess(&self, prompt: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        use std::process::Command;
        use std::fs;

        // Try the ultra-fast persistent server first
        if std::path::Path::new("tools/sdxl_server.py").exists() {
            println!("⚡ Using ultra-fast SDXL server...");
            return self.try_persistent_server(prompt).await;
        }

        // Fallback to the optimized single-shot script
        if std::path::Path::new("tools/temp_sd_generator.py").exists() {
            println!("🚀 Using optimized SDXL script...");
            return self.try_optimized_script(prompt).await;
        }

        // Last resort: create basic script
        println!("🔧 Creating basic SDXL script...");
        let script_content = self.create_basic_script();
        fs::write("tools/temp_sd_generator.py", script_content)?;
        self.try_optimized_script(prompt).await
    }

    async fn try_persistent_server(&self, prompt: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        use std::process::Command;
        use std::fs;

        println!("⚡ Generating with persistent SDXL server (ultra-fast)...");
        
        // Use the persistent server which keeps model loaded
        let output = Command::new("python")
            .arg("tools/sdxl_server.py")
            .arg("single")
            .arg(prompt)
            .arg("temp_sd_texture.png")
            .output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    if std::path::Path::new("temp_sd_texture.png").exists() {
                        let image_bytes = fs::read("temp_sd_texture.png")?;
                        let _ = fs::remove_file("temp_sd_texture.png");
                        
                        // Print performance info from server output
                        let output_text = String::from_utf8_lossy(&result.stdout);
                        if let Some(time_line) = output_text.lines().find(|line| line.contains("Generated texture in")) {
                            println!("🎯 {}", time_line.trim());
                        }
                        
                        println!("✅ Ultra-fast generation complete: {} bytes", image_bytes.len());
                        return Ok(image_bytes);
                    } else {
                        return Err("Server succeeded but no image generated".into());
                    }
                } else {
                    let error = String::from_utf8_lossy(&result.stderr);
                    let output_text = String::from_utf8_lossy(&result.stdout);
                    println!("Server stdout: {}", output_text);
                    return Err(format!("SDXL server failed: {}", error).into());
                }
            }
            Err(e) => {
                println!("❌ Persistent server failed: {}, falling back to regular script", e);
                // Fallback to regular script
                return self.try_optimized_script(prompt).await;
            }
        }
    }

    async fn try_optimized_script(&self, prompt: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        use std::process::Command;
        use std::fs;

        // Create prompt file to avoid command line argument issues
        fs::write("temp_prompt.txt", prompt)?;

        println!("🚀 Running optimized SDXL generation...");
        
        // Run the existing optimized script with prompt file input
        let output = Command::new("python")
            .arg("-c")
            .arg(r#"
import sys
import os

# Read prompt from file
with open('temp_prompt.txt', 'r') as f:
    prompt = f.read().strip()

# Modify the temp_sd_generator.py to use the prompt
with open('tools/temp_sd_generator.py', 'r') as f:
    script = f.read()

# Replace the hardcoded prompt with our dynamic one
script = script.replace(
    'prompt = "tech panel surface texture, metallic blue industrial design, geometric control interface pattern, electronic components layout, seamless repeating pattern"',
    f'prompt = "{prompt}"'
)

# Execute the modified script
exec(script)
"#)
            .output();

        // Clean up prompt file
        let _ = fs::remove_file("temp_prompt.txt");

        match output {
            Ok(result) => {
                if result.status.success() {
                    if std::path::Path::new("temp_sd_texture.png").exists() {
                        let image_bytes = fs::read("temp_sd_texture.png")?;
                        let _ = fs::remove_file("temp_sd_texture.png");
                        println!("✅ Generated texture: {} bytes", image_bytes.len());
                        return Ok(image_bytes);
                    } else {
                        return Err("Python script succeeded but no image generated".into());
                    }
                } else {
                    let error = String::from_utf8_lossy(&result.stderr);
                    let output_text = String::from_utf8_lossy(&result.stdout);
                    println!("Python stdout: {}", output_text);
                    return Err(format!("Python script failed: {}", error).into());
                }
            }
            Err(e) => {
                return Err(format!("Failed to run Python: {}", e).into());
            }
        }
    }

    fn create_basic_script(&self) -> String {
        // Fallback basic script if the optimized one doesn't exist
        r#"
import sys
try:
    from diffusers import StableDiffusionXLPipeline, EulerAncestralDiscreteScheduler
    import torch
    from PIL import Image
    import gc
    
    print("Loading SDXL with speed optimizations...")
    pipe = StableDiffusionXLPipeline.from_pretrained(
        "stabilityai/stable-diffusion-xl-base-1.0",
        torch_dtype=torch.float16,
        use_safetensors=True,
        variant="fp16"
    )
    
    if torch.cuda.is_available():
        pipe = pipe.to("cuda")
        pipe.scheduler = EulerAncestralDiscreteScheduler.from_config(pipe.scheduler.config)
        try:
            pipe.enable_xformers_memory_efficient_attention()
        except:
            pass
        try:
            pipe.unet = torch.compile(pipe.unet, mode="reduce-overhead", fullgraph=True)
        except:
            pass
    
    prompt = "tech panel surface texture, metallic blue industrial design, geometric control interface pattern, electronic components layout, seamless repeating pattern"
    
    with torch.inference_mode():
        image = pipe(
            prompt,
            num_inference_steps=20,
            guidance_scale=5.0,
            width=512,
            height=512,
            generator=torch.Generator(device="cuda").manual_seed(42)
        ).images[0]
    
    torch.cuda.empty_cache()
    gc.collect()
    image.save("temp_sd_texture.png")
    print("SUCCESS: Generated texture with optimizations")
    
except Exception as e:
    print(f"ERROR: {e}")
    sys.exit(1)
"#.to_string()
    }



    /// Generate all texture types and save them to the assets directory
    pub async fn generate_all_textures(&self, output_dir: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Create output directory if it doesn't exist
        tokio::fs::create_dir_all(output_dir).await?;

        println!("🚀 Starting AI texture generation for all texture types...");

        // Generate wall textures
        for wall_type in [WallType::TechPanel, WallType::HullPlating, WallType::ControlSystem, WallType::EnergyConduit] {
            let filename = match wall_type {
                WallType::TechPanel => "tech_panel.png",
                WallType::HullPlating => "hull_plating.png", 
                WallType::ControlSystem => "control_system.png",
                WallType::EnergyConduit => "energy_conduit.png",
                WallType::Empty => continue, // Skip empty wall types
            };

            let output_path = output_dir.join(filename);

            println!("🎨 Generating wall texture for {:?}...", wall_type);
            
            match self.generate_texture(wall_type).await {
                Ok(image_data) => {
                    tokio::fs::write(&output_path, &image_data).await?;
                    println!("💾 Saved texture: {}", output_path.display());
                }
                Err(e) => {
                    eprintln!("❌ FATAL: Failed to generate texture for {:?}: {}", wall_type, e);
                    eprintln!("❌ Stopping texture generation due to error.");
                    return Err(e);
                }
            }

            // Add delay between requests to be respectful to the API
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }

        // Generate floor texture
        println!("🎨 Generating floor texture...");
        match self.generate_floor_texture().await {
            Ok(image_data) => {
                let floor_path = output_dir.join("floor.png");
                tokio::fs::write(&floor_path, &image_data).await?;
                println!("💾 Saved texture: {}", floor_path.display());
            }
            Err(e) => {
                eprintln!("❌ FATAL: Failed to generate floor texture: {}", e);
                eprintln!("❌ Stopping texture generation due to error.");
                return Err(e);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Generate ceiling texture
        println!("🎨 Generating ceiling texture...");
        match self.generate_ceiling_texture().await {
            Ok(image_data) => {
                let ceiling_path = output_dir.join("ceiling.png");
                tokio::fs::write(&ceiling_path, &image_data).await?;
                println!("💾 Saved texture: {}", ceiling_path.display());
            }
            Err(e) => {
                eprintln!("❌ FATAL: Failed to generate ceiling texture: {}", e);
                eprintln!("❌ Stopping texture generation due to error.");
                return Err(e);
            }
        }

        println!("🎯 AI texture generation completed!");
        Ok(())
    }

    /// Test the API connection with a simple prompt
    pub async fn test_connection(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("🧪 Testing Hugging Face API connection...");
        
        let test_prompt = "pixel art blue metallic surface, simple test texture";
        let result = self.generate_from_prompt(test_prompt).await?;
        
        println!("✅ API connection successful! Generated {} bytes", result.len());
        Ok(())
    }
}

/// Helper function to load API token from environment or file
pub fn load_api_token() -> Option<String> {
    // Try environment variable first
    if let Ok(token) = std::env::var("HUGGINGFACE_API_TOKEN") {
        return Some(token);
    }

    // Try loading from .env file
    if let Ok(contents) = std::fs::read_to_string(".env") {
        for line in contents.lines() {
            if let Some(token) = line.strip_prefix("HUGGINGFACE_API_TOKEN=") {
                return Some(token.trim().to_string());
            }
        }
    }

    // Try loading from api_token.txt file
    if let Ok(token) = std::fs::read_to_string("api_token.txt") {
        return Some(token.trim().to_string());
    }

    None
}



 