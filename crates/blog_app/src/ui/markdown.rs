//! Markdown rendering for blog posts.

use egui::{Hyperlink, ImageSource, Pos2, Rect, RichText, Sense, Shape, TextStyle, Ui, vec2};
use egui_extras::syntax_highlighting::{CodeTheme, highlight};
use pulldown_cmark::{Alignment, HeadingLevel};
#[cfg(test)]
use pulldown_cmark::{Event, Parser, Tag};

use crate::ui::owned_pulldown_cmark::{OwnedCodeBlockKind, OwnedEvent, OwnedTag, SpannedEvent};
use crate::ui::table_renderer::TableConfig;
use crate::widgets::WidgetConfig;
use crate::{MathAssetManager, ui::table_renderer};

/// Get the bold variant of a text style
/// Since egui's `.strong()` only changes color, not font weight,
/// we need to use different text styles for bold text
pub(crate) fn bold_text_style(text_style: &TextStyle) -> TextStyle {
    match text_style {
        // Content body variants
        TextStyle::Name(name) if name == &"ContentBody".into() => {
            TextStyle::Name("ContentBodyBold".into())
        }
        TextStyle::Name(name) if name == &"ContentBodyRegular".into() => {
            TextStyle::Name("ContentBodyBold".into())
        }
        TextStyle::Name(name) if name == &"ContentBodyMedium".into() => {
            TextStyle::Name("ContentBodyBold".into())
        }
        TextStyle::Name(name) if name == &"ContentBodyBold".into() => {
            text_style.clone() // Already bold
        }
        // Small text variants
        TextStyle::Name(name) if name == &"ContentSmall".into() => {
            TextStyle::Name("ContentSmallBold".into())
        }
        TextStyle::Name(name) if name == &"ContentSmallRegular".into() => {
            TextStyle::Name("ContentSmallBold".into())
        }
        TextStyle::Name(name) if name == &"ContentSmallMedium".into() => {
            TextStyle::Name("ContentSmallBold".into())
        }
        TextStyle::Name(name) if name == &"ContentSmallBold".into() => {
            text_style.clone() // Already bold
        }
        // Headings are already bold, keep as is
        TextStyle::Name(name)
            if name == &"ContentHeading".into()
                || name == &"ContentHeading2".into()
                || name == &"ContentHeading3".into()
                || name == &"ContentHeading4".into()
                || name == &"ContentHeading5".into()
                || name == &"ContentHeading6".into() =>
        {
            text_style.clone()
        }
        // Default: try to append "Bold" to the style name
        TextStyle::Name(name) => {
            let name_str = name.to_string();
            if name_str.ends_with("Bold") {
                text_style.clone()
            } else {
                TextStyle::Name(format!("{name_str}Bold").into())
            }
        }
        // For standard text styles, we can't change weight, so use .strong() color
        _ => text_style.clone(),
    }
}

/// Get the italic variant of a text style
pub(crate) fn italic_text_style(text_style: &TextStyle) -> TextStyle {
    match text_style {
        // Content body variants
        TextStyle::Name(name) if name == &"ContentBody".into() => {
            TextStyle::Name("ContentBodyItalic".into())
        }
        TextStyle::Name(name) if name == &"ContentBodyRegular".into() => {
            TextStyle::Name("ContentBodyItalic".into())
        }
        TextStyle::Name(name) if name == &"ContentBodyMedium".into() => {
            TextStyle::Name("ContentBodyItalic".into())
        }
        TextStyle::Name(name) if name == &"ContentBodyBold".into() => {
            // Bold + Italic - we don't have this variant, use italic for now
            TextStyle::Name("ContentBodyItalic".into())
        }
        // For other styles, we don't have italic variants
        _ => text_style.clone(),
    }
}

/// GitHub-inspired spacing constants for consistent markdown rendering
/// Based on GitHub's markdown CSS: <https://github.com/sindresorhus/github-markdown-css>
mod spacing {
    /// Base font size in pixels (GitHub: 16px)
    pub const BASE_FONT_SIZE: f32 = 16.0;

    /// Paragraph bottom margin (GitHub: 10px, but using 16px for better visual separation)
    pub const PARAGRAPH_BOTTOM: f32 = 16.0;

    /// Heading top margin for all headings (GitHub: 24px, but using 16px for better spacing)
    pub const HEADING_TOP: f32 = 16.0;

    /// Heading bottom margin for all headings (GitHub: 16px)
    pub const HEADING_BOTTOM: f32 = 16.0;

    /// List item spacing (GitHub: 0.25em of 16px = 4px)
    pub const LIST_ITEM_SPACING: f32 = BASE_FONT_SIZE * 0.25;

    /// Blockquote bottom margin (GitHub: 16px)
    pub const BLOCKQUOTE_BOTTOM: f32 = 16.0;

    /// Code block bottom margin (GitHub: 16px)
    pub const CODE_BLOCK_BOTTOM: f32 = 16.0;

    /// Horizontal rule spacing (GitHub: 24px top and bottom)
    pub const HORIZONTAL_RULE_SPACING: f32 = 24.0;
}

use spacing::*;

/// Debug flag for baseline visualization
/// Set to true to enable debug lines and numeric overlay
const DEBUG_BASELINE: bool = false;

/// Ascent ratio for baseline estimation (76% = 0.76)
/// This is the estimated fraction of row height where text baseline is located
/// Adjust this based on visual alignment testing
/// 0.76 provides perfect baseline alignment based on visual testing
const ASCENT_RATIO: f32 = 0.76;

/// Maximum height factor for inline images relative to text height
/// If an image is taller than this factor × `text_height`, it will be scaled down
/// This prevents tall images from disrupting line spacing and text alignment
const MAX_HEIGHT_FACTOR: f32 = 1.0;

/// Render an image with baseline alignment
pub(crate) fn render_baseline_aligned_image(
    ui: &mut Ui,
    image_source: ImageSource<'static>,
    mut image_size: egui::Vec2,
    baseline_from_top: f32,
) {
    // Get text metrics (estimated)
    // Use configurable ascent ratio for baseline estimation
    let text_height = ui.text_style_height(&TextStyle::Name("ContentBody".into()));
    let estimated_ascent = text_height * ASCENT_RATIO;

    // Calculate offset accounting for vertical centering in horizontal_wrapped
    // Both text and image widgets are centered vertically in the row
    // text_baseline_y = center_y - (text_height/2) + ascent
    // svg_baseline_y = center_y - (image_height/2) + svg_baseline_from_top
    // offset_y = text_baseline_y - svg_baseline_y
    let text_baseline_offset = estimated_ascent - (text_height / 2.0);
    let image_baseline_offset = baseline_from_top - (image_size.y / 2.0);
    let mut offset_y = text_baseline_offset - image_baseline_offset;

    // Handle tall SVGs: if image is too tall, discard baseline offset
    // This prevents tall images from disrupting line spacing
    let max_height = text_height * MAX_HEIGHT_FACTOR;
    let mut scaled = false;
    let mut offset_discarded = false;

    if image_size.y > max_height {
        // Image is too tall - discard baseline offset
        offset_y = 0.0;
        offset_discarded = true;

        // If still too tall after discarding offset, scale image proportionally
        // Note: image_size.y hasn't changed yet, so we check original size
        let original_height = image_size.y;
        if original_height > max_height {
            let scale_factor = max_height / original_height;
            let scaled_size = image_size * scale_factor;

            // Recalculate with scaled size
            let scaled_image_baseline_offset =
                baseline_from_top * scale_factor - (scaled_size.y / 2.0);
            offset_y = text_baseline_offset - scaled_image_baseline_offset;

            // Use scaled size for allocation
            image_size = scaled_size;
            scaled = true;
        }
    }

    // Allocate space for image
    let (rect, _) = ui.allocate_exact_size(image_size, Sense::hover());

    // Create image with tint
    let image = egui::Image::new(image_source)
        .fit_to_exact_size(image_size)
        .tint(ui.visuals().text_color())
        .corner_radius(0.0);

    // Draw image with offset
    let translated_rect = rect.translate(egui::Vec2::new(0.0, offset_y));
    image.paint_at(ui, translated_rect);

    // DEBUG: Draw baselines and visualization
    if DEBUG_BASELINE {
        let row_center_y = rect.center().y;
        let text_baseline_y = row_center_y + text_baseline_offset;
        let svg_baseline_y = row_center_y + image_baseline_offset + offset_y;

        // 1. Text baseline (red) - estimated text baseline position
        ui.painter().line_segment(
            [
                egui::Pos2::new(rect.left() - 10.0, text_baseline_y),
                egui::Pos2::new(rect.right() + 10.0, text_baseline_y),
            ],
            (1.0, egui::Color32::RED),
        );

        // 2. SVG baseline (green) - where SVG baseline actually is
        ui.painter().line_segment(
            [
                egui::Pos2::new(rect.left() - 5.0, svg_baseline_y),
                egui::Pos2::new(rect.right() + 5.0, svg_baseline_y),
            ],
            (1.0, egui::Color32::GREEN),
        );

        // 3. Image bounds (blue) - allocated space before offset
        ui.painter().rect_stroke(
            rect,
            0.0,
            egui::Stroke::new(1.0, egui::Color32::BLUE),
            egui::StrokeKind::Outside,
        );

        // 4. Actual text baseline (yellow) - where text baseline should be (accounting for centering)
        ui.painter().line_segment(
            [
                egui::Pos2::new(rect.left() - 15.0, text_baseline_y),
                egui::Pos2::new(rect.right() + 15.0, text_baseline_y),
            ],
            (1.5, egui::Color32::YELLOW),
        );

        // 5. Row center line (magenta) - where widgets are centered vertically
        ui.painter().line_segment(
            [
                egui::Pos2::new(rect.left() - 20.0, row_center_y),
                egui::Pos2::new(rect.right() + 20.0, row_center_y),
            ],
            (1.0, egui::Color32::from_rgb(255, 0, 255)), // Magenta
        );

        // 6. Text widget bounds (cyan) - estimated text widget area
        let text_widget_height = text_height;
        let text_widget_rect = Rect::from_center_size(
            egui::Pos2::new(rect.center().x, row_center_y),
            egui::Vec2::new(rect.width(), text_widget_height),
        );
        ui.painter().rect_stroke(
            text_widget_rect,
            0.0,
            egui::Stroke::new(1.0, egui::Color32::CYAN),
            egui::StrokeKind::Outside,
        );

        // 7. Numeric overlay for debugging
        let scaling_info = if scaled {
            format!("SCALED (max: {max_height:.1})")
        } else if offset_discarded {
            "OFFSET DISCARDED".to_owned()
        } else {
            String::new()
        };

        let debug_text = format!(
            "offset: {:.2}\nascent: {:.2} ({}%)\ntext_h: {:.2}\nimg_h: {:.2}\nsvg_base: {:.2}\n{}",
            offset_y,
            estimated_ascent,
            (ASCENT_RATIO * 100.0) as i32,
            text_height,
            image_size.y,
            baseline_from_top,
            scaling_info
        );
        ui.painter().text(
            rect.left_bottom() + egui::Vec2::new(0.0, 5.0),
            egui::Align2::LEFT_TOP,
            debug_text,
            egui::FontId::monospace(10.0),
            egui::Color32::WHITE,
        );
    }
}

