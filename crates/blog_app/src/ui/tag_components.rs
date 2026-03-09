//! Tag-related UI components.

use egui::{Color32, Response, RichText, Ui};

use crate::tags::{Tag, TagSearchState};

/// Determine text color for good contrast on a background color
/// Uses appropriate text color based on background luminance and current theme
fn text_color_for_background(bg_color: Color32, ui: &Ui) -> Color32 {
    // Get current text color from theme
    let current_text_color = ui.visuals().widgets.noninteractive.fg_stroke.color;

    // Calculate luminance of background and current text
    let bg_luminance = calculate_luminance(bg_color);
    let text_luminance = calculate_luminance(current_text_color);

    // Calculate contrast ratio between background and current text
    let contrast_ratio = if bg_luminance > text_luminance {
        (bg_luminance + 0.05) / (text_luminance + 0.05)
    } else {
        (text_luminance + 0.05) / (bg_luminance + 0.05)
    };

    // WCAG requires at least 4.5:1 contrast for normal text
    if contrast_ratio >= 4.5 {
        // Current text color has sufficient contrast
        current_text_color
    } else {
        // Insufficient contrast, choose opposite luminance text
        if bg_luminance < 128.0 {
            // Dark background, use light text
            // Use theme's text color (should be light in dark mode)
            current_text_color
        } else {
            // Light background, use dark text
            // If current text is already dark, use it
            if text_luminance < 128.0 {
                current_text_color
            } else {
                // Current text is light, need dark text
                // Use a dark gray that works in both themes
                Color32::from_gray(30) // Dark gray
            }
        }
    }
}

/// Calculate relative luminance of a color (0-255 scale)
fn calculate_luminance(color: Color32) -> f32 {
    let r = color.r() as f32 / 255.0;
    let g = color.g() as f32 / 255.0;
    let b = color.b() as f32 / 255.0;

    // Convert to linear RGB
    let r_lin = if r <= 0.04045 {
        r / 12.92
    } else {
        ((r + 0.055) / 1.055).powf(2.4)
    };

    let g_lin = if g <= 0.04045 {
        g / 12.92
    } else {
        ((g + 0.055) / 1.055).powf(2.4)
    };

    let b_lin = if b <= 0.04045 {
        b / 12.92
    } else {
        ((b + 0.055) / 1.055).powf(2.4)
    };

    // Relative luminance formula
    (0.2126 * r_lin + 0.7152 * g_lin + 0.0722 * b_lin) * 255.0
}

/// Display a tag as an interactive chip.
pub fn tag_chip(ui: &mut Ui, tag: &Tag, search_state: &mut TagSearchState) -> Response {
    let text_color = text_color_for_background(tag.color, ui);
    let response = ui.add(
        egui::Button::new(
            RichText::new(format!("#{}", tag.name))
                .small()
                .color(text_color),
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

            let text_color = text_color_for_background(tag_color, ui);
            let response = ui.add(
                egui::Button::new(
                    RichText::new(format!("#{} ✕", tag_name))
                        .small()
                        .color(text_color),
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
                    let new_tag_input = tag_part.to_string();

                    // Only update suggestions if tag input actually changed
                    if new_tag_input != search_state.tag_input {
                        search_state.tag_input = new_tag_input;

                        // Update suggestions
                        update_tag_suggestions(search_state, all_tags);
                    }
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
            egui::Area::new(egui::Id::new("tag_autocomplete_dropdown"))
                .order(egui::Order::Foreground)
                .fixed_pos(ui.cursor().left_bottom())
                .show(ui.ctx(), |ui| {
                    egui::Frame::popup(ui.style())
                        .inner_margin(egui::Margin::same(4))
                        .show(ui, |ui| {
                            // Clone suggestions to avoid borrow issues
                            let suggestions = search_state.suggestions.clone();

                            // Limit the number of suggestions shown
                            let max_suggestions = 10;
                            let suggestions_to_show = suggestions.iter().take(max_suggestions);

                            egui::ScrollArea::vertical()
                                .max_height(200.0)
                                .show(ui, |ui| {
                                    for (idx, tag) in suggestions_to_show.enumerate() {
                                        let is_highlighted = idx == search_state.highlighted_index;

                                        let response = ui.selectable_label(
                                            is_highlighted,
                                            format!("#{} ({})", tag.name, tag.post_count),
                                        );

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
                        });
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
