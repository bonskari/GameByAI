use std::time::{Duration, Instant};

/// Test result tracking
#[derive(Debug, Clone)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub duration: Duration,
}

/// Test execution framework
pub struct TestRunner {
    pub results: Vec<TestResult>,
    pub verbose: bool,
    pub timeout: Duration,
}

impl TestRunner {
    /// Create a new test runner
    pub fn new(verbose: bool, timeout: u64) -> Self {
        println!("=== GAMEBYAI - INTEGRATED TEST SYSTEM ===");
        println!("Timeout: {}s per test | Verbose: {}", timeout, verbose);
        println!("Platform: {} | Graphics: macroquad", std::env::consts::OS);
        println!("");
        
        Self {
            results: Vec::new(),
            verbose,
            timeout: Duration::from_secs(timeout),
        }
    }
    
    /// Run a single test
    pub fn run_test<F>(&mut self, name: &str, test_fn: F) 
    where F: FnOnce(&mut Self) -> Result<String, String>
    {
        println!("🔧 Starting test: {}", name);
        
        let start = Instant::now();
        let result = test_fn(self);
        let duration = start.elapsed();
        
        let (passed, message) = match result {
            Ok(msg) => (true, msg),
            Err(err) => (false, err),
        };
        
        let status = if passed { "✓ PASS" } else { "✗ FAIL" };
        let color = if passed { "\x1b[32m" } else { "\x1b[31m" };
        println!("{}{} {}\x1b[0m - {} ({:.2}s)", color, status, name, message, duration.as_secs_f32());
        
        self.results.push(TestResult {
            name: name.to_string(),
            passed,
            message,
            duration,
        });
        
        // Add a small pause between visual tests for better UX
        if name.contains("Graphics") || name.contains("Game Loop") {
            println!("   ⏳ Preparing next test...");
            std::thread::sleep(Duration::from_millis(100));
        }
    }
    
    /// Print test summary and return success status
    pub fn print_summary(&self) -> bool {
        let passed = self.results.iter().filter(|r| r.passed).count();
        let total = self.results.len();
        
        println!("");
        println!("=== TEST SUMMARY ===");
        println!("Results: {}/{} tests passed", passed, total);
        
        if passed < total {
            println!("\x1b[31mFailed tests:\x1b[0m");
            for result in &self.results {
                if !result.passed {
                    println!("  • {}: {}", result.name, result.message);
                }
            }
        }
        
        let success_rate = (passed as f32 / total as f32) * 100.0;
        println!("Success rate: {:.1}%", success_rate);
        
        passed == total
    }
} 