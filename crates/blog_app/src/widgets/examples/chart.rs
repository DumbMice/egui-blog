//! Example chart widget.

use crate::widgets::{Widget, WidgetConfig};
use egui::{Color32, Pos2, Rect, Stroke, StrokeKind, Ui};

/// Simple bar chart widget
#[derive(Default)]
pub struct ChartWidget {
    data: Vec<(String, f32)>,
    colors: Vec<Color32>,
    width: f32,
    height: f32,
}

impl ChartWidget {
    fn generate_sample_data(&mut self) {
        self.data = vec![
            ("Jan".to_string(), 45.0),
            ("Feb".to_string(), 52.0),
            ("Mar".to_string(), 48.0),
            ("Apr".to_string(), 65.0),
            ("May".to_string(), 58.0),
            ("Jun".to_string(), 72.0),
        ];

        self.colors = vec![
            Color32::from_rgb(255, 100, 100),
            Color32::from_rgb(100, 255, 100),
            Color32::from_rgb(100, 100, 255),
            Color32::from_rgb(255, 255, 100),
            Color32::from_rgb(255, 100, 255),
            Color32::from_rgb(100, 255, 255),
        ];
    }

    fn generate_from_config(&mut self, data_config: &serde_json::Value) {
        self.data.clear();
        self.colors.clear();

        if let Some(data_array) = data_config.as_array() {
            for (i, item) in data_array.iter().enumerate() {
                if let Some(obj) = item.as_object() {
                    let label = obj
                        .get("label")
                        .and_then(|v| v.as_str())
                        .unwrap_or(&format!("Item {}", i))
                        .to_string();

                    let value = obj
                        .get("value")
                        .and_then(|v| v.as_f64())
                        .map(|v| v as f32)
                        .unwrap_or(0.0);

                    let color = obj
                        .get("color")
                        .and_then(|v| v.as_str())
                        .and_then(|s| parse_color(s))
                        .unwrap_or_else(|| Self::default_color(i));

                    self.data.push((label, value));
                    self.colors.push(color);
                }
            }
        }

        if self.data.is_empty() {
            self.generate_sample_data();
        }
    }

    fn default_color(index: usize) -> Color32 {
        // Convert HSV to RGB manually
        let hues = [0.0, 120.0, 240.0, 60.0, 180.0, 300.0];
        let hue = hues[index % hues.len()];
        let s = 0.8;
        let v = 0.9;

        // HSV to RGB conversion
        let c = v * s;
        let x = c * (1.0 - f32::abs((hue / 60.0) % 2.0 - 1.0));
        let m = v - c;

        let (r, g, b) = if hue < 60.0 {
            (c, x, 0.0)
        } else if hue < 120.0 {
            (x, c, 0.0)
        } else if hue < 180.0 {
            (0.0, c, x)
        } else if hue < 240.0 {
            (0.0, x, c)
        } else if hue < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Color32::from_rgb(
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }
}

fn parse_color(color_str: &str) -> Option<Color32> {
    match color_str {
        "red" => Some(Color32::from_rgb(255, 100, 100)),
        "green" => Some(Color32::from_rgb(100, 255, 100)),
        "blue" => Some(Color32::from_rgb(100, 100, 255)),
        "yellow" => Some(Color32::from_rgb(255, 255, 100)),
        "purple" => Some(Color32::from_rgb(255, 100, 255)),
        "cyan" => Some(Color32::from_rgb(100, 255, 255)),
        _ => {
            if color_str.starts_with('#') && color_str.len() == 7 {
                let r = u8::from_str_radix(&color_str[1..3], 16).ok()?;
                let g = u8::from_str_radix(&color_str[3..5], 16).ok()?;
                let b = u8::from_str_radix(&color_str[5..7], 16).ok()?;
                Some(Color32::from_rgb(r, g, b))
            } else {
                None
            }
        }
    }
}

impl Widget for ChartWidget {
    fn name(&self) -> &'static str {
        "chart"
    }