/// Content that can appear within a paragraph
impl Clone for ParagraphContent {
    fn clone(&self) -> Self {
        match self {
            Self::Text(arg0) => Self::Text(arg0.clone()),
            Self::MathImage {
                image_source,
                size,
                is_display,
                baseline_from_top,
            } => Self::MathImage {
                image_source: image_source.clone(),
                size: *size,
                is_display: *is_display,
                baseline_from_top: *baseline_from_top,
            },
            Self::MathCode {
                content,
                is_display,
            } => Self::MathCode {
                content: content.clone(),
                is_display: *is_display,
            },
            Self::InlineCode(arg0) => Self::InlineCode(arg0.clone()),
            Self::Strong(arg0) => Self::Strong(arg0.clone()),
            Self::Emphasis(arg0) => Self::Emphasis(arg0.clone()),
            Self::Link { text, url } => Self::Link {
                text: text.clone(),
                url: url.clone(),
            },
            Self::Strikethrough(arg0) => Self::Strikethrough(arg0.clone()),
            Self::Widget {
                name,
                config,
                width,
                height,
            } => Self::Widget {
                name: name.clone(),
                config: config.clone(),
                width: *width,
                height: *height,
            },
        }
    }
}

#[derive(Debug)]
pub enum ParagraphContent {
    Text(String),
    MathImage {
        image_source: ImageSource<'static>,
        size: egui::Vec2,
        is_display: bool,
        baseline_from_top: Option<f32>,
    },
    MathCode {
        content: String,
        is_display: bool,
    },
    InlineCode(String),
    Strong(String),
    Emphasis(String),
    Link {
        text: String,
        url: String,
    },
    Strikethrough(String),
    Widget {
        name: String,
        config: WidgetConfig,
        width: Option<f32>,
        height: Option<f32>,
    },
}

