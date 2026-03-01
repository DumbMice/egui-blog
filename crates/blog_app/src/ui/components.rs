//! Reusable UI components for the blog app.

use egui::{Context, Ui};

/// Theme configuration for the blog.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

impl Theme {
    /// Apply this theme to the egui context.
    pub fn apply(&self, ctx: &Context) {
        match self {
            Self::Light => ctx.set_visuals(egui::Visuals::light()),
            Self::Dark => ctx.set_visuals(egui::Visuals::dark()),
            Self::Auto => {
                // Auto theme based on system preference
                // For now, default to light
                ctx.set_visuals(egui::Visuals::light());
            }
        }
    }

    /// Get the name of the theme.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Dark => "Dark",
            Self::Auto => "Auto",
        }
    }
}

/// A theme toggle widget.
pub fn theme_toggle(ui: &mut Ui, current_theme: &mut Theme) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label("Theme:");

        if ui
            .button("🌙")
            .on_hover_text("Switch to dark theme")
            .clicked()
        {
            *current_theme = Theme::Dark;
            changed = true;
        }

        if ui
            .button("☀")
            .on_hover_text("Switch to light theme")
            .clicked()
        {
            *current_theme = Theme::Light;
            changed = true;
        }

        if ui.button("⚙").on_hover_text("Auto theme").clicked() {
            *current_theme = Theme::Auto;
            changed = true;
        }

        ui.label(format!("({})", current_theme.name()));
    });

    changed
}

/// Debug menu widget (only in debug builds).
#[cfg(debug_assertions)]
pub fn debug_menu(ui: &mut Ui, debug_state: &mut crate::debug_windows::DebugState) -> bool {
    let mut interacted = false;

    ui.horizontal(|ui| {
        ui.label("Debug:");

        // Debug dropdown menu
        egui::ComboBox::from_id_salt("debug_menu")
            .selected_text("🐛")
            .width(60.0)
            .show_ui(ui, |ui| {
                // Font book button - toggle display
                if ui.button("Toggle font book").clicked() {
                    debug_state.show_font_book = !debug_state.show_font_book;
                    interacted = true;
                }

                ui.separator();

                // Frame rate button - toggle display
                if ui.button("Toggle frame rate").clicked() {
                    debug_state.show_frame_rate = !debug_state.show_frame_rate;
                    interacted = true;
                }

                ui.separator();

                // Clear cache button
                if ui.button("Clear cache").clicked() {
                    log::debug!("Clear cache requested");
                    interacted = true;
                }
            });
    });

    interacted
}

/// A search bar widget.
pub fn search_bar(ui: &mut Ui, query: &mut String) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label("🔍");

        let response = ui.text_edit_singleline(query);
        if response.changed() {
            changed = true;
        }

        if !query.is_empty() && ui.button("❌").on_hover_text("Clear search").clicked() {
            query.clear();
            changed = true;
        }
    });

    changed
}

/// Display post metadata (date, tags).
pub fn post_metadata(ui: &mut Ui, date: &str, tags: &[String]) {
    ui.horizontal(|ui| {
        ui.label("📅");
        ui.label(date);

        if !tags.is_empty() {
            ui.add_space(8.0);
            ui.label("🏷");
            for tag in tags {
                ui.label(
                    egui::RichText::new(tag)
                        .small()
                        .color(ui.visuals().weak_text_color()),
                );
            }
        }
    });
}

/// A navigation bar for moving between posts.
pub fn post_navigation(ui: &mut Ui, current_index: usize, total_posts: usize) -> Option<usize> {
    let mut new_index = None;

    ui.horizontal(|ui| {
        ui.label("Navigation:");

        if ui
            .button("⏮ First")
            .on_hover_text("Go to first post")
            .clicked()
            && total_posts > 0
        {
            new_index = Some(0);
        }

        if ui.button("◀ Prev").on_hover_text("Previous post").clicked() && current_index > 0 {
            new_index = Some(current_index - 1);
        }

        ui.label(format!("{} of {}", current_index + 1, total_posts));

        if ui.button("▶ Next").on_hover_text("Next post").clicked()
            && current_index + 1 < total_posts
        {
            new_index = Some(current_index + 1);
        }

        if ui
            .button("⏭ Last")
            .on_hover_text("Go to last post")
            .clicked()
            && total_posts > 0
        {
            new_index = Some(total_posts - 1);
        }
    });

    new_index
}

