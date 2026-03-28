//! Simple search bar test to isolate cursor positioning issues

use egui::{Response, Ui};

/// Simple search bar without tag functionality
/// Used to test if cursor positioning issues exist in basic egui `TextEdit`
#[derive(Default)]
pub struct SimpleSearchTest {
    pub search_text: String,
    pub change_count: u32,
    pub last_change_frame: u64,
}

impl SimpleSearchTest {
    pub fn new() -> Self {
        Self::default()
    }

    /// Display a simple search bar and return if text changed
    pub fn show(&mut self, ui: &mut Ui) -> (bool, Response) {
        let mut changed = false;

        ui.horizontal(|ui| {
            ui.label("🔍");

            // Simple text edit without any tag functionality
            let response = ui.text_edit_singleline(&mut self.search_text);

            if response.changed() {
                changed = true;
                self.change_count += 1;

                // Debug logging removed for performance
                // #[cfg(target_arch = "wasm32")]
                // log::debug!(
                //     "Simple search changed #{}, text: '{}' (len: {})",
                //     self.change_count,
                //     self.search_text,
                //     self.search_text.len()
                // );
            }

            // Clear button
            if !self.search_text.is_empty()
                && ui.button("❌").on_hover_text("Clear search").clicked()
            {
                self.search_text.clear();
                changed = true;
                self.change_count += 1;

                // Debug logging removed for performance
                // #[cfg(target_arch = "wasm32")]
                // log::debug!("Simple search cleared");
            }
        });

        (changed, ui.response())
    }

    /// Alternative version using `TextEdit` builder with ID
    pub fn show_with_id(&mut self, ui: &mut Ui) -> (bool, Response) {
        let mut changed = false;

        ui.horizontal(|ui| {
            ui.label("🔍");

            // TextEdit with stable ID
            let response = ui.add(
                egui::TextEdit::singleline(&mut self.search_text)
                    .id(egui::Id::new("simple_search_test"))
                    .desired_width(200.0),
            );

            if response.changed() {
                changed = true;
                self.change_count += 1;

                // Debug logging removed for performance
                // #[cfg(target_arch = "wasm32")]
                // log::debug!(
                //     "Simple search (with ID) changed #{}, text: '{}' (len: {})",
                //     self.change_count,
                //     self.search_text,
                //     self.search_text.len()
                // );
            }

            // Clear button
            if !self.search_text.is_empty()
                && ui.button("❌").on_hover_text("Clear search").clicked()
            {
                self.search_text.clear();
                changed = true;
                self.change_count += 1;

                // Debug logging removed for performance
                // #[cfg(target_arch = "wasm32")]
                // log::debug!("Simple search (with ID) cleared");
            }
        });

        (changed, ui.response())
    }
}
