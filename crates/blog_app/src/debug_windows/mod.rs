//! Debug utilities for the blog app (only available in debug builds).

/// Debug state for the blog app (only available in debug builds).
#[cfg(debug_assertions)]
pub struct DebugState {
    /// Show font book window
    pub show_font_book: bool,
    /// Selected font family in font book
    pub font_book_selected_family: egui::FontFamily,
    /// Filter text for font book
    pub font_book_filter: String,
    /// Show frame rate window
    pub show_frame_rate: bool,
    /// Frame time history for FPS calculation (works on both native and WASM)
    pub frame_time_history: egui::util::History<f32>,
    /// Last frame time for delta calculation
    pub last_frame_time: Option<f64>,
}

#[cfg(debug_assertions)]
impl Default for DebugState {
    fn default() -> Self {
        Self {
            show_font_book: false,
            font_book_selected_family: egui::FontFamily::Proportional,
            font_book_filter: String::new(),
            show_frame_rate: false,
            frame_time_history: egui::util::History::new(2..100, 1.0), // Keep up to 1 second of history
            last_frame_time: None,
        }
    }
}

/// Update frame rate calculation.
#[cfg(debug_assertions)]
pub fn update_frame_rate(ctx: &egui::Context, debug_state: &mut DebugState) {
    let now = ctx.input(|i| i.time);

    if let Some(last_time) = debug_state.last_frame_time {
        let delta = (now - last_time) as f32;
        if delta > 0.0 {
            // Add frame time to history (in seconds)
            debug_state.frame_time_history.add(now, delta);
        }
    }

    debug_state.last_frame_time = Some(now);
}

/// Get average frame rate from history.
#[cfg(debug_assertions)]
pub fn get_average_frame_rate(debug_state: &DebugState) -> f32 {
    1.0 / debug_state
        .frame_time_history
        .mean_time_interval()
        .unwrap_or_default()
}

/// Frame rate window for monitoring performance.
#[cfg(debug_assertions)]
pub fn show_frame_rate_window(ui: &egui::Ui, debug_state: &mut DebugState) {
    // Calculate stats before opening window to avoid borrowing issues
    let avg_fps = get_average_frame_rate(debug_state);
    let current_frame_time = debug_state.frame_time_history.latest().unwrap_or(0.0);
    let current_fps = if current_frame_time > 0.0 {
        1.0 / current_frame_time
    } else {
        0.0
    };

    egui::Window::new("Frame Rate")
        .default_size([250.0, 150.0])
        .open(&mut debug_state.show_frame_rate)
        .show(ui.ctx(), |ui| {
            ui.heading("Frame Rate Monitor");
            ui.separator();

            // Current FPS with color coding
            ui.horizontal(|ui| {
                ui.label("Current:");
                let color = if current_fps >= 30.0 {
                    egui::Color32::GREEN
                } else if current_fps >= 15.0 {
                    egui::Color32::YELLOW
                } else {
                    egui::Color32::RED
                };
                ui.colored_label(color, format!("{current_fps:.1} FPS"));
            });

            // Average FPS
            ui.horizontal(|ui| {
                ui.label("Average:");
                ui.label(format!("{avg_fps:.1} FPS"));
            });

            // Frame time
            ui.horizontal(|ui| {
                ui.label("Frame time:");
                ui.label(format!("{:.1} ms", current_frame_time * 1000.0));
            });

            ui.separator();

            // Simple FPS indicator
            ui.label("Performance:");
            let performance_text = if current_fps >= 60.0 {
                "Excellent (≥ 60 FPS)"
            } else if current_fps >= 30.0 {
                "Good (≥ 30 FPS)"
            } else if current_fps >= 15.0 {
                "Fair (≥ 15 FPS)"
            } else {
                "Poor (< 15 FPS)"
            };

            let performance_color = if current_fps >= 30.0 {
                egui::Color32::GREEN
            } else if current_fps >= 15.0 {
                egui::Color32::YELLOW
            } else {
                egui::Color32::RED
            };

            ui.colored_label(performance_color, performance_text);

            ui.separator();
            ui.label("Note: Based on recent frame times");
        });
}

