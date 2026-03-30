//! Custom table rendering for markdown tables with enhanced styling.

use egui::{Align, Layout, Pos2, Stroke, StrokeKind, TextStyle, Ui};
use pulldown_cmark::Alignment;

/// Render table cell content without wrapping (for proper column width calculation)
fn render_table_cell_content(
    ui: &mut Ui,
    content: &[crate::ui::markdown::ParagraphContent],
    text_style: &TextStyle,
) {
    // Render horizontally without wrapping to maintain column width
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        for item in content {
            match item {
                crate::ui::markdown::ParagraphContent::Text(text) => {
                    let rich_text = egui::RichText::new(text).text_style((*text_style).clone());
                    ui.label(rich_text);
                }
                crate::ui::markdown::ParagraphContent::Strong(text) => {
                    // Use bold text style
                    let bold_style = crate::ui::markdown::bold_text_style(text_style);
                    let rich_text = egui::RichText::new(text).text_style(bold_style);
                    ui.label(rich_text);
                }
                crate::ui::markdown::ParagraphContent::Emphasis(text) => {
                    // Use italic text style if available
                    let italic_style = crate::ui::markdown::italic_text_style(text_style);
                    let rich_text = if italic_style == *text_style {
                        // No italic variant available, use .italics() for slant
                        egui::RichText::new(text)
                            .italics()
                            .text_style(text_style.clone())
                    } else {
                        // Use italic font variant
                        egui::RichText::new(text).text_style(italic_style)
                    };
                    ui.label(rich_text);
                }
                crate::ui::markdown::ParagraphContent::InlineCode(text) => {
                    // Render inline code with monospace font
                    ui.code(text);
                }
                crate::ui::markdown::ParagraphContent::MathImage {
                    image_source,
                    size,
                    is_display: _,
                    baseline_from_top,
                } => {
                    // Render math image with baseline alignment
                    if let Some(baseline) = baseline_from_top {
                        crate::ui::markdown::render_baseline_aligned_image(
                            ui,
                            image_source.clone(),
                            *size,
                            *baseline,
                        );
                    } else {
                        let image = egui::Image::new(image_source.clone())
                            .tint(ui.visuals().text_color())
                            .fit_to_exact_size(*size);
                        ui.add(image);
                    }
                }
                crate::ui::markdown::ParagraphContent::MathCode { content, .. } => {
                    // Fallback: render math as code if no SVG available
                    ui.code(content);
                }
                crate::ui::markdown::ParagraphContent::Link { text, url } => {
                    // Render link (without underline in tables for simplicity)
                    ui.hyperlink_to(text, url);
                }
                crate::ui::markdown::ParagraphContent::Strikethrough(text) => {
                    // Render strikethrough text
                    let rich_text = egui::RichText::new(text)
                        .strikethrough()
                        .text_style((*text_style).clone());
                    ui.label(rich_text);
                }
            }
        }
    });
}

/// Helper function to render a table cell with math formula support
fn render_table_cell(
    ui: &mut Ui,
    cell: &str,
    alignment: Alignment,
    is_header: bool,
    math_asset_manager: &mut Option<&mut crate::math::MathAssetManager>,
    math_resolution_scale: f32,
    text_segment_cache: &mut crate::ui::text_cache::TextSegmentCache,
) {
    // Load manifest for math formula lookup
    let manifest = crate::math::load_manifest();

    // Process text with math placeholders using cached version
    let paragraph_content = crate::ui::markdown::process_text_with_math_cached(
        cell,
        manifest,
        math_asset_manager,
        math_resolution_scale,
        text_segment_cache,
    );

    // Determine text style based on whether it's a header or data cell
    let text_style = if is_header {
        // Headers use medium weight for emphasis
        TextStyle::Name("ContentBodyMedium".into())
    } else {
        // Data cells use regular body text
        TextStyle::Name("ContentBody".into())
    };

    // Apply alignment
    match alignment {
        Alignment::Left => {
            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                render_table_cell_content(ui, &paragraph_content, &text_style);
            });
        }
        Alignment::Center => {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                render_table_cell_content(ui, &paragraph_content, &text_style);
            });
        }
        Alignment::Right => {
            ui.with_layout(Layout::left_to_right(Align::Max), |ui| {
                render_table_cell_content(ui, &paragraph_content, &text_style);
            });
        }
        Alignment::None => {
            render_table_cell_content(ui, &paragraph_content, &text_style);
        }
    }
}

/// Configuration for table rendering.
#[derive(Clone, Debug)]
pub struct TableConfig {
    /// Show outer border around the table
    pub show_border: bool,
    /// Border width in points
    pub border_width: f32,
    /// Show vertical separators between columns
    /// Note: Currently disabled by default because we can't track column positions easily
    pub(crate) _show_column_separators: bool,
    /// Show horizontal separators between rows
    pub show_row_separators: bool,
    /// Use background color for header row
    pub header_background: bool,
    /// Use alternating row colors (striped)
    pub striped_rows: bool,
    /// Margin around the table
    pub outer_margin: f32,
    /// Padding inside cells
    pub cell_padding: f32,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            show_border: true,
            border_width: 1.0,
            _show_column_separators: false, // Disabled because columns aren't equal width
            show_row_separators: false,     // Already have striped rows
            header_background: true,
            striped_rows: true,
            outer_margin: 8.0,
            cell_padding: 4.0,
        }
    }
}

