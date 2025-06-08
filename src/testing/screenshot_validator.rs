use std::path::{Path, PathBuf};
use std::fs;
use macroquad::prelude::*;
use image::{ImageBuffer, Rgba};

/// Screenshot validation system for automated visual testing
pub struct ScreenshotValidator {
    base_path: PathBuf,
    reference_path: PathBuf,
    current_path: PathBuf,
    diff_path: PathBuf,
    tolerance: f32,
}

/// Result of screenshot comparison
#[derive(Debug, Clone)]
pub struct ComparisonResult {
    pub matches: bool,
    pub difference_percentage: f32,
    pub max_pixel_diff: f32,
    pub reference_exists: bool,
    pub screenshot_path: PathBuf,
    pub diff_path: Option<PathBuf>,
}

impl ScreenshotValidator {
    /// Create a new screenshot validator
    pub fn new(base_path: &str, tolerance: f32) -> Self {
        let base = PathBuf::from(base_path);
        let reference = base.join("reference");
        let current = base.join("current");
        let diff = base.join("diff");
        
        // Create directories if they don't exist
        let _ = fs::create_dir_all(&reference);
        let _ = fs::create_dir_all(&current);
        let _ = fs::create_dir_all(&diff);
        
        ScreenshotValidator {
            base_path: base,
            reference_path: reference,
            current_path: current,
            diff_path: diff,
            tolerance,
        }
    }
    
    /// Capture and validate a screenshot
    pub async fn capture_and_validate(&self, test_name: &str, description: &str) -> ComparisonResult {
        let filename = format!("{}.png", test_name);
        let current_file = self.current_path.join(&filename);
        let reference_file = self.reference_path.join(&filename);
        
        // Capture current screenshot
        println!("📸 Capturing screenshot: {} - {}", test_name, description);
        
        // Get screen dimensions
        let width = screen_width() as u32;
        let height = screen_height() as u32;
        
        // Capture the screen
        let image = get_screen_data();
        
        // Convert to RGBA image
        let mut rgba_image = ImageBuffer::new(width, height);
        
        // macroquad returns Color data, convert to RGBA
        for (i, color) in image.bytes.chunks(4).enumerate() {
            let x = (i as u32) % width;
            let y = (i as u32) / width;
            
            if x < width && y < height {
                let pixel = Rgba([color[0], color[1], color[2], color[3]]);
                rgba_image.put_pixel(x, y, pixel);
            }
        }
        
        // Save current screenshot
        if let Err(e) = rgba_image.save(&current_file) {
            println!("⚠️ Failed to save screenshot: {}", e);
        }
        
        // Compare with reference if it exists
        if reference_file.exists() {
            self.compare_images(&current_file, &reference_file, test_name).await
        } else {
            println!("📋 No reference image found for '{}' - saving current as reference", test_name);
            
            // Copy current to reference for future comparisons
            if let Err(e) = fs::copy(&current_file, &reference_file) {
                println!("⚠️ Failed to copy reference: {}", e);
            }
            
            ComparisonResult {
                matches: true, // First run always "matches"
                difference_percentage: 0.0,
                max_pixel_diff: 0.0,
                reference_exists: false,
                screenshot_path: current_file,
                diff_path: None,
            }
        }
    }
    