/// Extract math formulas from text and replace with (hash.typ) placeholders
/// Returns text with placeholders
pub fn extract_and_replace_math_formulas(
    text: &str,
    manifest: &crate::math::MathManifest,
) -> String {
    let mut result = String::with_capacity(text.len());
    let mut i = 0;
    let chars: Vec<char> = text.chars().collect();

    while i < chars.len() {
        if chars[i] == '$' {
            // Save the original formula text for fallback
            let formula_start = i;

            // Check if this is Typst math
            let mut j = i + 1;
            let mut is_display = false;

            // Check if there's a space after the opening $ (Typst display math)
            if j < chars.len() && chars[j] == ' ' {
                is_display = true;
                j += 1; // Skip the space
            }

            // Find closing $
            while j < chars.len() && chars[j] != '$' {
                j += 1;
            }

            if j < chars.len() && chars[j] == '$' {
                let formula_start_idx = if is_display { i + 2 } else { i + 1 };
                let formula_end_idx = if is_display && j > 0 && chars[j - 1] == ' ' {
                    j - 1 // Exclude the space before closing $
                } else {
                    j
                };

                let formula: String = chars[formula_start_idx..formula_end_idx].iter().collect();
                let formula = formula.trim();

                if !formula.is_empty() {
                    // Look up hash in manifest
                    if let Some(hash) = manifest.find_hash(formula, is_display) {
                        // Always add parentheses around placeholder for consistent parsing
                        // The rendering logic will handle nested parentheses
                        result.push_str(&format!("({hash}.typ)"));

                        i = j + 1;
                        continue;
                    }
                }
            }

            // If we get here, formula extraction failed or hash not found
            // Copy the original formula text as fallback
            for ch in chars.iter().take(i + 1).skip(formula_start) {
                result.push(*ch);
            }
        }

        // Not a formula (or failed formula), copy the character
        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Render preprocessed markdown content to an egui UI with math support.
/// This function accepts markdown that has already been processed with math placeholders.
pub fn render_cached_preprocessed_markdown(
    ui: &mut Ui,
    cached_events: &[SpannedEvent],
    math_asset_manager: Option<&mut crate::math::MathAssetManager>,
    math_resolution_scale: f32,
    fragment_to_scroll_to: Option<&str>,
    text_segment_cache: &mut crate::ui::text_cache::TextSegmentCache,
) {
    render_markdown_with_cache_impl(
        ui,
        cached_events,
        math_asset_manager,
        math_resolution_scale,
        fragment_to_scroll_to,
        text_segment_cache,
    );
}

fn render_markdown_with_cache_impl(
    ui: &mut Ui,
    cached_events: &[SpannedEvent],
    mut math_asset_manager: Option<&mut crate::math::MathAssetManager>,
    math_resolution_scale: f32,
    fragment_to_scroll_to: Option<&str>,
    text_segment_cache: &mut crate::ui::text_cache::TextSegmentCache,
) {
    // Load manifest for metadata lookup (needed for both preprocessed and raw content)
    let manifest = crate::math::load_manifest();

    // Set vertical spacing to 0 so we have full control over spacing
    // This prevents default egui spacing from adding to our GitHub-inspired spacing
    ui.spacing_mut().item_spacing.y = 0.0;

    // Simplified margin collapsing: track previous element's bottom margin
    let mut previous_bottom_margin = 0.0;

    // Track heading IDs for duplicate detection and scrolling
    let mut heading_ids = std::collections::HashSet::new();
    let mut scroll_to_heading_requested = false;

    // Helper function to add bottom margin and track it
    fn add_bottom_margin(ui: &mut Ui, previous_bottom: &mut f32, margin: f32) {
        ui.add_space(margin);
        *previous_bottom = margin;
    }

    // Helper function for margin collapsing at element start
    fn add_top_margin_with_collapsing(ui: &mut Ui, previous_bottom: &f32, top_margin: f32) {
        let spacing_to_add = top_margin.max(*previous_bottom) - *previous_bottom;
        if spacing_to_add > 0.0 {
            ui.add_space(spacing_to_add);
        }
        // Don't reset previous_bottom here - it will be updated when element adds its bottom margin
    }

    // State for accumulating paragraph content
    let mut in_paragraph = false;
    let mut paragraph_content = Vec::new();

    let mut events = cached_events.iter();

    while let Some(SpannedEvent { event, .. }) = events.next() {
        match event {
            OwnedEvent::Start(tag) => {
                match tag {
                    OwnedTag::Paragraph => {
                        // Paragraphs don't have top margin in GitHub's CSS
                        // Spacing comes from previous element's bottom margin
                        in_paragraph = true;
                        paragraph_content.clear();
                    }
                    OwnedTag::Heading(level, _, _) => {
                        // Headings
                        let mut heading_text = String::new();
                        for SpannedEvent { event, .. } in events.by_ref() {
                            match event {
                                OwnedEvent::End(OwnedTag::Heading(_, _, _)) => break,
                                OwnedEvent::Text(text) => heading_text.push_str(text),
                                OwnedEvent::SoftBreak => heading_text.push(' '),
                                OwnedEvent::HardBreak => heading_text.push('\n'),
                                _ => {} // Skip other events (code, html, etc.)
                            }
                        }

                        // All headings have the same top margin in GitHub's CSS
                        let top_margin = HEADING_TOP;

                        // Apply margin collapsing for heading top margin
                        add_top_margin_with_collapsing(ui, &previous_bottom_margin, top_margin);

                        // Generate heading ID
                        let heading_id = crate::posts::BlogPost::generate_heading_id(
                            &heading_text,
                            &mut heading_ids,
                        );

                        // Check if we should scroll to this heading
                        let should_scroll_to_heading = fragment_to_scroll_to
                            .map(|fragment| fragment == heading_id)
                            .unwrap_or(false);

                        // Process text with math placeholders
                        let paragraph_content = process_text_with_math_cached(
                            &heading_text,
                            manifest,
                            &mut math_asset_manager,
                            math_resolution_scale,
                            text_segment_cache,
                        );

                        // Determine text style based on heading level
                        let text_style = match level {
                            HeadingLevel::H1 => TextStyle::Name("ContentHeading".into()),
                            HeadingLevel::H2 => TextStyle::Name("ContentHeading2".into()),
                            HeadingLevel::H3 => TextStyle::Name("ContentHeading3".into()),
                            HeadingLevel::H4 => TextStyle::Name("ContentHeading4".into()),
                            HeadingLevel::H5 => TextStyle::Name("ContentHeading5".into()),
                            HeadingLevel::H6 => TextStyle::Name("ContentHeading6".into()),
                        };

                        // Create a heading area with the ID
                        let mut heading_response = ui.vertical(|ui| {
                            render_paragraph_content_vec(ui, &paragraph_content, &text_style);
                        });

                        // Store the heading ID in the response for potential scrolling
                        heading_response.response.id = heading_id.clone().into();

                        // Scroll to this heading if requested
                        if should_scroll_to_heading && !scroll_to_heading_requested {
                            ui.scroll_to_rect(
                                heading_response.response.rect,
                                Some(egui::Align::Center),
                            );
                            scroll_to_heading_requested = true;
                        }

                        // Add bottom border for h1 and h2 (GitHub style)
                        match level {
                            HeadingLevel::H1 | HeadingLevel::H2 => {
                                ui.add_space(7.2); // GitHub: 0.3em padding-bottom (24px * 0.3 = 7.2px for h2)
                                ui.separator();
                            }
                            _ => {}
                        }

                        // All headings have the same bottom margin in GitHub's CSS
                        let bottom_margin = HEADING_BOTTOM;

                        // Add heading bottom margin and track it
                        add_bottom_margin(ui, &mut previous_bottom_margin, bottom_margin);
                    }
                    OwnedTag::List(ordered) => {
                        // Lists don't have top margin in GitHub's CSS
                        // Spacing comes from previous element's bottom margin

                        // Lists
                        let mut list_items = Vec::new();
                        while let Some(SpannedEvent { event, .. }) = events.next() {
                            match event {
                                OwnedEvent::End(OwnedTag::List(_)) => break,
                                OwnedEvent::Start(OwnedTag::Item) => {
                                    let mut item_text = String::new();
                                    for SpannedEvent { event, .. } in events.by_ref() {
                                        match event {
                                            OwnedEvent::End(OwnedTag::Item) => break,
                                            OwnedEvent::Text(text) => item_text.push_str(text),
                                            OwnedEvent::SoftBreak => item_text.push(' '),
                                            OwnedEvent::HardBreak => item_text.push('\n'),
                                            _ => {} // Skip other events for now
                                        }
                                    }
                                    if !item_text.is_empty() {
                                        list_items.push(item_text);
                                    }
                                }
                                _ => {} // Skip other events
                            }
                        }

                        let row_height =
                            ui.text_style_height(&TextStyle::Name("ContentBody".into()));
                        let one_indent = row_height / 2.0;

                        for (i, item) in list_items.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 0.0;
                                ui.set_row_height(row_height);
                                // Add indentation for the list
                                ui.add_space(one_indent);

                                if let Some(start) = ordered {
                                    let number = (start + i as u64).to_string();
                                    // Render number as text label (part of the text flow)
                                    ui.label(RichText::new(format!("{number}.")));
                                } else {
                                    // Render bullet as text character (•) instead of drawn circle
                                    ui.label(RichText::new("•"));
                                }
                                ui.add_space(one_indent / 3.0);

                                // Process text with math placeholders
                                let paragraph_content = process_text_with_math_cached(
                                    item,
                                    manifest,
                                    &mut math_asset_manager,
                                    math_resolution_scale,
                                    text_segment_cache,
                                );

                                render_paragraph_content_vec(
                                    ui,
                                    &paragraph_content,
                                    &TextStyle::Name("ContentBody".into()),
                                );
                            });

                            // Add spacing between list items (GitHub: 0.25em = 4px)
                            if i < list_items.len() - 1 {
                                ui.add_space(LIST_ITEM_SPACING);
                            }
                        }

                        // Add list bottom margin (same as paragraph) and track it
                        add_bottom_margin(ui, &mut previous_bottom_margin, PARAGRAPH_BOTTOM);
                    }
                    OwnedTag::Item => {
                        // Already handled in List
                    }
                    OwnedTag::CodeBlock(kind) => {
                        // Code blocks don't have top margin in GitHub's CSS
                        // Spacing comes from previous element's bottom margin

                        // Code blocks
                        let mut code_text = String::new();
                        for SpannedEvent { event, .. } in events.by_ref() {
                            match event {
                                OwnedEvent::End(OwnedTag::CodeBlock(_)) => break,
                                OwnedEvent::Text(text) => code_text.push_str(text),
                                OwnedEvent::SoftBreak | OwnedEvent::HardBreak => {
                                    code_text.push('\n');
                                }
                                _ => {} // Skip other events
                            }
                        }

                        // Display language label if present
                        let language = match kind {
                            OwnedCodeBlockKind::Fenced(lang) if !lang.is_empty() => {
                                Some(lang.clone())
                            }
                            _ => None,
                        };

                        if let Some(lang) = &language {
                            ui.horizontal(|ui| {
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Min),
                                    |ui| {
                                        ui.label(RichText::new(lang).small().weak());
                                    },
                                );
                            });
                        }

                        // Syntax highlighting
                        let theme = CodeTheme::from_style(ui.style());

                        // Map common language names to syntect recognized names
                        let lang_str = language.as_deref().unwrap_or("");
                        let mapped_lang = match lang_str.to_lowercase().as_str() {
                            "rust" | "rs" => "rs",
                            "javascript" | "js" => "js",
                            "python" | "py" => "py",
                            "typescript" | "ts" => "ts",
                            "cpp" | "c++" => "cpp",
                            "c" => "c",
                            "java" => "java",
                            "go" => "go",
                            "html" => "html",
                            "css" => "css",
                            "bash" | "sh" | "shell" => "bash",
                            "json" => "json",
                            "toml" => "toml",
                            "yaml" | "yml" => "yaml",
                            "markdown" | "md" => "markdown",
                            _ => lang_str,
                        };

                        let layout_job =
                            highlight(ui.ctx(), ui.style(), &theme, &code_text, mapped_lang);

                        // Display with background (EasyMark style)
                        let where_to_put_background = ui.painter().add(Shape::Noop);
                        let response = ui.add(egui::Label::new(layout_job).selectable(true));
                        let mut rect = response.rect;
                        rect = rect.expand(1.0); // looks better
                        rect.max.x = ui.max_rect().max.x;
                        let code_bg_color = ui.visuals().code_bg_color;
                        ui.painter().set(
                            where_to_put_background,
                            Shape::rect_filled(rect, 1.0, code_bg_color),
                        );
                        // Add code block bottom margin and track it
                        add_bottom_margin(ui, &mut previous_bottom_margin, CODE_BLOCK_BOTTOM);
                    }
                    OwnedTag::Strong => {
                        // Bold text
                        let mut bold_text = String::new();
                        for SpannedEvent { event, .. } in events.by_ref() {
                            match event {
                                OwnedEvent::End(OwnedTag::Strong) => break,
                                OwnedEvent::Text(text) => bold_text.push_str(text),
                                OwnedEvent::SoftBreak => bold_text.push(' '),
                                _ => {} // Skip other events
                            }
                        }
                        if in_paragraph {
                            paragraph_content.push(ParagraphContent::Strong(bold_text));
                        } else {
                            // Process text with math placeholders
                            let paragraph_content_vec = process_text_with_math_cached(
                                &bold_text,
                                manifest,
                                &mut math_asset_manager,
                                math_resolution_scale,
                                text_segment_cache,
                            );

                            // Render with bold styling (uses bold font, no need for .strong() color)
                            render_paragraph_content_vec(
                                ui,
                                &paragraph_content_vec,
                                &TextStyle::Name("ContentBody".into()),
                            );
                        }
                    }
                    OwnedTag::Emphasis => {
                        // Italic text
                        let mut italic_text = String::new();
                        for SpannedEvent { event, .. } in events.by_ref() {
                            match event {
                                OwnedEvent::End(OwnedTag::Emphasis) => break,
                                OwnedEvent::Text(text) => italic_text.push_str(text),
                                OwnedEvent::SoftBreak => italic_text.push(' '),
                                _ => {} // Skip other events
                            }
                        }
                        if in_paragraph {
                            paragraph_content.push(ParagraphContent::Emphasis(italic_text));
                        } else {
                            // Process text with math placeholders
                            let paragraph_content_vec = process_text_with_math_cached(
                                &italic_text,
                                manifest,
                                &mut math_asset_manager,
                                math_resolution_scale,
                                text_segment_cache,
                            );

                            // Render with italic styling
                            render_paragraph_content_vec(
                                ui,
                                &paragraph_content_vec,
                                &TextStyle::Name("ContentBody".into()),
                            );
                        }
                    }
                    OwnedTag::Link(_, url, _) => {
                        // Links
                        let url = url.clone();
                        let mut link_text = String::new();
                        for SpannedEvent { event, .. } in events.by_ref() {
                            match event {
                                OwnedEvent::End(OwnedTag::Link(_, _, _)) => break,
                                OwnedEvent::Text(text) => link_text.push_str(text),
                                OwnedEvent::SoftBreak => link_text.push(' '),
                                _ => {} // Skip other events
                            }
                        }

                        if in_paragraph {
                            paragraph_content.push(ParagraphContent::Link {
                                text: link_text,
                                url,
                            });
                        } else {
                            // Process text with math placeholders
                            let paragraph_content_vec = process_text_with_math_cached(
                                &link_text,
                                manifest,
                                &mut math_asset_manager,
                                math_resolution_scale,
                                text_segment_cache,
                            );

                            // For links outside paragraphs, we need to handle them differently
                            // since Hyperlink doesn't support rich text with math
                            // For now, render as plain text with link styling
                            ui.horizontal_wrapped(|ui| {
                                ui.spacing_mut().item_spacing.x = 0.0;
                                for item in paragraph_content_vec {
                                    match item {
                                        ParagraphContent::Text(text) => {
                                            ui.add(Hyperlink::from_label_and_url(&text, &url));
                                        }
                                        ParagraphContent::MathImage {
                                            image_source,
                                            size,
                                            is_display: _,
                                            baseline_from_top,
                                        } => {
                                            // Render math image (same for display and inline in links)
                                            if let Some(baseline) = baseline_from_top {
                                                render_baseline_aligned_image(
                                                    ui,
                                                    image_source.clone(),
                                                    size,
                                                    baseline,
                                                );
                                            } else {
                                                let image = egui::Image::new(image_source.clone())
                                                    .tint(ui.visuals().text_color())
                                                    .fit_to_exact_size(size)
                                                    .corner_radius(0.0);
                                                ui.add(image);
                                            }
                                        }
                                        ParagraphContent::MathCode {
                                            content,
                                            is_display: _,
                                        } => {
                                            ui.label(RichText::new(content).code());
                                        }
                                        ParagraphContent::Widget { .. } => {
                                            // Widgets in links not supported - skip
                                        }
                                        _ => {
                                            // Other content types in links - render as text
                                            let text = match item {
                                                ParagraphContent::Strong(t)
                                                | ParagraphContent::Emphasis(t)
                                                | ParagraphContent::Strikethrough(t)
                                                | ParagraphContent::InlineCode(t) => t,
                                                _ => String::new(),
                                            };
                                            if !text.is_empty() {
                                                ui.add(Hyperlink::from_label_and_url(&text, &url));
                                            }
                                        }
                                    }
                                }
                            });
                        }
                    }
                    OwnedTag::Strikethrough => {
                        // Strikethrough text
                        let mut strike_text = String::new();
                        for SpannedEvent { event, .. } in events.by_ref() {
                            match event {
                                OwnedEvent::End(OwnedTag::Strikethrough) => break,
                                OwnedEvent::Text(text) => strike_text.push_str(text),
                                OwnedEvent::SoftBreak => strike_text.push(' '),
                                _ => {} // Skip other events
                            }
                        }
                        if in_paragraph {
                            paragraph_content.push(ParagraphContent::Strikethrough(strike_text));
                        } else {
                            // Process text with math placeholders
                            let paragraph_content_vec = process_text_with_math_cached(
                                &strike_text,
                                manifest,
                                &mut math_asset_manager,
                                math_resolution_scale,
                                text_segment_cache,
                            );

                            // Render with strikethrough styling
                            render_paragraph_content_vec(
                                ui,
                                &paragraph_content_vec,
                                &TextStyle::Name("ContentBody".into()),
                            );
                        }
                    }
                    OwnedTag::BlockQuote => {
                        // Blockquotes don't have top margin in GitHub's CSS
                        // Spacing comes from previous element's bottom margin

                        // Collect all text from the blockquote (simple approach for now)
                        let mut quote_text = String::new();
                        for SpannedEvent { event, .. } in events.by_ref() {
                            match event {
                                OwnedEvent::End(OwnedTag::BlockQuote) => break,
                                OwnedEvent::Text(text) => quote_text.push_str(text),
                                OwnedEvent::SoftBreak | OwnedEvent::HardBreak => {
                                    quote_text.push('\n');
                                }
                                _ => {} // Skip other events for now
                            }
                        }

                        // Trim trailing whitespace
                        let quote_text = quote_text.trim_end();

                        if !quote_text.is_empty() {
                            // Calculate dimensions for blockquote
                            let row_height =
                                ui.text_style_height(&TextStyle::Name("ContentBody".into()));
                            let border_width = 4.0; // GitHub-style 4px solid border
                            let horizontal_padding = row_height; // One row height of padding
                            let vertical_padding = row_height * 0.5; // Half row height vertical padding

                            // Create the blockquote layout
                            ui.horizontal(|ui| {
                                // We'll paint the border after we know the total height
                                // First, allocate a placeholder for the border
                                let border_id = ui.id().with("blockquote_border");
                                let border_placeholder = ui.allocate_response(
                                    vec2(border_width, 0.0), // 0 height initially
                                    Sense::hover(),
                                );

                                // Store the border position (we'll recreate the rect with correct height later)
                                ui.data_mut(|data| {
                                    data.insert_temp(border_id, border_placeholder.rect.min);
                                });

                                // Add padding between border and text
                                ui.add_space(horizontal_padding - border_width);

                                // Render quote text with proper padding and color
                                ui.vertical(|ui| {
                                    ui.add_space(vertical_padding);

                                    // Process text with math placeholders
                                    let paragraph_content = process_text_with_math_cached(
                                        quote_text,
                                        manifest,
                                        &mut math_asset_manager,
                                        math_resolution_scale,
                                        text_segment_cache,
                                    );

                                    // Render with weak text color
                                    ui.scope(|ui| {
                                        ui.style_mut().visuals.override_text_color =
                                            Some(ui.visuals().weak_text_color());
                                        render_paragraph_content_vec(
                                            ui,
                                            &paragraph_content,
                                            &TextStyle::Name("ContentBody".into()),
                                        );
                                    });

                                    ui.add_space(vertical_padding);
                                });

                                // Now we know the total height, update and paint the border
                                let total_height = ui.min_rect().height();
                                if let Some(border_pos) =
                                    ui.data_mut(|data| data.get_temp::<Pos2>(border_id))
                                {
                                    // Create the border rect using the stored position and calculated height
                                    let mut border_rect = Rect::from_min_size(
                                        border_pos,
                                        vec2(border_width, total_height),
                                    );

                                    // Center the border vertically with the content
                                    // The placeholder was allocated at the top, but we want it centered
                                    let content_top = ui.min_rect().top();
                                    border_rect.set_top(content_top);

                                    ui.painter().rect_filled(
                                        border_rect,
                                        0.0,
                                        ui.visuals().weak_text_color(),
                                    );
                                }
                            });
                        }

                        // Add blockquote bottom margin and track it
                        add_bottom_margin(ui, &mut previous_bottom_margin, BLOCKQUOTE_BOTTOM);
                    }
                    OwnedTag::FootnoteDefinition(_) => {
                        // Skip footnotes for now
                        for SpannedEvent { event, .. } in events.by_ref() {
                            if matches!(event, OwnedEvent::End(OwnedTag::FootnoteDefinition(_))) {
                                break;
                            }
                        }
                    }
                    OwnedTag::Table(alignments) => {
                        // Tables don't have top margin in GitHub's CSS
                        // Spacing comes from previous element's bottom margin

                        let (headers, rows) = parse_table(&mut events, alignments);
                        table_renderer::render_table(
                            ui,
                            alignments,
                            &headers,
                            &rows,
                            &TableConfig::default(),
                            &mut math_asset_manager,
                            math_resolution_scale,
                            text_segment_cache,
                        );

                        // Add table bottom margin (same as paragraph) and track it
                        add_bottom_margin(ui, &mut previous_bottom_margin, PARAGRAPH_BOTTOM);
                    }
                    OwnedTag::TableHead | OwnedTag::TableRow | OwnedTag::TableCell => {
                        // Skip table elements that appear outside a table (should not happen)
                        for SpannedEvent { event, .. } in events.by_ref() {
                            if matches!(
                                event,
                                OwnedEvent::End(
                                    OwnedTag::TableHead | OwnedTag::TableRow | OwnedTag::TableCell
                                )
                            ) {
                                break;
                            }
                        }
                    }
                    OwnedTag::Image(_, url, _) => {
                        let url_str = url.clone();

                        // Check if this is a widget (ends with .rs)
                        if url_str.contains(".rs") {
                            // Parse widget name and query parameters
                            let (widget_name, query_params) = if let Some(pos) = url_str.find('?') {
                                let name_part = &url_str[..pos];
                                let query_part = &url_str[pos + 1..];
                                (
                                    name_part.trim_end_matches(".rs").to_owned(),
                                    Some(query_part.to_owned()),
                                )
                            } else {
                                (url_str.trim_end_matches(".rs").to_owned(), None)
                            };

                            let mut alt_text = String::new();

                            for SpannedEvent { event, .. } in events.by_ref() {
                                match event {
                                    OwnedEvent::End(OwnedTag::Image(_, _, _)) => break,
                                    OwnedEvent::Text(text) => alt_text.push_str(text),
                                    OwnedEvent::SoftBreak => alt_text.push(' '),
                                    _ => {} // Skip other events
                                }
                            }

                            // Parse configuration from query parameters
                            let mut config = WidgetConfig::default();
                            let mut width = None;
                            let mut height = None;

                            if let Some(query) = query_params {
                                // Parse simple width/height parameters
                                for param in query.split('&') {
                                    if let Some((key, value)) = param.split_once('=') {
                                        match key {
                                            "width" => {
                                                if let Ok(w) = value.parse::<f32>() {
                                                    width = Some(w);
                                                }
                                            }
                                            "height" => {
                                                if let Ok(h) = value.parse::<f32>() {
                                                    height = Some(h);
                                                }
                                            }
                                            _ => {
                                                // Try to parse as JSON configuration
                                                if key.is_empty() && value.starts_with('{') {
                                                    // JSON config like ?{"type":"sine"}
                                                    if let Ok(json_value) =
                                                        serde_json::from_str(value)
                                                    {
                                                        config.config = json_value;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if in_paragraph {
                                paragraph_content.push(ParagraphContent::Widget {
                                    name: widget_name,
                                    config,
                                    width,
                                    height,
                                });
                            } else {
                                // Widget outside paragraph - render directly
                                ui.horizontal(|ui| {
                                    // Apply size constraints to config before creating instance
                                    let mut final_config = config;
                                    if let Some(w) = width {
                                        final_config.width = Some(w);
                                    }
                                    if let Some(h) = height {
                                        final_config.height = Some(h);
                                    }

                                    match crate::widgets::WidgetInstance::new(
                                        &widget_name,
                                        final_config,
                                    ) {
                                        Ok(mut instance) => {
                                            let _ = instance.render(ui);
                                        }
                                        Err(e) => {
                                            ui.label(format!("Widget error: {e}",));
                                        }
                                    }
                                });
                            }
                        } else {
                            // Regular image - display alt text as placeholder
                            let mut alt_text = String::new();
                            for SpannedEvent { event, .. } in events.by_ref() {
                                match event {
                                    OwnedEvent::End(OwnedTag::Image(_, _, _)) => break,
                                    OwnedEvent::Text(text) => alt_text.push_str(text),
                                    OwnedEvent::SoftBreak => alt_text.push(' '),
                                    _ => {} // Skip other events
                                }
                            }
                            ui.label(
                                RichText::new(format!("[Image: {alt_text}]"))
                                    .italics()
                                    .weak(),
                            );
                        }
                    }
                }
            }
            OwnedEvent::End(tag) => {
                if *tag == OwnedTag::Paragraph {
                    if in_paragraph && !paragraph_content.is_empty() {
                        // Render the accumulated paragraph content in a horizontal layout
                        ui.horizontal_wrapped(|ui| {
                            // Remove horizontal spacing between inline elements
                            // This eliminates excessive spacing between text and math images
                            ui.spacing_mut().item_spacing.x = 0.0;

                            for content in &paragraph_content {
                                render_paragraph_content(ui, content);
                            }
                        });
                        // Add paragraph bottom margin and track it
                        add_bottom_margin(ui, &mut previous_bottom_margin, PARAGRAPH_BOTTOM);
                        paragraph_content.clear();
                    }
                    in_paragraph = false;
                } else {
                    // Other end tags are handled within Start match
                }
            }
            OwnedEvent::Text(text) => {
                if in_paragraph {
                    // Accumulate text content for paragraph rendering
                    accumulate_text_content_cached(
                        text,
                        manifest,
                        &mut math_asset_manager,
                        &mut paragraph_content,
                        math_resolution_scale,
                        text_segment_cache,
                    );
                } else {
                    // Fallback for text outside paragraphs (shouldn't happen in proper markdown)
                    // No spacing for standalone text

                    // Check for math placeholders in the text (format: (hash.typ))
                    let mut remaining = &text[..];
                    let _ = 0;

                    while let Some(start) = remaining.find('(') {
                        // Render text before the placeholder
                        if start > 0 {
                            let before_text = &remaining[..start];
                            render_text_with_latex(
                                ui,
                                before_text,
                                &mut math_asset_manager,
                                math_resolution_scale,
                            );
                        }

                        // Find the end of the placeholder - look for closing ')'
                        if let Some(end) = remaining[start..].find(')') {
                            let placeholder = &remaining[start..=start + end];

                            // Check if this is a math placeholder: contains (hash.typ)
                            // It could be nested like ((hash.typ)), so we need to find the .typ) pattern
                            if let Some(typ_start) = placeholder.find(".typ)") {
                                // Extract the part from the opening '(' before .typ) to the end
                                // Find the '(' that starts the math placeholder
                                let mut paren_start = typ_start;
                                while paren_start > 0
                                    && placeholder.chars().nth(paren_start - 1) != Some('(')
                                {
                                    paren_start -= 1;
                                }

                                if paren_start > 0
                                    && placeholder.chars().nth(paren_start - 1) == Some('(')
                                {
                                    // We found the opening '(' for the math placeholder
                                    // Render any text before the math placeholder (e.g., the first '(' in "((hash.typ))")
                                    if paren_start - 1 > 0 {
                                        let before_math = &placeholder[..paren_start - 1];
                                        render_text_with_latex(
                                            ui,
                                            before_math,
                                            &mut math_asset_manager,
                                            math_resolution_scale,
                                        );
                                    }

                                    let math_placeholder =
                                        &placeholder[paren_start - 1..=typ_start + 4]; // +4 for ".typ)"
                                    let hash = &math_placeholder[1..math_placeholder.len() - 5]; // Remove '(' and '.typ)'

                                    // Look up metadata in manifest
                                    if let Some(metadata) = manifest.get_metadata(hash) {
                                        if let Some(_asset_manager) = &mut math_asset_manager {
                                            // Try to render as SVG using hash with resolution scale
                                            if let Some(image_source) =
                                                MathAssetManager::get_image_source_for_hash_with_resolution(
                                                    hash,
                                                    math_resolution_scale,
                                                )
                                            {
                                            // Get the SVG's intrinsic size
                                            let svg_size = _asset_manager.get_svg_size(hash);

                                                if let Some(size) = svg_size {
                                                    // Size stays the same - resolution scale affects rasterization quality, not display size

                                                    if metadata.is_display {
                                                        // Display math: center with spacing
                                                        ui.add_space(8.0);
                                                        ui.horizontal(|ui| {
                                                            ui.add_space(
                                                                (ui.available_width() - size.x)
                                                                    / 2.0,
                                                            );

                                                            // Create image with crisp rendering using SVG's intrinsic size
                                                            let image = egui::Image::new(image_source)
                                                                .tint(ui.visuals().text_color()) // Theme-aware tinting
                                                                .fit_to_exact_size(size)
                                                                .corner_radius(0.0); // No rounding for crisp edges

                                                            ui.add(image);
                                                        });
                                                        ui.add_space(8.0);
                                                    } else {
                                                        // Inline math: render at SVG's intrinsic size
                                                        // Create image with crisp rendering using SVG's intrinsic size
                                                        let image = egui::Image::new(image_source)
                                                            .tint(ui.visuals().text_color()) // Theme-aware tinting
                                                            .fit_to_exact_size(size)
                                                            .corner_radius(0.0); // No rounding for crisp edges

                                                        ui.add(image);
                                                    }
                                                } else {
                                                    // Fallback: use reasonable default size if SVG size not available

                                                    if metadata.is_display {
                                                        // Display math: reasonable default
                                                        let display_size = egui::vec2(200.0, 50.0);
                                                        ui.add_space(8.0);
                                                        ui.horizontal(|ui| {
                                                            ui.add_space(
                                                                (ui.available_width() - display_size.x)
                                                                    / 2.0,
                                                            );
                                                            let image = egui::Image::new(image_source)
                                                                .tint(ui.visuals().text_color()) // Theme-aware tinting
                                                                .fit_to_exact_size(display_size)
                                                                .corner_radius(0.0);
                                                            ui.add(image);
                                                        });
                                                        ui.add_space(8.0);
                                                    } else {
                                                        // Inline math: reasonable default
                                                        let inline_size = egui::vec2(100.0, 20.0);
                                                        let image = egui::Image::new(image_source)
                                                            .tint(ui.visuals().text_color()) // Theme-aware tinting
                                                            .fit_to_exact_size(inline_size)
                                                            .corner_radius(0.0);
                                                        ui.add(image);
                                                    }
                                                }
                                            } else {
                                                // Fallback: render as code block
                                                render_math_as_code(
                                                    ui,
                                                    &format!("Math formula: {hash}"),
                                                    metadata.is_display,
                                                );
                                            }
                                        } else {
                                            // No asset manager, render as code block
                                            render_math_as_code(
                                                ui,
                                                &format!("Math formula: {hash}"),
                                                metadata.is_display,
                                            );
                                        }
                                    } else {
                                        // Hash not found in manifest, render placeholder as text
                                        ui.label(placeholder);
                                    }

                                } else {
                                    // Couldn't find opening '(' for math placeholder
                                    // Not a math placeholder, render as normal text
                                    ui.label(placeholder);
                                }
                            } else {
                                // Not a math placeholder, render as normal text
                                ui.label(placeholder);
                            }
                            // Skip past the placeholder
                            remaining = &remaining[start + end + 1..];
                        } else {
                            // No closing ')', render the '(' and continue
                            ui.label("(");
                            remaining = &remaining[start + 1..];
                        }
                    }

                    // Render any remaining text after the last placeholder
                    if !remaining.is_empty() {
                        render_text_with_latex(
                            ui,
                            remaining,
                            &mut math_asset_manager,
                            math_resolution_scale,
                        );
                    }

                    // Add bottom margin for standalone text (same as paragraph)
                    add_bottom_margin(ui, &mut previous_bottom_margin, PARAGRAPH_BOTTOM);
                }
            }
            OwnedEvent::Code(code) => {
                // Inline code
                if in_paragraph {
                    paragraph_content.push(ParagraphContent::InlineCode(code.clone()));
                } else {
                    // No spacing before standalone inline code

                    ui.label(RichText::new(code).code());

                    // Add bottom margin for standalone inline code (same as paragraph)
                    add_bottom_margin(ui, &mut previous_bottom_margin, PARAGRAPH_BOTTOM);
                }
            }
            OwnedEvent::Html(_) | OwnedEvent::FootnoteReference(_) => {
                // Skip HTML and footnotes
            }
            OwnedEvent::SoftBreak => {
                // Soft line break (treated as space)
                if in_paragraph {
                    paragraph_content.push(ParagraphContent::Text(" ".to_owned()));
                } else {
                    ui.label(" ");
                }
            }
            OwnedEvent::HardBreak => {
                // Hard line break
                if in_paragraph {
                    // For hard breaks within paragraphs, we need to handle them specially
                    // Since we're using horizontal_wrapped, we can't easily add vertical space
                    // We'll add a special marker that we can handle during rendering
                    paragraph_content.push(ParagraphContent::Text("\n".to_owned()));
                } else {
                    // No spacing before standalone hard break

                    ui.add_space(4.0);

                    // Add bottom margin for standalone hard break (same as paragraph)
                    add_bottom_margin(ui, &mut previous_bottom_margin, PARAGRAPH_BOTTOM);
                }
            }
            OwnedEvent::Rule => {
                // Horizontal rule - always breaks paragraph context
                if in_paragraph {
                    // Render accumulated paragraph content first
                    if !paragraph_content.is_empty() {
                        ui.horizontal_wrapped(|ui| {
                            for content in &paragraph_content {
                                render_paragraph_content(ui, content);
                            }
                        });
                        // Add paragraph bottom spacing
                        add_bottom_margin(ui, &mut previous_bottom_margin, PARAGRAPH_BOTTOM);
                        paragraph_content.clear();
                    }
                    in_paragraph = false;
                }

                // Apply margin collapsing for horizontal rule top margin
                add_top_margin_with_collapsing(
                    ui,
                    &previous_bottom_margin,
                    HORIZONTAL_RULE_SPACING,
                );

                ui.separator();

                // Add horizontal rule bottom margin and track it
                add_bottom_margin(ui, &mut previous_bottom_margin, HORIZONTAL_RULE_SPACING);
            }
            OwnedEvent::TaskListMarker(checked) => {
                // Task list marker
                let marker = if *checked { "[x]" } else { "[ ]" };
                if in_paragraph {
                    paragraph_content.push(ParagraphContent::Text(marker.to_owned()));
                } else {
                    // No spacing before standalone task list marker

                    ui.label(marker);

                    // Add bottom margin for standalone task list marker (same as paragraph)
                    add_bottom_margin(ui, &mut previous_bottom_margin, PARAGRAPH_BOTTOM);
                }
            }
        }
    }

    // Render any remaining paragraph content
    if in_paragraph && !paragraph_content.is_empty() {
        ui.horizontal_wrapped(|ui| {
            for content in &paragraph_content {
                render_paragraph_content(ui, content);
            }
        });
    }
}

/// Parse a markdown table from the event stream.
pub(crate) fn parse_table(
    events: &mut std::slice::Iter<'_, SpannedEvent>,
    _alignments: &[Alignment],
) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let mut headers = Vec::new();
    let mut rows = Vec::new();
    let mut current_row = Vec::new();
    let mut in_header = false;

    while let Some(SpannedEvent { event, .. }) = events.next() {
        match event {
            OwnedEvent::Start(OwnedTag::TableHead) => {
                in_header = true;
            }
            OwnedEvent::End(OwnedTag::TableHead) => {
                if !current_row.is_empty() {
                    headers.push(current_row.clone());
                    current_row.clear();
                }
                in_header = false;
            }
            OwnedEvent::Start(OwnedTag::TableRow) => {
                current_row.clear();
            }
            OwnedEvent::End(OwnedTag::TableRow) => {
                if !current_row.is_empty() {
                    if in_header {
                        headers.push(current_row.clone());
                    } else {
                        rows.push(current_row.clone());
                    }
                    current_row.clear();
                }
            }
            OwnedEvent::Start(OwnedTag::TableCell) => {
                let mut cell_text = String::new();
                for SpannedEvent { event, .. } in events.by_ref() {
                    match event {
                        OwnedEvent::End(OwnedTag::TableCell) => break,
                        OwnedEvent::Text(text) => cell_text.push_str(text),
                        OwnedEvent::SoftBreak => cell_text.push(' '),
                        OwnedEvent::HardBreak => cell_text.push('\n'),
                        _ => {}
                    }
                }
                current_row.push(cell_text);
            }
            OwnedEvent::End(OwnedTag::Table(_)) => break,
            _ => {}
        }
    }

    (headers, rows)
}

/// Render text that may contain Typst math expressions.
fn render_text_with_latex(
    ui: &mut Ui,
    text: &str,
    math_asset_manager: &mut Option<&mut crate::math::MathAssetManager>,
    math_resolution_scale: f32,
) {
    render_text_with_math_impl(ui, text, math_asset_manager, math_resolution_scale);
}

/// Internal implementation for rendering text with math formulas.
fn render_text_with_math_impl(
    ui: &mut Ui,
    text: &str,
    math_asset_manager: &mut Option<&mut crate::math::MathAssetManager>,
    math_resolution_scale: f32,
) {
    // If we have an asset manager, try to render actual SVG textures
    if let Some(asset_manager) = math_asset_manager {
        render_text_with_math_and_assets(ui, text, asset_manager, math_resolution_scale);
    } else {
        // Fall back to code rendering
        render_text_with_math(ui, text);
    }
}

/// Render text with math formulas using SVG assets
fn render_text_with_math_and_assets(
    ui: &mut Ui,
    text: &str,
    asset_manager: &mut crate::math::MathAssetManager,
    math_resolution_scale: f32,
) {
    let mut remaining = text;

    while let Some(start) = remaining.find('$') {
        // Check if it's Typst display math ($ formula $) or inline math ($formula$)
        // Look ahead to see if there's a space after the opening $
        let next_char = remaining.get(start + 1..start + 2);
        let is_display_math = next_char == Some(" ");
        let end_marker = "$";

        // Find the closing $
        if let Some(end) = remaining[start + end_marker.len()..].find(end_marker) {
            let math_start = start + end_marker.len();
            let math_end = math_start + end;
            let math_content = &remaining[math_start..math_end];

            // Render text before the $
            if start > 0 {
                ui.label(&remaining[..start]);
            }

            // Try to render as SVG
            if let Some(image_source) = asset_manager.get_image_source_for_formula_with_resolution(
                math_content.trim(), // Trim whitespace
                is_display_math,
                math_resolution_scale,
            ) {
                // Get the SVG's intrinsic size and baseline data
                let svg_size_with_baseline = asset_manager.get_svg_size_with_baseline_scaled(
                    math_content.trim(),
                    is_display_math,
                    math_resolution_scale,
                );

                if let Some((size, baseline_from_top)) = svg_size_with_baseline {
                    if is_display_math {
                        // Display math: center with spacing
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.add_space((ui.available_width() - size.x) / 2.0);

                            // Create image with crisp rendering using SVG's intrinsic size
                            let image = egui::Image::new(image_source)
                                .tint(ui.visuals().text_color()) // Theme-aware tinting
                                .fit_to_exact_size(size)
                                .corner_radius(0.0); // No rounding for crisp edges

                            ui.add(image);
                        });
                        ui.add_space(8.0);
                    } else {
                        // Inline math: use baseline alignment if available
                        if let Some(baseline) = baseline_from_top {
                            // Use baseline-aligned rendering
                            render_baseline_aligned_image(ui, image_source, size, baseline);
                        } else {
                            // Fallback: render at SVG's intrinsic size
                            // Create image with crisp rendering using SVG's intrinsic size
                            let image = egui::Image::new(image_source)
                                .tint(ui.visuals().text_color()) // Theme-aware tinting
                                .fit_to_exact_size(size)
                                .corner_radius(0.0); // No rounding for crisp edges

                            ui.add(image);
                        }
                    }
                } else {
                    // Fallback: use reasonable default size if SVG size not available
                    if is_display_math {
                        // Display math: reasonable default
                        let display_size = egui::vec2(200.0, 50.0);
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.add_space((ui.available_width() - display_size.x) / 2.0);
                            let image = egui::Image::new(image_source)
                                .tint(ui.visuals().text_color()) // Theme-aware tinting
                                .fit_to_exact_size(display_size)
                                .corner_radius(0.0);
                            ui.add(image);
                        });
                        ui.add_space(8.0);
                    } else {
                        // Inline math: reasonable default
                        let inline_size = egui::vec2(100.0, 20.0);
                        let image = egui::Image::new(image_source)
                            .tint(ui.visuals().text_color()) // Theme-aware tinting
                            .fit_to_exact_size(inline_size)
                            .corner_radius(0.0);
                        ui.add(image);
                    }
                }
            } else {
                // Fall back to code rendering if image source not available
                render_math_as_code(ui, math_content, is_display_math);
            }

            // Skip past the math
            remaining = &remaining[math_end + end_marker.len()..];
        } else {
            // No closing $, render the rest as text
            ui.label(&remaining[start..]);
            remaining = "";
        }
    }

    // Render any remaining text
    if !remaining.is_empty() {
        ui.label(remaining);
    }
}

