//! Debug utilities for the blog app.

/// Debug state for the blog app.
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
    /// Show animation configuration window
    pub show_animation_config: bool,
    /// Animation configuration parameters
    pub animation_config: crate::animation::FocusAnimationConfig,
    /// Show simple search test window
    pub show_simple_search_test: bool,
    /// Simple search test state
    pub simple_search_test: crate::ui::simple_search_test::SimpleSearchTest,
    /// Show math resolution configuration window
    pub show_math_resolution_config: bool,
    /// Show text segment cache statistics window
    pub show_text_cache_stats: bool,
    /// Enable continuous rendering for smooth animations
    pub continuous_rendering: bool,
}

impl Default for DebugState {
    fn default() -> Self {
        Self {
            show_font_book: false,
            font_book_selected_family: egui::FontFamily::Proportional,
            font_book_filter: String::new(),
            show_frame_rate: false,
            frame_time_history: egui::util::History::new(2..100, 1.0), // Keep up to 1 second of history
            last_frame_time: None,
            show_animation_config: false,
            animation_config: crate::animation::FocusAnimationConfig::default(),
            show_simple_search_test: false,
            simple_search_test: crate::ui::simple_search_test::SimpleSearchTest::new(),
            show_math_resolution_config: false,
            show_text_cache_stats: false,
            continuous_rendering: false,
        }
    }
}

pub fn update_frame_rate(ctx: &egui::Context, frame: &eframe::Frame, debug_state: &mut DebugState) {
    let now = ctx.input(|i| i.time);

    if let Some(cpu_usage) = frame.info().cpu_usage {
        if let Some(latest) = debug_state.frame_time_history.latest_mut() {
            *latest = cpu_usage;
        }
        debug_state.frame_time_history.add(now, cpu_usage);
    } else if let Some(last_time) = debug_state.last_frame_time {
        let delta = (now - last_time) as f32;
        if delta > 0.0 {
            debug_state.frame_time_history.add(now, delta);
        }
        debug_state.last_frame_time = Some(now);
    } else {
        debug_state.last_frame_time = Some(now);
    }
}

pub fn get_average_frame_rate(debug_state: &DebugState) -> f32 {
    1.0 / debug_state
        .frame_time_history
        .mean_time_interval()
        .unwrap_or_default()
}

/// Frame rate window for monitoring performance.
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
                let color = if current_fps >= 15.0 {
                    ui.visuals().warn_fg_color // Use Catppuccin yellow for warning (≥15 FPS)
                } else {
                    ui.visuals().error_fg_color // Use Catppuccin red for error (<15 FPS)
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

            let performance_color = if current_fps >= 15.0 {
                ui.visuals().warn_fg_color // Use Catppuccin yellow for warning (≥15 FPS)
            } else {
                ui.visuals().error_fg_color // Use Catppuccin red for error (<15 FPS)
            };

            ui.colored_label(performance_color, performance_text);

            ui.separator();

            // Continuous rendering toggle
            ui.horizontal(|ui| {
                ui.label("Rendering mode:");
                let is_continuous = debug_state.continuous_rendering;
                ui.toggle_value(
                    &mut debug_state.continuous_rendering,
                    if is_continuous {
                        "Continuous (smooth)"
                    } else {
                        "Reactive (lazy)"
                    },
                );
            });

            let is_continuous = debug_state.continuous_rendering;
            ui.label(if is_continuous {
                "Continuous mode: Repaints every frame for smooth animations"
            } else {
                "Reactive mode: Only repaints on input (saves CPU)"
            });

            ui.separator();
            ui.label("Note: Based on recent frame times");
        });
}

/// Font book window for debugging font availability.
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
                            .is_some_and(|name| name.to_string().to_lowercase().contains(&filter_lower)) ||
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
                            .map_or_else(|| "Unknown".to_owned(), |s| s.to_string());

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
                                            .map_or_else(|| "Unknown".to_owned(), |n| n.to_string())
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

