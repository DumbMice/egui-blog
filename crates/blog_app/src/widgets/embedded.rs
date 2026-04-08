//! Embedded widget registry loaded at compile time.

use chrono;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metadata for a compiled widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetMetadata {
    /// Unique widget identifier (e.g., "counter", "plot")
    pub name: String,
    /// Display name for UI
    pub display_name: String,
    /// Path to widget source file relative to posts directory
    pub source_path: String,
    /// Compiled WASM module hash (for dynamic loading)
    pub module_hash: String,
    /// Widget dependencies
    pub dependencies: Vec<String>,
    /// Whether widget supports persistence
    pub persistent: bool,
}

/// Manifest of all available widgets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetManifest {
    /// Map of widget name to metadata
    pub widgets: HashMap<String, WidgetMetadata>,
    /// Build timestamp
    pub built_at: String,
}

impl WidgetManifest {
    /// Create empty manifest
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            built_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Add a widget to the manifest
    pub fn add_widget(&mut self, metadata: WidgetMetadata) {
        self.widgets.insert(metadata.name.clone(), metadata);
    }

    /// Get widget metadata by name
    pub fn get_widget(&self, name: &str) -> Option<&WidgetMetadata> {
        self.widgets.get(name)
    }

    /// Get all widget names
    pub fn all_widgets(&self) -> Vec<&str> {
        self.widgets.keys().map(|k| k.as_str()).collect()
    }

    /// Check if widget exists
    pub fn has_widget(&self, name: &str) -> bool {
        self.widgets.contains_key(name)
    }
}

/// Load the embedded widget manifest
pub fn load_manifest() -> &'static WidgetManifest {
    // Parse the embedded JSON
    static MANIFEST: std::sync::OnceLock<WidgetManifest> = std::sync::OnceLock::new();

    MANIFEST.get_or_init(|| {
        let manifest_bytes = include_bytes!("../../assets/widgets/manifest.json");
        let manifest_str =
            std::str::from_utf8(manifest_bytes).expect("Widget manifest is not valid UTF-8");

        serde_json::from_str(manifest_str).expect("Failed to parse widget manifest JSON")
    })
}

/// Get embedded widget source code
#[allow(dead_code)]
pub fn get_widget_source(name: &str) -> Option<&'static [u8]> {
    use blog_macros::embed_file_map;

    // Create a closure that maps path to source code
    // The macro scans the directory at compile time and generates a match statement
    // Path is relative from src/widgets/embedded.rs to assets/widgets/
    let get_source = embed_file_map!("../../assets/widgets/", pattern = "*.rs");

    let path = format!("{}.rs", name);
    get_source(&path)
}

/// Get compiled widget module (for dynamic loading)
#[allow(dead_code)]
pub fn get_widget_module(name: &str) -> Option<&'static [u8]> {
    use blog_macros::embed_file_map;

    // Create a closure that maps path to WASM module
    // The macro scans the directory at compile time and generates a match statement
    // Path is relative from src/widgets/embedded.rs to assets/widgets/
    let get_module = embed_file_map!("../../assets/widgets/", pattern = "*.wasm");

    let path = format!("{}.wasm", name);
    get_module(&path)
}
