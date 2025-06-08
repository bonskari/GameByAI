//! Testing framework modules
//! 
//! This module contains the integrated testing system:
//! - Test execution framework
//! - Individual test implementations
//! - Test result reporting

pub mod runner;
pub mod tests;

// Re-export main functions for convenience
pub use tests::run_tests; 