/// Render math formula as code (fallback)
fn render_math_as_code(ui: &mut Ui, math_content: &str, is_display_math: bool) {
    let style = if is_display_math {
        RichText::new(math_content)
            .code()
            .background_color(ui.visuals().code_bg_color)
    } else {
        RichText::new(math_content).code()
    };
    ui.label(style);
}

/// Common implementation for rendering text with math formulas.
fn render_text_with_math(ui: &mut Ui, text: &str) {
    // Simple Typst math detection
    let mut remaining = text;

    while let Some(start) = remaining.find('$') {
        // Check if it's Typst display math ($ formula $) or inline math ($formula$)
        // Look ahead to see if there's a space after the opening $
        let next_char = remaining.get(start + 1..start + 2);
        let is_display_math = next_char == Some(" ");
        let end_marker = "$";

        // Find the closing $
        if let Some(end) = remaining[start + end_marker.len()..].find(end_marker) {
            let math_start = start + end_marker.len();
            let math_end = math_start + end;
            let math_content = &remaining[math_start..math_end];

            // Render text before the $
            if start > 0 {
                ui.label(&remaining[..start]);
            }

            // Render as code (fallback when no asset manager)
            render_math_as_code(ui, math_content, is_display_math);

            // Skip past the math
            remaining = &remaining[math_end + end_marker.len()..];
        } else {
            // No closing $, render the rest as text
            ui.label(&remaining[start..]);
            remaining = "";
        }
    }

    // Render any remaining text
    if !remaining.is_empty() {
        ui.label(remaining);
    }
}

