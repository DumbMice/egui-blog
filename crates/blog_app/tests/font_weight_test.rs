//! Test to verify font weight rendering in markdown.

use blog_app::typography;
use egui::{FontFamily, FontId, TextStyle};

#[test]
fn test_bold_text_style_mapping() {
    // Test that bold text styles use the correct font families

    let text_styles = typography::content_text_styles();

    // Verify ContentBody uses Light weight (300)
    let content_body = TextStyle::Name("ContentBody".into());
    if let Some(font_id) = text_styles.get(&content_body) {
        assert_eq!(
            font_id.family,
            FontFamily::Name("Content".into()),
            "ContentBody should use Content family (Light weight)"
        );
    } else {
        panic!("ContentBody text style not found");
    }

    // Verify ContentBodyBold uses Bold weight (700)
    let content_body_bold = TextStyle::Name("ContentBodyBold".into());
    if let Some(font_id) = text_styles.get(&content_body_bold) {
        assert_eq!(
            font_id.family,
            FontFamily::Name("ContentBold".into()),
            "ContentBodyBold should use ContentBold family (Bold weight)"
        );
    } else {
        panic!("ContentBodyBold text style not found");
    }

    // Verify ContentBodyItalic uses Italic variant
    let content_body_italic = TextStyle::Name("ContentBodyItalic".into());
    if let Some(font_id) = text_styles.get(&content_body_italic) {
        assert_eq!(
            font_id.family,
            FontFamily::Name("ContentItalic".into()),
            "ContentBodyItalic should use ContentItalic family"
        );
    } else {
        panic!("ContentBodyItalic text style not found");
    }

    // Verify headings use Bold weight
    let headings = [
        "ContentHeading",
        "ContentHeading2",
        "ContentHeading3",
        "ContentHeading4",
        "ContentHeading5",
        "ContentHeading6",
    ];

    for heading in &headings {
        let text_style = TextStyle::Name((*heading).into());
        if let Some(font_id) = text_styles.get(&text_style) {
            assert_eq!(
                font_id.family,
                FontFamily::Name("ContentBold".into()),
                "Heading {} should use ContentBold family (Bold weight)",
                heading
            );
        } else {
            panic!("Heading {} text style not found", heading);
        }
    }
}

#[test]
fn test_font_weight_progression() {
    // Test that we have a proper weight progression
    let text_styles = typography::content_text_styles();

    // Check body text weights
    let body_styles = [
        ("ContentBody", "Content"),               // Light (300)
        ("ContentBodyRegular", "ContentRegular"), // Regular (400)
        ("ContentBodyMedium", "ContentMedium"),   // Medium (500)
        ("ContentBodyBold", "ContentBold"),       // Bold (700)
    ];

    for (style_name, expected_family) in &body_styles {
        let text_style = TextStyle::Name((*style_name).into());
        if let Some(font_id) = text_styles.get(&text_style) {
            assert_eq!(
                font_id.family,
                FontFamily::Name((*expected_family).into()),
                "Style {} should use {} family",
                style_name,
                expected_family
            );
            assert_eq!(
                font_id.size, 16.0,
                "Style {} should have size 16.0",
                style_name
            );
        } else {
            panic!("Style {} not found", style_name);
        }
    }
}
