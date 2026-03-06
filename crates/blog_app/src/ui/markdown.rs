//! Markdown rendering for blog posts.

use egui::{vec2, Hyperlink, ImageSource, Pos2, Rect, RichText, Sense, Shape, TextStyle, Ui};
use egui_extras::syntax_highlighting::{highlight, CodeTheme};
use pulldown_cmark::{Alignment, CodeBlockKind, Event, HeadingLevel, Parser, Tag};

use crate::ui::table_renderer::TableConfig;
use crate::{ui::table_renderer, MathAssetManager};

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
fn render_baseline_aligned_image(
    ui: &mut Ui,
    image_source: ImageSource<'static>,
    mut image_size: egui::Vec2,
    baseline_from_top: f32,
) {
    // Get text metrics (estimated)
    // Use configurable ascent ratio for baseline estimation
    let text_height = ui.text_style_height(&TextStyle::Body);
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
#[derive(Clone)]
enum ParagraphContent {
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
                        let placeholder = format!("({hash}.typ)");
                        result.push_str(&placeholder);
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
pub fn render_preprocessed_markdown(
    ui: &mut Ui,
    preprocessed_markdown: &str,
    math_asset_manager: Option<&mut crate::math::MathAssetManager>,
) {
    render_markdown_impl(ui, preprocessed_markdown, math_asset_manager, true);
}

fn render_markdown_impl(
    ui: &mut Ui,
    markdown: &str,
    mut math_asset_manager: Option<&mut crate::math::MathAssetManager>,
    is_preprocessed: bool,
) {
    let protected_text = if is_preprocessed {
        // Content is already preprocessed with math placeholders
        markdown.to_owned()
    } else {
        // Extract math formulas and replace with (hash.typ) placeholders
        let manifest = crate::math::load_manifest();
        extract_and_replace_math_formulas(markdown, manifest)
    };

    // Load manifest for metadata lookup (needed for both preprocessed and raw content)
    let manifest = crate::math::load_manifest();

    let mut events =
        pulldown_cmark::Parser::new_ext(&protected_text, pulldown_cmark::Options::ENABLE_TABLES)
            .peekable();

    // Set vertical spacing to 0 so we have full control over spacing
    // This prevents default egui spacing from adding to our GitHub-inspired spacing
    ui.spacing_mut().item_spacing.y = 0.0;

    // Simplified margin collapsing: track previous element's bottom margin
    let mut previous_bottom_margin = 0.0;

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

    while let Some(event) = events.next() {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Paragraph => {
                        // Paragraphs don't have top margin in GitHub's CSS
                        // Spacing comes from previous element's bottom margin
                        in_paragraph = true;
                        paragraph_content.clear();
                    }
                    Tag::Heading(level, _, _) => {
                        // Headings
                        let mut heading_text = String::new();
                        while let Some(Event::Text(text)) = events.next() {
                            heading_text.push_str(&text);
                            if let Some(Event::End(Tag::Heading(_, _, _))) = events.peek() {
                                break;
                            }
                        }

                        // All headings have the same top margin in GitHub's CSS
                        let top_margin = HEADING_TOP;

                        // Apply margin collapsing for heading top margin
                        add_top_margin_with_collapsing(ui, &previous_bottom_margin, top_margin);

                        let rich_text = match level {
                            HeadingLevel::H1 => RichText::new(heading_text).heading(), // Uses TextStyle::Heading (32px)
                            HeadingLevel::H2 => RichText::new(heading_text)
                                .text_style(TextStyle::Name("Heading2".into())), // 24px
                            HeadingLevel::H3 => RichText::new(heading_text)
                                .text_style(TextStyle::Name("Heading3".into())), // 20px
                            HeadingLevel::H4 => RichText::new(heading_text)
                                .text_style(TextStyle::Name("Heading4".into())), // 16px
                            HeadingLevel::H5 => RichText::new(heading_text)
                                .text_style(TextStyle::Name("Heading5".into())), // 14px
                            HeadingLevel::H6 => RichText::new(heading_text)
                                .text_style(TextStyle::Name("Heading6".into())), // 13.6px
                        };

                        ui.label(rich_text);

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
                    Tag::List(ordered) => {
                        // Lists don't have top margin in GitHub's CSS
                        // Spacing comes from previous element's bottom margin

                        // Lists
                        let mut list_items = Vec::new();
                        while let Some(event) = events.next() {
                            match event {
                                Event::End(Tag::List(_)) => break,
                                Event::Start(Tag::Item) => {
                                    let mut item_text = String::new();
                                    for event in events.by_ref() {
                                        match event {
                                            Event::End(Tag::Item) => break,
                                            Event::Text(text) => item_text.push_str(&text),
                                            Event::SoftBreak => item_text.push(' '),
                                            Event::HardBreak => item_text.push('\n'),
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

                        let row_height = ui.text_style_height(&TextStyle::Body);
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
                                ui.label(item);
                            });

                            // Add spacing between list items (GitHub: 0.25em = 4px)
                            if i < list_items.len() - 1 {
                                ui.add_space(LIST_ITEM_SPACING);
                            }
                        }

                        // Add list bottom margin (same as paragraph) and track it
                        add_bottom_margin(ui, &mut previous_bottom_margin, PARAGRAPH_BOTTOM);
                    }
                    Tag::Item => {
                        // Already handled in List
                    }
                    Tag::CodeBlock(kind) => {
                        // Code blocks don't have top margin in GitHub's CSS
                        // Spacing comes from previous element's bottom margin

                        // Code blocks
                        let mut code_text = String::new();
                        for event in events.by_ref() {
                            match event {
                                Event::End(Tag::CodeBlock(_)) => break,
                                Event::Text(text) => code_text.push_str(&text),
                                Event::SoftBreak | Event::HardBreak => code_text.push('\n'),
                                _ => {} // Skip other events
                            }
                        }

                        // Display language label if present
                        let language = match kind {
                            CodeBlockKind::Fenced(lang) if !lang.is_empty() => {
                                Some(lang.to_string())
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
                    Tag::Strong => {
                        // Bold text
                        let mut bold_text = String::new();
                        for event in events.by_ref() {
                            match event {
                                Event::End(Tag::Strong) => break,
                                Event::Text(text) => bold_text.push_str(&text),
                                Event::SoftBreak => bold_text.push(' '),
                                _ => {} // Skip other events
                            }
                        }
                        if in_paragraph {
                            paragraph_content.push(ParagraphContent::Strong(bold_text));
                        } else {
                            ui.label(RichText::new(bold_text).strong());
                        }
                    }
                    Tag::Emphasis => {
                        // Italic text
                        let mut italic_text = String::new();
                        for event in events.by_ref() {
                            match event {
                                Event::End(Tag::Emphasis) => break,
                                Event::Text(text) => italic_text.push_str(&text),
                                Event::SoftBreak => italic_text.push(' '),
                                _ => {} // Skip other events
                            }
                        }
                        if in_paragraph {
                            paragraph_content.push(ParagraphContent::Emphasis(italic_text));
                        } else {
                            ui.label(RichText::new(italic_text).italics());
                        }
                    }
                    Tag::Link(_, url, _) => {
                        // Links
                        let url = url.to_string();
                        let mut link_text = String::new();
                        for event in events.by_ref() {
                            match event {
                                Event::End(Tag::Link(_, _, _)) => break,
                                Event::Text(text) => link_text.push_str(&text),
                                Event::SoftBreak => link_text.push(' '),
                                _ => {} // Skip other events
                            }
                        }

                        if in_paragraph {
                            paragraph_content.push(ParagraphContent::Link {
                                text: link_text,
                                url,
                            });
                        } else {
                            ui.add(Hyperlink::from_label_and_url(&link_text, &url));
                        }
                    }
                    Tag::Strikethrough => {
                        // Strikethrough text
                        let mut strike_text = String::new();
                        for event in events.by_ref() {
                            match event {
                                Event::End(Tag::Strikethrough) => break,
                                Event::Text(text) => strike_text.push_str(&text),
                                Event::SoftBreak => strike_text.push(' '),
                                _ => {} // Skip other events
                            }
                        }
                        if in_paragraph {
                            paragraph_content.push(ParagraphContent::Strikethrough(strike_text));
                        } else {
                            ui.label(RichText::new(strike_text).strikethrough());
                        }
                    }
                    Tag::BlockQuote => {
                        // Blockquotes don't have top margin in GitHub's CSS
                        // Spacing comes from previous element's bottom margin

                        // Collect all text from the blockquote (simple approach for now)
                        let mut quote_text = String::new();
                        for event in events.by_ref() {
                            match event {
                                Event::End(Tag::BlockQuote) => break,
                                Event::Text(text) => quote_text.push_str(&text),
                                Event::SoftBreak | Event::HardBreak => quote_text.push('\n'),
                                _ => {} // Skip other events for now
                            }
                        }

                        // Trim trailing whitespace
                        let quote_text = quote_text.trim_end();

                        if !quote_text.is_empty() {
                            // Calculate dimensions for blockquote
                            let row_height = ui.text_style_height(&TextStyle::Body);
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
                                    ui.label(
                                        RichText::new(quote_text)
                                            .color(ui.visuals().weak_text_color())
                                            .text_style(TextStyle::Body),
                                    );
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
                    Tag::FootnoteDefinition(_) => {
                        // Skip footnotes for now
                        for event in events.by_ref() {
                            if matches!(event, Event::End(Tag::FootnoteDefinition(_))) {
                                break;
                            }
                        }
                    }
                    Tag::Table(alignments) => {
                        // Tables don't have top margin in GitHub's CSS
                        // Spacing comes from previous element's bottom margin

                        let (headers, rows) = parse_table(&mut events, &alignments);
                        table_renderer::render_table(
                            ui,
                            &alignments,
                            &headers,
                            &rows,
                            &TableConfig::default(),
                        );

                        // Add table bottom margin (same as paragraph) and track it
                        add_bottom_margin(ui, &mut previous_bottom_margin, PARAGRAPH_BOTTOM);
                    }
                    Tag::TableHead | Tag::TableRow | Tag::TableCell => {
                        // Skip table elements that appear outside a table (should not happen)
                        for event in events.by_ref() {
                            if matches!(
                                event,
                                Event::End(Tag::TableHead | Tag::TableRow | Tag::TableCell)
                            ) {
                                break;
                            }
                        }
                    }
                    Tag::Image(_, url, _) => {
                        // Images - display alt text as placeholder
                        let _url = url.to_string();
                        let mut alt_text = String::new();
                        for event in events.by_ref() {
                            match event {
                                Event::End(Tag::Image(_, _, _)) => break,
                                Event::Text(text) => alt_text.push_str(&text),
                                Event::SoftBreak => alt_text.push(' '),
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
            Event::End(tag) => {
                if tag == Tag::Paragraph {
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
            Event::Text(text) => {
                if in_paragraph {
                    // Accumulate text content for paragraph rendering
                    accumulate_text_content(
                        &text,
                        manifest,
                        &mut math_asset_manager,
                        &mut paragraph_content,
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
                            render_text_with_latex(ui, before_text, &mut math_asset_manager);
                        }

                        // Find the end of the placeholder - look for closing ')'
                        if let Some(end) = remaining[start..].find(')') {
                            let placeholder = &remaining[start..=start + end];

                            // Check if this is a math placeholder: (hash.typ)
                            if placeholder.ends_with(".typ)") && placeholder.len() > 6 {
                                // Extract hash: remove '(' and '.typ)'
                                let hash = &placeholder[1..placeholder.len() - 5];

                                // Look up metadata in manifest
                                if let Some(metadata) = manifest.get_metadata(hash) {
                                    if let Some(_asset_manager) = &mut math_asset_manager {
                                        // Try to render as SVG using hash
                                        if let Some(image_source) =
                                            MathAssetManager::get_image_source_for_hash(hash)
                                        {
                                            // Get the SVG's intrinsic size
                                            let svg_size = MathAssetManager::get_svg_size(hash);

                                            if let Some(size) = svg_size {
                                                // Use SVG's intrinsic size directly (both in points)
                                                // No scaling needed - SVG size is already in points

                                                if metadata.is_display {
                                                    // Display math: center with spacing
                                                    ui.add_space(8.0);
                                                    ui.horizontal(|ui| {
                                                        ui.add_space(
                                                            (ui.available_width() - size.x) / 2.0,
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
                        render_text_with_latex(ui, remaining, &mut math_asset_manager);
                    }

                    // Add bottom margin for standalone text (same as paragraph)
                    add_bottom_margin(ui, &mut previous_bottom_margin, PARAGRAPH_BOTTOM);
                }
            }
            Event::Code(code) => {
                // Inline code
                if in_paragraph {
                    paragraph_content.push(ParagraphContent::InlineCode(code.to_string()));
                } else {
                    // No spacing before standalone inline code

                    ui.label(RichText::new(&*code).code());

                    // Add bottom margin for standalone inline code (same as paragraph)
                    add_bottom_margin(ui, &mut previous_bottom_margin, PARAGRAPH_BOTTOM);
                }
            }
            Event::Html(_) | Event::FootnoteReference(_) => {
                // Skip HTML and footnotes
            }
            Event::SoftBreak => {
                // Soft line break (treated as space)
                if in_paragraph {
                    paragraph_content.push(ParagraphContent::Text(" ".to_owned()));
                } else {
                    ui.label(" ");
                }
            }
            Event::HardBreak => {
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
            Event::Rule => {
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
            Event::TaskListMarker(checked) => {
                // Task list marker
                let marker = if checked { "[x]" } else { "[ ]" };
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
}

/// Parse a markdown table from the event stream.
pub(crate) fn parse_table<'a>(
    events: &mut std::iter::Peekable<Parser<'a, 'a>>,
    _alignments: &[Alignment],
) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let mut headers = Vec::new();
    let mut rows = Vec::new();
    let mut current_row = Vec::new();
    let mut in_header = false;

    while let Some(event) = events.next() {
        match event {
            Event::Start(Tag::TableHead) => {
                in_header = true;
            }
            Event::End(Tag::TableHead) => {
                if !current_row.is_empty() {
                    headers.push(current_row.clone());
                    current_row.clear();
                }
                in_header = false;
            }
            Event::Start(Tag::TableRow) => {
                current_row.clear();
            }
            Event::End(Tag::TableRow) => {
                if !current_row.is_empty() {
                    if in_header {
                        headers.push(current_row.clone());
                    } else {
                        rows.push(current_row.clone());
                    }
                    current_row.clear();
                }
            }
            Event::Start(Tag::TableCell) => {
                let mut cell_text = String::new();
                for event in events.by_ref() {
                    match event {
                        Event::End(Tag::TableCell) => break,
                        Event::Text(text) => cell_text.push_str(&text),
                        Event::SoftBreak => cell_text.push(' '),
                        Event::HardBreak => cell_text.push('\n'),
                        _ => {}
                    }
                }
                current_row.push(cell_text);
            }
            Event::End(Tag::Table(_)) => break,
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
) {
    render_text_with_math_impl(ui, text, math_asset_manager);
}

/// Internal implementation for rendering text with math formulas.
fn render_text_with_math_impl(
    ui: &mut Ui,
    text: &str,
    math_asset_manager: &mut Option<&mut crate::math::MathAssetManager>,
) {
    // If we have an asset manager, try to render actual SVG textures
    if let Some(asset_manager) = math_asset_manager {
        render_text_with_math_and_assets(ui, text, asset_manager);
    } else {
        // Fall back to code rendering
        render_text_with_math(ui, text);
    }
}

/// Render text with math formulas using SVG assets
fn render_text_with_math_and_assets(
    ui: &mut Ui,
    text: &str,
    asset_manager: &crate::math::MathAssetManager,
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
            if let Some(image_source) = asset_manager.get_image_source_for_formula(
                math_content.trim(), // Trim whitespace
                is_display_math,
            ) {
                // Get the SVG's intrinsic size and baseline data
                let svg_size_with_baseline =
                    asset_manager.get_svg_size_with_baseline(math_content.trim(), is_display_math);

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
fn render_paragraph_content(ui: &mut Ui, content: &ParagraphContent) {
    match content {
        ParagraphContent::Text(text) => {
            // Handle hard breaks within text
            if text.contains('\n') {
                let parts: Vec<&str> = text.split('\n').collect();
                for (i, part) in parts.iter().enumerate() {
                    if !part.is_empty() {
                        ui.label(*part);
                    }
                    if i < parts.len() - 1 {
                        ui.add_space(4.0); // Add vertical space for hard break
                    }
                }
            } else {
                ui.label(text);
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
            ui.label(RichText::new(text).strong());
        }
        ParagraphContent::Emphasis(text) => {
            ui.label(RichText::new(text).italics());
        }
        ParagraphContent::Link { text, url } => {
            ui.add(Hyperlink::from_label_and_url(text, url));
        }
        ParagraphContent::Strikethrough(text) => {
            ui.label(RichText::new(text).strikethrough());
        }
    }
}

/// Accumulate text content for paragraph rendering
fn accumulate_text_content(
    text: &str,
    manifest: &crate::math::MathManifest,
    math_asset_manager: &mut Option<&mut crate::math::MathAssetManager>,
    paragraph_content: &mut Vec<ParagraphContent>,
) {
    let mut remaining = text;

    while let Some(start) = remaining.find('(') {
        // Add text before the placeholder
        if start > 0 {
            let before_text = &remaining[..start];
            if !before_text.is_empty() {
                paragraph_content.push(ParagraphContent::Text(before_text.to_owned()));
            }
        }

        // Find the end of the placeholder - look for closing ')'
        if let Some(end) = remaining[start..].find(')') {
            let placeholder = &remaining[start..=start + end];

            // Check if this is a math placeholder: (hash.typ)
            if placeholder.ends_with(".typ)") && placeholder.len() > 6 {
                // Extract hash: remove '(' and '.typ)'
                let hash = &placeholder[1..placeholder.len() - 5];

                // Look up metadata in manifest
                if let Some(metadata) = manifest.get_metadata(hash) {
                    // Inline math - accumulate in paragraph content
                    if let Some(_asset_manager) = math_asset_manager {
                        // Try to get SVG using hash
                        if let Some(image_source) =
                            MathAssetManager::get_image_source_for_hash(hash)
                        {
                            // Get the SVG's intrinsic size
                            let svg_size = MathAssetManager::get_svg_size(hash);

                            if let Some(size) = svg_size {
                                paragraph_content.push(ParagraphContent::MathImage {
                                    image_source,
                                    size,
                                    is_display: metadata.is_display,
                                    baseline_from_top: metadata.baseline_from_top,
                                });
                            } else {
                                // Fallback: use code rendering
                                paragraph_content.push(ParagraphContent::MathCode {
                                    content: format!("Math formula: {hash}"),
                                    is_display: metadata.is_display,
                                });
                            }
                        } else {
                            // Fallback: render as code
                            paragraph_content.push(ParagraphContent::MathCode {
                                content: format!("Math formula: {hash}"),
                                is_display: metadata.is_display,
                            });
                        }
                    } else {
                        // No asset manager, render as code
                        paragraph_content.push(ParagraphContent::MathCode {
                            content: format!("Math formula: {hash}"),
                            is_display: metadata.is_display,
                        });
                    }
                } else {
                    // Hash not found in manifest, add placeholder as text
                    paragraph_content.push(ParagraphContent::Text(placeholder.to_owned()));
                }
            } else {
                // Not a math placeholder, add as normal text
                paragraph_content.push(ParagraphContent::Text(placeholder.to_owned()));
            }

            // Skip past the placeholder
            remaining = &remaining[start + end + 1..];
        } else {
            // No closing ')', add the '(' and continue
            paragraph_content.push(ParagraphContent::Text("(".to_owned()));
            remaining = &remaining[start + 1..];
        }
    }

    // Add any remaining text
    if !remaining.is_empty() {
        paragraph_content.push(ParagraphContent::Text(remaining.to_owned()));
    }
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
        let mut events = parser.peekable();

        // The parser yields events; we need to skip to the Table start
        // For simplicity, we'll just test parse_table by feeding it events after Table start
        // But we can also test the full rendering by calling render_markdown with a dummy UI?
        // Let's manually iterate to find Table start
        while let Some(event) = events.next() {
            if let Event::Start(Tag::Table(alignments)) = event {
                let (headers, rows) = parse_table(&mut events, &alignments);
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
