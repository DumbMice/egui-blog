//! UI modules for the blog app.

pub mod components;
pub mod layout;
pub mod markdown;
pub mod table_renderer;
pub mod responsive;

// Re-exports for convenient access
pub use components::Theme;
pub use layout::LayoutConfig;
pub use responsive::ResponsiveConfig;
