use clap::{Parser, Subcommand};

/// Command line interface for Wolfenstein by AI
#[derive(Parser)]
#[command(name = "wolfenstein-ai")]
#[command(about = "A Wolfenstein-style game created with AI assistance")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available commands
#[derive(Subcommand)]
pub enum Commands {
    /// Run automated tests
    Test {
        /// Specific test to run (all, graphics, movement, collision)
        #[arg(default_value = "all")]
        test_type: String,
        /// Timeout in seconds for each test
        #[arg(short, long, default_value = "10")]
        timeout: u64,
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Run automated visual tests with bot movement
    VisualTest {
        /// Test duration in seconds
        #[arg(short, long, default_value = "15")]
        duration: u64,
        /// Disable auto-close after test completion
        #[arg(long)]
        no_auto_close: bool,
    },
} 