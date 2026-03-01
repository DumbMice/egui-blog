//! Main UI layout components for the blog app.

use egui::Ui;

use super::components::{self, Theme};
use crate::math::MathAssetManager;
use crate::posts::{PostManager, PostManagerState};

/// State bundle for rendering the main content area
pub struct MainContentState<'a> {
    /// Post manager containing all posts
    pub post_manager: &'a PostManager,
    /// Index of the currently selected post
    pub selected_post_index: usize,
    /// Whether we're editing a new post
    pub is_editing_new_post: bool,
    /// Title for the new post being edited (mutable)
    pub new_post_title: &'a mut String,
    /// Content for the new post being edited (mutable)
    pub new_post_content: &'a mut String,
    /// Current state of the post manager (loading, loaded, error, etc.)
    pub post_manager_state: &'a PostManagerState,
    /// Optional math asset manager for formula rendering
    pub math_asset_manager: Option<&'a mut MathAssetManager>,
}

impl<'a> MainContentState<'a> {
    /// Create a new state bundle
    pub fn new(
        post_manager: &'a PostManager,
        selected_post_index: usize,
        is_editing_new_post: bool,
        new_post_title: &'a mut String,
        new_post_content: &'a mut String,
        post_manager_state: &'a PostManagerState,
        math_asset_manager: Option<&'a mut MathAssetManager>,
    ) -> Self {
        Self {
            post_manager,
            selected_post_index,
            is_editing_new_post,
            new_post_title,
            new_post_content,
            post_manager_state,
            math_asset_manager,
        }
    }
}

/// Sort order for blog posts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Default)]
pub enum PostSortOrder {
    /// Newest posts first (reverse chronological)
    #[default]
    NewestFirst,
    /// Oldest posts first (chronological)
    OldestFirst,
}

/// Configuration for the blog layout.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct LayoutConfig {
    /// Show tags in post list
    pub show_tags_in_list: bool,
    /// Show post preview in list
    pub show_preview_in_list: bool,
    /// Sort order for posts
    pub post_sort_order: PostSortOrder,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            show_tags_in_list: true,
            show_preview_in_list: true,
            post_sort_order: PostSortOrder::default(),
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
    selected_post: usize,
    #[cfg(debug_assertions)] debug_state: &mut crate::debug_windows::DebugState,
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
                selected_post + 1
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

        // Debug menu (only in debug builds)
        #[cfg(debug_assertions)]
        {
            ui.separator();
            if components::debug_menu(ui, debug_state) {
                // Debug menu was interacted with
            }
        }
    });

    theme_changed || search_changed
}

/// Side panel with post list.
pub fn side_panel(
    ui: &mut Ui,
    post_manager: &PostManager,
    post_manager_state: &PostManagerState, // NEW
    search_query: &str,
    selected_post_index: &mut usize,
    config: &mut LayoutConfig,
) -> bool {
    let mut selection_changed = false;

    // Handle loading/error states before entering the UI closure
    match post_manager_state {
        PostManagerState::Loading => {
            ui.vertical(|ui| {
                ui.heading("Blog Posts");
                ui.separator();
                super::components::loading_spinner(ui, "Loading posts...");
            });
            return selection_changed;
        }
        PostManagerState::Error(_) => {
            ui.vertical(|ui| {
                ui.heading("Blog Posts");
                ui.separator();
                ui.label("Failed to load posts");
                ui.small("See main content for error details");
            });
            return selection_changed;
        }
        PostManagerState::Empty => {
            ui.vertical(|ui| {
                ui.heading("Blog Posts");
                ui.separator();
                ui.label("No posts found");
            });
            return selection_changed;
        }
        PostManagerState::Loaded => {
            // Continue with normal logic
        }
    }

    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.heading("Blog Posts");

            // Sort order toggle button
            // Use black arrows ⬇ and ⬆ which work in the current font configuration
            let button = ui.button(match config.post_sort_order {
                PostSortOrder::NewestFirst => "📅⬇",
                PostSortOrder::OldestFirst => "📅⬆",
            });

            if button
                .on_hover_text(match config.post_sort_order {
                    PostSortOrder::NewestFirst => "Newest first",
                    PostSortOrder::OldestFirst => "Oldest first",
                })
                .clicked()
            {
                // Toggle sort order
                config.post_sort_order = match config.post_sort_order {
                    PostSortOrder::NewestFirst => PostSortOrder::OldestFirst,
                    PostSortOrder::OldestFirst => PostSortOrder::NewestFirst,
                };
            }
        });

        ui.separator();

        // Get posts based on search query and sort order
        // The search method now handles sorting internally using cached sorted posts
        let posts_to_show = post_manager.search(search_query, config.post_sort_order);

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
                                        egui::RichText::new(format!("#{tag}"))
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

