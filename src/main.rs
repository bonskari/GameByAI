//! GameByAI - Main Entry Point
//!
//! A 3D first-person game created with AI assistance using Rust and macroquad.
//! Features integrated testing system and modular architecture.

use macroquad::prelude::*;
use clap::Parser;

mod cli;
mod game;
mod testing;
mod ecs;

use cli::{Cli, Commands};
use game::GameState;

/// Window configuration for fullscreen mode with native resolution
fn window_conf() -> Conf {
    Conf {
        window_title: "GameByAI - 3D Game Engine".to_owned(),
        fullscreen: true,
        window_width: 0,  // 0 means use native resolution
        window_height: 0, // 0 means use native resolution
        window_resizable: false,
        ..Default::default()
    }
}

/// Initialize a new game state with all necessary setup
async fn initialize_game() -> GameState {
    let mut game_state = GameState::new();
    
    // Initialize ECS with wall meshes and textures
    game_state.initialize().await;
    
    // Textures are already loaded in initialize()
    
    game_state
}

/// Run the main game loop
async fn run_game_loop(mut game_state: GameState, test_duration: Option<u64>) {
    let mut frame_counter = 0;
    let start_time = std::time::Instant::now();
    
    loop {
        let dt = get_frame_time();
        
        game_state.update(dt);
        game_state.draw();
        
        frame_counter += 1;
        
        // Reduced performance monitoring - only every 5 seconds to avoid console I/O bottleneck
        if frame_counter % 300 == 0 {
            let fps = get_fps() as f32;
            let performance_rating = if fps > 200.0 { "🚀 BLAZING" } 
                                   else if fps > 150.0 { "🔥 EXCELLENT" }
                                   else if fps > 120.0 { "✅ GOOD" }
                                   else if fps > 60.0 { "⚠️ OKAY" }
                                   else { "❌ SLOW" };
            
            println!("Performance: Frame {}, FPS: {:.0} {}", frame_counter, fps, performance_rating);
        }
        
        // Check for exit conditions
        if is_key_pressed(KeyCode::Escape) {
            println!("ESC pressed - exiting game");
            break;
        }
        
        // For visual tests, check if test duration exceeded or test bot finished
        if let Some(duration) = test_duration {
            if start_time.elapsed().as_secs() >= duration || game_state.ecs_state.is_test_bot_finished() {
                println!("🤖 Visual test completed");
                break;
            }
        }
        
        // Toggle mouse capture with M key
        if is_key_pressed(KeyCode::M) {
            // Toggle between mouse capture states
            use std::sync::atomic::{AtomicBool, Ordering};
            static MOUSE_CAPTURED: AtomicBool = AtomicBool::new(true);
            
            let current = MOUSE_CAPTURED.load(Ordering::Relaxed);
            let new_state = !current;
            MOUSE_CAPTURED.store(new_state, Ordering::Relaxed);
            
            set_cursor_grab(new_state);
            show_mouse(!new_state);
            println!("Mouse capture toggled: {}", new_state);
        }
        
        next_frame().await;
    }
    
    println!("Game loop ended");
}

/// Run the game in interactive mode
async fn run_game() {
    println!("Starting GameByAI - Interactive Mode");
    println!("Controls: WASD to move/strafe, Mouse to look, SPACE to jump, ESC to exit");
    
    // Enable mouse capture for FPS-style mouse look
    set_cursor_grab(true);
    show_mouse(false);
    
    // Try to disable V-sync for higher framerates
    #[cfg(not(target_arch = "wasm32"))]
    {
        use macroquad::window::*;
        // Request uncapped framerate if possible
        request_new_screen_size(screen_width(), screen_height());
    }
    
    let game_state = initialize_game().await;
    run_game_loop(game_state, None).await;
    
    println!("GameByAI shutting down...");
}

/// Run visual tests (lighting performance tests + bot navigation tests)
pub async fn run_visual_tests(test_duration: u64, auto_close: bool) {
    println!("🤖 Starting comprehensive visual tests...");
    println!("   Bot navigation duration: {}s", test_duration);
    println!("   Auto-close: {}", auto_close);
    
    // Initialize exactly like normal game
    let mut game_state = initialize_game().await;
    
    // Start lighting performance tests (component-based)
    game_state.ecs_state.start_lighting_tests();
    
    // Add test bot on top of normal game
    game_state.ecs_state.attach_test_bot(test_duration);
    
    // Run normal game loop with test bot active
    run_game_loop(game_state, Some(test_duration)).await;
}

