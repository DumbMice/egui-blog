//! Example counter widget.

use crate::widgets::{Widget, WidgetConfig};
use egui::Ui;

/// Simple counter widget
#[derive(Default)]
pub struct CounterWidget {
    count: i32,
    label: String,
}

impl Widget for CounterWidget {
    fn name(&self) -> &'static str {
        "counter"
    }

    fn display_name(&self) -> &'static str {
        "Counter"
    }

    fn render(&mut self, ui: &mut Ui, config: &WidgetConfig) -> egui::Vec2 {
        // Extract configuration
        let label = config
            .config
            .get("label")
            .and_then(|v| v.as_str())
            .unwrap_or("Count");

        let step: i32 = config
            .config
            .get("step")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .unwrap_or(1);

        self.label = label.to_string();

        // Generate a unique ID using ui's id generator with source string
        // This ensures each widget instance gets a unique ID even with same config
        let id_source = format!("counter_{:?}", config.config);
        let widget_id = ui.id().with(id_source);

        // Load state from memory
        if let Some(saved_state) = ui.data(|d| d.get_temp::<serde_json::Value>(widget_id)) {
            self.deserialize_state(saved_state);
        }

        // Render the widget
        ui.vertical(|ui| {
            ui.heading(&format!("{}: {}", self.label, self.count));

            ui.horizontal(|ui| {
                if ui.button("−").clicked() {
                    self.count -= step;
                }

                if ui.button("Reset").clicked() {
                    self.count = 0;
                }

                if ui.button("+").clicked() {
                    self.count += step;
                }
            });
        });

        // Save state to memory
        ui.data_mut(|d| d.insert_temp(widget_id, self.serialize_state()));

        // Return estimated size
        egui::vec2(200.0, 100.0)
    }

    fn update(&mut self, _ctx: &egui::Context) {
        // No continuous updates needed for counter
    }

    fn serialize_state(&self) -> serde_json::Value {
        serde_json::json!({
            "count": self.count,
            "label": self.label,
        })
    }

    fn deserialize_state(&mut self, state: serde_json::Value) {
        if let Some(count) = state.get("count").and_then(|v| v.as_i64()) {
            self.count = count as i32;
        }
        if let Some(label) = state.get("label").and_then(|v| v.as_str()) {
            self.label = label.to_string();
        }
    }
}