/// Render a single paragraph content item
/// Render a vector of paragraph content with optional styling
pub(crate) fn render_paragraph_content_vec(
    ui: &mut Ui,
    content: &[ParagraphContent],
    text_style: &TextStyle,
) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        for item in content {
            match item {
                ParagraphContent::Text(text) => {
                    let rich_text = RichText::new(text).text_style((*text_style).clone());
                    ui.label(rich_text);
                }
                ParagraphContent::Strong(text) => {
                    // Use bold text style (true bold font, no color emphasis needed)
                    let bold_style = bold_text_style(text_style);
                    let rich_text = RichText::new(text).text_style(bold_style);
                    ui.label(rich_text);
                }
                ParagraphContent::Emphasis(text) => {
                    // Use italic text style if available
                    let italic_style = italic_text_style(text_style);
                    let rich_text = if italic_style == *text_style {
                        // No italic variant available, use .italics() for slant
                        RichText::new(text).italics().text_style(text_style.clone())
                    } else {
                        // Use italic font variant
                        RichText::new(text).text_style(italic_style)
                    };
                    ui.label(rich_text);
                }
                ParagraphContent::Strikethrough(text) => {
                    let rich_text = RichText::new(text)
                        .strikethrough()
                        .text_style(text_style.clone());
                    ui.label(rich_text);
                }
                ParagraphContent::InlineCode(code) => {
                    let rich_text = RichText::new(code).code().text_style(text_style.clone());
                    ui.label(rich_text);
                }
                ParagraphContent::Link { text, url } => {
                    ui.add(Hyperlink::from_label_and_url(
                        RichText::new(text).text_style(text_style.clone()),
                        url,
                    ));
                }
                ParagraphContent::MathImage {
                    image_source,
                    size,
                    is_display: _,
                    baseline_from_top,
                } => {
                    // Display math should be handled differently, not in horizontal layout
                    // For now, render inline but this should be improved
                    if let Some(baseline) = baseline_from_top {
                        render_baseline_aligned_image(ui, image_source.clone(), *size, *baseline);
                    } else {
                        let image = egui::Image::new(image_source.clone())
                            .tint(ui.visuals().text_color())
                            .fit_to_exact_size(*size)
                            .corner_radius(0.0);
                        ui.add(image);
                    }
                }
                ParagraphContent::MathCode {
                    content,
                    is_display: _,
                } => {
                    // Display math code - render as code
                    ui.label(RichText::new(content).code().text_style(text_style.clone()));
                }
                ParagraphContent::Widget {
                    name,
                    config,
                    width,
                    height,
                } => {
                    // Create a child UI with optional size constraints
                    let mut child_ui = ui.new_child(egui::UiBuilder::new());

                    // Apply size constraints if specified
                    if let Some(w) = width {
                        child_ui.set_width(*w);
                    }
                    if let Some(h) = height {
                        child_ui.set_height(*h);
                    }

                    // Render the widget
                    let registry = crate::widgets::global_registry();
                    match registry.create_instance(name) {
                        Ok(mut widget) => {
                            let _size = widget.render(&mut child_ui, config);
                        }
                        Err(e) => {
                            child_ui.label(format!("Widget error: {e}"));
                        }
                    }
                }
            }
        }
    });
}

