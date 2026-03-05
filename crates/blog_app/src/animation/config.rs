//! Configuration parameters for panel focus animation.
//!
//! These parameters are adjustable via the debug configuration window.

/// Configuration parameters for panel focus animation.
#[derive(Debug, Clone, Copy)]
pub struct FocusAnimationConfig {
    /// Overall intensity multiplier for animation effect (0.0 to 1.0).
    pub intensity: f32,
    /// Duration of the flash phase in milliseconds.
    pub flash_duration_ms: u32,
    /// Maximum opacity during flash phase (0.0 to 1.0).
    pub flash_max_opacity: f32,
    /// Thickness of flash border in points.
    pub border_thickness: f32,
}

impl Default for FocusAnimationConfig {
    fn default() -> Self {
        Self {
            intensity: 0.7,
            flash_duration_ms: 100, // Very short - just enough to see
            flash_max_opacity: 0.9,
            border_thickness: 3.0,
        }
    }
}

impl FocusAnimationConfig {
    /// Validate configuration parameters to ensure they are within reasonable bounds.
    ///
    /// # Returns
    /// `true` if all parameters are valid, `false` otherwise.
    #[expect(dead_code)]
    pub fn validate(&self) -> bool {
        self.intensity >= 0.0
            && self.intensity <= 1.0
            && self.flash_duration_ms >= 50
            && self.flash_duration_ms <= 300 // Shorter max duration
            && self.flash_max_opacity >= 0.1
            && self.flash_max_opacity <= 1.0
            && self.border_thickness >= 1.0
            && self.border_thickness <= 6.0
    }

    /// Get the flash duration in seconds.
    pub fn flash_duration(&self) -> f64 {
        self.flash_duration_ms as f64 / 1000.0
    }
}
