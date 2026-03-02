//! Embedded math assets for the blog application.
//!
//! This module provides access to pre-rendered math formula SVGs.
//! The SVGs are embedded at compile time using procedural macros.

use log;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Metadata for a rendered formula
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormulaMetadata {
    /// The original formula text
    pub formula: String,
    /// Whether this is display math (true) or inline math (false)
    pub is_display: bool,
    /// When this formula was last rendered (ISO 8601)
    pub rendered_at: String,
    /// Size of the SVG file in bytes
    pub svg_size: usize,
    /// The hash used as filename
    pub hash: String,
    /// Whether this is a placeholder SVG (true) or actual Typst rendering (false)
    pub is_placeholder: bool,
    /// Whether this SVG has been processed for theme adaptation (true)
    #[serde(default = "default_theme_processed")]
    pub theme_processed: bool,
}

fn default_theme_processed() -> bool {
    true
}

/// Manifest tracking all rendered formulas
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MathManifest {
    /// Map from formula hash to metadata
    pub formulas: HashMap<String, FormulaMetadata>,
    /// When this manifest was last updated
    pub updated_at: String,
    /// Reverse index for O(1) lookup: (`formula`, `is_display`) -> hash
    #[serde(skip)]
    reverse_index: HashMap<(String, bool), String>,
}

impl MathManifest {
    /// Build or rebuild the reverse index from formulas
    fn build_reverse_index(&mut self) {
        self.reverse_index.clear();
        // Sort by hash for deterministic iteration
        let mut entries: Vec<_> = self.formulas.iter().collect();
        entries.sort_by_key(|(hash, _)| *hash);
        for (hash, metadata) in entries {
            self.reverse_index.insert(
                (metadata.formula.clone(), metadata.is_display),
                hash.clone(),
            );
        }
    }

    /// Find the hash for a given formula text and display type
    pub fn find_hash(&self, formula: &str, is_display: bool) -> Option<&str> {
        self.reverse_index
            .get(&(formula.to_owned(), is_display))
            .map(String::as_str)
    }

    /// Get all formula hashes
    pub fn all_hashes(&self) -> Vec<&str> {
        self.formulas.keys().map(String::as_str).collect()
    }

    /// Get metadata for a hash
    pub fn get_metadata(&self, hash: &str) -> Option<&FormulaMetadata> {
        self.formulas.get(hash)
    }
}

static MANIFEST_CACHE: OnceLock<MathManifest> = OnceLock::new();

/// Load the embedded manifest
///
/// The manifest is stored as a JSON file in `assets/math/manifest.json`
/// and embedded at compile time using `include_str!`.
/// Uses `OnceLock` for thread-safe, one-time initialization.
pub fn load_manifest() -> &'static MathManifest {
    MANIFEST_CACHE.get_or_init(|| {
        let manifest_bytes = include_bytes!("../../assets/math/manifest.json");
        let mut manifest: MathManifest =
            serde_json::from_slice(manifest_bytes).unwrap_or_else(|e| {
                log::error!("Failed to parse embedded math manifest: {e}");
                MathManifest::default()
            });
        manifest.build_reverse_index();
        manifest
    })
}

/// Get SVG bytes for a formula hash
///
/// Uses the `embed_file_map!` macro to embed SVG files from `assets/math/`
/// at compile time and create a lookup map from hash to SVG bytes.
pub fn get_svg_bytes(hash: &str) -> Option<&'static [u8]> {
    use blog_macros::embed_file_map;

    // Create a closure that maps hash to SVG bytes
    // The macro scans the directory at compile time and generates a match statement
    // Path is relative from src/math/embedded.rs to assets/math/
    let get_bytes = embed_file_map!("../../assets/math/", pattern = "*.svg");

    // Call the generated closure
    get_bytes(hash)
}