/// Show animation configuration window for tuning focus animation parameters
pub fn show_animation_config_window(ui: &egui::Ui, debug_state: &mut DebugState) {
    egui::Window::new("Focus Animation Configuration")
        .default_size([500.0, 400.0])
        .open(&mut debug_state.show_animation_config)
        .show(ui.ctx(), |ui| {
            ui.heading("Focus Animation Configuration");
            ui.separator();

            ui.label("Adjust flash parameters for panel focus visualization.");
            ui.label("Once optimal parameters are found, they will be compiled as defaults.");
            ui.separator();

            // Intensity control
            ui.horizontal(|ui| {
                ui.label("Intensity:");
                ui.add(
                    egui::Slider::new(&mut debug_state.animation_config.intensity, 0.1..=1.0)
                        .step_by(0.05)
                        .suffix("x"),
                );
                ui.label(format!("{:.2}", debug_state.animation_config.intensity));
            });
            ui.label("Overall strength of the flash effect");
            ui.separator();

            // Flash duration control
            ui.horizontal(|ui| {
                ui.label("Flash Duration:");
                ui.add(
                    egui::Slider::new(
                        &mut debug_state.animation_config.flash_duration_ms,
                        50..=300,
                    )
                    .step_by(10.0)
                    .suffix("ms"),
                );
                ui.label(format!(
                    "{} ms",
                    debug_state.animation_config.flash_duration_ms
                ));
            });
            ui.label("Duration of the flash when focus changes (shorter = less distracting)");
            ui.separator();

            // Border thickness control (need to check if this field exists)
            // Note: We need to check if border_thickness field exists in the config
            ui.horizontal(|ui| {
                ui.label("Border Thickness:");
                ui.add(
                    egui::Slider::new(
                        &mut debug_state.animation_config.border_thickness,
                        1.0..=6.0,
                    )
                    .step_by(0.5)
                    .suffix("px"),
                );
                ui.label(format!(
                    "{:.1} px",
                    debug_state.animation_config.border_thickness
                ));
            });
            ui.label("Thickness of the flash border");
            ui.separator();

            // Reset to defaults button
            if ui.button("Reset to Defaults").clicked() {
                debug_state.animation_config = crate::animation::FocusAnimationConfig::default();
            }

            ui.separator();
            ui.label("Flash Animation:");
            ui.label("• Quick blue flash when focus changes between panels");
            ui.label("• Uses Catppuccin blue color from current theme");
            ui.label("• Very short duration (100ms default) - just enough to see");
            ui.label("• No persistent tint - panel returns to normal after flash");
        });
}

/// Show simple search test window
pub fn show_simple_search_test_window(ui: &egui::Ui, debug_state: &mut DebugState) {
    egui::Window::new("Simple Search Test")
        .default_width(400.0)
        .default_height(300.0)
        .open(&mut debug_state.show_simple_search_test)
        .show(ui.ctx(), |ui| {
            ui.heading("Simple Search Bar Test");
            ui.label("This tests cursor positioning without tag functionality.");
            ui.separator();

            ui.label("Test 1: Basic text_edit_singleline()");
            let (changed1, _) = debug_state.simple_search_test.show(ui);
            if changed1 {
                ui.label("Text changed in basic version");
            }

            ui.separator();

            ui.label("Test 2: TextEdit with stable ID");
            let (changed2, _) = debug_state.simple_search_test.show_with_id(ui);
            if changed2 {
                ui.label("Text changed in ID version");
            }

            ui.separator();

            ui.label("Debug Info:");
            ui.label(format!(
                "Total changes: {}",
                debug_state.simple_search_test.change_count
            ));
            ui.label(format!(
                "Current text: '{}' (len: {})",
                debug_state.simple_search_test.search_text,
                debug_state.simple_search_test.search_text.len()
            ));

            ui.separator();

            ui.label("Instructions:");
            ui.label("1. Click in search bar");
            ui.label("2. Type text at normal speed");
            ui.label("3. Observe if cursor moves correctly");
            ui.label("4. Check browser console for debug logs");

            if ui.button("Reset Test").clicked() {
                debug_state.simple_search_test =
                    crate::ui::simple_search_test::SimpleSearchTest::new();
            }
        });
}

/// Show math resolution configuration window for controlling formula rendering quality
pub fn show_math_resolution_config_window(
    ui: &egui::Ui,
    debug_state: &mut DebugState,
    math_resolution_scale: &mut f32,
) {
    egui::Window::new("Math Resolution Configuration")
        .default_size([400.0, 250.0])
        .open(&mut debug_state.show_math_resolution_config)
        .show(ui.ctx(), |ui| {
            ui.heading("Math Formula Resolution Scaling");
            ui.separator();

            ui.label("Control the rasterization resolution of math formulas.");
            ui.label("Higher values produce crisper rendering but use more memory.");
            ui.separator();

            // Resolution scale control
            ui.horizontal(|ui| {
                ui.label("Resolution Scale:");
                ui.add(
                    egui::Slider::new(math_resolution_scale, 1.0..=25.0)
                        .step_by(0.1)
                        .suffix("x"),
                );
                ui.label(format!("{math_resolution_scale:.1}x"));
            });

            // Memory impact warning
            let memory_factor = *math_resolution_scale * *math_resolution_scale;
            ui.label(format!("Memory impact: {memory_factor:.1}x (scale²)"));

            if *math_resolution_scale > 5.0 {
                ui.colored_label(
                    ui.visuals().warn_fg_color,
                    "Warning: High resolution scales use significant memory",
                );
            }

            ui.separator();

            // Baseline information
            ui.label("Baseline Alignment:");
            ui.label("• Baseline is measured in SVG coordinates (points), not pixels");
            ui.label("• Baseline does NOT scale with resolution");
            ui.label("• Ensures proper vertical alignment with text");

            ui.separator();

            // Reset to default button
            if ui.button("Reset to Default (1.0x)").clicked() {
                *math_resolution_scale = 1.0;
            }

            ui.separator();
            ui.label("Note: Changes take effect on next formula render");
        });
}

