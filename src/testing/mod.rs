//! Testing framework modules
//! 
//! This module contains the integrated testing system:
//! - Test execution framework
//! - Individual test implementations
//! - Test result reporting

pub mod runner;
pub mod tests;
pub mod screenshot_validator;

// Re-export main functions for convenience
pub use runner::{TestResult, TestRunner};
pub use tests::run_tests; 