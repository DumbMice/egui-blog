//! Test crisp SVG rendering with the new ImageSource-based approach

use blog_app::math::{self, MathAssetManager};
use egui::{Image, ImageSource};

/// Test that ImageSource is created correctly for crisp SVG rendering
#[test]
fn test_crisp_svg_image_source() {
    let asset_manager = MathAssetManager::default();
    let manifest = math::load_manifest();

    // Test a known formula
    let formula = "E = m c^2";
    let is_display = false;

    let hash = manifest
        .find_hash(formula, is_display)
        .expect("Should find hash");

    println!("Testing crisp SVG rendering for formula: {}", formula);
    println!("Hash: {}", hash);

    // Get image source
    let image_source =
        MathAssetManager::get_image_source_for_hash(hash).expect("Should get image source");

    // Verify it's a Bytes variant with correct URI pattern
    match &image_source {
        ImageSource::Bytes { uri, bytes } => {
            println!("  URI: {}", uri);
            println!("  Bytes length: {}", bytes.len());

            // Check URI pattern matches what egui expects for SVG loading
            assert!(
                uri.starts_with("bytes://math/"),
                "URI should start with bytes://math/ for proper caching"
            );
            assert!(
                uri.ends_with(".svg"),
                "URI should end with .svg for proper SVG loader detection"
            );
            assert!(
                uri.contains(&hash),
                "URI should contain hash for unique identification"
            );

            // Verify bytes are valid SVG data
            let svg_str = std::str::from_utf8(bytes).expect("SVG should be valid UTF-8");
            assert!(svg_str.contains("<svg"), "Bytes should contain SVG markup");
            assert!(
                svg_str.contains("xmlns=\"http://www.w3.org/2000/svg\""),
                "SVG should have proper namespace"
            );

            println!("  ✓ ImageSource created correctly for crisp SVG rendering");
        }
        _ => {
            panic!("Expected ImageSource::Bytes variant for SVG rendering");
        }
    }
}

/// Test that Image widget can be created from the ImageSource
#[test]
fn test_image_widget_creation() {
    let asset_manager = MathAssetManager::default();
    let manifest = math::load_manifest();

    // Test a known formula
    let formula = "a^2 + b^2 = c^2";
    let is_display = false;

    let hash = manifest
        .find_hash(formula, is_display)
        .expect("Should find hash");

    println!("Testing Image widget creation for formula: {}", formula);

    // Get image source
    let image_source =
        MathAssetManager::get_image_source_for_hash(hash).expect("Should get image source");

    // Create Image widget (simulating what markdown.rs does)
    let _image = Image::new(image_source)
        .fit_to_exact_size(egui::vec2(100.0, 50.0)) // Example size
        .corner_radius(0.0); // No rounding for crisp edges

    println!("  ✓ Image widget created successfully");
    println!("  - Size hint: fit_to_exact_size(100x50)");
    println!("  - Corner radius: 0.0 (crisp edges)");

    // Note: We can't actually render without a UI context in tests,
    // but creating the Image widget successfully is a good sign.
}

/// Test multiple formulas to ensure consistent behavior
#[test]
fn test_multiple_formulas() {
    let asset_manager = MathAssetManager::default();
    let manifest = math::load_manifest();

    let test_formulas = [
        ("E = m c^2", false),
        ("a^2 + b^2 = c^2", false),
        ("x = (-b +- sqrt(b^2 - 4 a c)) / (2a)", true),
    ];

    for (formula, is_display) in test_formulas.iter() {
        if let Some(hash) = manifest.find_hash(formula, *is_display) {
            println!("Testing formula: {} (display: {})", formula, is_display);

            let image_source =
                MathAssetManager::get_image_source_for_hash(hash).expect("Should get image source");

            match image_source {
                ImageSource::Bytes { uri, bytes } => {
                    assert!(!bytes.is_empty(), "SVG bytes should not be empty");
                    assert!(uri.ends_with(".svg"), "URI should end with .svg");
                    println!("  ✓ Valid ImageSource created");
                }
                _ => panic!("Expected ImageSource::Bytes"),
            }
        } else {
            println!("  Formula not in manifest (expected for some test formulas)");
        }
    }

    println!("✓ All available formulas produce valid ImageSources");
}
