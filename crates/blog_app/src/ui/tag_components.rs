//! Tag-related UI components.

use egui::{Color32, Response, RichText, Ui};

use crate::tags::{Tag, TagSearchState};

/// Get cursor position with fallback to prevent NaN crashes in WASM
/// Always returns a valid position, using fallback if needed
fn get_dropdown_position(ui: &Ui) -> egui::Pos2 {
    let cursor_rect = ui.cursor();
    let mut pos = cursor_rect.left_bottom();

    // Check for NaN or infinite values
    if pos.x.is_nan() || pos.y.is_nan() || !pos.x.is_finite() || !pos.y.is_finite() {
        #[cfg(target_arch = "wasm32")]
        log::warn!("Invalid cursor position in WASM: {:?}, using fallback", pos);

        // Use a fallback position based on the text edit widget area
        // Get the last widget rect (should be the text edit or clear button)
        if let Some(last_widget) = ui
            .ctx()
            .data(|d| d.get_temp::<egui::Rect>(egui::Id::new("last_search_widget")))
        {
            if last_widget.is_positive() {
                pos = last_widget.right_bottom();
            } else {
                // Ultimate fallback: position below search icon
                pos = ui.cursor().min; // Top-left of current cursor
                pos.y += 30.0; // Move down a bit
            }
        } else {
            // Fallback: position below search icon
            pos = ui.cursor().min; // Top-left of current cursor
            pos.y += 30.0; // Move down a bit
        }
    }

    pos
}

/// Determine text color for good contrast on a background color
/// Uses Catppuccin theme colors: text color for contrast, falls back to base color if needed
fn text_color_for_background(bg_color: Color32, ui: &Ui) -> Color32 {
    let visuals = ui.visuals();

    // Get theme text color (should be appropriate for normal text on panel_fill)
    let text_color = visuals.widgets.noninteractive.fg_stroke.color;
    // Simple luminance check - if background is dark, text should be light, and vice versa
    let bg_luminance =
        bg_color.r() as f32 * 0.299 + bg_color.g() as f32 * 0.587 + bg_color.b() as f32 * 0.114;

    let text_luminance = text_color.r() as f32 * 0.299
        + text_color.g() as f32 * 0.587
        + text_color.b() as f32 * 0.114;

    // Check if text color has good contrast with background
    // Simple heuristic: if both are light or both are dark, we need opposite
    let bg_is_light = bg_luminance > 128.0;
    let text_is_light = text_luminance > 128.0;

    if bg_is_light == text_is_light {
        // Both light or both dark - need opposite
        // For Catppuccin themes, we can use the panel_fill color as opposite
        // (light theme: panel_fill is light, text is dark; dark theme: panel_fill is dark, text is light)
        visuals.panel_fill
    } else {
        // Good contrast
        text_color
    }
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

    if let Some(description) = &tag.description {
        response.on_hover_text(description)
    } else {
        response.on_hover_text(format!("{} posts", tag.post_count))
    }
}

