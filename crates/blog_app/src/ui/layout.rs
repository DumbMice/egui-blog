//! Main UI layout components for the blog app.

use egui::Ui;

use crate::posts::PostManager;
use super::components::{self, Theme};

/// Configuration for the blog layout.
pub struct LayoutConfig {
    /// Show tags in post list
    pub show_tags_in_list: bool,
    /// Show post preview in list
    pub show_preview_in_list: bool,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            show_tags_in_list: true,
            show_preview_in_list: false,
        }
    }
}

/// Top panel with blog title and controls.
pub fn top_panel(
    ui: &mut Ui,
    title: &str,
    theme: &mut Theme,
    search_query: &mut String,
    post_manager: &PostManager,
    current_post_index: usize,
) -> bool {
    let mut theme_changed = false;
    let mut search_changed = false;

    ui.horizontal(|ui| {
        // Blog title
        ui.heading(title);

        ui.separator();

        // Search bar
        if components::search_bar(ui, search_query) {
            search_changed = true;
        }

        ui.separator();

        // Post counter
        ui.label(format!(
            "Posts: {}/{}",
            if post_manager.count() > 0 {
                current_post_index + 1
            } else {
                0
            },
            post_manager.count()
        ));

        ui.separator();

        // Theme toggle
        if components::theme_toggle(ui, theme) {
            theme_changed = true;
        }
    });

    theme_changed || search_changed
}

/// Side panel with post list.
pub fn side_panel(
    ui: &mut Ui,
    post_manager: &PostManager,
    search_query: &str,
    selected_post_index: &mut usize,
    config: &LayoutConfig,
) -> bool {
    let mut selection_changed = false;

    ui.vertical(|ui| {
        ui.heading("Blog Posts");
        ui.separator();

        let posts_to_show = post_manager.search(search_query);

        if posts_to_show.is_empty() {
            ui.label("No posts found");
            if !search_query.is_empty() {
                ui.label("Try a different search term");
            }
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (idx, post) in posts_to_show.iter().enumerate() {
                    // Find the original index in the post manager
                    let original_index = post_manager
                        .posts()
                        .iter()
                        .position(|p| p.id == post.id)
                        .unwrap_or(idx);

                    let is_selected = original_index == *selected_post_index;

                    ui.vertical(|ui| {
                        let clicked = components::post_preview(ui, post, is_selected);

                        if config.show_preview_in_list {
                            ui.small(post.preview());
                        }

                        if config.show_tags_in_list && !post.tags.is_empty() {
                            ui.horizontal_wrapped(|ui| {
                                for tag in &post.tags {
                                    ui.label(
                                        egui::RichText::new(format!("#{}", tag))
                                            .small()
                                            .color(ui.visuals().weak_text_color()),
                                    );
                                }
                            });
                        }

                        ui.separator();

                        if clicked {
                            *selected_post_index = original_index;
                            selection_changed = true;
                        }
                    });
                }
            });
        }
    });

    selection_changed
}

/// Main content area showing a post or editor.
pub fn main_content(
    ui: &mut Ui,
    post_manager: &PostManager,
    selected_post_index: usize,
    is_editing_new_post: bool,
    new_post_title: &mut String,
    new_post_content: &mut String,
) -> (bool, bool, Option<usize>) {
    let mut post_saved = false;
    let mut editing_cancelled = false;
    let mut navigation_index = None;

    if is_editing_new_post {
        // New post editor
        ui.heading("Create New Post");
        ui.separator();

        ui.label("Title:");
        ui.text_edit_singleline(new_post_title);

        ui.label("Content (markdown):");
        ui.add(
            egui::TextEdit::multiline(new_post_content)
                .desired_rows(20)
                .desired_width(f32::INFINITY),
        );

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("💾 Save").clicked() && !new_post_title.trim().is_empty() {
                post_saved = true;
            }

            if ui.button("❌ Cancel").clicked() {
                editing_cancelled = true;
            }
        });
    } else if let Some(post) = post_manager.get(selected_post_index) {
        // Display existing post
        ui.vertical(|ui| {
            ui.heading(&post.title);
            ui.separator();

            components::post_metadata(ui, &post.date, &post.tags);
            ui.separator();

            // Render markdown content
            super::render_markdown(ui, &post.content);

            ui.separator();

            // Navigation buttons
            if let Some(new_index) = components::post_navigation(
                ui,
                selected_post_index,
                post_manager.count(),
            ) {
                navigation_index = Some(new_index);
            }
        });
    } else {
        // No posts
        ui.vertical_centered(|ui| {
            ui.heading("No posts found");
            ui.label("Create your first post to get started!");
        });
    }

    (post_saved, editing_cancelled, navigation_index)
}

/// Bottom panel with status information.
pub fn bottom_panel(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label("Powered by egui");
            ui.hyperlink_to("(source)", "https://github.com/emilk/egui");
        });
    });
}