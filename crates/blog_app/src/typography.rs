//! Typography configuration for the blog app.
//!
//! This module handles font loading and text style configuration
//! with separation between UI elements and content.

use egui::{Context, FontData, FontDefinitions, FontFamily, FontId, TextStyle};
use std::{collections::BTreeMap, sync::Arc};

/// Configure custom typography for the blog app.
///
/// This sets up:
/// 1. Ubuntu font family with proper weights (Light, Regular, Medium, Bold)
/// 2. Italic variants for proper emphasis
/// 3. Hack for monospace code
/// 4. Proper fallback chains for emoji and missing characters
///
/// Returns true if fonts were configured successfully
pub fn configure_typography(cc: &eframe::CreationContext<'_>) -> bool {
    log::info!("Configuring typography with Ubuntu font family variants");
    log::debug!(
        "Available text styles before configuration: {:?}",
        cc.egui_ctx
            .global_style()
            .text_styles
            .keys()
            .collect::<Vec<_>>()
    );

    // Create new font definitions
    let mut fonts = FontDefinitions::default();

    // Add Ubuntu font variants from epaint_default_fonts
    fonts.font_data.insert(
        "Ubuntu-Light".to_owned(),
        Arc::new(FontData::from_static(epaint_default_fonts::UBUNTU_LIGHT)),
    );

    fonts.font_data.insert(
        "Ubuntu-Regular".to_owned(),
        Arc::new(FontData::from_static(epaint_default_fonts::UBUNTU_REGULAR)),
    );

    fonts.font_data.insert(
        "Ubuntu-Medium".to_owned(),
        Arc::new(FontData::from_static(epaint_default_fonts::UBUNTU_MEDIUM)),
    );

    fonts.font_data.insert(
        "Ubuntu-Bold".to_owned(),
        Arc::new(FontData::from_static(epaint_default_fonts::UBUNTU_BOLD)),
    );

    fonts.font_data.insert(
        "Ubuntu-Italic".to_owned(),
        Arc::new(FontData::from_static(epaint_default_fonts::UBUNTU_ITALIC)),
    );

    // Add other required fonts
    fonts.font_data.insert(
        "Hack-Regular".to_owned(),
        Arc::new(FontData::from_static(epaint_default_fonts::HACK_REGULAR)),
    );

    fonts.font_data.insert(
        "NotoEmoji-Regular".to_owned(),
        Arc::new(FontData::from_static(
            epaint_default_fonts::NOTO_EMOJI_REGULAR,
        )),
    );

    fonts.font_data.insert(
        "emoji-icon-font".to_owned(),
        Arc::new(FontData::from_static(epaint_default_fonts::EMOJI_ICON)),
    );

    // Configure Proportional family with Ubuntu weight variants
    // Use entry().or_default().insert(0, ...) pattern to add to existing families
    // This preserves default fonts and adds Ubuntu variants with highest priority
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "Ubuntu-Light".to_owned()); // Primary font (300 weight - matches default)
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(1, "Ubuntu-Regular".to_owned()); // Regular variant (400 weight)
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(2, "Ubuntu-Medium".to_owned()); // Medium weight (500)
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(3, "Ubuntu-Bold".to_owned()); // Bold variant (700 weight)
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(4, "Ubuntu-Italic".to_owned()); // Italic variant

    // Ensure emoji fallbacks are at the end (lowest priority)
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .push("NotoEmoji-Regular".to_owned()); // Emoji fallback
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .push("emoji-icon-font".to_owned()); // Icon fallback

    // Configure "Content" family with Ubuntu-Light as primary (300 weight - matches default egui)
    // This is for normal text (body, paragraphs, etc.)
    let mut content_family = Vec::new();
    content_family.push("Ubuntu-Light".to_owned()); // Primary font for normal text (300 weight)
    content_family.push("NotoEmoji-Regular".to_owned()); // Emoji fallback
    content_family.push("emoji-icon-font".to_owned()); // Icon fallback

    fonts
        .families
        .insert(FontFamily::Name("Content".into()), content_family);

    // Configure "ContentRegular" family with Ubuntu-Regular (400 weight)
    // This is for slightly heavier text if needed
    let mut content_regular_family = Vec::new();
    content_regular_family.push("Ubuntu-Regular".to_owned()); // Regular weight (400)
    content_regular_family.push("NotoEmoji-Regular".to_owned()); // Emoji fallback
    content_regular_family.push("emoji-icon-font".to_owned()); // Icon fallback

    fonts.families.insert(
        FontFamily::Name("ContentRegular".into()),
        content_regular_family,
    );

    // Configure "ContentMedium" family with Ubuntu-Medium (500 weight)
    // This is for medium weight text
    let mut content_medium_family = Vec::new();
    content_medium_family.push("Ubuntu-Medium".to_owned()); // Medium weight (500)
    content_medium_family.push("NotoEmoji-Regular".to_owned()); // Emoji fallback
    content_medium_family.push("emoji-icon-font".to_owned()); // Icon fallback

    fonts.families.insert(
        FontFamily::Name("ContentMedium".into()),
        content_medium_family,
    );

    // Configure "ContentBold" family with Ubuntu-Bold as primary (700 weight)
    // This is for headings and other bold text
    let mut content_bold_family = Vec::new();
    content_bold_family.push("Ubuntu-Bold".to_owned()); // Primary font for bold text
    content_bold_family.push("NotoEmoji-Regular".to_owned()); // Emoji fallback
    content_bold_family.push("emoji-icon-font".to_owned()); // Icon fallback

    fonts
        .families
        .insert(FontFamily::Name("ContentBold".into()), content_bold_family);

    // Configure "ContentItalic" family with Ubuntu-Italic as primary
    // This is for italic text
    let mut content_italic_family = Vec::new();
    content_italic_family.push("Ubuntu-Italic".to_owned()); // Primary font for italic text
    content_italic_family.push("NotoEmoji-Regular".to_owned()); // Emoji fallback
    content_italic_family.push("emoji-icon-font".to_owned()); // Icon fallback

    fonts.families.insert(
        FontFamily::Name("ContentItalic".into()),
        content_italic_family,
    );

    // Log detailed font family information
    for (family, fonts_list) in &fonts.families {
        log::debug!(
            "Font family {:?} has {} fonts: {:?}",
            family,
            fonts_list.len(),
            fonts_list
        );
    }

    cc.egui_ctx.set_fonts(fonts);
    log::info!("Fonts set successfully");

    true
}

