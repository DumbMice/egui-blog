//! Reusable UI components for the blog app.

use egui::{Context, Ui};

/// Theme configuration for the blog.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    /// Cycle to the next theme.
    pub fn next(&self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Auto,
            Self::Auto => Self::Light,
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
            .button("☀ ")
            .on_hover_text("Switch to light theme")
            .clicked()
        {
            *current_theme = Theme::Light;
            changed = true;
        }

        if ui.button("⚙ ").on_hover_text("Auto theme").clicked() {
            *current_theme = Theme::Auto;
            changed = true;
        }

        ui.label(format!("({})", current_theme.name()));
    });

    changed
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