/// Display selected tags as removable chips.
pub fn selected_tags_chips(
    ui: &mut Ui,
    search_state: &mut TagSearchState,
    all_tags: &[Tag],
) -> bool {
    let mut changed = false;

    ui.horizontal_wrapped(|ui| {
        let tag_names: Vec<String> = search_state.selected_tags.clone();
        for tag_name in tag_names {
            // Find the tag to get its color
            let tag_color = all_tags
                .iter()
                .find(|t| t.name == tag_name)
                .map(|t| t.color)
                .unwrap_or(Color32::GRAY);

            let text_color = text_color_for_background(tag_color, ui);
            let response = ui.add(
                egui::Button::new(
                    RichText::new(format!("#{tag_name} ❌"))
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
) -> (bool, bool, bool) {
    let mut search_changed = false;
    let mut tags_changed = false;
    let mut search_committed_flag = false;

    ui.vertical(|ui| {
        // Selected tags chips
        if !search_state.selected_tags.is_empty() {
            tags_changed = selected_tags_chips(ui, search_state, all_tags);
        } else {
            // When no tags, add minimal invisible spacer to maintain consistent layout
            ui.add_space(4.0);
        }

        // Search input
        ui.horizontal(|ui| {
            ui.label("🔍");

            // Use TextEdit builder with stable ID and desired width for more reliable cursor handling
            let response = ui.add(
                egui::TextEdit::singleline(&mut search_state.search_text)
                    .id(egui::Id::new("tag_search_input"))
                    .desired_width(200.0), // Fixed width for more stable layout
            );

            // Store the text edit widget rect for dropdown positioning fallback
            if response.rect.is_positive() {
                ui.ctx().data_mut(|d| {
                    d.insert_temp(egui::Id::new("last_search_widget"), response.rect);
                });
            }

            // Debug: Log when text changes
            if response.changed() {
                #[cfg(target_arch = "wasm32")]
                log::debug!(
                    "TAG SEARCH text changed: '{}' (len: {}) - in_tag_mode: {}",
                    search_state.search_text,
                    search_state.search_text.len(),
                    search_state.in_tag_mode
                );
            }

            // Check for Enter key - indicates search should be committed to URL
            let enter_pressed =
                response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if enter_pressed {
                #[cfg(target_arch = "wasm32")]
                log::debug!(
                    "Enter key pressed in search (search committed): '{}'",
                    search_state.search_text
                );
                // Optionally blur the input field
                response.surrender_focus();
                search_committed_flag = true;
            }

            // Handle tag mode - but buffer updates to avoid interfering with cursor
            // Process tag logic AFTER text input to prevent cursor corruption
            let current_text = search_state.search_text.clone();
            let was_in_tag_mode = search_state.in_tag_mode;

            // Check if we should enter tag mode
            if current_text.ends_with('#') && !was_in_tag_mode {
                search_state.in_tag_mode = true;
                search_state.tag_input.clear();
                #[cfg(target_arch = "wasm32")]
                log::debug!("Entered tag mode");
            } else if search_state.in_tag_mode {
                // Update tag input - but only if text actually contains tag
                if current_text.ends_with(' ') {
                    // Space ends tag mode
                    search_state.in_tag_mode = false;
                    if !search_state.tag_input.is_empty() {
                        search_state.add_tag(search_state.tag_input.clone());
                        tags_changed = true;
                        #[cfg(target_arch = "wasm32")]
                        log::debug!("Added tag from space: {}", search_state.tag_input);
                    }
                    search_state.tag_input.clear();
                } else if let Some(tag_part) = current_text.strip_prefix('#') {
                    let new_tag_input: String = tag_part.to_owned();

                    // Only update suggestions if tag input actually changed
                    if new_tag_input != search_state.tag_input {
                        search_state.tag_input = new_tag_input;

                        // Update suggestions
                        update_tag_suggestions(search_state, all_tags);

                        #[cfg(target_arch = "wasm32")]
                        if !search_state.tag_input.is_empty() {
                            log::debug!("Tag input updated: '{}'", search_state.tag_input);
                        }
                    }
                } else {
                    // Text doesn't start with # anymore - exit tag mode
                    search_state.in_tag_mode = false;
                    search_state.tag_input.clear();
                    #[cfg(target_arch = "wasm32")]
                    log::debug!("Exited tag mode (no # prefix)");
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
            // Get dropdown position with fallback to prevent NaN crashes
            let cursor_pos = get_dropdown_position(ui);

            egui::Area::new(egui::Id::new("tag_autocomplete_dropdown"))
                .order(egui::Order::Foreground)
                .fixed_pos(cursor_pos)
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
                                            // Clear the #tag_name from search text when tag is selected from dropdown
                                            // This provides better UX - user doesn't need to manually delete it
                                            if search_state.search_text.starts_with('#') {
                                                search_state.search_text.clear();
                                            }
                                            search_state.tag_input.clear();
                                            tags_changed = true;
                                            search_changed = true;
                                            #[cfg(target_arch = "wasm32")]
                                            log::debug!("Tag selected from dropdown: {}, cleared search text", tag.name);
                                        }
                                    }
                                });
                        });
                });
        }
    });

    (search_changed, tags_changed, search_committed_flag)
}

/// Update tag suggestions based on current input.
pub fn update_tag_suggestions(search_state: &mut TagSearchState, all_tags: &[Tag]) {
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