    /// Compare two images and generate diff
    async fn compare_images(&self, current: &Path, reference: &Path, test_name: &str) -> ComparisonResult {
        let current_img = match image::open(current) {
            Ok(img) => img.to_rgba8(),
            Err(e) => {
                println!("❌ Failed to open current image: {}", e);
                return self.error_result(current);
            }
        };
        
        let reference_img = match image::open(reference) {
            Ok(img) => img.to_rgba8(),
            Err(e) => {
                println!("❌ Failed to open reference image: {}", e);
                return self.error_result(current);
            }
        };
        
        // Check dimensions match
        if current_img.dimensions() != reference_img.dimensions() {
            println!("❌ Image dimensions don't match! Current: {:?}, Reference: {:?}", 
                    current_img.dimensions(), reference_img.dimensions());
            return self.error_result(current);
        }
        
        let (width, height) = current_img.dimensions();
        let total_pixels = (width * height) as f32;
        
        // Create diff image
        let mut diff_img = ImageBuffer::new(width, height);
        let mut different_pixels = 0u32;
        let mut max_pixel_diff = 0f32;
        let mut total_diff = 0f32;
        
        // Compare pixel by pixel
        for y in 0..height {
            for x in 0..width {
                let current_pixel = current_img.get_pixel(x, y);
                let reference_pixel = reference_img.get_pixel(x, y);
                
                // Calculate pixel difference (ignoring alpha for now)
                let r_diff = (current_pixel[0] as f32 - reference_pixel[0] as f32).abs();
                let g_diff = (current_pixel[1] as f32 - reference_pixel[1] as f32).abs();
                let b_diff = (current_pixel[2] as f32 - reference_pixel[2] as f32).abs();
                
                let pixel_diff = (r_diff + g_diff + b_diff) / 3.0;
                max_pixel_diff = max_pixel_diff.max(pixel_diff);
                total_diff += pixel_diff;
                
                // Create diff visualization
                let diff_intensity = (pixel_diff / 255.0 * 255.0) as u8;
                if pixel_diff > self.tolerance {
                    different_pixels += 1;
                    // Highlight differences in red
                    diff_img.put_pixel(x, y, Rgba([255, diff_intensity, diff_intensity, 255]));
                } else {
                    // Keep original for similar pixels (dimmed)
                    let dim_factor = 0.3;
                    diff_img.put_pixel(x, y, Rgba([
                        (current_pixel[0] as f32 * dim_factor) as u8,
                        (current_pixel[1] as f32 * dim_factor) as u8,
                        (current_pixel[2] as f32 * dim_factor) as u8,
                        255
                    ]));
                }
            }
        }
        
        let difference_percentage = (different_pixels as f32 / total_pixels) * 100.0;
        let avg_pixel_diff = total_diff / total_pixels;
        let matches = difference_percentage <= (self.tolerance * 100.0);
        
        // Save diff image if there are significant differences
        let diff_path = if different_pixels > 0 {
            let diff_filename = format!("{}_diff.png", test_name);
            let diff_file = self.diff_path.join(&diff_filename);
            
            if let Err(e) = diff_img.save(&diff_file) {
                println!("⚠️ Failed to save diff image: {}", e);
                None
            } else {
                Some(diff_file)
            }
        } else {
            None
        };
        
        // Print comparison results
        if matches {
            println!("✅ Screenshot MATCH: {:.1}% diff (max: {:.1}, avg: {:.1})", 
                    difference_percentage, max_pixel_diff, avg_pixel_diff);
        } else {
            println!("❌ Screenshot DIFF: {:.1}% diff (max: {:.1}, avg: {:.1}) - tolerance: {:.1}%", 
                    difference_percentage, max_pixel_diff, avg_pixel_diff, self.tolerance * 100.0);
            if let Some(ref diff_path) = diff_path {
                println!("   Diff saved: {:?}", diff_path);
            }
        }
        
        ComparisonResult {
            matches,
            difference_percentage,
            max_pixel_diff,
            reference_exists: true,
            screenshot_path: current.to_path_buf(),
            diff_path,
        }
    }
    
    /// Create error result
    fn error_result(&self, current: &Path) -> ComparisonResult {
        ComparisonResult {
            matches: false,
            difference_percentage: 100.0,
            max_pixel_diff: 255.0,
            reference_exists: true,
            screenshot_path: current.to_path_buf(),
            diff_path: None,
        }
    }
    
    /// Generate a test report
    pub fn generate_report(&self, results: &[ComparisonResult]) -> String {
        let total_tests = results.len();
        let passed_tests = results.iter().filter(|r| r.matches).count();
        let failed_tests = total_tests - passed_tests;
        
        let mut report = String::new();
        report.push_str("📊 SCREENSHOT VALIDATION REPORT\n");
        report.push_str("================================\n");
        report.push_str(&format!("Total Tests: {}\n", total_tests));
        report.push_str(&format!("✅ Passed: {}\n", passed_tests));
        report.push_str(&format!("❌ Failed: {}\n", failed_tests));
        report.push_str(&format!("Success Rate: {:.1}%\n\n", 
                                (passed_tests as f32 / total_tests as f32) * 100.0));
        
        // Detailed results
        for (i, result) in results.iter().enumerate() {
            let status = if result.matches { "✅ PASS" } else { "❌ FAIL" };
            report.push_str(&format!("Test {}: {} ({:.1}% diff)\n", 
                                   i + 1, status, result.difference_percentage));
        }
        
        report
    }
    
    /// Clean up old screenshots (keep last N sets)
    pub fn cleanup_old_screenshots(&self, keep_last: usize) {
        // Implementation for cleaning up old test runs
        println!("🧹 Cleaning up old screenshots (keeping last {})", keep_last);
    }
} 