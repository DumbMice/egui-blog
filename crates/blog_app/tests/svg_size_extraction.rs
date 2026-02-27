//! Test SVG size extraction functionality

use blog_app::math::{self, MathAssetManager};

#[test]
fn test_svg_size_extraction() {
    let asset_manager = MathAssetManager::default();
    let manifest = math::load_manifest();

    // Test a known formula
    let formula = "E = m c^2";
    let is_display = false;

    let hash = manifest
        .find_hash(formula, is_display)
        .expect("Should find hash");

    println!("Testing SVG size extraction for formula: {}", formula);
    println!("Hash: {}", hash);

    // Get SVG size
    let svg_size = asset_manager
        .get_svg_size(hash)
        .expect("Should get SVG size");

    println!("  SVG size: {}x{}", svg_size.x, svg_size.y);

    // Verify size is reasonable (not zero, not huge)
    assert!(svg_size.x > 0.0, "SVG width should be > 0");
    assert!(svg_size.y > 0.0, "SVG height should be > 0");
    assert!(svg_size.x < 1000.0, "SVG width should be reasonable");
    assert!(svg_size.y < 1000.0, "SVG height should be reasonable");

    // Check aspect ratio is reasonable for math formulas
    let aspect_ratio = svg_size.x / svg_size.y;
    println!("  Aspect ratio: {:.2}", aspect_ratio);
    assert!(
        aspect_ratio > 0.5,
        "Math formulas should have reasonable width"
    );
    assert!(
        aspect_ratio < 10.0,
        "Math formulas should have reasonable width"
    );

    println!("  ✓ SVG size extracted correctly");
}

#[test]
fn test_svg_size_for_formula() {
    let asset_manager = MathAssetManager::default();

    // Test getting size for formula directly
    let formula = "a^2 + b^2 = c^2";
    let is_display = false;

    println!("Testing SVG size extraction for formula: {}", formula);

    let svg_size = asset_manager
        .get_svg_size_for_formula(formula, is_display)
        .expect("Should get SVG size");

    println!("  SVG size: {}x{}", svg_size.x, svg_size.y);

    // Verify size is reasonable
    assert!(svg_size.x > 0.0, "SVG width should be > 0");
    assert!(svg_size.y > 0.0, "SVG height should be > 0");

    println!("  ✓ SVG size extracted correctly for formula");
}

#[test]
fn test_multiple_svg_sizes() {
    let asset_manager = MathAssetManager::default();
    let manifest = math::load_manifest();

    println!("Testing SVG sizes for all formulas in manifest:");

    let mut total_formulas = 0;
    let mut formulas_with_size = 0;

    for (hash, metadata) in &manifest.formulas {
        total_formulas += 1;

        if let Some(svg_size) = asset_manager.get_svg_size(hash) {
            formulas_with_size += 1;

            println!(
                "  {}: {}x{} (aspect: {:.2})",
                &metadata.formula[..metadata.formula.len().min(30)],
                svg_size.x,
                svg_size.y,
                svg_size.x / svg_size.y
            );

            // Basic sanity checks
            assert!(svg_size.x > 0.0, "SVG width should be > 0");
            assert!(svg_size.y > 0.0, "SVG height should be > 0");
        } else {
            println!(
                "  {}: Could not extract size",
                &metadata.formula[..metadata.formula.len().min(30)]
            );
        }
    }

    println!("Total formulas: {}", total_formulas);
    println!("Formulas with size extracted: {}", formulas_with_size);

    // Most formulas should have size information
    let success_rate = formulas_with_size as f32 / total_formulas as f32;
    println!("Success rate: {:.1}%", success_rate * 100.0);

    assert!(
        success_rate > 0.5,
        "Should be able to extract size for most formulas (got {:.1}%)",
        success_rate * 100.0
    );

    println!("  ✓ SVG size extraction works for most formulas");
}

// Note: The extract_svg_size method is private, so we can't test it directly.
// Instead, we test through the public API which uses it internally.
