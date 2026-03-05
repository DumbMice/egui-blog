//! Modular keyboard shortcut system for the blog app.
//!
//! This module provides a configurable keyboard shortcut system with:
//! - TOML configuration file support
//! - Panel-based context awareness
//! - Vim-style key sequences
//! - Focus management and persistence
//!
//! # Configuration
//!
//! Shortcuts are configured via `shortcuts.toml` file. The file is required
//! for shortcuts to work. It should be placed in the project root directory
//! (where Cargo.toml is located).
//!
//! # Usage
//!
//! ```rust
//! use blog_app::shortcuts::{ShortcutManager, ShortcutConfig};
//! use egui::Context;
//!
//! // Create and load shortcuts
//! let mut manager = ShortcutManager::new();
//! manager.load_config().expect("Failed to load shortcuts config");
//!
//! // In UI loop (example with dummy app):
//! # struct DummyApp;
//! # impl blog_app::shortcuts::ActionExecutor for DummyApp {
//! #     fn execute_action(&mut self, _action: &blog_app::shortcuts::ShortcutAction) -> bool { false }
//! #     fn navigate_post(&mut self, _navigation: blog_app::shortcuts::PostNavigation) -> bool { false }
//! #     fn switch_tab(&mut self, _direction: blog_app::shortcuts::TabDirection) -> bool { false }
//! #     fn scroll(&mut self, _direction: blog_app::shortcuts::ScrollDirection, _amount: blog_app::shortcuts::ScrollAmount) -> bool { false }
//! #     fn focus_panel(&mut self, _panel: blog_app::shortcuts::FocusedPanel) -> bool { false }
//! #     fn focus_search(&mut self) -> bool { false }
//! #     fn find_in_content(&mut self) -> bool { false }
//! #     fn find_next(&mut self) -> bool { false }
//! #     fn find_previous(&mut self) -> bool { false }
//! #     fn toggle_theme(&mut self) -> bool { false }
//! #     fn show_help(&mut self) -> bool { false }
//! #     fn browser_address(&mut self) -> bool { false }
//! #     fn execute_custom(&mut self, _action: &str) -> bool { false }
//! # }
//! # impl blog_app::shortcuts::ContextProvider for DummyApp {
//! #     fn focused_panel(&self) -> blog_app::shortcuts::FocusedPanel { blog_app::shortcuts::FocusedPanel::None }
//! #     fn search_has_focus(&self, _ctx: &Context) -> bool { false }
//! #     fn editor_has_focus(&self, _ctx: &Context) -> bool { false }
//! #     fn find_mode_active(&self) -> bool { false }
//! # }
//! # let ctx = Context::default();
//! # let mut app = DummyApp;
//! manager.update_context(&ctx, &mut app);
//! if manager.handle_input(&ctx, &mut app) {
//!     // Shortcut was handled
//! }
//! ```

mod actions;
mod config;
mod context;
mod help;
mod integration;
mod loader;
mod manager;
mod sequences;

pub use actions::ActionExecutor;
pub use config::{
    FocusedPanel, KeySequence, PostNavigation, ScrollAmount, ScrollDirection, ShortcutAction,
    ShortcutConfig, ShortcutContext, ShortcutDefinition, TabDirection,
};
pub use context::{ContextDetector, ContextProvider};
pub use help::HelpOverlay;
pub use integration::{ShortcutIntegration, TestIntegration};
pub use loader::{load_shortcuts_config, ShortcutConfigError};
pub use manager::ShortcutManager;

/// Initialize the shortcut system
pub fn init() -> ShortcutManager {
    ShortcutManager::new()
}