fn render_paragraph_content(ui: &mut Ui, content: &ParagraphContent) {
    match content {
        ParagraphContent::Text(text) => {
            // Handle hard breaks within text
            if text.contains('\n') {
                let parts: Vec<&str> = text.split('\n').collect();
                for (i, part) in parts.iter().enumerate() {
                    if !part.is_empty() {
                        ui.label(
                            RichText::new(*part).text_style(TextStyle::Name("ContentBody".into())),
                        );
                    }
                    if i < parts.len() - 1 {
                        ui.add_space(4.0); // Add vertical space for hard break
                    }
                }
            } else {
                ui.label(RichText::new(text).text_style(TextStyle::Name("ContentBody".into())));
            }
        }
        ParagraphContent::MathImage {
            image_source,
            size,
            is_display,
            baseline_from_top,
        } => {
            if *is_display {
                // Display math: center with spacing
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space((ui.available_width() - size.x) / 2.0);
                    let image = egui::Image::new(image_source.clone())
                        .tint(ui.visuals().text_color())
                        .fit_to_exact_size(*size)
                        .corner_radius(0.0);
                    ui.add(image);
                });
                ui.add_space(8.0);
            } else {
                // Inline math: use baseline alignment if available
                if let Some(baseline) = baseline_from_top {
                    // Use baseline-aligned rendering
                    render_baseline_aligned_image(ui, image_source.clone(), *size, *baseline);
                } else {
                    // Fallback: render inline with adjusted spacing (current behavior)
                    // Reduce the image size slightly to account for SVG padding
                    let adjusted_size = *size * 0.9; // Reduce by 10% to account for padding
                    let image = egui::Image::new(image_source.clone())
                        .tint(ui.visuals().text_color())
                        .fit_to_exact_size(adjusted_size)
                        .corner_radius(0.0);
                    ui.add(image);
                }
            }
        }
        ParagraphContent::MathCode {
            content,
            is_display,
        } => {
            if *is_display {
                // Display math code: center with background
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    // For text centering, we can use available space calculation
                    // We'll render the label and it will take up space, then we can center it
                    ui.with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            ui.label(
                                RichText::new(content)
                                    .code()
                                    .background_color(ui.visuals().code_bg_color),
                            );
                        },
                    );
                });
                ui.add_space(8.0);
            } else {
                // Inline math code
                ui.label(RichText::new(content).code());
            }
        }
        ParagraphContent::InlineCode(code) => {
            ui.label(RichText::new(code).code());
        }
        ParagraphContent::Strong(text) => {
            // Use ContentBodyBold for bold text
            ui.label(RichText::new(text).text_style(TextStyle::Name("ContentBodyBold".into())));
        }
        ParagraphContent::Emphasis(text) => {
            // Use ContentBodyItalic for italic text
            ui.label(RichText::new(text).text_style(TextStyle::Name("ContentBodyItalic".into())));
        }
        ParagraphContent::Link { text, url } => {
            ui.add(Hyperlink::from_label_and_url(
                RichText::new(text).text_style(TextStyle::Name("ContentBody".into())),
                url,
            ));
        }
        ParagraphContent::Strikethrough(text) => {
            ui.label(
                RichText::new(text)
                    .strikethrough()
                    .text_style(TextStyle::Name("ContentBody".into())),
            );
        }
        ParagraphContent::Widget {
            name,
            config,
            width,
            height,
        } => {
            // Create a container for the widget
            let frame = egui::Frame::new()
                .fill(ui.visuals().panel_fill)
                .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                .corner_radius(4.0);

            frame.show(ui, |ui| {
                // Apply size constraints if specified
                if let Some(w) = width {
                    ui.set_width(*w);
                }
                if let Some(h) = height {
                    ui.set_height(*h);
                }

                // Center the widget
                ui.centered_and_justified(|ui| {
                    // Create widget instance from registry
                    let registry = crate::widgets::global_registry();
                    match registry.create_instance(name) {
                        Ok(mut widget) => {
                            let _size = widget.render(ui, config);
                        }
                        Err(e) => {
                            ui.label(format!("Widget error: {e}"));
                        }
                    }
                });
            });
        }
    }
}

