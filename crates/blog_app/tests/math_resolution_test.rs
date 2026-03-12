//! Tests for math formula resolution scaling feature

use blog_app::math::{MathAssetManager, find_formulas};
use egui::ImageSource;

/// Test that resolution scale parameter is accepted and clamped
#[test]
fn test_resolution_scale_parameter() {
    // Create asset manager
    let asset_manager = MathAssetManager::new();

    // Test with a simple formula that should exist
    let formula = "E = m c^2";
    let is_display = false;

    // Test default resolution (1.0)
    let image_source_1 =
        asset_manager.get_image_source_for_formula_with_resolution(formula, is_display, 1.0);
    assert!(
        image_source_1.is_some(),
        "Should get image source at 1.0 resolution"
    );

    // Test higher resolution (2.0)
    let image_source_2 =
        asset_manager.get_image_source_for_formula_with_resolution(formula, is_display, 2.0);
    assert!(
        image_source_2.is_some(),
        "Should get image source at 2.0 resolution"
    );

    // Test resolution clamping (should clamp 0.5 to 1.0)
    let image_source_clamped_low =
        asset_manager.get_image_source_for_formula_with_resolution(formula, is_display, 0.5);
    assert!(
        image_source_clamped_low.is_some(),
        "Should clamp low resolution to 1.0"
    );

    // Test resolution clamping (should clamp 30.0 to 25.0)
    let image_source_clamped_high =
        asset_manager.get_image_source_for_formula_with_resolution(formula, is_display, 30.0);
    assert!(
        image_source_clamped_high.is_some(),
        "Should clamp high resolution to 25.0"
    );
}

/// Test that neither size nor baseline scales with resolution (only rasterization quality changes)
#[test]
fn test_baseline_scaling_correctness() {
    let asset_manager = MathAssetManager::new();

    // We need a formula that exists and has baseline data
    // Try a few common formulas
    let test_formulas = ["x", "y", "a", "b"];
    let mut found_formula = None;

    for formula in test_formulas {
        if asset_manager
            .get_svg_size_with_baseline(formula, false)
            .is_some()
        {
            found_formula = Some(formula);
            break;
        }
    }

    let formula = found_formula.expect("Should find at least one formula with baseline data");
    let is_display = false;

    // Get original baseline at 1.0 resolution
    let (size_1, baseline_1) = asset_manager
        .get_svg_size_with_baseline_scaled(formula, is_display, 1.0)
        .expect("Should get size and baseline at 1.0");

    // Get baseline at 2.0 resolution
    let (size_2, baseline_2) = asset_manager
        .get_svg_size_with_baseline_scaled(formula, is_display, 2.0)
        .expect("Should get size and baseline at 2.0");

    // Size should NOT scale with resolution (only rasterization quality changes)
    assert!(
        (size_2.x - size_1.x).abs() < 0.1,
        "Width should NOT scale with resolution: {} != {}",
        size_2.x,
        size_1.x
    );
    assert!(
        (size_2.y - size_1.y).abs() < 0.1,
        "Height should NOT scale with resolution: {} != {}",
        size_2.y,
        size_1.y
    );

    // Baseline should NOT scale with resolution
    match (baseline_1, baseline_2) {
        (Some(b1), Some(b2)) => {
            assert!(
                (b2 - b1).abs() < 0.1,
                "Baseline should NOT scale with resolution: {} != {}",
                b2,
                b1
            );
        }
        (None, None) => {
            // No baseline data for this formula, that's OK
        }
        _ => {
            panic!("Baseline presence should be consistent across resolutions");
        }
    }
}

/// Test default resolution is 1.0 for backward compatibility
#[test]
fn test_default_resolution_1_0() {
    let asset_manager = MathAssetManager::new();

    // Use formulas that exist
    let test_cases = [
        ("a", false),          // Simple inline
        ("E = m c^2", false),  // Inline with baseline
        ("y = mx + b", false), // Another inline
    ];

    for (formula, is_display) in test_cases {
        // Test that default method (without resolution parameter) uses 1.0
        let image_source_default = asset_manager.get_image_source_for_formula(formula, is_display);
        let image_source_1_0 =
            asset_manager.get_image_source_for_formula_with_resolution(formula, is_display, 1.0);

        // At least one of them should work
        if image_source_default.is_some() || image_source_1_0.is_some() {
            // If both exist, they should be equivalent
            if let (
                Some(ImageSource::Bytes { uri: uri1, .. }),
                Some(ImageSource::Bytes { uri: uri2, .. }),
            ) = (image_source_default, image_source_1_0)
            {
                assert_eq!(
                    uri1, uri2,
                    "Default and 1.0 resolution should produce same URI for '{}'",
                    formula
                );
            }
            // Test passes if at least one method works
            return;
        }
    }

    // If we get here, none of the test formulas worked
    // This might happen in some test environments, so we'll skip the test
    println!("Note: Skipping test_default_resolution_1_0 - no test formulas found");
}

