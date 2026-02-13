//! Markdown rendering for blog posts.

use egui::{RichText, Ui};
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};

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

                        for (i, item) in list_items.iter().enumerate() {
                            let prefix = match ordered {
                                Some(start) => format!("{}. ", start + i as u64),
                                None => "• ".to_string(),
                            };
                            ui.horizontal(|ui| {
                                ui.add_space(16.0);
                                ui.label(prefix);
                                ui.label(item);
                            });
                        }
                        ui.add_space(4.0);
                    }
                    Tag::Item => {
                        // Already handled in List
                    }
                    Tag::CodeBlock(_kind) => {
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
                        // Display code in monospace font with background
                        let mut display_text = code_text.clone();
                        ui.add(
                            egui::TextEdit::multiline(&mut display_text)
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY)
                                .frame(true)
                                .interactive(false) // Make it read-only
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
                        let mut link_text = String::new();
                        while let Some(event) = events.next() {
                            match event {
                                Event::End(Tag::Link(_, _, _)) => break,
                                Event::Text(text) => link_text.push_str(&*text),
                                Event::SoftBreak => link_text.push(' '),
                                _ => {} // Skip other events
                            }
                        }

                        if ui.link(&link_text).clicked() {
                            // In a real app, you might want to open the URL
                            // For now, just display as clickable text
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
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                            ui.add_space(8.0);
                            ui.vertical(|ui| {
                                ui.label(RichText::new(quote_text).color(ui.visuals().weak_text_color()));
                            });
                        });
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
                        // Skip strikethrough for now
                        while let Some(event) = events.next() {
                            if matches!(event, Event::End(Tag::Strikethrough)) {
                                break;
                            }
                        }
                    }
                    _ => {
                        // Skip any other tags we don't handle
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