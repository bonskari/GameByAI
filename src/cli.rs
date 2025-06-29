use clap::{Parser, Subcommand};

/// Command line interface for GameByAI
#[derive(Parser)]
#[command(name = "game-by-ai")]
#[command(about = "A 3D first-person game created with AI assistance")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available commands
#[derive(Subcommand)]
pub enum Commands {
    /// Run automated tests
    Test {
        /// Specific test to run (all, graphics, movement, collision, texture, pitch, position, lighting)
        #[arg(default_value = "all")]
        test_type: String,
        /// Timeout in seconds for each test
        #[arg(short, long, default_value = "10")]
        timeout: u64,
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Run automated visual tests with bot movement and lighting performance
    #[command(name = "visual-test")]
    VisualTest {
        /// Test duration in seconds (for bot navigation phase)
        #[arg(short, long, default_value = "15")]
        duration: u64,
        /// Disable auto-close after test completion
        #[arg(long)]
        no_auto_close: bool,
    },
    /// Generate AI textures using Stable Diffusion (Local or API)
    #[command(name = "generate-textures")]
    GenerateTextures {
        /// Output directory for generated textures
        #[arg(short, long, default_value = "assets/textures")]
        output: String,
        /// Hugging Face API token (or use HUGGINGFACE_API_TOKEN env var)
        #[arg(short, long)]
        token: Option<String>,
        /// Stable Diffusion model to use
        #[arg(short, long, default_value = "stabilityai/stable-diffusion-2-1")]
        model: String,
        /// Test API connection only
        #[arg(long)]
        test_only: bool,
        /// Generate specific texture type (tech-panel, hull-plating, control-system, energy-conduit, floor, ceiling)
        #[arg(long)]
        texture_type: Option<String>,
        /// Force API-only generation (skip local SD)
        #[arg(long)]
        api_only: bool,
        /// Force local-only generation (skip API)
        #[arg(long)]
        local_only: bool,
    },
    /// Control lighting in the game
    #[command(name = "lighting")]
    Lighting {
        /// Lighting action to perform
        #[command(subcommand)]
        action: LightingAction,
    },
    /// Export generated meshes to external file formats
    #[command(name = "export-meshes")]
    ExportMeshes {
        /// Output directory for exported meshes
        #[arg(short, long, default_value = "assets/meshes")]
        output: String,
        /// Export format (gltf only - modern game-ready format)
        #[arg(short, long, default_value = "gltf")]
        format: String,
        /// Export all meshes (walls, floor, ceiling)
        #[arg(long)]
        all: bool,
        /// Export only walls
        #[arg(long)]
        walls_only: bool,
        /// Export only floor
        #[arg(long)]
        floor_only: bool,
        /// Export only ceiling
        #[arg(long)]
        ceiling_only: bool,
    },
}

/// Lighting control actions
#[derive(Subcommand)]
pub enum LightingAction {
    /// Create a single omni light with sphere mesh in the center
    #[command(name = "single-omni")]
    SingleOmni,
    /// Remove all lights from the scene
    #[command(name = "remove-all")]
    RemoveAll,
    /// Start lighting performance tests
    #[command(name = "test")]
    Test,
} 