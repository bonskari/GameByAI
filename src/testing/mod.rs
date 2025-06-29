//! Testing framework modules
//! 
//! This module contains the integrated testing system:
//! - Test execution framework
//! - Individual test implementations
//! - Test result reporting

pub mod runner;
pub mod tests;
pub mod screenshot_validator;
pub mod performance_test;
// Lighting tests are now integrated into ECS state

// Re-export main functions for convenience
pub use tests::run_tests;
// Lighting test functionality moved to ECS state 