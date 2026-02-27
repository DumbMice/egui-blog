//! Markdown rendering for blog posts.

use egui::{vec2, Hyperlink, ImageSource, RichText, Sense, Shape, TextStyle, Ui};
use egui_extras::syntax_highlighting::{highlight, CodeTheme};
use log;
use pulldown_cmark::{Alignment, CodeBlockKind, Event, HeadingLevel, Parser, Tag};

use crate::ui::table_renderer;

/// Content that can appear within a paragraph
#[derive(Clone)]
enum ParagraphContent {
    Text(String),
    MathImage {
        image_source: ImageSource<'static>,
        size: egui::Vec2,
        is_display: bool,
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
fn extract_and_replace_math_formulas(text: &str, manifest: &crate::math::MathManifest) -> String {
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
                let mut formula_end_idx = j;

                // For Typst display math, check if there's a space before closing $
                if is_display && j > 0 && chars[j - 1] == ' ' {
                    formula_end_idx = j - 1; // Exclude the space before closing $
                }

                let formula: String = chars[formula_start_idx..formula_end_idx].iter().collect();
                let formula = formula.trim();

                if !formula.is_empty() {
                    // Look up hash in manifest
                    if let Some(hash) = manifest.find_hash(&formula, is_display) {
                        let placeholder = format!("({}.typ)", hash);
                        result.push_str(&placeholder);
                        i = j + 1;
                        continue;
                    }
                }
            }

            // If we get here, formula extraction failed or hash not found
            // Copy the original formula text as fallback
            for k in formula_start..=i {
                result.push(chars[k]);
            }
        }

