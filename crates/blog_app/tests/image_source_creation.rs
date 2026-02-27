//! Test image source creation and rendering

use blog_app::math::{self, MathAssetManager};

#[test]
fn test_image_source_creation() {
    let asset_manager = MathAssetManager::default();
    let manifest = math::load_manifest();

    // Test a known formula
    let formula = "E = m c^2";
    let is_display = false;

    let hash = manifest
        .find_hash(formula, is_display)
        .expect("Should find hash");

    println!("Testing image source creation for formula: {}", formula);
    println!("Hash: {}", hash);

    // Try to get image source
    let image_source = asset_manager.get_image_source_for_hash(hash);

    match image_source {
        Some(image_source) => {
            println!("SUCCESS: ImageSource created");

            // Check that it's a Bytes variant
            match image_source {
                egui::ImageSource::Bytes { uri, bytes } => {
                    println!("  URI: {}", uri);
                    println!("  Bytes length: {}", bytes.len());
                    assert!(
                        uri.starts_with("bytes://math/"),
                        "URI should start with bytes://math/"
                    );
                    assert!(uri.ends_with(".svg"), "URI should end with .svg");
                    assert!(!bytes.is_empty(), "Bytes should not be empty");
                }
                _ => {
                    panic!("Expected ImageSource::Bytes variant");
                }
            }
        }
        None => {
            println!("ERROR: Failed to create ImageSource");
            panic!("ImageSource creation failed for formula: {}", formula);
        }
    }
}

#[test]
fn test_image_source_for_formula() {
    let asset_manager = MathAssetManager::default();

    // Test getting image source for formula directly
    let formula = "E = m c^2";
    let is_display = false;

    println!("Testing get_image_source_for_formula for: {}", formula);

    let image_source = asset_manager.get_image_source_for_formula(formula, is_display);

    match image_source {
        Some(image_source) => {
            println!("SUCCESS: ImageSource created via get_image_source_for_formula");

            // Check that it's a Bytes variant
            match image_source {
                egui::ImageSource::Bytes { uri, bytes } => {
                    println!("  URI: {}", uri);
                    println!("  Bytes length: {}", bytes.len());
                    assert!(
                        uri.starts_with("bytes://math/"),
                        "URI should start with bytes://math/"
                    );
                    assert!(uri.ends_with(".svg"), "URI should end with .svg");
                    assert!(!bytes.is_empty(), "Bytes should not be empty");
                }
                _ => {
                    panic!("Expected ImageSource::Bytes variant");
                }
            }
        }
        None => {
            println!("ERROR: get_image_source_for_formula returned None");
            println!("This could mean:");
            println!("  1. Formula not found in manifest");
            println!("  2. SVG bytes not found");

            // Check each step
            let manifest = math::load_manifest();
            let hash = manifest.find_hash(formula, is_display);
            println!("  Hash found: {}", hash.is_some());

            if let Some(hash) = hash {
                let svg_bytes = math::get_svg_bytes(hash);
                println!("  SVG bytes available: {}", svg_bytes.is_some());
            }

            panic!("get_image_source_for_formula failed");
        }
    }
}
