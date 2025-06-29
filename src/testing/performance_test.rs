//! Performance testing for enabled/disabled entity systems

use std::time::Instant;
use crate::ecs::{World, Transform, StaticRenderer, Wall};

/// Simple entity with enabled field for comparison testing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimpleEntity {
    pub id: u32,
    pub generation: u32,
    pub enabled: bool,
}

impl SimpleEntity {
    pub fn new(id: u32, generation: u32) -> Self {
        Self { id, generation, enabled: true }
    }
}

/// Performance test results
#[derive(Debug)]
pub struct PerformanceTestResult {
    pub vec_bool_time_ms: f64,
    pub entity_enabled_time_ms: f64,
    pub iterations: usize,
    pub entity_count: usize,
    pub performance_difference_percent: f64,
}

/// Performance testing for enabled/disabled systems
pub struct PerformanceTest;

impl PerformanceTest {
    /// Run performance test for component-level enabled state vs simple entity.enabled
    pub fn run_enabled_state_benchmark(iterations: usize, entity_count: usize) -> PerformanceTestResult {
        println!("🏃‍♂️ Running enabled state performance test...");
        println!("   Iterations: {}, Entities: {}", iterations, entity_count);

        // Test 1: Component-level enabled state (current approach)
        let component_enabled_time = Self::benchmark_component_enabled_approach(iterations, entity_count);
        
        // Test 2: Simple entity.enabled approach (for comparison)
        let entity_enabled_time = Self::benchmark_entity_enabled_approach(iterations, entity_count);
        
        let performance_difference = ((component_enabled_time - entity_enabled_time) / entity_enabled_time * 100.0).abs();
        
        let result = PerformanceTestResult {
            vec_bool_time_ms: component_enabled_time,  // Reusing field name for component approach
            entity_enabled_time_ms: entity_enabled_time,
            iterations,
            entity_count,
            performance_difference_percent: performance_difference,
        };
        
        Self::print_benchmark_results(&result);
        result
    }
    
    /// Benchmark component-level enabled state checking
    fn benchmark_component_enabled_approach(iterations: usize, entity_count: usize) -> f64 {
        let mut total_time = 0.0;
        
        for _ in 0..iterations {
            // Create world with entities
            let mut world = World::new();
            let mut entities = Vec::new();
            
            // Create entities
            for i in 0..entity_count {
                let mut static_renderer = StaticRenderer::wall("test.png".to_string());
                // Disable every 3rd component (same pattern as before)
                if i % 3 == 0 {
                    static_renderer.disable();
                }
                
                let entity = world.spawn()
                    .with(Transform::new(macroquad::prelude::Vec3::new(i as f32, 0.0, 0.0)))
                    .with(static_renderer)
                    .with(Wall::new())
                    .build();
                entities.push(entity);
            }
            
            // Benchmark: Check enabled state for all entities
            let start = Instant::now();
            
            let mut enabled_count = 0;
            for &entity in &entities {
                if let Some(static_renderer) = world.get::<StaticRenderer>(entity) {
                    if static_renderer.is_enabled() {
                        enabled_count += 1;
                    }
                }
            }
            
            let elapsed = start.elapsed();
            total_time += elapsed.as_secs_f64() * 1000.0; // Convert to milliseconds
            
            // Prevent optimization
            assert!(enabled_count > 0);
        }
        
        total_time / iterations as f64
    }
    
    /// Benchmark the simple entity.enabled approach
    fn benchmark_entity_enabled_approach(iterations: usize, entity_count: usize) -> f64 {
        let mut total_time = 0.0;
        
        for _ in 0..iterations {
            // Create simple entities
            let mut entities = Vec::new();
            
            for i in 0..entity_count {
                let mut entity = SimpleEntity::new(i as u32, 1);
                // Disable every 3rd entity (same pattern as Vec<bool> test)
                if i % 3 == 0 {
                    entity.enabled = false;
                }
                entities.push(entity);
            }
            
            // Benchmark: Check enabled state for all entities
            let start = Instant::now();
            
            let mut enabled_count = 0;
            for entity in &entities {
                if entity.enabled {
                    enabled_count += 1;
                }
            }
            
            let elapsed = start.elapsed();
            total_time += elapsed.as_secs_f64() * 1000.0; // Convert to milliseconds
            
            // Prevent optimization
            assert!(enabled_count > 0);
        }
        
        total_time / iterations as f64
    }
    
    /// Print benchmark results in a nice format
    fn print_benchmark_results(result: &PerformanceTestResult) {
        println!("\n📊 PERFORMANCE TEST RESULTS:");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🔍 Test Configuration:");
        println!("   • Iterations: {}", result.iterations);
        println!("   • Entities per iteration: {}", result.entity_count);
        println!("   • Total operations: {}", result.iterations * result.entity_count);
        println!();
        println!("⏱️  Performance Results:");
        println!("   • Component.enabled approach: {:.6} ms/iteration", result.vec_bool_time_ms);
        println!("   • Entity.enabled approach:    {:.6} ms/iteration", result.entity_enabled_time_ms);
        println!();
        
        if result.vec_bool_time_ms < result.entity_enabled_time_ms {
            println!("🏆 Winner: Component.enabled approach");
            println!("   • Faster by: {:.6} ms ({:.2}%)", 
                     result.entity_enabled_time_ms - result.vec_bool_time_ms,
                     result.performance_difference_percent);
        } else {
            println!("🏆 Winner: Entity.enabled approach");
            println!("   • Faster by: {:.6} ms ({:.2}%)", 
                     result.vec_bool_time_ms - result.entity_enabled_time_ms,
                     result.performance_difference_percent);
        }
        
        println!();
        println!("💡 Analysis:");
        if result.performance_difference_percent < 5.0 {
            println!("   • Performance difference is negligible (<5%)");
            println!("   • Code simplicity should be prioritized");
        } else if result.performance_difference_percent < 20.0 {
            println!("   • Moderate performance difference (5-20%)");
            println!("   • Consider complexity vs performance trade-off");
        } else {
            println!("   • Significant performance difference (>20%)");
            println!("   • Performance optimization may be worth the complexity");
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
    
    /// Quick test with game-realistic parameters
    pub fn run_game_realistic_test() -> PerformanceTestResult {
        // Test with realistic game parameters: 253 entities, 1000 iterations
        Self::run_enabled_state_benchmark(1000, 253)
    }
    
    /// Stress test with many entities
    pub fn run_stress_test() -> PerformanceTestResult {
        // Stress test: 10,000 entities, 100 iterations
        Self::run_enabled_state_benchmark(100, 10000)
    }
} 