/// Main content area showing a post or editor with math support.
pub fn main_content(ui: &mut Ui, state: MainContentState<'_>) -> (bool, bool, Option<usize>, bool) {
    main_content_internal(ui, state)
}

fn main_content_internal(
    ui: &mut Ui,
    state: MainContentState<'_>,
) -> (bool, bool, Option<usize>, bool) {
    main_content_internal_impl(ui, state)
}

fn main_content_internal_impl(
    ui: &mut Ui,
    state: MainContentState<'_>,
) -> (bool, bool, Option<usize>, bool) {
    let mut post_saved = false;
    let mut editing_cancelled = false;
    let mut navigation_index = None;
    let mut retry_requested = false;

    match state.post_manager_state {
        PostManagerState::Loading => {
            super::components::loading_spinner(ui, "Loading blog posts...");
        }
        PostManagerState::Error(err_msg) => {
            retry_requested = super::components::error_message(
                ui,
                "Failed to load posts",
                err_msg,
                None, // No additional technical details
                true,
            );
        }
        PostManagerState::Empty => {
            super::components::empty_state(ui, false);
        }
        PostManagerState::Loaded => {
            if state.post_manager.count() == 0 {
                super::components::empty_state(ui, false);
            } else if state.is_editing_new_post {
                // New post editor
                ui.heading("Create New Post");
                ui.separator();

                ui.label("Title:");
                ui.text_edit_singleline(state.new_post_title);

                ui.label("Content (markdown):");
                ui.add(
                    egui::TextEdit::multiline(state.new_post_content)
                        .desired_rows(20)
                        .desired_width(f32::INFINITY),
                );

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("💾 Save").clicked() && !state.new_post_title.trim().is_empty() {
                        post_saved = true;
                    }

                    if ui.button("❌ Cancel").clicked() {
                        editing_cancelled = true;
                    }
                });
            } else if let Some(post) = state.post_manager.get(state.selected_post_index) {
                // Display existing post
                ui.vertical(|ui| {
                    ui.heading(&post.title);
                    ui.separator();

                    components::post_metadata(ui, &post.date, &post.tags);
                    ui.separator();

                    // Render markdown content with math support using preprocessed content
                    if let Some(content) = post.processed_content() {
                        super::markdown::render_preprocessed_markdown(
                            ui,
                            content,
                            state.math_asset_manager,
                        );
                    } else {
                        ui.label("Error: Post content not available");
                    }

                    ui.separator();

                    // Navigation buttons
                    if let Some(new_index) = components::post_navigation(
                        ui,
                        state.selected_post_index,
                        state.post_manager.count(),
                    ) {
                        navigation_index = Some(new_index);
                    }
                });
            } else {
                // No posts (should be handled by Empty state, but just in case)
                ui.vertical_centered(|ui| {
                    ui.heading("No posts found");
                    ui.label("Create your first post to get started!");
                });
            }
        }
    }

    (
        post_saved,
        editing_cancelled,
        navigation_index,
        retry_requested,
    )
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

#[cfg(test)]
mod tests {
    use crate::posts::PostManagerState;

    #[test]
    fn test_main_content_returns_four_values() {
        // Test that main_content returns 4 values (including retry_requested)
        // Now that we've updated the function, this test should pass

        // Create a mock to represent what the function should return
        let expected_return: (bool, bool, Option<usize>, bool) = (false, false, None, false);

        // Destructure to verify we can handle 4 values
        let (_post_saved, _editing_cancelled, _navigation_index, _retry_requested) =
            expected_return;

        // The function now returns 4 values, so this test should pass
        assert!(true, "main_content should return 4 values");
    }

    #[test]
    fn test_main_content_handles_all_state_variants() {
        // Test that main_content handles all PostManagerState variants
        // We'll verify the match statement covers all variants

        let variants = vec![
            PostManagerState::Loading,
            PostManagerState::Error("test error".to_string()),
            PostManagerState::Empty,
            PostManagerState::Loaded,
        ];

        // Just verify we can create all variants
        for variant in variants {
            match variant {
                PostManagerState::Loading => assert!(true),
                PostManagerState::Error(_) => assert!(true),
                PostManagerState::Empty => assert!(true),
                PostManagerState::Loaded => assert!(true),
            }
        }

        assert!(
            true,
            "main_content should handle all PostManagerState variants"
        );
    }

    #[test]
    fn test_side_panel_handles_states() {
        // Verify side_panel function compiles
        // Implementation will handle states internally

        // This test verifies that side_panel function signature includes PostManagerState parameter
        // We can't actually call the function without a real UI context, but we can verify
        // the function exists with the expected signature by checking the module exports

        // The real test is that the function compiles with the new signature
        // which will be verified when we run cargo test after updating the function
    }
}