/// Process text with math placeholders and return paragraph content
/// Cached version of `process_text_with_math` that avoids re-parsing the same text segments.
pub(crate) fn process_text_with_math_cached(
    text: &str,
    manifest: &crate::math::MathManifest,
    math_asset_manager: &mut Option<&mut crate::math::MathAssetManager>,
    math_resolution_scale: f32,
    text_segment_cache: &mut crate::ui::text_cache::TextSegmentCache,
) -> Vec<ParagraphContent> {
    // Check cache first
    if let Some(cached) = text_segment_cache.get(text, math_resolution_scale) {
        return cached.clone();
    }

    // Cache miss: parse normally
    let result = crate::ui::math_parser::parse_text_with_math(
        text,
        manifest,
        math_asset_manager,
        math_resolution_scale,
    );

    // Store in cache
    text_segment_cache.insert(text, math_resolution_scale, result.clone());
    result
}

/// Cached version of `accumulate_text_content`
fn accumulate_text_content_cached(
    text: &str,
    manifest: &crate::math::MathManifest,
    math_asset_manager: &mut Option<&mut crate::math::MathAssetManager>,
    paragraph_content: &mut Vec<ParagraphContent>,
    math_resolution_scale: f32,
    text_segment_cache: &mut crate::ui::text_cache::TextSegmentCache,
) {
    let processed = process_text_with_math_cached(
        text,
        manifest,
        math_asset_manager,
        math_resolution_scale,
        text_segment_cache,
    );
    paragraph_content.extend(processed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_table() {
        let markdown = r#"| Technology | Language | Target | Performance |
|------------|----------|--------|-------------|
| egui | Rust | WebAssembly/Native | Excellent |
| React | JavaScript | Web | Good |
| Flutter | Dart | Mobile/Web | Very Good |
| GTK | C | Desktop | Good |"#;

        let mut options = pulldown_cmark::Options::empty();
        options.insert(pulldown_cmark::Options::ENABLE_TABLES);
        let parser = Parser::new_ext(markdown, options);

        let spanned_events: Vec<SpannedEvent> = parser
            .map(|e| SpannedEvent {
                span: 0..0,
                event: e.into(),
            })
            .collect();
        let mut iter = spanned_events.iter();

        while let Some(SpannedEvent { event, .. }) = iter.next() {
            if let OwnedEvent::Start(OwnedTag::Table(alignments)) = event {
                let (headers, rows) = parse_table(&mut iter, alignments);
                assert_eq!(headers.len(), 1); // one header row
                assert_eq!(headers[0].len(), 4); // four columns
                assert_eq!(rows.len(), 4); // four data rows
                assert_eq!(headers[0][0], "Technology");
                assert_eq!(rows[0][0], "egui");
                assert_eq!(rows[0][1], "Rust");
                assert_eq!(rows[0][2], "WebAssembly/Native");
                assert_eq!(rows[0][3], "Excellent");
                return;
            }
        }
        panic!("No table found in markdown");
    }

    #[test]
    fn test_list_parsing() {
        // Test unordered list
        let unordered_markdown = r#"### Features

- **Fast**: Compiled to WebAssembly
- **Simple**: No JavaScript framework
- **Rust**: Safety and performance"#;

        let mut options = pulldown_cmark::Options::empty();
        options.insert(pulldown_cmark::Options::ENABLE_TABLES);
        let parser = Parser::new_ext(unordered_markdown, options);
        let mut events = parser.peekable();

        let mut found_list = false;
        while let Some(event) = events.next() {
            if let Event::Start(Tag::List(ordered)) = event {
                found_list = true;
                assert_eq!(ordered, None); // Unordered list
                // Skip through the list events
                while let Some(event) = events.next() {
                    if let Event::End(Tag::List(_)) = event {
                        break;
                    }
                }
                break;
            }
        }
        assert!(found_list, "Should find unordered list in markdown");

        // Test ordered list
        let ordered_markdown = r#"I plan to add more features to this blog:

1. Markdown rendering
2. Code syntax highlighting
3. Dark/light theme toggle"#;

        let parser = Parser::new_ext(ordered_markdown, options);
        let mut events = parser.peekable();

        let mut found_ordered_list = false;
        while let Some(event) = events.next() {
            if let Event::Start(Tag::List(ordered)) = event {
                found_ordered_list = true;
                assert!(ordered.is_some()); // Ordered list
                assert_eq!(ordered.unwrap(), 1); // Starting at 1
                break;
            }
        }
        assert!(found_ordered_list, "Should find ordered list in markdown");
    }
}
