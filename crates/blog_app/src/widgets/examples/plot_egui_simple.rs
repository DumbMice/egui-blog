use crate::widgets::{Widget, WidgetConfig};
use egui::{Color32, Ui};
use egui_plot::{Line, Plot, PlotPoints};
use std::f32::consts::PI;

#[derive(Default)]
pub struct EguiPlotWidgetSimple {
    time: f32,
}

impl EguiPlotWidgetSimple {
    fn generate_sine_wave(&self, points: usize, amplitude: f32, frequency: f32) -> PlotPoints<'_> {
        let mut points_vec = Vec::with_capacity(points);
        for i in 0..points {
            let x = i as f64 / points as f64 * 2.0 * PI as f64 * frequency as f64;
            let y = amplitude as f64 * (x + self.time as f64).sin();
            points_vec.push([x, y]);
        }
        PlotPoints::from_iter(points_vec)
    }

    fn generate_cosine_wave(
        &self,
        points: usize,
        amplitude: f32,
        frequency: f32,
    ) -> PlotPoints<'_> {
        let mut points_vec = Vec::with_capacity(points);
        for i in 0..points {
            let x = i as f64 / points as f64 * 2.0 * PI as f64 * frequency as f64;
            let y = amplitude as f64 * (x + self.time as f64).cos();
            points_vec.push([x, y]);
        }
        PlotPoints::from_iter(points_vec)
    }

    fn generate_random(&self, points: usize, amplitude: f32) -> PlotPoints<'_> {
        let mut points_vec = Vec::with_capacity(points);
        for i in 0..points {
            let x = i as f64;
            let y = amplitude as f64 * (rand::random::<f64>() * 2.0 - 1.0);
            points_vec.push([x, y]);
        }
        PlotPoints::from_iter(points_vec)
    }
}

impl Widget for EguiPlotWidgetSimple {
    fn name(&self) -> &'static str {
        "egui_plot_simple"
    }

    fn display_name(&self) -> &'static str {
        "Egui Plot (Simple)"
    }

    fn render(&mut self, ui: &mut Ui, config: &WidgetConfig) -> egui::Vec2 {
        let plot_type = config
            .config
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("sine");

        let points: usize = config
            .config
            .get("points")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(50);

        let amplitude: f32 = config
            .config
            .get("amplitude")
            .and_then(|v| v.as_f64())
            .map(|v| v as f32)
            .unwrap_or(1.0);

        let frequency: f32 = config
            .config
            .get("frequency")
            .and_then(|v| v.as_f64())
            .map(|v| v as f32)
            .unwrap_or(1.0);

        let width = config.width.unwrap_or(300.0);
        let height = config.height.unwrap_or(200.0);

        let state_id_source = format!(
            "egui_plot_simple_state_{}_{}_{}_{}_{}",
            plot_type, width, height, points, amplitude
        );
        let state_id = ui.id().with(&state_id_source);

        use rand::Rng;
        let plot_id_source = format!(
            "egui_plot_simple_{}_{}",
            &state_id_source,
            rand::thread_rng().r#gen::<u32>()
        );
        let plot_id = ui.id().with(plot_id_source);

        if let Some(saved_state) = ui.data(|d| d.get_temp::<serde_json::Value>(state_id)) {
            self.deserialize_state(saved_state);
        }

        ui.vertical(|ui| {
            ui.heading(format!("{} Plot", plot_type));

            let plot = Plot::new(plot_id)
                .width(width)
                .height(height)
                .view_aspect(2.0)
                .show_axes([true, true])
                .show_grid([true, true]);

            let points_data = match plot_type {
                "sine" => self.generate_sine_wave(points, amplitude, frequency),
                "cosine" => self.generate_cosine_wave(points, amplitude, frequency),
                "random" => self.generate_random(points, amplitude),
                _ => self.generate_sine_wave(points, amplitude, frequency),
            };

            let line_color = Color32::from_rgb(100, 150, 255);
            let line = Line::new(plot_type, points_data)
                .color(line_color)
                .width(2.0);

            plot.show(ui, |plot_ui| {
                plot_ui.line(line);
            });
        });

        if plot_type == "sine" || plot_type == "cosine" {
            self.time += 0.05;
        }

        ui.data_mut(|d| d.insert_temp(state_id, self.serialize_state()));

        egui::vec2(width, height + 40.0)
    }

    fn update(&mut self, _ctx: &egui::Context) {
        // Widget updates are handled by the app's update method
        // which controls continuous vs reactive rendering mode
    }

    fn serialize_state(&self) -> serde_json::Value {
        serde_json::json!({
            "time": self.time,
        })
    }

    fn deserialize_state(&mut self, state: serde_json::Value) {
        if let Some(time) = state.get("time").and_then(|v| v.as_f64()) {
            self.time = time as f32;
        }
    }
}