/// Show text segment cache statistics window for monitoring performance
pub fn show_text_cache_stats_window(
    ui: &egui::Ui,
    debug_state: &mut DebugState,
    text_segment_cache: &crate::ui::text_cache::TextSegmentCache,
) {
    egui::Window::new("Text Segment Cache Statistics")
        .default_size([400.0, 300.0])
        .open(&mut debug_state.show_text_cache_stats)
        .show(ui.ctx(), |ui| {
            ui.heading("Text Segment Cache Performance");
            ui.separator();

            let stats = text_segment_cache.stats();

            // Basic cache info
            ui.label("Cache Status:");
            ui.indent("cache_indent", |ui| {
                ui.label(format!("Entries: {}/{}", stats.entries, stats.max_size));
                ui.label(format!("Hit rate: {:.1}%", stats.hit_rate));
                ui.label(format!("Hits: {}", stats.hits));
                ui.label(format!("Misses: {}", stats.misses));
                ui.label(format!("Inserts: {}", stats.inserts));
            });

            ui.separator();

            // Performance interpretation
            ui.label("Performance Analysis:");
            if stats.hits == 0 && stats.misses == 0 {
                ui.colored_label(ui.visuals().weak_text_color(), "No cache activity yet");
            } else if stats.hit_rate > 80.0 {
                ui.colored_label(
                    ui.visuals().strong_text_color(),
                    "✓ Excellent cache performance",
                );
                ui.label("Most text segments are being reused, reducing parsing overhead.");
            } else if stats.hit_rate > 50.0 {
                ui.colored_label(ui.visuals().text_color(), "✓ Good cache performance");
                ui.label("Cache is providing significant performance benefits.");
            } else if stats.hit_rate > 20.0 {
                ui.colored_label(ui.visuals().warn_fg_color, "⚠ Moderate cache performance");
                ui.label("Cache is helping but many segments are unique.");
            } else {
                ui.colored_label(ui.visuals().error_fg_color, "⚠ Low cache performance");
                ui.label("Most text segments are unique or cache is not being utilized.");
            }

            ui.separator();

            // Cache management
            ui.label("Cache Management:");
            ui.indent("management_indent", |ui| {
                if stats.entries >= stats.max_size {
                    ui.colored_label(
                        ui.visuals().warn_fg_color,
                        "Cache is at capacity (will clear on next insert)",
                    );
                } else {
                    let usage_percent = (stats.entries as f64 / stats.max_size as f64) * 100.0;
                    ui.label(format!("Usage: {:.1}% of capacity", usage_percent));
                }

                // Memory estimate (rough)
                let estimated_memory_kb = (stats.entries * 200) / 1024; // ~200 bytes per entry
                ui.label(format!("Estimated memory: ~{} KB", estimated_memory_kb));
            });

            ui.separator();

            // Explanation
            ui.collapsing("How This Cache Works", |ui| {
                ui.label("The text segment cache stores parsed markdown text segments to avoid:");
                ui.indent("explain_indent", |ui| {
                    ui.label("• O(n) math placeholder parsing every frame");
                    ui.label("• UTF-8 character iteration for each text segment");
                    ui.label("• Math manifest lookups for formula metadata");
                });

                ui.label("\nCache Key:");
                ui.indent("key_indent", |ui| {
                    ui.label("• Hash of text content");
                    ui.label("• Math resolution scale factor");
                });

                ui.label("\nTypical Performance:");
                ui.indent("perf_indent", |ui| {
                    ui.label("• First render: Parse everything (cache misses)");
                    ui.label("• Subsequent renders: 80-90% cache hits");
                    ui.label("• Scrolling/typing: Cache hits for static content");
                });
            });

            ui.separator();
            ui.label("Note: Cache is cleared when capacity (10,000 entries) is reached");
        });
}
