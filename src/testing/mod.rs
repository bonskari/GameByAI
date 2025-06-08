//! Testing framework modules
//! 
//! This module contains the integrated testing system:
//! - Test execution framework
//! - Individual test implementations
//! - Test result reporting

pub mod runner;
pub mod tests;
pub mod visual_tests;
pub mod screenshot_validator;

// Re-export main functions for convenience
pub use tests::run_tests;
pub use visual_tests::run_visual_tests; 