        // Not a formula (or failed formula), copy the character
        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Render markdown content to an egui UI.
#[cfg(not(feature = "math"))]
pub fn render_markdown(ui: &mut Ui, markdown: &str) {
    render_markdown_internal(ui, markdown)
}

/// Render markdown content to an egui UI with math support.
#[cfg(feature = "math")]
pub fn render_markdown(
    ui: &mut Ui,
    markdown: &str,
    math_asset_manager: Option<&mut crate::math::MathAssetManager>,
) {
    render_markdown_internal(ui, markdown, math_asset_manager)
}

#[cfg(not(feature = "math"))]
fn render_markdown_internal(ui: &mut Ui, markdown: &str) {
    render_markdown_impl(ui, markdown, None)
}

#[cfg(feature = "math")]
fn render_markdown_internal(
    ui: &mut Ui,
    markdown: &str,
    math_asset_manager: Option<&mut crate::math::MathAssetManager>,
) {
    render_markdown_impl(ui, markdown, math_asset_manager)
}

#[cfg(not(feature = "math"))]
fn render_markdown_impl(ui: &mut Ui, markdown: &str, _math_asset_manager: Option<()>) {
    render_markdown_with_math(ui, markdown, None)
}

#[cfg(feature = "math")]
fn render_markdown_impl(
    ui: &mut Ui,
    markdown: &str,
    math_asset_manager: Option<&mut crate::math::MathAssetManager>,
) {
    render_markdown_with_math(ui, markdown, math_asset_manager)
}

fn render_markdown_with_math(
    ui: &mut Ui,
    markdown: &str,
    mut math_asset_manager: Option<&mut crate::math::MathAssetManager>,
) {
    // Extract math formulas and replace with (hash.typ) placeholders
    let manifest = crate::math::load_manifest();

    let protected_text = extract_and_replace_math_formulas(markdown, &manifest);

    let mut events =
        pulldown_cmark::Parser::new_ext(&protected_text, pulldown_cmark::Options::ENABLE_TABLES)
            .peekable();

    // State for accumulating paragraph content
    let mut in_paragraph = false;
    let mut paragraph_content = Vec::new();

    while let Some(event) = events.next() {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Paragraph => {
                        in_paragraph = true;
                        paragraph_content.clear();
                    }
                    Tag::Heading(level, _, _) => {
                        // Headings
                        let mut heading_text = String::new();
                        while let Some(Event::Text(text)) = events.next() {
                            heading_text.push_str(&*text);
                            if let Some(Event::End(Tag::Heading(_, _, _))) = events.peek() {
                                break;
                            }
                        }

                        // Add spacing before heading (proportional to heading level)
                        let spacing_before = match level {
                            HeadingLevel::H1 => 24.0,
                            HeadingLevel::H2 => 20.0,
                            HeadingLevel::H3 => 16.0,
                            HeadingLevel::H4 => 12.0,
                            HeadingLevel::H5 => 8.0,
                            HeadingLevel::H6 => 4.0,
                        };
                        ui.add_space(spacing_before);

                        let rich_text = match level {
                            HeadingLevel::H1 => RichText::new(heading_text).heading().size(28.0),
                            HeadingLevel::H2 => RichText::new(heading_text).heading().size(24.0),
                            HeadingLevel::H3 => RichText::new(heading_text).heading().size(20.0),
                            HeadingLevel::H4 => RichText::new(heading_text).size(18.0),
                            HeadingLevel::H5 => RichText::new(heading_text).size(16.0),
                            HeadingLevel::H6 => RichText::new(heading_text).size(14.0),
                        };

                        ui.label(rich_text);

                        // Add spacing after heading (slightly less than before)
                        let spacing_after = spacing_before * 0.75;
                        ui.add_space(spacing_after);
                    }
                    Tag::List(ordered) => {
                        log::debug!("Start List, ordered: {:?}", ordered);
                        // Lists
                        let mut list_items = Vec::new();
                        while let Some(event) = events.next() {
                            match event {
                                Event::End(Tag::List(_)) => break,
                                Event::Start(Tag::Item) => {
                                    let mut item_text = String::new();
                                    while let Some(event) = events.next() {
                                        match event {
                                            Event::End(Tag::Item) => break,
                                            Event::Text(text) => item_text.push_str(&*text),
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

                                match ordered {
                                    Some(start) => {
                                        let number = (start + i as u64).to_string();
                                        // Render number as text label (part of the text flow)
                                        ui.label(RichText::new(format!("{}.", number)));
                                        ui.add_space(one_indent / 3.0);
                                    }
                                    None => {
                                        // Render bullet as text character (•) instead of drawn circle
                                        ui.label(RichText::new("•"));
                                        ui.add_space(one_indent / 3.0);
                                    }
                                }
                                ui.label(item);
                            });
                        }
                        ui.add_space(4.0);
                    }
                    Tag::Item => {
                        // Already handled in List
                    }
                    Tag::CodeBlock(kind) => {
                        // Code blocks
                        let mut code_text = String::new();
                        while let Some(event) = events.next() {
                            match event {
                                Event::End(Tag::CodeBlock(_)) => break,
                                Event::Text(text) => code_text.push_str(&*text),
                                Event::SoftBreak => code_text.push('\n'),
                                Event::HardBreak => code_text.push('\n'),
                                _ => {} // Skip other events
                            }
                        }

                        ui.add_space(4.0);

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
                        ui.add_space(4.0);
                    }
                    Tag::Strong => {
                        // Bold text
                        let mut bold_text = String::new();
                        while let Some(event) = events.next() {
                            match event {
                                Event::End(Tag::Strong) => break,
                                Event::Text(text) => bold_text.push_str(&*text),
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
                        while let Some(event) = events.next() {
                            match event {
                                Event::End(Tag::Emphasis) => break,
                                Event::Text(text) => italic_text.push_str(&*text),
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
                        while let Some(event) = events.next() {
                            match event {
                                Event::End(Tag::Link(_, _, _)) => break,
                                Event::Text(text) => link_text.push_str(&*text),
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
                        while let Some(event) = events.next() {
                            match event {
                                Event::End(Tag::Strikethrough) => break,
                                Event::Text(text) => strike_text.push_str(&*text),
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
                        // Block quotes
                        let mut quote_text = String::new();
                        while let Some(event) = events.next() {
                            match event {
                                Event::End(Tag::BlockQuote) => break,
                                Event::Text(text) => quote_text.push_str(&*text),
                                Event::SoftBreak => quote_text.push('\n'),
                                Event::HardBreak => quote_text.push('\n'),
                                _ => {} // Skip other events
                            }
                        }

                        ui.add_space(4.0);
                        let row_height = ui.text_style_height(&TextStyle::Body);
                        let one_indent = row_height / 2.0;

                        // Draw vertical line for quote (EasyMark style)
                        let rect = ui
                            .allocate_exact_size(vec2(2.0 * one_indent, row_height), Sense::hover())
                            .0;
                        let rect = rect.expand2(ui.style().spacing.item_spacing * 0.5);
                        ui.painter().line_segment(
                            [rect.center_top(), rect.center_bottom()],
                            (1.0, ui.visuals().weak_text_color()),
                        );

                        // Render quote text with weak color
                        ui.label(RichText::new(quote_text).color(ui.visuals().weak_text_color()));
                        ui.add_space(4.0);
                    }
                    Tag::FootnoteDefinition(_) => {
                        // Skip footnotes for now
                        while let Some(event) = events.next() {
                            if matches!(event, Event::End(Tag::FootnoteDefinition(_))) {
                                break;
                            }
                        }
                    }
                    Tag::Table(alignments) => {
                        log::info!("Found table with {} alignments", alignments.len());
                        if let Some((headers, rows)) = parse_table(&mut events, &alignments) {
                            log::info!(
                                "Parsed table: {} headers (first: {:?}), {} rows",
                                headers.len(),
                                headers.first(),
                                rows.len()
                            );
                            render_table(ui, &alignments, &headers, &rows);
                        } else {
                            log::warn!("Failed to parse table");
                        }
                    }
                    Tag::TableHead | Tag::TableRow | Tag::TableCell => {
                        // Skip table elements that appear outside a table (should not happen)
                        while let Some(event) = events.next() {
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
                        while let Some(event) = events.next() {
                            match event {
                                Event::End(Tag::Image(_, _, _)) => break,
                                Event::Text(text) => alt_text.push_str(&*text),
                                Event::SoftBreak => alt_text.push(' '),
                                _ => {} // Skip other events
                            }
                        }
                        ui.label(
                            RichText::new(format!("[Image: {}]", alt_text))
                                .italics()
                                .weak(),
                        );
                    }
                }
            }
            Event::End(tag) => {
                match tag {
                    Tag::Paragraph => {
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
                            ui.add_space(4.0); // Add spacing after paragraph
                            paragraph_content.clear();
                        }
                        in_paragraph = false;
                    }
                    _ => {
                        // Other end tags are handled within Start match
                    }
                }
            }
            Event::Text(text) => {
                if in_paragraph {
                    // Accumulate text content for paragraph rendering
                    accumulate_text_content(
                        &text,
                        &manifest,
                        &mut math_asset_manager,
                        &mut paragraph_content,
                    );
                } else {
                    // Fallback for text outside paragraphs (shouldn't happen in proper markdown)
                    // Check for math placeholders in the text (format: (hash.typ))
                    let mut remaining = &text[..];
                    let _last_pos = 0;

                    while let Some(start) = remaining.find('(') {
                        // Render text before the placeholder
                        if start > 0 {
                            let before_text = &remaining[..start];
                            #[cfg(not(feature = "math"))]
                            render_text_with_latex(ui, before_text);
                            #[cfg(feature = "math")]
                            render_text_with_latex(ui, before_text, &mut math_asset_manager);
                        }

                        // Find the end of the placeholder - look for closing ')'
                        if let Some(end) = remaining[start..].find(')') {
                            let placeholder = &remaining[start..start + end + 1];

                            // Check if this is a math placeholder: (hash.typ)
                            if placeholder.ends_with(".typ)") && placeholder.len() > 6 {
                                // Extract hash: remove '(' and '.typ)'
                                let hash = &placeholder[1..placeholder.len() - 5];

                                // Look up metadata in manifest
                                if let Some(metadata) = manifest.get_metadata(hash) {
                                    if let Some(asset_manager) = &mut math_asset_manager {
                                        // Try to render as SVG using hash
                                        if let Some(image_source) =
                                            asset_manager.get_image_source_for_hash(hash)
                                        {
                                            // Get the SVG's intrinsic size
                                            let svg_size = asset_manager.get_svg_size(hash);

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
                                                &format!("Math formula: {}", hash),
                                                metadata.is_display,
                                            );
                                        }
                                    } else {
                                        // No asset manager, render as code block
                                        render_math_as_code(
                                            ui,
                                            &format!("Math formula: {}", hash),
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
                        #[cfg(not(feature = "math"))]
                        render_text_with_latex(ui, remaining);
                        #[cfg(feature = "math")]
                        render_text_with_latex(ui, remaining, &mut math_asset_manager);
                    }
                }
            }
            Event::Code(code) => {
                // Inline code
                if in_paragraph {
                    paragraph_content.push(ParagraphContent::InlineCode(code.to_string()));
                } else {
                    ui.label(RichText::new(&*code).code());
                }
            }
            Event::Html(_) => {
                // Skip HTML
            }
            Event::FootnoteReference(_) => {
                // Skip footnotes
            }
            Event::SoftBreak => {
                // Soft line break (treated as space)
                if in_paragraph {
                    paragraph_content.push(ParagraphContent::Text(" ".to_string()));
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
                    paragraph_content.push(ParagraphContent::Text("\n".to_string()));
                } else {
                    ui.add_space(4.0);
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
                        ui.add_space(4.0);
                        paragraph_content.clear();
                    }
                    in_paragraph = false;
                }
                ui.separator();
                ui.add_space(8.0);
            }
            Event::TaskListMarker(checked) => {
                // Task list marker
                let marker = if checked { "[x]" } else { "[ ]" };
                if in_paragraph {
                    paragraph_content.push(ParagraphContent::Text(marker.to_string()));
                } else {
                    ui.label(marker);
                }
            }
        }
    }
}

/// Parse a markdown table from the event stream.
pub(crate) fn parse_table<'a>(
    events: &mut std::iter::Peekable<Parser<'a, 'a>>,
    _alignments: &[Alignment],
) -> Option<(Vec<Vec<String>>, Vec<Vec<String>>)> {
    log::debug!("parse_table called");

    let mut headers = Vec::new();
    let mut rows = Vec::new();
    let mut current_row = Vec::new();
    let mut in_header = false;

    while let Some(event) = events.next() {
        log::debug!("parse_table event: {:?}", event);
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
                while let Some(event) = events.next() {
                    match event {
                        Event::End(Tag::TableCell) => break,
                        Event::Text(text) => cell_text.push_str(&*text),
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

    log::debug!(
        "parse_table returning: headers={} rows={}",
        headers.len(),
        rows.len()
    );
    Some((headers, rows))
}

/// Render text that may contain Typst math expressions.
#[cfg(not(feature = "math"))]
fn render_text_with_latex(ui: &mut Ui, text: &str) {
    render_text_with_math_impl(ui, text, None)
}

/// Render text that may contain Typst math expressions.
#[cfg(feature = "math")]
fn render_text_with_latex(
    ui: &mut Ui,
    text: &str,
    math_asset_manager: &mut Option<&mut crate::math::MathAssetManager>,
) {
    render_text_with_math_impl(ui, text, math_asset_manager)
}

/// Internal implementation for rendering text with math formulas.
#[cfg(not(feature = "math"))]
fn render_text_with_math_impl(ui: &mut Ui, text: &str, _math_asset_manager: Option<()>) {
    render_text_with_math(ui, text)
}

/// Internal implementation for rendering text with math formulas.
#[cfg(feature = "math")]
fn render_text_with_math_impl(
    ui: &mut Ui,
    text: &str,
    math_asset_manager: &mut Option<&mut crate::math::MathAssetManager>,
) {
    // If we have an asset manager, try to render actual SVG textures
    if let Some(asset_manager) = math_asset_manager {
        render_text_with_math_and_assets(ui, text, asset_manager)
    } else {
        // Fall back to code rendering
        render_text_with_math(ui, text)
    }
}

/// Render text with math formulas using SVG assets
#[cfg(feature = "math")]
fn render_text_with_math_and_assets(
    ui: &mut Ui,
    text: &str,
    asset_manager: &mut crate::math::MathAssetManager,
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
                // Get the SVG's intrinsic size
                let svg_size =
                    asset_manager.get_svg_size_for_formula(math_content.trim(), is_display_math);

                if let Some(size) = svg_size {
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
                // Inline math: render inline with adjusted spacing
                // Reduce the image size slightly to account for SVG padding
                let adjusted_size = *size * 0.9; // Reduce by 10% to account for padding
                let image = egui::Image::new(image_source.clone())
                    .tint(ui.visuals().text_color())
                    .fit_to_exact_size(adjusted_size)
                    .corner_radius(0.0);
                ui.add(image);
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
                paragraph_content.push(ParagraphContent::Text(before_text.to_string()));
            }
        }

        // Find the end of the placeholder - look for closing ')'
        if let Some(end) = remaining[start..].find(')') {
            let placeholder = &remaining[start..start + end + 1];

            // Check if this is a math placeholder: (hash.typ)
            if placeholder.ends_with(".typ)") && placeholder.len() > 6 {
                // Extract hash: remove '(' and '.typ)'
                let hash = &placeholder[1..placeholder.len() - 5];

                // Look up metadata in manifest
                if let Some(metadata) = manifest.get_metadata(hash) {
                    // Inline math - accumulate in paragraph content
                    if let Some(asset_manager) = math_asset_manager {
                        // Try to get SVG using hash
                        if let Some(image_source) = asset_manager.get_image_source_for_hash(hash) {
                            // Get the SVG's intrinsic size
                            let svg_size = asset_manager.get_svg_size(hash);

                            if let Some(size) = svg_size {
                                paragraph_content.push(ParagraphContent::MathImage {
                                    image_source,
                                    size,
                                    is_display: metadata.is_display,
                                });
                            } else {
                                // Fallback: use code rendering
                                paragraph_content.push(ParagraphContent::MathCode {
                                    content: format!("Math formula: {}", hash),
                                    is_display: metadata.is_display,
                                });
                            }
                        } else {
                            // Fallback: render as code
                            paragraph_content.push(ParagraphContent::MathCode {
                                content: format!("Math formula: {}", hash),
                                is_display: metadata.is_display,
                            });
                        }
                    } else {
                        // No asset manager, render as code
                        paragraph_content.push(ParagraphContent::MathCode {
                            content: format!("Math formula: {}", hash),
                            is_display: metadata.is_display,
                        });
                    }
                } else {
                    // Hash not found in manifest, add placeholder as text
                    paragraph_content.push(ParagraphContent::Text(placeholder.to_string()));
                }
            } else {
                // Not a math placeholder, add as normal text
                paragraph_content.push(ParagraphContent::Text(placeholder.to_string()));
            }

            // Skip past the placeholder
            remaining = &remaining[start + end + 1..];
        } else {
            // No closing ')', add the '(' and continue
            paragraph_content.push(ParagraphContent::Text("(".to_string()));
            remaining = &remaining[start + 1..];
        }
    }

    // Add any remaining text
    if !remaining.is_empty() {
        paragraph_content.push(ParagraphContent::Text(remaining.to_string()));
    }
}

/// Render a markdown table.
fn render_table(
    ui: &mut Ui,
    alignments: &[Alignment],
    headers: &[Vec<String>],
    rows: &[Vec<String>],
) {
    log::debug!(
        "render_table: headers={}, rows={}",
        headers.len(),
        rows.len()
    );

    // Use the enhanced table renderer with default configuration
    table_renderer::render_table_simple(ui, alignments, headers, rows);
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
                let result = parse_table(&mut events, &alignments);
                assert!(result.is_some());
                let (headers, rows) = result.unwrap();
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
