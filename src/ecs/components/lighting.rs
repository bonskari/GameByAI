//! Lighting components

use macroquad::prelude::*;
use crate::ecs::Component;

/// Light source component for dynamic lighting effects
#[derive(Clone, Debug)]
pub struct LightSource {
    pub color: Color,
    pub intensity: f32,
    pub radius: f32,
    pub light_type: LightSourceType,
    pub enabled: bool,
}

/// Types of light sources for different atmospheric effects
#[derive(Clone, Debug)]
pub enum LightSourceType {
    /// Pulsing warning lights (tech panels)
    Warning { pulse_speed: f32 },
    /// Flowing energy lights (conduits)
    Energy { flow_speed: f32 },
    /// Flickering control lights (control systems)
    Control { flicker_speed: f32 },
    /// Static ambient lighting (hull plating)
    Ambient,
}

impl LightSource {
    /// Create a new light source
    pub fn new(color: Color, intensity: f32, radius: f32, light_type: LightSourceType) -> Self {
        Self {
            color,
            intensity,
            radius,
            light_type,
            enabled: true,
        }
    }

    /// Create a warning light (orange, pulsing)
    pub fn warning(intensity: f32, radius: f32) -> Self {
        Self::new(
            Color::new(1.0, 0.6, 0.2, 1.0), // Orange
            intensity,
            radius,
            LightSourceType::Warning { pulse_speed: 2.0 }
        )
    }

    /// Create an energy light (teal, flowing)
    pub fn energy(intensity: f32, radius: f32) -> Self {
        Self::new(
            Color::new(0.2, 0.8, 0.9, 1.0), // Teal
            intensity,
            radius,
            LightSourceType::Energy { flow_speed: 1.5 }
        )
    }

    /// Create a control light (light blue, flickering)
    pub fn control(intensity: f32, radius: f32) -> Self {
        Self::new(
            Color::new(0.4, 0.9, 1.0, 1.0), // Light blue
            intensity,
            radius,
            LightSourceType::Control { flicker_speed: 0.1 }
        )
    }

    /// Create ambient lighting (dim gray)
    pub fn ambient(intensity: f32, radius: f32) -> Self {
        Self::new(
            Color::new(0.3, 0.3, 0.35, 1.0), // Dim gray
            intensity,
            radius,
            LightSourceType::Ambient
        )
    }

    /// Get the current animated intensity based on time
    pub fn get_animated_intensity(&self, time: f32) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        match &self.light_type {
            LightSourceType::Warning { pulse_speed } => {
                let pulse = (time * pulse_speed).sin() * 0.3 + 0.7;
                self.intensity * pulse
            },
            LightSourceType::Energy { flow_speed } => {
                let flow = (time * flow_speed).sin() * 0.2 + 0.8;
                self.intensity * flow
            },
            LightSourceType::Control { flicker_speed } => {
                let flicker = if (time * 10.0) % 1.0 < *flicker_speed { 0.5 } else { 1.0 };
                self.intensity * flicker
            },
            LightSourceType::Ambient => self.intensity,
        }
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Component for LightSource {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn enable(&mut self) {
        self.enabled = true;
    }

    fn disable(&mut self) {
        self.enabled = false;
    }
}

/// Component that receives lighting from nearby light sources
#[derive(Clone, Debug)]
pub struct LightReceiver {
    pub current_lighting: Color,
    pub ambient_color: Color,
    pub enabled: bool,
}

impl LightReceiver {
    pub fn new() -> Self {
        Self {
            current_lighting: Color::new(0.15, 0.15, 0.2, 1.0), // Dark blue ambient
            ambient_color: Color::new(0.15, 0.15, 0.2, 1.0),
            enabled: true,
        }
    }

    pub fn with_ambient(mut self, ambient: Color) -> Self {
        self.ambient_color = ambient;
        self.current_lighting = ambient;
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Update the current lighting color
    pub fn update_lighting(&mut self, lighting: Color) {
        if self.enabled {
            self.current_lighting = lighting;
        }
    }
}

impl Default for LightReceiver {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for LightReceiver {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn enable(&mut self) {
        self.enabled = true;
    }

    fn disable(&mut self) {
        self.enabled = false;
    }
}

/// Lighting test component
#[derive(Debug, Clone)]
pub struct LightingTest {
    pub start_time: std::time::Instant,
    pub current_phase: usize,
    pub phase_start_time: std::time::Instant,
    pub phases: Vec<LightingTestPhase>,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct LightingTestPhase {
    pub name: String,
    pub light_count: usize,
    pub duration_seconds: f32,
    pub background_color: Color,
}

impl LightingTest {
    pub fn new() -> Self {
        let phases = vec![
            LightingTestPhase {
                name: "Baseline (No Lights)".to_string(),
                light_count: 0,
                duration_seconds: 3.0,
                background_color: Color::new(0.1, 0.1, 0.2, 1.0), // Dark blue
            },
            LightingTestPhase {
                name: "Single Light".to_string(),
                light_count: 1,
                duration_seconds: 3.0,
                background_color: Color::new(0.2, 0.1, 0.1, 1.0), // Dark red
            },
            LightingTestPhase {
                name: "Few Lights".to_string(),
                light_count: 8,
                duration_seconds: 3.0,
                background_color: Color::new(0.1, 0.2, 0.1, 1.0), // Dark green
            },
            LightingTestPhase {
                name: "Many Lights".to_string(),
                light_count: 50,
                duration_seconds: 3.0,
                background_color: Color::new(0.2, 0.2, 0.1, 1.0), // Dark yellow
            },
            LightingTestPhase {
                name: "Stress Test".to_string(),
                light_count: 100,
                duration_seconds: 3.0,
                background_color: Color::new(0.2, 0.1, 0.2, 1.0), // Dark purple
            },
        ];

        Self {
            start_time: std::time::Instant::now(),
            current_phase: 0,
            phase_start_time: std::time::Instant::now(),
            phases,
            enabled: true,
        }
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_current_phase(&self) -> Option<&LightingTestPhase> {
        self.phases.get(self.current_phase)
    }

    pub fn get_progress(&self) -> (usize, usize, f32) {
        let total_phases = self.phases.len();
        let progress = if total_phases > 0 {
            (self.current_phase + 1) as f32 / total_phases as f32
        } else {
            1.0
        };
        (self.current_phase + 1, total_phases, progress)
    }

    pub fn is_finished(&self) -> bool {
        self.current_phase >= self.phases.len()
    }

    pub fn get_phase_elapsed_time(&self) -> f32 {
        self.phase_start_time.elapsed().as_secs_f32()
    }

    pub fn should_advance_phase(&self) -> bool {
        if let Some(phase) = self.get_current_phase() {
            self.get_phase_elapsed_time() >= phase.duration_seconds
        } else {
            false
        }
    }

    pub fn advance_phase(&mut self) {
        if self.current_phase < self.phases.len() {
            self.current_phase += 1;
            self.phase_start_time = std::time::Instant::now();
        }
    }

    pub fn get_total_elapsed_time(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }
}

impl Component for LightingTest {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn enable(&mut self) {
        self.enabled = true;
    }

    fn disable(&mut self) {
        self.enabled = false;
    }
} 