/// Renders a markdown table with enhanced styling.
pub fn render_table(
    ui: &mut Ui,
    alignments: &[Alignment],
    headers: &[Vec<String>],
    rows: &[Vec<String>],
    config: &TableConfig,
    math_asset_manager: &mut Option<&mut crate::math::MathAssetManager>,
    math_resolution_scale: f32,
    text_segment_cache: &mut crate::ui::text_cache::TextSegmentCache,
) {
    if headers.is_empty() && rows.is_empty() {
        return;
    }

    // Determine number of columns from first row
    let col_count = headers
        .first()
        .map(Vec::len)
        .or_else(|| rows.first().map(Vec::len))
        .unwrap_or(0);

    if col_count == 0 {
        return;
    }

    ui.add_space(config.outer_margin);

    // Check if all header cells are empty
    let all_headers_empty = headers
        .iter()
        .all(|row| row.iter().all(|cell| cell.trim().is_empty()));

    // Simple ID for the table
    let table_id_source = "markdown_table";

    // Use push_id to create a unique ID scope for the entire table
    ui.push_id(table_id_source, |ui| {
        // Store the initial cursor position
        let _initial_cursor = ui.cursor();

        // Create a grid with the appropriate number of columns
        // Use a unique ID for each grid
        let grid_id = ui.auto_id_with("table_grid");
        let grid_response = egui::Grid::new(grid_id)
            .striped(config.striped_rows)
            .min_col_width(40.0)
            .spacing([config.cell_padding, config.cell_padding])
            .show(ui, |ui| {
                // Render header rows (skip if all empty)
                if !all_headers_empty {
                    for header_row in headers {
                        for (col_idx, cell) in header_row.iter().enumerate() {
                            let alignment =
                                alignments.get(col_idx).copied().unwrap_or(Alignment::None);

                            render_table_cell(
                                ui,
                                cell,
                                alignment,
                                true, // is_header = true
                                math_asset_manager,
                                math_resolution_scale,
                                text_segment_cache,
                            );
                        }
                        ui.end_row();
                    }
                }

                // Render data rows
                for row in rows {
                    for (col_idx, cell) in row.iter().enumerate() {
                        let alignment = alignments.get(col_idx).copied().unwrap_or(Alignment::None);

                        render_table_cell(
                            ui,
                            cell,
                            alignment,
                            false, // is_header = false
                            math_asset_manager,
                            math_resolution_scale,
                            text_segment_cache,
                        );
                    }
                    ui.end_row();
                }
            });

        // Get the actual bounds of the rendered grid from the response
        let grid_bounds = grid_response.response.rect;

        // Draw borders and separators
        let painter = ui.painter();
        let stroke_color = ui.visuals().widgets.noninteractive.bg_stroke.color;
        let stroke = Stroke::new(config.border_width, stroke_color);
        let header_stroke_color = ui.visuals().widgets.active.bg_stroke.color;
        let header_stroke = Stroke::new(config.border_width, header_stroke_color);

        // Draw outer border
        if config.show_border {
            let border_rect = grid_bounds.expand(config.cell_padding);
            painter.rect_stroke(border_rect, 0.0, stroke, StrokeKind::Inside);
        }

        // Note: Column separators are disabled by default because we can't easily
        // track column positions with egui::Grid. If needed, we could implement
        // a custom table renderer that tracks column boundaries.

        // Draw row separators (if enabled and not using striped rows)
        if config.show_row_separators && !config.striped_rows {
            let total_rows = headers.len() + rows.len();
            if total_rows > 1 {
                let row_height = grid_bounds.height() / total_rows as f32;
                for row in 1..total_rows {
                    let y = grid_bounds.min.y + row_height * row as f32;
                    let line_start = Pos2::new(grid_bounds.min.x, y);
                    let line_end = Pos2::new(grid_bounds.max.x, y);
                    painter.line_segment([line_start, line_end], stroke);
                }
            }
        }

        // Draw header bottom border (if header background is enabled and there are non-empty headers)
        if config.header_background && !headers.is_empty() && !all_headers_empty {
            let header_height =
                grid_bounds.height() * (headers.len() as f32 / (headers.len() + rows.len()) as f32);
            let header_bottom_y = grid_bounds.min.y + header_height;
            let line_start = Pos2::new(grid_bounds.min.x, header_bottom_y);
            let line_end = Pos2::new(grid_bounds.max.x, header_bottom_y);
            painter.line_segment([line_start, line_end], header_stroke);
        }

        ui.add_space(config.outer_margin);
    });
}
