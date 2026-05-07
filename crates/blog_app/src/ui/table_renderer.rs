//! Custom table rendering for markdown tables with enhanced styling.

use egui::{Align, Layout, Pos2, Rect, Stroke, StrokeKind, TextStyle, Ui, UiBuilder, Vec2};
use pulldown_cmark::Alignment;

#[derive(Clone)]
struct TableMeasurements {
    col_widths: Vec<f32>,
    row_height: f32,
    cell_widths: Vec<Vec<f32>>,
}

/// Render table cell content without wrapping (for proper column width calculation)
fn render_table_cell_content(
    ui: &mut Ui,
    content: &[crate::ui::markdown::ParagraphContent],
    text_style: &TextStyle,
    alignment: Alignment,
) {
    // Add vertical padding similar to GitHub's table styling (4px top/bottom)
    ui.vertical(|ui| {
        ui.add_space(4.0);

        // Render horizontally without wrapping to maintain column width
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            // Apply padding based on alignment
            match alignment {
                Alignment::Left | Alignment::None => {
                    ui.add_space(8.0); // Left padding
                    render_cell_content_items(ui, content, text_style);
                    ui.add_space(8.0); // Right padding
                }
                Alignment::Center => {
                    // Center alignment: let layout center the content
                    // Add equal padding on both sides for visual spacing
                    ui.add_space(4.0); // Half of normal padding
                    render_cell_content_items(ui, content, text_style);
                    ui.add_space(4.0); // Half of normal padding
                }
                Alignment::Right => {
                    // Right alignment: content ends at right edge
                    // Add flexible space before content
                    ui.add(egui::Label::new("").wrap());
                    render_cell_content_items(ui, content, text_style);
                    ui.add_space(8.0); // Right padding
                }
            }
        });

        ui.add_space(4.0);
    });
}