    fn display_name(&self) -> &'static str {
        "Chart"
    }

    fn render(&mut self, ui: &mut Ui, config: &WidgetConfig) -> egui::Vec2 {
        // Generate a unique ID using ui's id generator
        let id_source = format!(
            "chart_{:?}_{:?}_{:?}",
            config
                .config
                .get("type")
                .unwrap_or(&serde_json::json!("bar")),
            config.width,
            config.height
        );
        let widget_id = ui.id().with(id_source);

        // Load state from memory
        if let Some(saved_state) = ui.data(|d| d.get_temp::<serde_json::Value>(widget_id)) {
            self.deserialize_state(saved_state);
        }

        // Extract configuration
        let chart_type = config
            .config
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("bar");

        let data_config = config
            .config
            .get("data")
            .unwrap_or(&serde_json::Value::Null);

        // Generate or update data
        if self.data.is_empty() || !data_config.is_null() {
            self.generate_from_config(data_config);
        }

        // Set dimensions
        self.width = config.width.unwrap_or(400.0);
        self.height = config.height.unwrap_or(300.0);

        // Render the chart
        ui.vertical(|ui| {
            ui.heading(format!("{} Chart", chart_type));

            // Create a frame for the chart
            let (response, painter) =
                ui.allocate_painter(egui::vec2(self.width, self.height), egui::Sense::hover());

            let rect = response.rect;
            let chart_rect = Rect::from_min_max(
                Pos2::new(rect.left() + 40.0, rect.top() + 20.0),
                Pos2::new(rect.right() - 20.0, rect.bottom() - 40.0),
            );

            // Draw background
            painter.rect_filled(rect, 4.0, Color32::from_gray(20));

            // Draw border
            painter.rect_stroke(
                rect,
                4.0,
                Stroke::new(1.0, Color32::from_gray(60)),
                StrokeKind::Inside,
            );

            // Draw axes
            let axis_color = Color32::from_gray(100);
            let axis_stroke = Stroke::new(1.5, axis_color);

            // X-axis
            painter.line_segment(
                [
                    Pos2::new(chart_rect.left(), chart_rect.bottom()),
                    Pos2::new(chart_rect.right(), chart_rect.bottom()),
                ],
                axis_stroke,
            );

            // Y-axis
            painter.line_segment(
                [
                    Pos2::new(chart_rect.left(), chart_rect.top()),
                    Pos2::new(chart_rect.left(), chart_rect.bottom()),
                ],
                axis_stroke,
            );

            // Find max value for scaling
            let max_value = self
                .data
                .iter()
                .map(|(_, value)| *value)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(1.0)
                .max(1.0);

            // Draw bars if we have data
            if !self.data.is_empty() {
                let bar_width = chart_rect.width() / self.data.len() as f32 * 0.8;
                let bar_spacing = chart_rect.width() / self.data.len() as f32 * 0.2;
                let value_scale = chart_rect.height() / max_value;

                for (i, ((label, value), color)) in
                    self.data.iter().zip(self.colors.iter()).enumerate()
                {
                    let x = chart_rect.left()
                        + i as f32 * (bar_width + bar_spacing)
                        + bar_spacing / 2.0;
                    let bar_height = value * value_scale;

                    let bar_rect = Rect::from_min_size(
                        Pos2::new(x, chart_rect.bottom() - bar_height),
                        egui::vec2(bar_width, bar_height),
                    );

                    // Draw bar
                    painter.rect_filled(bar_rect, 2.0, *color);
                    painter.rect_stroke(
                        bar_rect,
                        2.0,
                        Stroke::new(1.0, Color32::from_gray(80)),
                        egui::StrokeKind::Inside,
                    );

                    // Draw value label on top of bar
                    if bar_height > 20.0 {
                        painter.text(
                            Pos2::new(bar_rect.center().x, bar_rect.top() - 10.0),
                            egui::Align2::CENTER_BOTTOM,
                            format!("{:.1}", value),
                            egui::FontId::default(),
                            Color32::from_gray(200),
                        );
                    }

                    // Draw x-axis label
                    painter.text(
                        Pos2::new(bar_rect.center().x, chart_rect.bottom() + 15.0),
                        egui::Align2::CENTER_TOP,
                        label,
                        egui::FontId::default(),
                        Color32::from_gray(200),
                    );
                }

                // Draw y-axis labels
                for i in 0..=5 {
                    let value = max_value * (i as f32 / 5.0);
                    let y = chart_rect.bottom() - value * value_scale;

                    painter.text(
                        Pos2::new(chart_rect.left() - 5.0, y),
                        egui::Align2::RIGHT_CENTER,
                        format!("{:.0}", value),
                        egui::FontId::default(),
                        Color32::from_gray(200),
                    );

                    // Draw grid line
                    if i > 0 {
                        painter.line_segment(
                            [
                                Pos2::new(chart_rect.left(), y),
                                Pos2::new(chart_rect.right(), y),
                            ],
                            Stroke::new(0.5, Color32::from_gray(40)),
                        );
                    }
                }
            }

            // Draw title
            if let Some(title) = config.config.get("title").and_then(|v| v.as_str()) {
                painter.text(
                    Pos2::new(rect.center().x, rect.top() + 10.0),
                    egui::Align2::CENTER_TOP,
                    title,
                    egui::FontId::proportional(14.0),
                    Color32::from_gray(220),
                );
            }
        });

        // Save state to memory
        ui.data_mut(|d| d.insert_temp(widget_id, self.serialize_state()));

        egui::vec2(self.width, self.height + 40.0)
    }

    fn update(&mut self, _ctx: &egui::Context) {
        // No continuous updates needed for static chart
    }

    fn serialize_state(&self) -> serde_json::Value {
        let data: Vec<serde_json::Value> = self
            .data
            .iter()
            .zip(self.colors.iter())
            .map(|((label, value), color)| {
                serde_json::json!({
                    "label": label,
                    "value": value,
                    "color": format!("#{:02x}{:02x}{:02x}", color.r(), color.g(), color.b()),
                })
            })
            .collect();

        serde_json::json!({
            "data": data,
            "width": self.width,
            "height": self.height,
        })
    }

    fn deserialize_state(&mut self, state: serde_json::Value) {
        if let Some(data_array) = state.get("data").and_then(|v| v.as_array()) {
            self.data.clear();
            self.colors.clear();

            for item in data_array {
                if let Some(label) = item.get("label").and_then(|v| v.as_str()) {
                    let value = item
                        .get("value")
                        .and_then(|v| v.as_f64())
                        .map(|v| v as f32)
                        .unwrap_or(0.0);

                    let color = item
                        .get("color")
                        .and_then(|v| v.as_str())
                        .and_then(|s| parse_color(s))
                        .unwrap_or(Color32::from_rgb(100, 100, 100));

                    self.data.push((label.to_string(), value));
                    self.colors.push(color);
                }
            }
        }

        if let Some(width) = state.get("width").and_then(|v| v.as_f64()) {
            self.width = width as f32;
        }
        if let Some(height) = state.get("height").and_then(|v| v.as_f64()) {
            self.height = height as f32;
        }
    }
}
