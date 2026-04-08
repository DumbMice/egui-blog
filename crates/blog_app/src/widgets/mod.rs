//! Embeddable UI widgets for blog posts.
//!
//! Architecture:
//! - Build-time: Rust build script compiles widget files, creates registry
//! - Runtime: Load embedded widget registry, instantiate widgets on demand
//! - Markdown: Widgets referenced like `![Widget](widget.rs?config=...)`

mod embedded;
pub mod examples;
mod registry;

#[cfg(test)]
mod test_integration;

pub use embedded::{load_manifest, WidgetManifest};
pub use examples::{ChartWidget, CounterWidget, EguiPlotWidgetSimple};
pub use registry::{global_registry, WidgetInstance, WidgetRegistry};

/// Trait for embeddable UI widgets
pub trait Widget: Send + Sync {
    /// Unique identifier for this widget type
    fn name(&self) -> &'static str;

    /// Display name for UI
    fn display_name(&self) -> &'static str;

    /// Render the widget with given configuration
    /// Returns the size of the rendered widget
    fn render(&mut self, ui: &mut egui::Ui, config: &WidgetConfig) -> egui::Vec2;

    /// Handle any state updates (called each frame)
    fn update(&mut self, ctx: &egui::Context);

    /// Serialize widget state to JSON
    fn serialize_state(&self) -> serde_json::Value;

    /// Deserialize widget state from JSON
    fn deserialize_state(&mut self, state: serde_json::Value);
}

/// Configuration for widget rendering
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WidgetConfig {
    /// Widget-specific configuration as JSON
    pub config: serde_json::Value,
    /// Width constraint (None for auto)
    pub width: Option<f32>,
    /// Height constraint (None for auto)
    pub height: Option<f32>,
    /// Whether widget is interactive
    pub interactive: bool,
}

impl Default for WidgetConfig {
    fn default() -> Self {
        Self {
            config: serde_json::json!({}),
            width: None,
            height: None,
            interactive: true,
        }
    }
}

/// Error type for widget operations
#[derive(Debug, thiserror::Error)]
pub enum WidgetError {
    #[error("Widget not found: {0}")]
    NotFound(String),

    #[error("Failed to instantiate widget: {0}")]
    Instantiation(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
