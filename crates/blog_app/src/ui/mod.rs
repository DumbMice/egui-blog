//! UI modules for the blog app.

pub mod components;
pub mod layout;
pub mod markdown;

// Re-exports for convenient access
pub use components::Theme;
pub use layout::LayoutConfig;
pub use markdown::render_markdown;