/// Helper function to render cell content items
fn render_cell_content_items(
    ui: &mut Ui,
    content: &[crate::ui::markdown::ParagraphContent],
    text_style: &TextStyle,
) {
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
            crate::ui::markdown::ParagraphContent::Widget { .. } => {
                // Widgets in tables not supported - skip
            }
        }
    }
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

    // Apply alignment - GitHub tables default to left-aligned for Alignment::None
    let actual_alignment = match alignment {
        Alignment::None => Alignment::Left, // GitHub default is left-aligned
        _ => alignment,
    };

    if matches!(actual_alignment, Alignment::Right) {
        ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
            render_table_cell_content(ui, &paragraph_content, &text_style, actual_alignment);
        });
    } else {
        render_table_cell_content(ui, &paragraph_content, &text_style, actual_alignment);
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

    let col_count = headers
        .first()
        .map(Vec::len)
        .or_else(|| rows.first().map(Vec::len))
        .unwrap_or(0);

    if col_count == 0 {
        return;
    }

    ui.add_space(config.outer_margin);

    let all_headers_empty = headers
        .iter()
        .all(|row| row.iter().all(|cell| cell.trim().is_empty()));

    ui.push_id("markdown_table", |ui| {
        let measurement_id = ui.auto_id_with("table_measurements");

        let has_measurements: bool = ui
            .ctx()
            .data(|data| data.get_temp::<TableMeasurements>(measurement_id).is_some());

        let (col_widths, row_height, cell_widths) = if has_measurements {
            let m = ui
                .ctx()
                .data(|data| data.get_temp::<TableMeasurements>(measurement_id))
                .expect("has_measurements was true");
            (m.col_widths, m.row_height, m.cell_widths)
        } else {
            let mut col_widths = vec![0.0f32; col_count];
            let mut row_height = 0.0f32;
            let mut cell_widths: Vec<Vec<f32>> = Vec::new();

            if !all_headers_empty {
                for header_row in headers {
                    let mut row_widths = Vec::with_capacity(col_count);
                    for (col, cell) in header_row.iter().enumerate() {
                        let alignment = alignments.get(col).copied().unwrap_or(Alignment::None);
                        let mut cell_size = Vec2::ZERO;
                        ui.scope_builder(UiBuilder::new().sizing_pass().invisible(), |ui| {
                            render_table_cell(
                                ui,
                                cell,
                                alignment,
                                true,
                                math_asset_manager,
                                math_resolution_scale,
                                text_segment_cache,
                            );
                            cell_size = ui.min_rect().size();
                        });
                        col_widths[col] = col_widths[col].max(cell_size.x);
                        row_height = row_height.max(cell_size.y);
                        row_widths.push(cell_size.x);
                    }
                    cell_widths.push(row_widths);
                }
            }

            for row in rows {
                let mut row_widths = Vec::with_capacity(col_count);
                for (col, cell) in row.iter().enumerate() {
                    let alignment = alignments.get(col).copied().unwrap_or(Alignment::None);
                    let mut cell_size = Vec2::ZERO;
                    ui.scope_builder(UiBuilder::new().sizing_pass().invisible(), |ui| {
                        render_table_cell(
                            ui,
                            cell,
                            alignment,
                            false,
                            math_asset_manager,
                            math_resolution_scale,
                            text_segment_cache,
                        );
                        cell_size = ui.min_rect().size();
                    });
                    col_widths[col] = col_widths[col].max(cell_size.x);
                    row_height = row_height.max(cell_size.y);
                    row_widths.push(cell_size.x);
                }
                cell_widths.push(row_widths);
            }

            ui.ctx().data_mut(|data| {
                data.insert_temp(
                    measurement_id,
                    TableMeasurements {
                        col_widths: col_widths.clone(),
                        row_height,
                        cell_widths: cell_widths.clone(),
                    },
                );
            });
            ui.ctx().request_discard("Table measurement pass");

            (col_widths, row_height, cell_widths)
        };

        if !has_measurements {
            return;
        }

        let total_rows = {
            let h = if all_headers_empty { 0 } else { headers.len() };
            h + rows.len()
        };
        let cell_padding = config.cell_padding;

        let available = ui.available_width();
        let table_width: f32 =
            col_widths.iter().sum::<f32>() + cell_padding * (col_count.saturating_sub(1)) as f32;
        let table_height =
            total_rows as f32 * row_height + (total_rows.saturating_sub(1)) as f32 * cell_padding;
        let left_pad = ((available - table_width) / 2.0).max(0.0);

        // Render the table in a horizontal layout. add_space moves the cursor
        // horizontally here, centering the table within the available width.
        ui.horizontal(|ui| {
            ui.add_space(left_pad);

            let start_x = ui.cursor().left();
            let start_y = ui.cursor().top();

            let mut row_idx = 0usize;
            let cell_padding = config.cell_padding;

            if !all_headers_empty {
                for header_row in headers {
                    let y = start_y + row_idx as f32 * (row_height + cell_padding);

                    if config.header_background {
                        let header_bg_rect = Rect::from_min_size(
                            Pos2::new(start_x, y),
                            Vec2::new(table_width, row_height),
                        );
                        ui.painter().rect_filled(
                            header_bg_rect,
                            0.0,
                            ui.visuals().widgets.noninteractive.bg_fill,
                        );
                    }

                    let mut x = start_x;
                    for (col, cell) in header_row.iter().enumerate() {
                        let alignment = alignments.get(col).copied().unwrap_or(Alignment::None);
                        let cell_rect = Rect::from_min_size(
                            Pos2::new(x, y),
                            Vec2::new(col_widths[col], row_height),
                        );
                        ui.scope_builder(UiBuilder::new().max_rect(cell_rect), |ui| {
                            if alignment == Alignment::Center {
                                let pw = cell_widths[row_idx][col];
                                ui.add_space((col_widths[col] - pw) / 2.0);
                            }
                            render_table_cell(
                                ui, cell, alignment, true,
                                math_asset_manager, math_resolution_scale,
                                text_segment_cache,
                            );
                        });
                        x += col_widths[col] + cell_padding;
                    }
                    row_idx += 1;
                }
            }

            for row in rows {
                let y = start_y + row_idx as f32 * (row_height + cell_padding);

                if config.striped_rows && row_idx % 2 == 1 {
                    let row_rect = Rect::from_min_size(
                        Pos2::new(start_x, y),
                        Vec2::new(table_width, row_height),
                    );
                    ui.painter()
                        .rect_filled(row_rect, 0.0, ui.visuals().weak_text_color().gamma_multiply(0.08));
                }

                let mut x = start_x;
                for (col, cell) in row.iter().enumerate() {
                    let alignment = alignments.get(col).copied().unwrap_or(Alignment::None);
                    let cell_rect = Rect::from_min_size(
                        Pos2::new(x, y),
                        Vec2::new(col_widths[col], row_height),
                    );
                    ui.scope_builder(UiBuilder::new().max_rect(cell_rect), |ui| {
                        if alignment == Alignment::Center {
                            let pw = cell_widths[row_idx][col];
                            ui.add_space((col_widths[col] - pw) / 2.0);
                        }
                        render_table_cell(
                            ui, cell, alignment, false,
                            math_asset_manager, math_resolution_scale,
                            text_segment_cache,
                        );
                    });
                    x += col_widths[col] + cell_padding;
                }
                row_idx += 1;
            }

            let painter = ui.painter();
            let separator_color = ui.visuals().weak_text_color();
            let stroke = Stroke::new(config.border_width, separator_color);
            let header_stroke_color = ui.visuals().widgets.active.bg_stroke.color;
            let header_stroke = Stroke::new(config.border_width, header_stroke_color);

            let table_rect = Rect::from_min_size(
                Pos2::new(start_x, start_y),
                Vec2::new(table_width, table_height),
            );

            if config.show_border {
                let border_rect = table_rect.expand(config.cell_padding);
                painter.rect_stroke(border_rect, 0.0, stroke, StrokeKind::Inside);
            }

            // Vertical column separators
            if col_count > 1 {
                let mut sep_x = start_x;
                for i in 0..col_count - 1 {
                    sep_x += col_widths[i] + cell_padding / 2.0;
                    let top = Pos2::new(sep_x, table_rect.min.y);
                    let bottom = Pos2::new(sep_x, table_rect.max.y);
                    painter.line_segment([top, bottom], stroke);
                    sep_x += cell_padding / 2.0;
                }
            }

            if config.show_row_separators && !config.striped_rows && total_rows > 1 {
                for row in 1..total_rows {
                    let y = start_y + row as f32 * (row_height + cell_padding) - cell_padding / 2.0;
                    let line_start = Pos2::new(table_rect.min.x, y);
                    let line_end = Pos2::new(table_rect.max.x, y);
                    painter.line_segment([line_start, line_end], stroke);
                }
            }

            if config.header_background && !headers.is_empty() && !all_headers_empty {
                let header_count = headers.len();
                let header_bottom_y = start_y + header_count as f32 * (row_height + cell_padding);
                let line_start = Pos2::new(table_rect.min.x, header_bottom_y);
                let line_end = Pos2::new(table_rect.max.x, header_bottom_y);
                painter.line_segment([line_start, line_end], header_stroke);
            }
        });

        ui.add_space(config.outer_margin);
    });
}
