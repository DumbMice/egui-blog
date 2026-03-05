//! Animation state machine for panel focus animation.
//!
//! Tracks the current animation phase and timing for focused panels.

use crate::shortcuts::FocusedPanel;

/// Current phase of the focus animation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationPhase {
    /// No panel is focused or animation is idle.
    Idle,
    /// Quick bright flash phase (immediately after focus change).
    Flash,
}

/// State machine for panel focus animation.
///
/// Tracks which panel is focused, when focus changed, and current animation phase.
#[derive(Debug, Clone)]
pub struct FocusAnimationState {
    /// Currently focused panel (or `FocusedPanel::None` if no panel focused).
    pub focused_panel: FocusedPanel,
    /// Time when focus last changed (in seconds from app start).
    pub focus_change_time: f64,
    /// Current animation phase.
    pub current_phase: AnimationPhase,
    /// Time spent in current phase (in seconds).
    pub time_in_phase: f64,
}

impl Default for FocusAnimationState {
    fn default() -> Self {
        Self {
            focused_panel: FocusedPanel::None,
            focus_change_time: 0.0,
            current_phase: AnimationPhase::Idle,
            time_in_phase: 0.0,
        }
    }
}

impl FocusAnimationState {
    /// Create a new animation state with no panel focused.
    pub fn new() -> Self {
        Self::default()
    }

    /// Update animation state based on current time and configuration.
    ///
    /// # Arguments
    /// * `current_time` - Current time in seconds from app start.
    /// * `config` - Animation configuration parameters.
    pub fn update(&mut self, current_time: f64, config: &super::config::FocusAnimationConfig) {
        if self.focused_panel == FocusedPanel::None {
            self.current_phase = AnimationPhase::Idle;
            self.time_in_phase = 0.0;
            return;
        }

        let elapsed = current_time - self.focus_change_time;

        // Determine current phase based on elapsed time
        let new_phase = if elapsed < config.flash_duration() {
            AnimationPhase::Flash
        } else {
            AnimationPhase::Idle
        };

        // Update phase and timing
        if new_phase != self.current_phase {
            self.current_phase = new_phase;
            self.time_in_phase = 0.0;
        } else {
            self.time_in_phase += 0.016; // Assume ~60fps, will be refined by actual delta
        }
    }

    /// Handle focus change to a new panel.
    ///
    /// # Arguments
    /// * `panel` - Newly focused panel.
    /// * `current_time` - Current time in seconds from app start.
    pub fn on_focus_change(&mut self, panel: FocusedPanel, current_time: f64) {
        if self.focused_panel != panel {
            self.focused_panel = panel;
            self.focus_change_time = current_time;
            self.current_phase = AnimationPhase::Flash;
            self.time_in_phase = 0.0;
        }
    }

    /// Calculate background and border opacities based on current animation state.
    ///
    /// # Arguments
    /// * `config` - Animation configuration parameters.
    /// * `current_time` - Current time in seconds from app start.
    ///
    /// # Returns
    /// Tuple of `(background_opacity, border_opacity)` where opacities are in range 0.0 to 1.0.
    pub fn calculate_opacities(
        &self,
        config: &super::config::FocusAnimationConfig,
        current_time: f64,
    ) -> (f32, f32) {
        if self.focused_panel == FocusedPanel::None {
            return (0.0, 0.0);
        }

        let elapsed = current_time - self.focus_change_time;

        match self.current_phase {
            AnimationPhase::Flash => {
                // Linear fade from flash_max_opacity to 0
                let progress = elapsed / config.flash_duration();
                let opacity = config.flash_max_opacity * (1.0 - progress as f32);
                let scaled_opacity = opacity * config.intensity;
                (0.0, scaled_opacity) // Only border, no background tint
            }

            AnimationPhase::Idle => (0.0, 0.0),
        }
    }

    /// Check if animation is currently active (not idle).
    #[expect(dead_code)]
    pub fn is_active(&self) -> bool {
        self.focused_panel != FocusedPanel::None
    }
}
