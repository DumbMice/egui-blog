//! Tests for typography configuration and font loading.

use blog_app::typography;
use egui::{FontFamily, FontId, TextStyle};
use std::collections::BTreeMap;

#[test]
fn test_content_text_styles_creation() {
    // Test that content text styles are created with correct font families
    let text_styles = typography::content_text_styles();

    // Verify we have the expected number of text styles
    assert_eq!(text_styles.len(), 16, "Should have 16 content text styles");

    // Verify specific text styles exist
    let required_styles = [
        ("ContentBody", FontFamily::Name("Content".into()), 16.0),
        (
            "ContentBodyRegular",
            FontFamily::Name("ContentRegular".into()),
            16.0,
        ),
        (
            "ContentBodyMedium",
            FontFamily::Name("ContentMedium".into()),
            16.0,
        ),
        (
            "ContentBodyBold",
            FontFamily::Name("ContentBold".into()),
            16.0,
        ),
        (
            "ContentBodyItalic",
            FontFamily::Name("ContentItalic".into()),
            16.0,
        ),
        (
            "ContentHeading",
            FontFamily::Name("ContentBold".into()),
            36.0,
        ),
        (
            "ContentHeading2",
            FontFamily::Name("ContentBold".into()),
            28.0,
        ),
        (
            "ContentHeading3",
            FontFamily::Name("ContentBold".into()),
            24.0,
        ),
        (
            "ContentHeading4",
            FontFamily::Name("ContentBold".into()),
            20.0,
        ),
        (
            "ContentHeading5",
            FontFamily::Name("ContentBold".into()),
            18.0,
        ),
        (
            "ContentHeading6",
            FontFamily::Name("ContentBold".into()),
            16.0,
        ),
        ("ContentSmall", FontFamily::Name("Content".into()), 14.0),
        (
            "ContentSmallRegular",
            FontFamily::Name("ContentRegular".into()),
            14.0,
        ),
        (
            "ContentSmallMedium",
            FontFamily::Name("ContentMedium".into()),
            14.0,
        ),
        (
            "ContentSmallBold",
            FontFamily::Name("ContentBold".into()),
            14.0,
        ),
        ("ContentMonospace", FontFamily::Monospace, 14.0),
    ];

    for (style_name, expected_family, expected_size) in &required_styles {
        let text_style = TextStyle::Name((*style_name).into());
        if let Some(font_id) = text_styles.get(&text_style) {
            assert_eq!(
                font_id.family, *expected_family,
                "Style {} should have font family {:?}, got {:?}",
                style_name, expected_family, font_id.family
            );
            assert!(
                (font_id.size - expected_size).abs() < 0.001,
                "Style {} should have size {}, got {}",
                style_name,
                expected_size,
                font_id.size
            );
        } else {
            panic!("Text style {} not found in content text styles", style_name);
        }
    }
}

#[test]
fn test_ui_text_styles_creation() {
    // Test that UI text styles are created with correct font families
    let text_styles = typography::ui_text_styles();

    // Verify we have a reasonable number of text styles
    assert!(
        text_styles.len() >= 6,
        "Should have at least 6 UI text styles"
    );

    // Verify standard text styles exist
    let standard_styles = [
        (TextStyle::Small, FontFamily::Proportional, 12.0),
        (TextStyle::Body, FontFamily::Proportional, 16.0),
        (TextStyle::Button, FontFamily::Proportional, 16.0),
        (TextStyle::Heading, FontFamily::Proportional, 32.0),
        (TextStyle::Monospace, FontFamily::Monospace, 13.6),
    ];

    for (text_style, expected_family, expected_size) in &standard_styles {
        if let Some(font_id) = text_styles.get(text_style) {
            assert_eq!(
                font_id.family, *expected_family,
                "Style {:?} should have font family {:?}, got {:?}",
                text_style, expected_family, font_id.family
            );
            assert!(
                (font_id.size - expected_size).abs() < 0.001,
                "Style {:?} should have size {}, got {}",
                text_style,
                expected_size,
                font_id.size
            );
        } else {
            panic!("Text style {:?} not found in UI text styles", text_style);
        }
    }
}

#[test]
fn test_text_style_maps_are_valid() {
    // Test that text style maps don't have duplicate keys
    let content_styles = typography::content_text_styles();
    let ui_styles = typography::ui_text_styles();

    // Check for duplicates within each map
    let content_keys: Vec<_> = content_styles.keys().collect();
    let unique_content_keys: std::collections::HashSet<_> = content_styles.keys().collect();
    assert_eq!(
        content_keys.len(),
        unique_content_keys.len(),
        "Content text styles have duplicate keys"
    );

    let ui_keys: Vec<_> = ui_styles.keys().collect();
    let unique_ui_keys: std::collections::HashSet<_> = ui_styles.keys().collect();
    assert_eq!(
        ui_keys.len(),
        unique_ui_keys.len(),
        "UI text styles have duplicate keys"
    );

    // Check that content and UI styles don't overlap (they shouldn't)
    let content_key_set: std::collections::HashSet<_> = content_styles.keys().collect();
    let ui_key_set: std::collections::HashSet<_> = ui_styles.keys().collect();
    let intersection: Vec<_> = content_key_set.intersection(&ui_key_set).collect();
    assert!(
        intersection.is_empty(),
        "Content and UI text styles should not share keys, but found: {:?}",
        intersection
    );
}
