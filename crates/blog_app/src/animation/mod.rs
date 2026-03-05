//! Panel focus animation system.
//!
//! Provides animated visual feedback for focused panels, replacing the static
//! blue border with a more polished animation featuring:
//! 1. Quick flash on focus change
//! 2. Sustained gentle pulse while focused
//! 3. Adjustable parameters via debug configuration
//!
//! # Usage
//!
//! ```rust
//! use crate::animation::{FocusAnimationState, FocusRenderer, FocusAnimationConfig};
//!
//! // In your app struct:
//! struct BlogApp {
//!     focus_animation: FocusAnimationState,
//!     // ...
//! }
//!
//! // In your UI loop:
//! fn ui(&mut self, ui: &mut egui::Ui) {
//!     let current_time = ui.ctx().input(|i| i.time);
//!     
//!     // Update animation state
//!     self.focus_animation.update(current_time, &config);
//!     
//!     // Draw focus indicator for a panel
//!     let renderer = FocusRenderer;
//!     renderer.draw_focus_indicator(
//!         ui.painter(),
//!         panel_rect,
//!         is_focused,
//!         &self.focus_animation,
//!         &config,
//!         current_time,
//!         accent_color,
//!     );
//! }
//! ```

pub mod config;
pub mod renderer;
pub mod state;

pub use config::FocusAnimationConfig;
pub use renderer::FocusRenderer;
pub use state::FocusAnimationState;
