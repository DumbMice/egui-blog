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
//! # use blog_app::animation::{FocusAnimationState, FocusRenderer, FocusAnimationConfig};
//! # use egui::{Rect, Ui};
//! #
//! # struct BlogApp {
//! #     focus_animation: FocusAnimationState,
//! # }
//! #
//! # impl BlogApp {
//! #     fn ui(&mut self, ui: &mut Ui) {
//! #         let current_time = ui.ctx().input(|i| i.time);
//! #         let config = FocusAnimationConfig::default();
//! #         self.focus_animation.update(current_time, &config);
//! #         
//! #         // Example usage:
//! #         let panel_rect = Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(100.0, 100.0));
//! #         let is_focused = true;
//! #         
//! #         FocusRenderer::draw_focus_indicator(
//! #             ui.painter(),
//! #             panel_rect,
//! #             is_focused,
//! #             &self.focus_animation,
//! #             &config,
//! #             current_time,
//! #             ui,
//! #         );
//! #     }
//! # }
//! ```

pub mod config;
pub mod renderer;
pub mod state;

pub use config::FocusAnimationConfig;
pub use renderer::FocusRenderer;
pub use state::FocusAnimationState;