/// Font book window for debugging font availability.
#[cfg(debug_assertions)]
pub fn show_font_book_window(ui: &egui::Ui, debug_state: &mut DebugState) {
    // Simple font book implementation showing available characters
    egui::Window::new("Font Book")
        .default_size([900.0, 700.0])
        .open(&mut debug_state.show_font_book)
        .show(ui.ctx(), |ui| {
            ui.heading("Font Book - Available Characters");
            ui.separator();

            // Get available font families
            let fonts = ui.ctx().fonts(|f| f.definitions().clone());

            ui.horizontal(|ui| {
                ui.label("Select font family:");

                // Show font families in a dropdown - only default fonts
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", debug_state.font_book_selected_family))
                    .width(200.0)
                    .show_ui(ui, |ui| {
                        // Only show default font families
                        let default_families =
                            [egui::FontFamily::Proportional, egui::FontFamily::Monospace];

                        for family in &default_families {
                            ui.selectable_value(
                                &mut debug_state.font_book_selected_family,
                                family.clone(),
                                format!("{family:?}"),
                            );
                        }
                    });

                // Show character count
                let char_count = ui.fonts_mut(|fonts| {
                    fonts
                        .fonts
                        .font(&debug_state.font_book_selected_family)
                        .characters()
                        .len()
                });
                ui.label(format!("({char_count} characters)"));
            });

            ui.separator();

            // Get available characters for selected font
            let available_chars = ui.fonts_mut(|fonts| {
                let mut font = fonts.fonts.font(&debug_state.font_book_selected_family);
                font.characters()
                    .iter()
                    .filter(|(chr, _fonts)| {
                        // Filter out control characters and whitespace
                        !chr.is_whitespace() && !chr.is_ascii_control()
                    })
                    .map(|(chr, _)| *chr)
                    .collect::<Vec<char>>()
            });

            // Show filter input
            ui.horizontal(|ui| {
                ui.label("Filter:");
                ui.add(
                    egui::TextEdit::singleline(&mut debug_state.font_book_filter)
                        .desired_width(200.0),
                );
                if ui.button("Clear").clicked() {
                    debug_state.font_book_filter.clear();
                }
            });

            ui.separator();

            // Show characters in a compact grid
            let filtered_chars: Vec<char> = if debug_state.font_book_filter.is_empty() {
                available_chars.clone()
            } else {
                let filter_lower = debug_state.font_book_filter.to_lowercase();
                available_chars
                    .iter()
                    .filter(|&&ch| {
                        // Filter by character itself
                        ch.to_string().to_lowercase().contains(&filter_lower) ||
                        // Filter by Unicode name
                        unicode_names2::name(ch)
                            .map(|name| name.to_string().to_lowercase().contains(&filter_lower))
                            .unwrap_or(false) ||
                        // Filter by hex code
                        format!("{:04X}", ch as u32).to_lowercase().contains(&filter_lower)
                    })
                    .copied()
                    .collect()
            };

            ui.label(format!(
                "Showing {} of {} characters",
                filtered_chars.len(),
                available_chars.len()
            ));

            // Compact character grid
            egui::ScrollArea::vertical().show(ui, |ui| {
                let button_size = egui::Vec2::new(30.0, 30.0);

                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = egui::Vec2::new(2.0, 2.0);

                    for ch in filtered_chars {
                        // Create a compact button with the character
                        let button = egui::Button::new(
                            egui::RichText::new(ch.to_string())
                                .family(debug_state.font_book_selected_family.clone())
                                .size(16.0),
                        )
                        .frame(false)
                        .min_size(button_size);

                        // Get character info for tooltip
                        let char_name = unicode_names2::name(ch)
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "Unknown".to_owned());

                        let tooltip_text = format!(
                            "'{}' - U+{:04X}\n{}\nClick to copy",
                            ch, ch as u32, char_name
                        );

                        let response = ui.add(button).on_hover_text(tooltip_text);
                        if response.clicked() {
                            // Copy character to clipboard on click
                            ui.ctx().copy_text(ch.to_string());
                        }
                    }
                });
            });

            ui.separator();

            // Show common character categories for quick testing
            ui.collapsing("Common Character Categories", |ui| {
                let categories = [
                    (
                        "Arrows",
                        vec!['⬇', '⬆', '↓', '↑', '▼', '▲', '▽', '△', '←', '→', '↔', '↕'],
                    ),
                    (
                        "Math",
                        vec!['π', '∑', '∫', '√', '∞', '≠', '≈', '≤', '≥', '×', '÷', '±'],
                    ),
                    ("Currency", vec!['$', '€', '£', '¥', '¢', '₹', '₿']),
                    (
                        "Symbols",
                        vec!['©', '®', '™', '✓', '✗', '★', '☆', '❤', '♡', '☀', '☁', '☂'],
                    ),
                    (
                        "Box Drawing",
                        vec!['─', '│', '┌', '┐', '└', '┘', '├', '┤', '┬', '┴', '┼'],
                    ),
                ];

                for (category_name, chars) in &categories {
                    ui.collapsing(*category_name, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing = egui::Vec2::new(2.0, 2.0);

                            for ch in chars {
                                let button = egui::Button::new(
                                    egui::RichText::new(ch.to_string())
                                        .family(debug_state.font_book_selected_family.clone())
                                        .size(16.0),
                                )
                                .min_size(egui::Vec2::new(30.0, 30.0));

                                if ui.add(button).clicked() {
                                    ui.ctx().copy_text(ch.to_string());
                                }

                                if ui
                                    .add(
                                        egui::Button::new("").min_size(egui::Vec2::new(30.0, 30.0)),
                                    )
                                    .on_hover_text(format!(
                                        "{} (U+{:04X}): {}",
                                        ch,
                                        *ch as u32,
                                        unicode_names2::name(*ch)
                                            .map(|n| n.to_string())
                                            .unwrap_or_else(|| "Unknown".to_owned())
                                    ))
                                    .clicked()
                                {
                                    ui.ctx().copy_text(ch.to_string());
                                }
                            }
                        });
                    });
                }
            });

            // Show font family info
            ui.collapsing("Font Family Details", |ui| {
                if let Some(fonts_in_family) =
                    fonts.families.get(&debug_state.font_book_selected_family)
                {
                    ui.label(format!(
                        "Fonts in {:?} family:",
                        debug_state.font_book_selected_family
                    ));
                    for font in fonts_in_family {
                        ui.label(format!("  - {font}"));
                    }
                } else {
                    ui.label(format!(
                        "No fonts found in {:?} family (default font)",
                        debug_state.font_book_selected_family
                    ));
                }
            });

            ui.separator();
            ui.label("Note: Click any character to copy it to clipboard");
            ui.label("If you see squares (□), the character is not available in this font");
            ui.label("Black arrows ⬇ and ⬆ are used for sorting buttons in the blog");
        });
}