/// Check if required text styles are available in the context
/// Fonts load asynchronously, so we need to verify they're ready before using them
pub fn verify_text_styles_available(ctx: &Context) -> bool {
    let style = ctx.global_style();
    let required_styles = [
        TextStyle::Name("ContentBody".into()),
        TextStyle::Name("ContentHeading".into()),
        TextStyle::Name("ContentHeading2".into()),
        TextStyle::Name("ContentHeading3".into()),
        TextStyle::Name("ContentHeading4".into()),
        TextStyle::Name("ContentHeading5".into()),
        TextStyle::Name("ContentHeading6".into()),
    ];

    for required_style in &required_styles {
        if !style.text_styles.contains_key(required_style) {
            log::warn!("Required text style not found: {:?}", required_style);
            return false;
        }
    }

    log::debug!("All required text styles are available");
    true
}

/// Configure text styles with separation between UI and content.
///
/// Returns a map of text styles to font configurations.
pub fn content_text_styles() -> BTreeMap<TextStyle, FontId> {
    [
        // Content Elements - Light weight (300) for normal text
        (
            TextStyle::Name("ContentBody".into()),
            FontId::new(16.0, FontFamily::Name("Content".into())), // Normal text (Light - 300)
        ),
        (
            TextStyle::Name("ContentBodyRegular".into()),
            FontId::new(16.0, FontFamily::Name("ContentRegular".into())), // Regular weight (400)
        ),
        (
            TextStyle::Name("ContentBodyMedium".into()),
            FontId::new(16.0, FontFamily::Name("ContentMedium".into())), // Medium weight (500)
        ),
        (
            TextStyle::Name("ContentBodyBold".into()),
            FontId::new(16.0, FontFamily::Name("ContentBold".into())), // Bold text (700)
        ),
        (
            TextStyle::Name("ContentBodyItalic".into()),
            FontId::new(16.0, FontFamily::Name("ContentItalic".into())), // Italic text
        ),
        // Headings (use bold font - 700 weight)
        (
            TextStyle::Name("ContentHeading".into()),
            FontId::new(36.0, FontFamily::Name("ContentBold".into())), // H1 - Largest
        ),
        (
            TextStyle::Name("ContentHeading2".into()),
            FontId::new(28.0, FontFamily::Name("ContentBold".into())), // H2
        ),
        (
            TextStyle::Name("ContentHeading3".into()),
            FontId::new(24.0, FontFamily::Name("ContentBold".into())), // H3
        ),
        (
            TextStyle::Name("ContentHeading4".into()),
            FontId::new(20.0, FontFamily::Name("ContentBold".into())), // H4 - Still clearly larger than body
        ),
        (
            TextStyle::Name("ContentHeading5".into()),
            FontId::new(18.0, FontFamily::Name("ContentBold".into())), // H5 - Slightly larger than body
        ),
        (
            TextStyle::Name("ContentHeading6".into()),
            FontId::new(16.0, FontFamily::Name("ContentBold".into())), // H6 - Same size as body but bold
        ),
        // Small text variants
        (
            TextStyle::Name("ContentSmall".into()),
            FontId::new(14.0, FontFamily::Name("Content".into())), // Small text (Light)
        ),
        (
            TextStyle::Name("ContentSmallRegular".into()),
            FontId::new(14.0, FontFamily::Name("ContentRegular".into())), // Small regular text
        ),
        (
            TextStyle::Name("ContentSmallMedium".into()),
            FontId::new(14.0, FontFamily::Name("ContentMedium".into())), // Small medium text
        ),
        (
            TextStyle::Name("ContentSmallBold".into()),
            FontId::new(14.0, FontFamily::Name("ContentBold".into())), // Small bold text
        ),
        // Monospace
        (
            TextStyle::Name("ContentMonospace".into()),
            FontId::new(14.0, FontFamily::Monospace), // Code in content uses monospace
        ),
    ]
    .into()
}

