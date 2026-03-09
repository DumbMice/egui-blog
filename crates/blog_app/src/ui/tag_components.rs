//! Tag-related UI components.

use egui::{Color32, Response, RichText, Ui};

use crate::tags::{Tag, TagSearchState};

/// Display a tag as an interactive chip.
pub fn tag_chip(ui: &mut Ui, tag: &Tag, search_state: &mut TagSearchState) -> Response {
    let response = ui.add(
        egui::Button::new(
            RichText::new(format!("#{}", tag.name))
                .small()
                .color(Color32::WHITE),
        )
        .fill(tag.color)
        .corner_radius(egui::CornerRadius::same(8))
        .min_size(egui::vec2(0.0, 20.0)),
    );

    if response.clicked() {
        search_state.add_tag(tag.name.clone());
    }

    let response = if let Some(description) = &tag.description {
        response.on_hover_text(description)
    } else {
        response.on_hover_text(format!("{} posts", tag.post_count))
    };

    response
}

/// Display selected tags as removable chips.
pub fn selected_tags_chips(
    ui: &mut Ui,
    search_state: &mut TagSearchState,
    all_tags: &[Tag],
) -> bool {
    let mut changed = false;

    ui.horizontal_wrapped(|ui| {
        for tag_name in search_state
            .selected_tags
            .iter()
            .cloned()
            .collect::<Vec<_>>()
        {
            // Find the tag to get its color
            let tag_color = all_tags
                .iter()
                .find(|t| t.name == tag_name)
                .map(|t| t.color)
                .unwrap_or(Color32::GRAY);

            let response = ui.add(
                egui::Button::new(
                    RichText::new(format!("#{} ✕", tag_name))
                        .small()
                        .color(Color32::WHITE),
                )
                .fill(tag_color)
                .corner_radius(egui::CornerRadius::same(8))
                .min_size(egui::vec2(0.0, 20.0)),
            );

            let response = response.on_hover_text("Click to remove");

            if response.clicked() {
                search_state.remove_tag(&tag_name);
                changed = true;
            }
        }
    });

    changed
}

/// Enhanced search bar with tag support.
pub fn tag_search_bar(
    ui: &mut Ui,
    search_state: &mut TagSearchState,
    all_tags: &[Tag],
) -> (bool, bool) {
    let mut search_changed = false;
    let mut tags_changed = false;

    ui.vertical(|ui| {
        // Selected tags chips
        if !search_state.selected_tags.is_empty() {
            tags_changed = selected_tags_chips(ui, search_state, all_tags);
        }

        // Search input
        ui.horizontal(|ui| {
            ui.label("🔍");

            let response = ui.text_edit_singleline(&mut search_state.search_text);

            // Handle tag mode
            if search_state.search_text.ends_with('#') && !search_state.in_tag_mode {
                search_state.in_tag_mode = true;
                search_state.tag_input.clear();
            } else if search_state.in_tag_mode {
                // Update tag input
                if search_state.search_text.ends_with(' ') {
                    // Space ends tag mode
                    search_state.in_tag_mode = false;
                    if !search_state.tag_input.is_empty() {
                        search_state.add_tag(search_state.tag_input.clone());
                        tags_changed = true;
                    }
                    search_state.tag_input.clear();
                } else if let Some(tag_part) = search_state.search_text.strip_prefix('#') {
                    search_state.tag_input = tag_part.to_string();

                    // Update suggestions
                    update_tag_suggestions(search_state, all_tags);
                }
            }

            if response.changed() {
                search_changed = true;
            }

            // Clear button
            if search_state.is_active() && ui.button("❌").on_hover_text("Clear search").clicked()
            {
                search_state.reset();
                search_changed = true;
                tags_changed = true;
            }
        });

        // Tag suggestions dropdown
        if search_state.in_tag_mode && !search_state.suggestions.is_empty() {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                // Clone suggestions to avoid borrow issues
                let suggestions = search_state.suggestions.clone();
                for (idx, tag) in suggestions.iter().enumerate() {
                    let is_highlighted = idx == search_state.highlighted_index;

                    let response = ui.selectable_label(is_highlighted, format!("#{}", tag.name));

                    if response.clicked() {
                        search_state.add_tag(tag.name.clone());
                        search_state.in_tag_mode = false;
                        search_state.search_text.clear();
                        search_state.tag_input.clear();
                        tags_changed = true;
                        search_changed = true;
                    }
                }
            });
        }
    });

    (search_changed, tags_changed)
}

/// Update tag suggestions based on current input.
fn update_tag_suggestions(search_state: &mut TagSearchState, all_tags: &[Tag]) {
    search_state.suggestions.clear();

    if search_state.tag_input.is_empty() {
        // Show all tags not already selected
        for tag in all_tags {
            if !search_state.has_tag(&tag.name) {
                search_state.suggestions.push(tag.clone());
            }
        }
    } else {
        // Filter tags by input
        let input_lower = search_state.tag_input.to_lowercase();
        for tag in all_tags {
            if !search_state.has_tag(&tag.name) && tag.name.to_lowercase().contains(&input_lower) {
                search_state.suggestions.push(tag.clone());
            }
        }
    }

    // Sort by post count (most popular first)
    search_state
        .suggestions
        .sort_by(|a, b| b.post_count.cmp(&a.post_count));

    // Reset highlighted index
    search_state.highlighted_index = 0;
}
