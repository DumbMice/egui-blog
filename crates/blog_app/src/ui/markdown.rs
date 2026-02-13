//! Markdown rendering for blog posts.

use egui::{Align, Align2, Color32, Hyperlink, LayoutJob, RichText, Sense, Shape, TextFormat, TextStyle, Ui, vec2};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag};

/// Render markdown content to an egui UI.
pub fn render_markdown(ui: &mut Ui, markdown: &str) {
    let parser = Parser::new(markdown);
    let mut events = parser.peekable();

    while let Some(event) = events.next() {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Paragraph => {
                        // Paragraphs are handled by accumulating text
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

                        let rich_text = match level {
                            HeadingLevel::H1 => RichText::new(heading_text).heading().strong(),
                            HeadingLevel::H2 => RichText::new(heading_text).heading(),
                            HeadingLevel::H3 => RichText::new(heading_text).strong(),
                            HeadingLevel::H4 => RichText::new(heading_text).strong(),
                            HeadingLevel::H5 => RichText::new(heading_text),
                            HeadingLevel::H6 => RichText::new(heading_text),
                        };

                        ui.label(rich_text);
                        ui.add_space(4.0);
                    }
                    Tag::List(ordered) => {
                        // Lists
                        let mut list_items = Vec::new();
                        while let Some(event) = events.next() {
                            match event {
                                Event::End(Tag::Item) => break,
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
                                match ordered {
                                    Some(start) => {
                                        let number = (start + i as u64).to_string();
                                        let width = 3.0 * one_indent;
                                        numbered_point(ui, width, &number);
                                        ui.add_space(one_indent / 3.0);
                                    }
                                    None => {
                                        let width = one_indent;
                                        bullet_point(ui, width);
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
                            CodeBlockKind::Fenced(lang) if !lang.is_empty() => Some(lang.to_string()),
                            _ => None,
                        };

                        if let Some(lang) = &language {
                            ui.horizontal(|ui| {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                                    ui.label(RichText::new(lang).small().weak());
                                });
                            });
                        }

                        // Display code in monospace font with background (EasyMark style)
                        let where_to_put_background = ui.painter().add(Shape::Noop);
                        let response = ui.monospace(&code_text);
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
                        ui.label(RichText::new(bold_text).strong());
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
                        ui.label(RichText::new(italic_text).italics());
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

                        ui.add(Hyperlink::from_label_and_url(&link_text, &url));
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
                    Tag::Table(_) => {
                        // Skip tables for now (complex to implement)
                        while let Some(event) = events.next() {
                            if matches!(event, Event::End(Tag::Table(_))) {
                                break;
                            }
                        }
                    }
                    Tag::TableHead | Tag::TableRow | Tag::TableCell => {
                        // Skip table elements
                        while let Some(event) = events.next() {
                            if matches!(event, Event::End(Tag::TableHead | Tag::TableRow | Tag::TableCell)) {
                                break;
                            }
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
                        ui.label(RichText::new(strike_text).strikethrough());
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
                        ui.label(RichText::new(format!("[Image: {}]", alt_text)).italics().weak());
                    }
                }
            }
            Event::End(_) => {
                // End tags are handled within Start match
            }
            Event::Text(text) => {
                // Plain text
                ui.label(&*text);
            }
            Event::Code(code) => {
                // Inline code
                ui.label(RichText::new(&*code).code());
            }
            Event::Html(_) => {
                // Skip HTML
            }
            Event::FootnoteReference(_) => {
                // Skip footnotes
            }
            Event::SoftBreak => {
                // Soft line break (treated as space)
                ui.label(" ");
            }
            Event::HardBreak => {
                // Hard line break
                ui.add_space(4.0);
            }
            Event::Rule => {
                // Horizontal rule
                ui.separator();
                ui.add_space(8.0);
            }
            Event::TaskListMarker(checked) => {
                // Task list marker
                let marker = if checked { "[x]" } else { "[ ]" };
                ui.label(marker);
            }
        }
    }
}

fn bullet_point(ui: &mut Ui, width: f32) -> egui::Response {
    let row_height = ui.text_style_height(&TextStyle::Body);
    let (rect, response) = ui.allocate_exact_size(vec2(width, row_height), Sense::hover());
    ui.painter().circle_filled(
        rect.center(),
        rect.height() / 8.0,
        ui.visuals().strong_text_color(),
    );
    response
}

fn numbered_point(ui: &mut Ui, width: f32, number: &str) -> egui::Response {
    let font_id = TextStyle::Body.resolve(ui.style());
    let row_height = ui.fonts_mut(|f| f.row_height(&font_id));
    let (rect, response) = ui.allocate_exact_size(vec2(width, row_height), Sense::hover());
    let text = format!("{number}.");
    let text_color = ui.visuals().strong_text_color();
    ui.painter().text(
        rect.right_center(),
        Align2::RIGHT_CENTER,
        text,
        font_id,
        text_color,
    );
    response
}

/// Simple markdown renderer for previews (first 200 chars)
pub fn render_markdown_preview(ui: &mut Ui, markdown: &str, max_chars: usize) {
    let preview = if markdown.len() > max_chars {
        format!("{}...", &markdown[..max_chars])
    } else {
        markdown.to_string()
    };

    // Simple rendering for preview - just show plain text
    ui.label(RichText::new(preview).small());
}