/// Configure UI text styles (buttons, labels, etc.)
///
/// Returns a map of text styles to font configurations.
pub fn ui_text_styles() -> BTreeMap<TextStyle, FontId> {
    [
        // UI Elements (use Proportional family - sans-serif)
        (
            TextStyle::Small,
            FontId::new(12.0, FontFamily::Proportional),
        ),
        (TextStyle::Body, FontId::new(16.0, FontFamily::Proportional)),
        (
            TextStyle::Button,
            FontId::new(16.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Heading,
            FontId::new(32.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Monospace,
            FontId::new(13.6, FontFamily::Monospace),
        ),
        // Custom UI heading styles
        (
            TextStyle::Name("UIHeading2".into()),
            FontId::new(24.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Name("UIHeading3".into()),
            FontId::new(20.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Name("UIHeading4".into()),
            FontId::new(16.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Name("UIHeading5".into()),
            FontId::new(14.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Name("UIHeading6".into()),
            FontId::new(13.6, FontFamily::Proportional),
        ),
    ]
    .into()
}

/// Apply text styles to the egui context using all_styles_mut pattern.
/// This ensures text styles are properly registered and available immediately.
pub fn apply_text_styles(ctx: &Context, text_styles: BTreeMap<TextStyle, FontId>) {
    log::info!(
        "Applying {} text styles using all_styles_mut",
        text_styles.len()
    );

    // Log what we're about to apply
    for (text_style, font_id) in &text_styles {
        log::debug!("Will set text style: {:?} -> {:?}", text_style, font_id);
    }

    // Clone the text styles for the closure
    let text_styles_clone = text_styles.clone();

    ctx.all_styles_mut(move |style| {
        // Log existing styles before modification
        log::debug!(
            "Existing text styles before application: {:?}",
            style.text_styles.keys().collect::<Vec<_>>()
        );

        // Merge new text styles with existing ones
        for (text_style, font_id) in &text_styles_clone {
            log::info!("Setting text style: {:?} -> {:?}", text_style, font_id);
            style
                .text_styles
                .insert(text_style.clone(), font_id.clone());
        }

        log::info!(
            "Text styles after application: {:?}",
            style.text_styles.keys().collect::<Vec<_>>()
        );
    });

    // Verify the styles were applied
    let style = ctx.global_style();
    for text_style in text_styles.keys() {
        if !style.text_styles.contains_key(text_style) {
            log::error!("Text style {:?} was NOT applied successfully!", text_style);
        } else {
            log::debug!("Text style {:?} verified as applied", text_style);
        }
    }
}
