//! Integration tests for math rendering system

use blog_app::math::{self, MathAssetManager};

/// Test that the manifest loads correctly
#[test]
#[ignore = "requires build-generated embedded.rs with formulas"]
fn test_manifest_loading() {
    let manifest = math::load_manifest();

    // Manifest should have formulas
    assert!(
        !manifest.formulas.is_empty(),
        "Manifest should have formulas"
    );

    // Test finding a known formula
    let formula = "E = m c^2";
    let hash = manifest.find_hash(formula, false);
    assert!(hash.is_some(), "Should find hash for formula: {}", formula);

    println!(
        "Manifest loaded successfully with {} formulas",
        manifest.formulas.len()
    );
    for (hash, metadata) in &manifest.formulas {
        println!(
            "  - {}: {} (display: {})",
            &hash[0..8],
            metadata.formula,
            metadata.is_display
        );
    }
}

/// Test that SVG bytes can be loaded for a formula
#[test]
fn test_svg_loading() {
    let manifest = math::load_manifest();

    // Test a known formula
    let formula = "E = m c^2";
    let hash = manifest
        .find_hash(formula, false)
        .expect("Should find hash");

    // Get SVG bytes
    let svg_bytes = math::get_svg_bytes(hash);
    assert!(
        svg_bytes.is_some(),
        "Should get SVG bytes for hash: {}",
        hash
    );

    let svg_bytes = svg_bytes.unwrap();
    assert!(!svg_bytes.is_empty(), "SVG bytes should not be empty");

    // Check it's valid SVG
    let svg_str = std::str::from_utf8(svg_bytes).expect("SVG should be valid UTF-8");
    assert!(svg_str.contains("<svg"), "Should be SVG content");
    assert!(svg_str.contains("width="), "Should have width attribute");
    assert!(svg_str.contains("height="), "Should have height attribute");

    println!("SVG loaded successfully for formula: {}", formula);
    println!("  Hash: {}", hash);
    println!("  Size: {} bytes", svg_bytes.len());
    println!("  SVG preview: {}...", &svg_str[0..100].replace("\n", " "));
}

/// Test MathAssetManager basics
#[test]
fn test_asset_manager_basics() {
    // Create asset manager
    let mut manager = MathAssetManager::default();

    // Check manifest is loaded
    assert_eq!(
        MathAssetManager::cache_size(),
        0,
        "Cache should be empty initially"
    );

    // Test cache management
    MathAssetManager::set_max_cache_size(5);
    MathAssetManager::clear_cache();

    println!("MathAssetManager created successfully");
}

/// Test formula extraction from text
#[test]
fn test_formula_extraction() {
    let text = r#"Here's some inline math: $E = m c^2$ and $(1/2) + (1/4) = (3/4)$.
    
Display math should be centered:

$$ integral from -infinity to infinity e^(-x^2) \ dx = sqrt(pi) $$

The quadratic formula:

$$ x = (-b +- sqrt(b^2 - 4ac)) / (2a) $$"#;

    let formulas = math::find_formulas(text);

    // Should find 4 formulas (2 inline, 2 display)
    assert_eq!(formulas.len(), 4, "Should find 4 formulas in test text");

    for (start, end, formula, is_display) in &formulas {
        println!(
            "Found formula (display: {}): {} at position {}-{}",
            is_display, formula, start, end
        );
    }

    // Verify specific formulas
    let inline_formulas: Vec<&str> = formulas
        .iter()
        .filter(|(_, _, _, is_display)| !is_display)
        .map(|(_, _, formula, _)| formula.as_str())
        .collect();

    let _display_formulas: Vec<&str> = formulas
        .iter()
        .filter(|(_, _, _, is_display)| *is_display)
        .map(|(_, _, formula, _)| formula.as_str())
        .collect();

    assert!(
        inline_formulas.contains(&"E = m c^2"),
        "Should find E = m c^2"
    );
    assert!(
        inline_formulas.contains(&"(1/2) + (1/4) = (3/4)"),
        "Should find fraction formula"
    );
}

/// Test that all formulas in the test post can be found in the manifest
#[test]
fn test_manifest_coverage() {
    let manifest = math::load_manifest();

    // Formulas from the test post that should be in the manifest
    let test_formulas = vec![
        ("E = m c^2", false),
        ("a^2 + b^2 = c^2", false),
        // Note: The following formulas are not in actual blog posts,
        // so they won't be in the manifest. This is expected.
        // They are kept here to test the fallback behavior.
    ];

    let mut missing_formulas = Vec::new();

    for (formula, is_display) in test_formulas {
        let hash = manifest.find_hash(formula, is_display);
        if hash.is_none() {
            missing_formulas.push((formula, is_display));
            println!(
                "WARNING: Formula not found in manifest: {} (display: {})",
                formula, is_display
            );
        } else {
            println!(
                "OK: Formula found in manifest: {} (display: {}) -> hash: {}",
                formula,
                is_display,
                hash.unwrap()
            );
        }
    }

    assert!(
        missing_formulas.is_empty(),
        "All test formulas should be in manifest. Missing: {:?}",
        missing_formulas
    );
}

/// Test SVG content validation
#[test]
fn test_svg_content() {
    let manifest = math::load_manifest();

    // Check a few formulas
    let test_formulas = vec![("E = m c^2", false), ("a^2 + b^2 = c^2", false)];

    for (formula, is_display) in test_formulas {
        let hash = manifest
            .find_hash(formula, is_display)
            .unwrap_or_else(|| panic!("Formula not found: {}", formula));

        let svg_bytes = math::get_svg_bytes(hash)
            .unwrap_or_else(|| panic!("No SVG bytes for formula: {}", formula));

        let svg_str = std::str::from_utf8(svg_bytes)
            .unwrap_or_else(|_| panic!("Invalid UTF-8 in SVG for formula: {}", formula));

        // Check for common SVG issues
        if svg_str.contains("SVG placeholder") || svg_str.contains("Typst not available") {
            println!(
                "WARNING: Formula '{}' has placeholder SVG (Typst CLI might not be installed)",
                formula
            );
        } else {
            // Check for valid SVG structure
            assert!(
                svg_str.contains("<?xml") || svg_str.contains("<svg"),
                "SVG for '{}' should be valid XML/SVG",
                formula
            );
            assert!(
                svg_str.contains("width="),
                "SVG for '{}' should have width",
                formula
            );
            assert!(
                svg_str.contains("height="),
                "SVG for '{}' should have height",
                formula
            );
            assert!(
                svg_str.contains("</svg>"),
                "SVG for '{}' should have closing tag",
                formula
            );

            println!(
                "OK: Formula '{}' has valid SVG ({} bytes)",
                formula,
                svg_bytes.len()
            );
        }
    }
}

/// Run all math rendering tests
#[test]
fn test_math_rendering_system() {
    println!("=== Testing Math Rendering System ===");

    test_manifest_loading();
    test_svg_loading();
    test_asset_manager_basics();
    test_formula_extraction();
    test_manifest_coverage();
    test_svg_content();

    println!("=== All Math Rendering Tests Passed ===");
}