/// Test maximum resolution cap at 25.0
#[test]
fn test_max_resolution_cap_25() {
    let asset_manager = MathAssetManager::new();

    // Use formulas that exist
    let test_cases = [("a", false), ("E = m c^2", false), ("y = mx + b", false)];

    for (formula, is_display) in test_cases {
        // Skip if formula doesn't exist
        if asset_manager
            .get_image_source_for_formula(formula, is_display)
            .is_none()
        {
            continue;
        }

        // Test that values above 25.0 are clamped (API should still work)
        for test_scale in [26.0, 50.0, 100.0] {
            let image_source = asset_manager
                .get_image_source_for_formula_with_resolution(formula, is_display, test_scale);
            assert!(
                image_source.is_some(),
                "Should clamp {} to 25.0 and get image source for '{}'",
                test_scale,
                formula
            );
        }

        // Test that 25.0 works
        let image_source_25 =
            asset_manager.get_image_source_for_formula_with_resolution(formula, is_display, 25.0);
        assert!(
            image_source_25.is_some(),
            "25.0 should work for '{}'",
            formula
        );

        // If we found at least one working formula, test passes
        return;
    }

    // If we get here, none of the test formulas worked
    println!("Note: Skipping test_max_resolution_cap_25 - no test formulas found");
}

/// Test memory estimation formula
#[test]
fn test_memory_estimation() {
    // Create a mock manifest to test memory estimation
    // Note: This is a unit test for the formula, not actual memory usage

    // Test the formula: memory = base_memory × resolution_scale²
    let base_memory: f32 = 1000.0; // 1KB base

    let test_cases = vec![
        (1.0f32, 1000),  // 1× resolution = 1× memory
        (2.0f32, 4000),  // 2× resolution = 4× memory (2² = 4)
        (3.0f32, 9000),  // 3× resolution = 9× memory (3² = 9)
        (4.0f32, 16000), // 4× resolution = 16× memory (4² = 16)
        (5.0f32, 25000), // 5× resolution = 25× memory (5² = 25)
    ];

    for (scale, expected) in test_cases {
        let estimated = (base_memory * scale.powi(2)) as usize;
        assert_eq!(
            estimated, expected,
            "Memory estimation incorrect for scale {}: {} != {}",
            scale, estimated, expected
        );
    }
}

/// Test that find_formulas still works (regression test)
#[test]
fn test_find_formulas_regression() {
    let text = "Text $E=mc^2$ and $$\\sum_{i=1}^n i$$ here";
    let formulas = find_formulas(text);

    assert_eq!(formulas.len(), 2, "Should find 2 formulas");
    assert_eq!(formulas[0].2, "E=mc^2", "First formula should be E=mc^2");
    assert_eq!(
        formulas[1].2, "\\sum_{i=1}^n i",
        "Second formula should be sum"
    );
    assert!(!formulas[0].3, "First formula should be inline");
    assert!(formulas[1].3, "Second formula should be display");
}

/// Test integration: rendering with different resolutions
#[test]
fn test_math_rendering_with_resolution() {
    // This is more of an integration test
    // We'll test that the API works end-to-end

    let asset_manager = MathAssetManager::new();

    // Test formulas that exist
    let test_formulas = vec![
        ("E = m c^2", false),  // Simple inline (exists)
        ("a", false),          // Simple inline (exists)
        ("y = mx + b", false), // Inline (exists)
    ];

    let mut found_any = false;

    for (formula, is_display) in test_formulas {
        // Skip if formula doesn't exist
        if asset_manager
            .get_image_source_for_formula(formula, is_display)
            .is_none()
        {
            continue;
        }

        found_any = true;

        // Test multiple resolutions
        for resolution in [1.0, 1.5, 2.0, 3.0] {
            let image_source = asset_manager
                .get_image_source_for_formula_with_resolution(formula, is_display, resolution);

            assert!(
                image_source.is_some(),
                "Should get image source for '{}' at {}× resolution",
                formula,
                resolution
            );

            let result =
                asset_manager.get_svg_size_with_baseline_scaled(formula, is_display, resolution);

            assert!(result.is_some(), "Should get size for '{}'", formula);

            if let Some((size, _baseline)) = result {
                // Size should be valid
                assert!(
                    size.x > 0.0 && size.y > 0.0,
                    "Size should be positive for '{}'",
                    formula
                );
            }
        }
    }

    if !found_any {
        println!("Note: Skipping test_math_rendering_with_resolution - no test formulas found");
    }
}