/// Display a post preview in a list.
pub fn post_preview(ui: &mut Ui, post: &crate::posts::BlogPost, is_selected: bool) -> bool {
    let mut clicked = false;

    ui.vertical(|ui| {
        let response = ui.selectable_label(is_selected, &post.title);
        if response.clicked() {
            clicked = true;
        }

        ui.small(&post.date);
        // Tags are displayed separately in side_panel based on config
    });

    clicked
}

/// Display a loading spinner with message.
pub fn loading_spinner(ui: &mut Ui, message: &str) {
    ui.vertical_centered(|ui| {
        ui.spinner(); // egui's built-in spinner
        ui.add_space(8.0);
        ui.label(message);
    });
}

/// Display an error message with optional retry button.
pub fn error_message(
    ui: &mut Ui,
    title: &str,
    description: &str,
    details: Option<&str>,
    show_retry: bool,
) -> bool {
    let mut retry_clicked = false;

    ui.vertical(|ui| {
        // Error header with icon
        ui.horizontal(|ui| {
            ui.colored_label(ui.visuals().error_fg_color, "⚠");
            ui.heading(title);
        });

        // Error description
        ui.label(description);

        // Optional technical details (collapsible)
        if let Some(details) = details {
            ui.collapsing("Technical details", |ui| {
                ui.monospace(details);
            });
        }

        // Action buttons
        ui.horizontal(|ui| {
            if show_retry && ui.button("🔄 Retry").clicked() {
                retry_clicked = true;
            }

            if ui.button("📝 Create example post").clicked() {
                // Will be implemented later
            }
        });
    });

    retry_clicked
}

/// Display empty state message with create post button.
pub fn empty_state(ui: &mut Ui, is_error: bool) -> bool {
    let mut create_clicked = false;

    ui.vertical_centered(|ui| {
        if is_error {
            ui.colored_label(ui.visuals().error_fg_color, "⚠ Failed to load posts");
            ui.label("Could not load any blog posts.");
        } else {
            ui.heading("📝 No blog posts yet");
            ui.label("Create your first post to get started!");
        }

        ui.add_space(16.0);

        if ui.button("📝 Create your first post").clicked() {
            create_clicked = true;
        }

        ui.add_space(8.0);
        ui.small("Posts directory: crates/blog_app/posts/");
    });

    create_clicked
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading_spinner_function() {
        // Test that loading_spinner function exists and compiles
        // We can't easily test UI rendering in unit tests, but we can
        // verify the function signature is correct
        use egui::Ui;

        // Just verify the function exists and has the right signature
        // by calling it in a closure that would be used in real UI code
        let _closure = |ui: &mut Ui| {
            loading_spinner(ui, "Loading...");
        };

        // If we get here without compilation errors, the test passes
        assert!(true);
    }

    #[test]
    fn test_error_message_function() {
        // Test that error_message function exists
        use egui::Ui;

        // Create a closure that would use the function
        let _closure = |ui: &mut Ui| {
            error_message(ui, "Error", "Something went wrong", None, true);
        };

        // If we get here without compilation errors, the test passes
        assert!(true);
    }

    #[test]
    fn test_empty_state_function() {
        // Test that empty_state function exists
        use egui::Ui;

        // Create a closure that would use the function
        let _closure = |ui: &mut Ui| {
            // This should fail to compile until we implement the function
            empty_state(ui, false);
        };

        // If we get here without compilation errors, the test passes
        assert!(true);
    }
}
