//! Asset management for math SVGs.

use egui::ImageSource;

/// Manages math SVG assets
pub struct MathAssetManager {
    /// Manifest of available formulas
    manifest: crate::math::embedded::MathManifest,
}

impl Default for MathAssetManager {
    fn default() -> Self {
        Self {
            manifest: crate::math::embedded::load_manifest(),
        }
    }
}

impl MathAssetManager {
    /// Create a new asset manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Get an `ImageSource` for a math formula hash
    pub fn get_image_source_for_hash(hash: &str) -> Option<ImageSource<'static>> {
        // Get SVG bytes from embedded assets
        let svg_bytes = crate::math::embedded::get_svg_bytes(hash)?;

        // Create ImageSource from bytes (similar to include_image! macro)
        // Use hash as part of URI for unique identification
        let uri = format!("bytes://math/{hash}.svg");

        Some(ImageSource::Bytes {
            uri: std::borrow::Cow::Owned(uri),
            bytes: egui::load::Bytes::Static(svg_bytes),
        })
    }

    /// Get the intrinsic size of an SVG from its bytes
    pub fn get_svg_size(hash: &str) -> Option<egui::Vec2> {
        // Get SVG bytes from embedded assets
        let svg_bytes = crate::math::embedded::get_svg_bytes(hash)?;

        // Parse SVG to extract size
        Self::extract_svg_size(svg_bytes)
    }

    /// Get the intrinsic size of an SVG for a formula
    pub fn get_svg_size_for_formula(&self, formula: &str, is_display: bool) -> Option<egui::Vec2> {
        // Find the hash for this formula
        let hash = self.manifest.find_hash(formula, is_display)?.to_owned();
        Self::get_svg_size(&hash)
    }

    /// Extract size from SVG bytes
    fn extract_svg_size(svg_bytes: &[u8]) -> Option<egui::Vec2> {
        let svg_str = std::str::from_utf8(svg_bytes).ok()?;

        // Try to parse viewBox first (most reliable)
        if let Some(viewbox) = Self::parse_viewbox(svg_str) {
            return Some(viewbox);
        }

        // Fall back to width/height attributes
        if let Some((width, height)) = Self::parse_width_height(svg_str) {
            return Some(egui::Vec2::new(width, height));
        }

        // Log when size extraction fails
        log::warn!("Could not extract SVG size - no viewBox or width/height attributes found");

        None
    }

    /// Parse viewBox attribute: "0 0 width height"
    fn parse_viewbox(svg_str: &str) -> Option<egui::Vec2> {
        let viewbox_start = svg_str.find("viewBox=\"")?;
        let viewbox_content_start = viewbox_start + "viewBox=\"".len();
        let viewbox_content_end = svg_str[viewbox_content_start..].find('"')?;
        let viewbox_str =
            &svg_str[viewbox_content_start..viewbox_content_start + viewbox_content_end];

        let parts: Vec<&str> = viewbox_str.split_whitespace().collect();
        if parts.len() >= 4 {
            let width = parts[2].parse::<f32>().ok()?;
            let height = parts[3].parse::<f32>().ok()?;
            Some(egui::Vec2::new(width, height))
        } else {
            None
        }
    }

    /// Parse width and height attributes
    fn parse_width_height(svg_str: &str) -> Option<(f32, f32)> {
        let width = Self::parse_svg_dimension(svg_str, "width")?;
        let height = Self::parse_svg_dimension(svg_str, "height")?;
        Some((width, height))
    }

    /// Parse a dimension attribute (width or height)
    fn parse_svg_dimension(svg_str: &str, attr: &str) -> Option<f32> {
        let pattern = format!("{attr}=\"");
        let start = svg_str.find(&pattern)? + pattern.len();
        let end = svg_str[start..].find('"')?;
        let dim_str = &svg_str[start..start + end];

        // Parse number, removing units like pt, px, etc.
        let number_str = dim_str
            .replace("pt", "")
            .replace("px", "")
            .replace("em", "")
            .replace("rem", "")
            .replace("in", "")
            .replace("cm", "")
            .replace("mm", "");

        number_str.parse::<f32>().ok()
    }

    /// Get an `ImageSource` for a math formula text
    pub fn get_image_source_for_formula(
        &self,
        formula: &str,
        is_display: bool,
    ) -> Option<ImageSource<'static>> {
        // Find the hash for this formula
        let hash = self.manifest.find_hash(formula, is_display)?.to_owned();
        Self::get_image_source_for_hash(&hash)
    }

    /// Clear any cached textures (no-op in new implementation)
    pub fn clear_cache() {
        // No texture cache to clear - egui handles caching internally
    }

    /// Get current cache size (always 0 in new implementation)
    pub fn cache_size() -> usize {
        0 // egui handles caching internally
    }

    /// Set maximum cache size (no-op in new implementation)
    pub fn set_max_cache_size(_size: usize) {
        // egui handles caching internally
    }
}
