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

// Re-export main functions for convenience
pub use runner::{TestResult, TestRunner};
pub use tests::run_tests;
pub use performance_test::*; 