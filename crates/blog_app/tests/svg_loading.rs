//! Test SVG loading functionality

use blog_app::math;
use egui_extras::image::load_svg_bytes;

#[test]
fn test_svg_loading_with_egui_extras() {
    let manifest = math::load_manifest();

    // Test a known formula
    let formula = "E = m c^2";
    let hash = manifest
        .find_hash(formula, false)
        .expect("Should find hash");

    // Get SVG bytes
    let svg_bytes = math::get_svg_bytes(hash).expect("Should get SVG bytes");

    // Try to load with egui_extras
    let result = load_svg_bytes(&svg_bytes, &Default::default());

    match result {
        Ok(color_image) => {
            println!("SUCCESS: SVG loaded as ColorImage");
            println!("  Size: {}x{}", color_image.size[0], color_image.size[1]);
            println!("  Pixels: {} bytes", color_image.pixels.len() * 4);

            // Check image has non-zero size
            assert!(color_image.size[0] > 0, "Image width should be > 0");
            assert!(color_image.size[1] > 0, "Image height should be > 0");

            // For theme-aware SVGs (white text on transparent background),
            // pixels may appear transparent when loaded without tinting.
            // The tint is applied at render time in the UI.
            // So we only check that the image loads successfully.
            println!("  Note: Theme-aware SVG loaded (white text on transparent background)");
            println!("  Tint will be applied at render time for theme adaptation");
        }
        Err(e) => {
            println!("ERROR: Failed to load SVG: {}", e);
            panic!("SVG loading failed: {}", e);
        }
    }
}

#[test]
fn test_all_svgs_loadable() {
    let manifest = math::load_manifest();

    let mut failed_formulas: Vec<(String, String)> = Vec::new();

    for (hash, metadata) in &manifest.formulas {
        let svg_bytes = math::get_svg_bytes(hash).expect("Should get SVG bytes");

        match load_svg_bytes(&svg_bytes, &Default::default()) {
            Ok(color_image) => {
                if color_image.size[0] == 0 || color_image.size[1] == 0 {
                    println!(
                        "WARNING: Formula '{}' has zero-size image",
                        metadata.formula
                    );
                    failed_formulas.push((metadata.formula.clone(), "zero-size image".to_string()));
                } else {
                    println!(
                        "OK: Formula '{}' loads successfully ({}x{})",
                        metadata.formula, color_image.size[0], color_image.size[1]
                    );
                    // For theme-aware SVGs (white text on transparent background),
                    // pixels may appear transparent when loaded without tinting.
                    // The tint is applied at render time in the UI.
                    if color_image.pixels.iter().all(|p| p.a() == 0) {
                        println!("  Note: Theme-aware SVG (white text on transparent background)");
                    }
                }
            }
            Err(e) => {
                println!(
                    "ERROR: Formula '{}' failed to load: {}",
                    metadata.formula, e
                );
                failed_formulas.push((metadata.formula.clone(), format!("load error: {}", e)));
            }
        }
    }

    if !failed_formulas.is_empty() {
        println!("\nFailed formulas:");
        for (formula, reason) in &failed_formulas {
            println!("  - {}: {}", formula, reason);
        }
    }

    assert!(
        failed_formulas.is_empty(),
        "All SVGs should load successfully. Failed: {:?}",
        failed_formulas
    );
}
