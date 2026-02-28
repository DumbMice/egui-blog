//! Math rendering module for Typst math formulas.
//! This module handles loading and displaying pre-generated math formula SVGs.
//!
//! Architecture:
//! - Build-time: Rust build script (build.rs) extracts formulas, generates SVGs, creates manifest
//! - Runtime: Load embedded manifest and SVGs, display textures with fallback

mod assets;
mod embedded;

pub use assets::MathAssetManager;
pub use embedded::{FormulaMetadata, MathManifest, get_svg_bytes, load_manifest};

/// Simple formula detection without regex
/// Returns vector of (start_index, end_index, formula_text, is_display_math)
pub fn find_formulas(text: &str) -> Vec<(usize, usize, String, bool)> {
    let mut formulas = Vec::new();
    let mut chars = text.char_indices().peekable();

    while let Some((i, ch)) = chars.next() {
        if ch == '$' {
            // Check if it's display math ($$)
            let is_display = chars
                .peek()
                .map(|(_, next_ch)| *next_ch == '$')
                .unwrap_or(false);
            let start = i;

            // Skip second $ if display math
            if is_display {
                chars.next(); // Skip the second $
            }

            // Find closing $ or $$
            let mut found = false;
            let mut escaped = false;
            let formula_start = chars.peek().map(|(pos, _)| *pos);

            while let Some((j, ch)) = chars.next() {
                if ch == '\\' && !escaped {
                    escaped = true;
                    continue;
                }

                if ch == '$' && !escaped {
                    // Check if we need another $ for display math
                    if is_display {
                        if let Some((_, next_ch)) = chars.peek() {
                            if *next_ch == '$' {
                                chars.next(); // Skip the second $
                                found = true;
                                let formula_text = if let Some(formula_start_idx) = formula_start {
                                    text[formula_start_idx..j].trim().to_string()
                                } else {
                                    String::new()
                                };
                                formulas.push((start, j + 2, formula_text, true)); // +2 for both $$
                                break;
                            }
                        }
                    } else {
                        found = true;
                        let formula_text = if let Some(formula_start_idx) = formula_start {
                            text[formula_start_idx..j].trim().to_string()
                        } else {
                            String::new()
                        };
                        formulas.push((start, j + 1, formula_text, false)); // +1 for single $
                        break;
                    }
                }

                escaped = false;
            }

            // If we didn't find a closing $, don't add it as a formula
            if !found {
                // Backtrack to continue searching from after the opening $
                let mut backtrack = text[start + (if is_display { 2 } else { 1 })..].char_indices();
                while let Some((_, _ch)) = backtrack.next() {
                    chars.next(); // Advance the iterator
                }
            }
        }
    }

    formulas
}

/// Extract formula text from the original string
pub fn extract_formula_text(text: &str, start: usize, end: usize, is_display: bool) -> String {
    let skip = if is_display { 2 } else { 1 };
    let formula_start = start + skip;

    if formula_start >= end || end > text.len() {
        return String::new();
    }

    let formula_end = end - skip;

    if formula_start < formula_end {
        text[formula_start..formula_end].trim().to_string()
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_formulas_simple() {
        let text = "Text $E=mc^2$ and $$\\sum_{i=1}^n i$$ here";
        let formulas = find_formulas(text);
        assert_eq!(formulas.len(), 2);

        // First formula: $E=mc^2$
        assert_eq!(formulas[0], (5, 13, "E=mc^2".to_string(), false));

        // Second formula: $$\sum_{i=1}^n i$$
        assert_eq!(formulas[1], (18, 36, "\\sum_{i=1}^n i".to_string(), true));
    }

    #[test]
    fn test_find_formulas_edge_cases() {
        // Escaped dollars
        assert_eq!(find_formulas(r"\$not math\$"), vec![]);

        // Unbalanced dollars
        assert_eq!(find_formulas("$unbalanced"), vec![]);

        // Empty formula (should not be detected)
        assert_eq!(find_formulas("$$"), vec![]);

        // Multiple inline
        let formulas = find_formulas("$a$ $b$ $c$");
        assert_eq!(formulas.len(), 3);
        assert_eq!(formulas[0], (0, 3, "a".to_string(), false));
        assert_eq!(formulas[1], (4, 7, "b".to_string(), false));
        assert_eq!(formulas[2], (8, 11, "c".to_string(), false));

        // Mixed display and inline
        let text = "Inline $x$ and display $$y$$";
        let formulas = find_formulas(text);
        assert_eq!(formulas.len(), 2);
        assert_eq!(formulas[0], (7, 10, "x".to_string(), false)); // $x$
        assert_eq!(formulas[1], (23, 28, "y".to_string(), true)); // $$y$$
    }

    #[test]
    fn test_extract_formula_text() {
        let text = "Test $formula$ end";
        assert_eq!(extract_formula_text(text, 5, 14, false), "formula");

        let text = "Test $$display$$ end";
        assert_eq!(extract_formula_text(text, 5, 16, true), "display");

        // With whitespace
        let text = "Test $  formula  $ end";
        assert_eq!(extract_formula_text(text, 5, 18, false), "formula");

        // Empty
        assert_eq!(extract_formula_text("", 0, 0, false), "");
    }

    #[test]
    #[ignore = "requires build-generated embedded.rs with formulas"]
    fn test_embedded_module_integration() {
        // Test that the embedded module is properly integrated
        use crate::math::embedded;

        // Load manifest
        let manifest = embedded::load_manifest();
        assert!(
            !manifest.formulas.is_empty(),
            "Manifest should contain formulas"
        );

        // Test find_hash for a known formula
        let formula = "E = m c^2";
        let hash = manifest.find_hash(formula, false);
        assert!(hash.is_some(), "Should find hash for formula: {}", formula);

        // Test get_svg_bytes
        if let Some(hash) = hash {
            let svg_bytes = embedded::get_svg_bytes(hash);
            assert!(
                svg_bytes.is_some(),
                "Should get SVG bytes for hash: {}",
                hash
            );

            // Verify it's valid data
            if let Some(bytes) = svg_bytes {
                assert!(!bytes.is_empty(), "SVG bytes should not be empty");

                // Check if it looks like SVG (or placeholder)
                let is_svg = bytes.starts_with(b"<?xml")
                    || bytes.starts_with(b"<svg")
                    || String::from_utf8_lossy(bytes).contains("<svg");
                assert!(is_svg, "Bytes should be SVG data");
            }
        }

        // Test all_hashes
        let all_hashes = manifest.all_hashes();
        assert_eq!(all_hashes.len(), manifest.formulas.len());

        // Test get_metadata
        if let Some(first_hash) = all_hashes.first() {
            let metadata = manifest.get_metadata(first_hash);
            assert!(metadata.is_some(), "Should get metadata for hash");

            if let Some(meta) = metadata {
                assert!(!meta.formula.is_empty(), "Formula should not be empty");
                assert!(!meta.hash.is_empty(), "Hash should not be empty");
                assert!(
                    !meta.rendered_at.is_empty(),
                    "Rendered_at should not be empty"
                );
            }
        }
    }
}
