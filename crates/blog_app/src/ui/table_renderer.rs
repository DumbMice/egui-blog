//! Custom table rendering for markdown tables with enhanced styling.

use egui::{Align, Layout, Pos2, RichText, Stroke, StrokeKind, Ui};
use pulldown_cmark::Alignment;

/// Configuration for table rendering.
#[derive(Clone, Debug)]
pub struct TableConfig {
    /// Show outer border around the table
    pub show_border: bool,
    /// Border width in points
    pub border_width: f32,
    /// Show vertical separators between columns
    /// Note: Currently disabled by default because we can't track column positions easily
    pub _show_column_separators: bool,
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
    config: TableConfig,
) {
    if headers.is_empty() && rows.is_empty() {
        return;
    }

    // Determine number of columns from first row
    let col_count = headers
        .first()
        .map(|r| r.len())
        .or_else(|| rows.first().map(|r| r.len()))
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
                            let label = if config.header_background {
                                RichText::new(cell).strong()
                            } else {
                                RichText::new(cell).strong()
                            };

                            // Apply alignment
                            match alignment {
                                Alignment::Left => {
                                    ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                        ui.label(label);
                                    });
                                }
                                Alignment::Center => {
                                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                                        ui.label(label);
                                    });
                                }
                                Alignment::Right => {
                                    ui.with_layout(Layout::left_to_right(Align::Max), |ui| {
                                        ui.label(label);
                                    });
                                }
                                Alignment::None => {
                                    ui.label(label);
                                }
                            }
                        }
                        ui.end_row();
                    }
                }

                // Render data rows
                for row in rows {
                    for (col_idx, cell) in row.iter().enumerate() {
                        let alignment = alignments.get(col_idx).copied().unwrap_or(Alignment::None);

                        // Apply alignment
                        match alignment {
                            Alignment::Left => {
                                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                    ui.label(cell);
                                });
                            }
                            Alignment::Center => {
                                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                                    ui.label(cell);
                                });
                            }
                            Alignment::Right => {
                                ui.with_layout(Layout::left_to_right(Align::Max), |ui| {
                                    ui.label(cell);
                                });
                            }
                            Alignment::None => {
                                ui.label(cell);
                            }
                        }
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

/// Simplified version with default configuration (backward compatible).
pub fn render_table_simple(
    ui: &mut Ui,
    alignments: &[Alignment],
    headers: &[Vec<String>],
    rows: &[Vec<String>],
) {
    render_table(ui, alignments, headers, rows, TableConfig::default());
}