/// Run texture generation
async fn run_texture_generation(
    output: &str,
    token: Option<String>,
    model: &str,
    test_only: bool,
    texture_type: Option<String>,
    api_only: bool,
    local_only: bool,
) {
    use game::textures::ai_generator::{AITextureGenerator, AITextureConfig, load_api_token};
    use game::map::WallType;
    use std::path::Path;

    println!("🎨 AI Texture Generation System");
    
    // Load API token from command line or environment (only needed for API generation)
    let api_token = token.or_else(load_api_token);
    
    // Only require token if not using local-only generation
    if api_token.is_none() && !local_only {
        println!("⚠️  No API token provided!");
        println!("   Set HUGGINGFACE_API_TOKEN environment variable");
        println!("   OR use --token argument");
        println!("   OR create api_token.txt file");
        println!("   OR create .env file with HUGGINGFACE_API_TOKEN=your_token");
        println!("\n💡 Free API access available at: https://huggingface.co/settings/tokens");
        println!("\n🤖 Alternative: Use --local-only for Python-based local generation");
        return;
    }

    // Determine generation preference from CLI flags
    let use_local = if api_only {
        false  // Force API only
    } else if local_only {
        true   // Force local only
    } else {
        true   // Default to local first, fallback to API
    };

    let config = AITextureConfig {
        api_token,
        model: model.to_string(),
        base_url: "https://api-inference.huggingface.co/models".to_string(),
        use_local,
        local_model_path: None,
    };

    let generator = AITextureGenerator::new(config);

    if test_only {
        println!("🧪 Testing API connection only...");
        match generator.test_connection().await {
            Ok(_) => println!("✅ API connection successful!"),
            Err(e) => {
                eprintln!("❌ API connection failed: {}", e);
                return;
            }
        }
        return;
    }

    let output_path = Path::new(output);

    if let Some(texture_name) = texture_type {
        // Generate specific texture type
        println!("🎨 Generating single texture: {}", texture_name);
        
        if let Err(e) = std::fs::create_dir_all(output_path) {
            eprintln!("❌ Failed to create output directory: {}", e);
            return;
        }

        let (filename, generation_result) = match texture_name.as_str() {
            "tech-panel" => ("tech_panel.png", generator.generate_texture(WallType::TechPanel).await),
            "hull-plating" => ("hull_plating.png", generator.generate_texture(WallType::HullPlating).await),
            "control-system" => ("control_system.png", generator.generate_texture(WallType::ControlSystem).await),
            "energy-conduit" => ("energy_conduit.png", generator.generate_texture(WallType::EnergyConduit).await),
            "floor" => ("floor.png", generator.generate_floor_texture().await),
            "ceiling" => ("ceiling.png", generator.generate_ceiling_texture().await),
            _ => {
                eprintln!("❌ Invalid texture type: {}", texture_name);
                eprintln!("   Valid options: tech-panel, hull-plating, control-system, energy-conduit, floor, ceiling");
                return;
            }
        };

        match generation_result {
            Ok(image_data) => {
                let file_path = output_path.join(filename);
                if let Err(e) = std::fs::write(&file_path, &image_data) {
                    eprintln!("❌ Failed to save texture: {}", e);
                } else {
                    println!("💾 Saved texture: {}", file_path.display());
                }
            }
            Err(e) => eprintln!("❌ Failed to generate texture: {}", e),
        }
    } else {
        // Generate all texture types
        match generator.generate_all_textures(output_path).await {
            Ok(_) => println!("🎯 All textures generated successfully!"),
            Err(e) => eprintln!("❌ Texture generation failed: {}", e),
        }
    }
}

/// Main entry point
fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Test { test_type, timeout, verbose }) => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                testing::run_tests(&test_type, timeout, verbose).await;
            });
        },
        Some(Commands::VisualTest { duration, no_auto_close }) => {
            macroquad::Window::from_config(window_conf(), async move {
                run_visual_tests(duration, !no_auto_close).await;
            });
        },
        Some(Commands::GenerateTextures { output, token, model, test_only, texture_type, api_only, local_only }) => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                run_texture_generation(&output, token, &model, test_only, texture_type, api_only, local_only).await;
            });
        },
        None => {
            macroquad::Window::from_config(window_conf(), run_game());
        }
    }
}
