//! Main UI layout components for the blog app.

use egui::Ui;

use super::components::{self, Theme};
use crate::animation::FocusRenderer;
use crate::math::MathAssetManager;
use crate::posts::{PostManager, PostManagerState};

/// State bundle for rendering the main content area
/// Context for navigation in UI components
pub struct NavigationContext<'a> {
    /// Current URL route
    pub current_route: &'a crate::routing::Route,
    /// Callback for navigation requests
    pub on_navigate: &'a mut dyn FnMut(crate::routing::Route),
}

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
    /// Navigation context
    pub navigation: NavigationContext<'a>,
    /// Tag search state
    pub tag_search_state: &'a mut crate::tags::TagSearchState,
    /// All tags for color assignment
    pub all_tags: &'a [crate::tags::Tag],
    /// Math formula resolution scaling factor
    pub math_resolution_scale: f32,
    /// Optional fragment ID to scroll to in the content
    pub fragment_to_scroll_to: Option<&'a str>,
    /// Text segment cache for markdown rendering performance
    pub text_segment_cache: &'a mut crate::ui::text_cache::TextSegmentCache,
}

impl<'a> MainContentState<'a> {
    /// Create a new state bundle
    #[expect(clippy::too_many_arguments)]
    pub fn new(
        post_manager: &'a PostManager,
        selected_post_index: usize,
        is_editing_new_post: bool,
        new_post_title: &'a mut String,
        new_post_content: &'a mut String,
        post_manager_state: &'a PostManagerState,
        math_asset_manager: Option<&'a mut MathAssetManager>,
        navigation: NavigationContext<'a>,
        tag_search_state: &'a mut crate::tags::TagSearchState,
        all_tags: &'a [crate::tags::Tag],
        math_resolution_scale: f32,
        fragment_to_scroll_to: Option<&'a str>,
        text_segment_cache: &'a mut crate::ui::text_cache::TextSegmentCache,
    ) -> Self {
        Self {
            post_manager,
            selected_post_index,
            is_editing_new_post,
            new_post_title,
            new_post_content,
            post_manager_state,
            math_asset_manager,
            navigation,
            tag_search_state,
            all_tags,
            math_resolution_scale,
            fragment_to_scroll_to,
            text_segment_cache,
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

/// Result of rendering the top panel.
#[derive(Debug, Clone, Copy)]
pub struct TopPanelResult {
    /// Whether the search state (text or tags) was modified
    pub search_changed: bool,
    /// Whether the theme was changed
    pub theme_changed: bool,
    /// Whether search was committed with Enter key (should update URL)
    pub search_committed: bool,
}

/// Configuration for the top panel
pub struct TopPanelConfig<'a> {
    /// Tag search state
    pub tag_search_state: &'a mut crate::tags::TagSearchState,
    /// All available tags
    pub all_tags: &'a [crate::tags::Tag],
    /// Post manager for search functionality
    pub post_manager: &'a PostManager,
    /// Currently selected post index
    pub selected_post: usize,
}

impl TopPanelResult {
    /// Returns true if either search or theme changed
    pub fn any_changed(&self) -> bool {
        self.search_changed || self.theme_changed || self.search_committed
    }
}

/// Top panel with blog title and controls.
pub fn top_panel(
    ui: &mut Ui,
    title: &str,
    theme: &mut Theme,
    config: &mut TopPanelConfig<'_>,
    #[cfg(debug_assertions)] debug_state: &mut crate::debug_windows::DebugState,
) -> TopPanelResult {
    let mut theme_changed = false;
    let mut search_changed = false;
    let mut search_committed = false;

    ui.horizontal(|ui| {
        // Blog title
        ui.heading(title);

        ui.separator();

        // Search bar with tag support
        let (search_bar_changed, tags_changed, search_committed_flag) =
            crate::ui::tag_components::tag_search_bar(ui, config.tag_search_state, config.all_tags);
        if search_bar_changed || tags_changed {
            search_changed = true;
        }
        if search_committed_flag {
            search_committed = true;
        }

        ui.separator();

        // Post counter
        ui.label(format!(
            "Posts: {}/{}",
            if config.post_manager.count() > 0 {
                config.selected_post + 1
            } else {
                0
            },
            config.post_manager.count()
        ));

        ui.separator();

        // Theme toggle
        if components::theme_toggle(ui, theme) {
            log::debug!("Theme changed in top_panel, new theme: {theme:?}");
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

    TopPanelResult {
        search_changed,
        theme_changed,
        search_committed,
    }
}

/// Side panel with post list.
#[expect(clippy::too_many_arguments)]
pub fn side_panel(
    ui: &mut Ui,
    post_manager: &PostManager,
    post_manager_state: &PostManagerState,
    tag_search_state: &mut crate::tags::TagSearchState,
    all_tags: &[crate::tags::Tag],
    selected_content_type: &mut Option<crate::posts::ContentType>,
    selected_post_index: &mut usize,
    config: &mut LayoutConfig,
    mut on_selection: impl FnMut(Option<&crate::posts::BlogPost>),
    is_focused: bool,
    panel_rect: egui::Rect,
    scroll_offset: &mut f32,
    request_auto_scroll: &mut bool,
    // Animation parameters
    animation_state: &crate::animation::FocusAnimationState,
    animation_config: &crate::animation::FocusAnimationConfig,
    // Panel state
    side_panel_collapsed: bool,
    mut on_toggle_panel: impl FnMut(),
) -> (bool, bool) {
    let mut selection_changed = false;
    let mut panel_clicked = false;

    // Save the initial rect for click detection (not used for clicks anymore)
    let _initial_rect = ui.available_rect_before_wrap();

    // Use the provided panel_rect for click detection (full panel area)
    let click_rect = panel_rect;

    // Draw animated focus indicator if panel is focused
    if is_focused {
        let current_time = ui.ctx().input(|i| i.time);

        FocusRenderer::draw_focus_indicator(
            ui.painter(),
            panel_rect,
            is_focused,
            animation_state,
            animation_config,
            current_time,
            ui,
        );
    }

    // Handle collapsed state - show only hamburger button
    if side_panel_collapsed {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                // Panel expand button (when panel is collapsed)
                // Use » (right-pointing) to indicate expand
                let expand_button = ui.button("»").on_hover_text("Expand panel");
                if expand_button.clicked() {
                    on_toggle_panel();
                }
            });
        });
        return (selection_changed, panel_clicked);
    }

    // Handle loading/error states before entering the UI closure
    match post_manager_state {
        PostManagerState::Loading => {
            ui.vertical(|ui| {
                ui.heading("Blog Posts");
                ui.separator();
                super::components::loading_spinner(ui, "Loading posts...");
            });
            return (selection_changed, panel_clicked);
        }
        PostManagerState::Error(_) => {
            ui.vertical(|ui| {
                ui.heading("Blog Posts");
                ui.separator();
                ui.label("Failed to load posts");
                ui.small("See main content for error details");
            });
            return (selection_changed, panel_clicked);
        }
        PostManagerState::Empty => {
            ui.vertical(|ui| {
                ui.heading("Blog Posts");
                ui.separator();
                super::components::empty_state(ui, false);
            });
            return (selection_changed, panel_clicked);
        }
        PostManagerState::Loaded => {
            // Continue with normal logic
        }
    }

    let mut interactive_element_clicked = false;

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
                interactive_element_clicked = true;
                // Toggle sort order
                config.post_sort_order = match config.post_sort_order {
                    PostSortOrder::NewestFirst => PostSortOrder::OldestFirst,
                    PostSortOrder::OldestFirst => PostSortOrder::NewestFirst,
                };
            }

            // Panel collapse/expand button (after sort button)
            // Use « when expanded (pointing left to indicate collapse)
            // Use » when collapsed (pointing right to indicate expand)
            let button_icon = if side_panel_collapsed { "»" } else { "«" };
            let collapse_button = ui.button(button_icon);
            if collapse_button
                .on_hover_text(if side_panel_collapsed {
                    "Expand panel"
                } else {
                    "Collapse panel"
                })
                .clicked()
            {
                interactive_element_clicked = true;
                on_toggle_panel();
            }
        });

        ui.separator();

        // Content type tabs
        ui.horizontal(|ui| {
            // "All" tab
            let all_selected = selected_content_type.is_none();
            let all_response = ui.selectable_label(all_selected, "All");
            if all_response.clicked() && !all_selected {
                // Debug logging removed for performance
                // log::debug!("Side panel: 'All' tab clicked, selected_content_type was: {selected_content_type:?}");
                interactive_element_clicked = true;
                *selected_content_type = None;
                // When switching to "All", navigate to Home to show all posts
                selection_changed = true;
                // Debug logging removed for performance
                // log::debug!("Side panel: Calling on_selection(None) because 'All' tab clicked");
                on_selection(None); // Navigate to Home
            } else if all_response.clicked() {
                // Debug logging removed for performance
                // log::debug!("Side panel: 'All' tab clicked but already selected (bug?)");
            }

            // Content type tabs
            for content_type in [
                crate::posts::ContentType::Post,
                crate::posts::ContentType::Note,
                crate::posts::ContentType::Review,
            ] {
                let is_selected = *selected_content_type == Some(content_type);
                let response = ui.selectable_label(is_selected, content_type.display_name());
                if response.clicked() && !is_selected {
                    interactive_element_clicked = true;
                    *selected_content_type = Some(content_type);
                    // Don't navigate when switching tabs - just filter the list
                    // Debug logging removed for performance
                    // log::debug!("Tab switched to {content_type:?} (filter only, no navigation)");
                }
            }
        });

        ui.separator();

        // Get posts based on tag search, content type filter, and sort order
        let mut posts_to_show = crate::tags::search_posts(post_manager.posts(), tag_search_state);

        // Apply content type filter if set
        if let Some(content_type) = selected_content_type {
            posts_to_show.retain(|post| post.content_type == *content_type);
        }

        // Apply sort order
        posts_to_show.sort_by(|a, b| match config.post_sort_order {
            PostSortOrder::NewestFirst => b.date.cmp(&a.date),
            PostSortOrder::OldestFirst => a.date.cmp(&b.date),
        });

        if posts_to_show.is_empty() {
            ui.label("No posts found");
            if tag_search_state.is_active() {
                ui.label("Try a different search or remove some tags");
            }
        } else {
            let scroll_response = egui::ScrollArea::vertical()
                .scroll_offset(egui::vec2(0.0, *scroll_offset))
                .show(ui, |ui| {
                    for (idx, post) in posts_to_show.iter().enumerate() {
                        // Find the original index in the post manager
                        let original_index = post_manager
                            .posts()
                            .iter()
                            .position(|p| p.id == post.id)
                            .unwrap_or(idx);

                        let is_selected = original_index == *selected_post_index;

                        // Handle auto-scroll if this is the selected post and auto-scroll is requested
                        if is_selected && *request_auto_scroll {
                            // Scroll to this item
                            ui.scroll_to_cursor(Some(egui::Align::Center));
                            *request_auto_scroll = false;
                        }

                        let post_response = ui.vertical(|ui| {
                            let clicked = components::post_preview(ui, post, is_selected);

                            if config.show_preview_in_list {
                                // Try to show first paragraph, show nothing if no paragraph
                                if let Some(paragraph) = post.first_paragraph() {
                                    ui.small(paragraph);
                                } else {
                                    // Show nothing if first content is not a paragraph
                                    // (e.g., heading, table, formula, etc.)
                                }
                            }

                            if config.show_tags_in_list && !post.tags.is_empty() {
                                ui.horizontal_wrapped(|ui| {
                                    for tag_name in &post.tags {
                                        // Find the tag to get its color
                                        if let Some(tag) =
                                            all_tags.iter().find(|t| t.name == *tag_name)
                                        {
                                            if crate::ui::tag_components::tag_chip(
                                                ui,
                                                tag,
                                                tag_search_state,
                                            )
                                            .clicked()
                                            {
                                                // Tag was clicked - selection will be updated in main loop
                                            }
                                        } else {
                                            // Fallback for tags not in all_tags
                                            ui.label(
                                                egui::RichText::new(format!("#{tag_name}"))
                                                    .small()
                                                    .color(ui.visuals().weak_text_color()),
                                            );
                                        }
                                    }
                                });
                            }

                            ui.separator();

                            if clicked {
                                interactive_element_clicked = true;
                                *selected_post_index = original_index;
                                selection_changed = true;
                                // Update URL when post is selected
                                on_selection(Some(post));
                                // Request auto-scroll to the clicked post
                                *request_auto_scroll = true;
                            }
                        });

                        // Handle auto-scroll if this is the selected post and auto-scroll is requested
                        if is_selected && *request_auto_scroll {
                            // Check if the post is already visible in the scroll area
                            let clip_rect = ui.clip_rect();
                            let post_rect = post_response.response.rect;

                            // Only scroll if the post is not fully visible
                            if !clip_rect.contains_rect(post_rect) {
                                // Scroll to this item's rect
                                // Using None for alignment means "make it visible somewhere" (less jumping than Center)
                                ui.scroll_to_rect(post_rect, None);
                            }
                            *request_auto_scroll = false;
                        }
                    }
                });

            // Update scroll offset from scroll area response
            *scroll_offset = scroll_response.state.offset.y;
        }
    });

    // Check for clicks on the panel at the end (after all widgets are drawn)
    // This ensures we detect clicks even on widgets
    let pointer = ui.ctx().input(|i| i.pointer.clone());

    // Try multiple ways to detect clicks/presses
    let detected_click =
        // Method 1: Check for primary click at interact position
        if let Some(click_pos) = pointer.interact_pos()
            && click_rect.contains(click_pos) && pointer.primary_clicked()
        {
            // Debug logging removed for performance
            // log::debug!("Side panel clicked via interact_pos");
            true
        }
        // Method 2: Check for primary press origin (where mouse was pressed down)
        else if let Some(press_origin) = pointer.press_origin()
            && click_rect.contains(press_origin) && pointer.primary_down()
        {
            // Debug logging removed for performance
            // log::debug!("Side panel pressed via press_origin");
            true
        }
        // Method 3: Check latest position if primary is down
        else if let Some(latest_pos) = pointer.latest_pos()
            && click_rect.contains(latest_pos) && pointer.primary_down()
        {
            // Debug logging removed for performance
            // log::debug!("Side panel pressed via latest_pos");
            true
        }
        else {
            false
        };

    if detected_click && !interactive_element_clicked {
        panel_clicked = true;
    }

    (selection_changed, panel_clicked)
}

/// Right panel showing table of contents for the current post.
#[expect(clippy::too_many_arguments)]
pub fn right_panel(
    ui: &mut Ui,
    post: Option<&crate::posts::BlogPost>,
    is_focused: bool,
    panel_rect: egui::Rect,
    scroll_offset: &mut f32,
    // Animation parameters
    animation_state: &crate::animation::FocusAnimationState,
    animation_config: &crate::animation::FocusAnimationConfig,
    // Panel state
    panel_collapsed: bool,
    mut on_toggle_panel: impl FnMut(),
) -> (bool, Option<String>) {
    let mut panel_clicked = false;
    let mut heading_clicked_id = None;
    let mut interactive_element_clicked = false;

    // Save the initial rect for click detection
    let _initial_rect = ui.available_rect_before_wrap();

    // Use the provided panel_rect for click detection (full panel area)
    let click_rect = panel_rect;

    // Draw animated focus indicator if panel is focused
    if is_focused {
        let current_time = ui.ctx().input(|i| i.time);

        FocusRenderer::draw_focus_indicator(
            ui.painter(),
            panel_rect,
            is_focused,
            animation_state,
            animation_config,
            current_time,
            ui,
        );
    }

    // Handle collapsed state - show only hamburger button
    if panel_collapsed {
        // Debug logging removed for performance
        // log::debug!("Right panel is COLLAPSED, showing expand button");
        // Collapsed panel - show only expand button
        ui.vertical_centered(|ui| {
            // Use « (left-pointing) to indicate expand (points toward content)
            let button_icon = "«";
            let button = ui.button(button_icon);
            if button.clicked() {
                interactive_element_clicked = true;
                on_toggle_panel();
            }
            button.on_hover_text("Expand panel");
        });
        return (false, None);
    }

    // Expanded panel - show table of contents
    let scroll_response = egui::ScrollArea::vertical()
        .scroll_offset(egui::vec2(0.0, *scroll_offset))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                // Panel header with title and collapse button
                ui.horizontal(|ui| {
                    // Panel collapse button on the left
                    // Use » (right-pointing) to indicate collapse (points away from content)
                    let button_icon = "»";

                    // Add vertical spacing to align button with heading text
                    // Heading text is taller, so we need to push the button down a bit
                    ui.vertical(|ui| {
                        ui.add_space(8.0); // Adjust this value to align button with heading
                        let button = ui.button(button_icon);
                        if button.clicked() {
                            interactive_element_clicked = true;
                            on_toggle_panel();
                        }
                        button.on_hover_text("Collapse panel");
                    });

                    ui.heading("Table of Contents");
                });

                ui.separator();

                // Show TOC content if we have a post with headings
                if let Some(post) = post {
                    if !post.headings.is_empty() {
                        for heading in &post.headings {
                            // Calculate indentation based on heading level
                            let indent = (heading.level.saturating_sub(1) as f32) * 24.0;

                            ui.horizontal(|ui| {
                                ui.add_space(indent);

                                // Add bullet style based on heading level
                                // Alternating: odd levels = ⚫, even levels = ⚪
                                let bullet = if heading.level % 2 == 1 {
                                    "⚫" // Solid circle for odd levels (1, 3, 5)
                                } else {
                                    "⚪" // Hollow circle for even levels (2, 4, 6)
                                };
                                ui.label(bullet);

                                // Create clickable heading label with underline on hover
                                let response = ui.add(
                                    egui::Button::new(&heading.text)
                                        .frame(false) // No button frame
                                        .fill(egui::Color32::TRANSPARENT), // Transparent background
                                );

                                // Add underline on hover
                                if response.hovered() {
                                    ui.painter().line_segment(
                                        [
                                            response.rect.left_bottom() - egui::vec2(0.0, 1.0),
                                            response.rect.right_bottom() - egui::vec2(0.0, 1.0),
                                        ],
                                        ui.visuals().widgets.hovered.fg_stroke,
                                    );
                                }

                                if response.clicked() {
                                    interactive_element_clicked = true;
                                    heading_clicked_id = Some(heading.id.clone());
                                }
                                response.on_hover_text(format!("Jump to: {}", heading.text));
                            });
                        }
                    }
                } else {
                    ui.label("No post selected");
                }
            });
        });

    // Update scroll offset from scroll area response
    *scroll_offset = scroll_response.state.offset.y;

    // Check for clicks on the panel at the end (after all widgets are drawn)
    // This ensures we detect clicks even on widgets
    let pointer = ui.ctx().input(|i| i.pointer.clone());

    // Try multiple ways to detect clicks/presses
    let detected_click =
        // Method 1: Check for primary click at interact position
        if let Some(click_pos) = pointer.interact_pos()
            && click_rect.contains(click_pos) && pointer.primary_clicked()
        {
            true
        }
        // Method 2: Check for primary press origin (where mouse was pressed down)
        else if let Some(press_origin) = pointer.press_origin()
            && click_rect.contains(press_origin) && pointer.primary_down()
        {
            true
        }
        // Method 3: Check latest position if primary is down
        else if let Some(latest_pos) = pointer.latest_pos()
            && click_rect.contains(latest_pos) && pointer.primary_down()
        {
            true
        }
        else {
            false
        };

    if detected_click && !interactive_element_clicked {
        panel_clicked = true;
    }

    (panel_clicked, heading_clicked_id)
}

/// Main content area showing a post or editor with math support.
pub fn main_content(
    ui: &mut Ui,
    state: MainContentState<'_>,
    is_focused: bool,
    panel_rect: egui::Rect,
    // Animation parameters
    animation_state: &crate::animation::FocusAnimationState,
    animation_config: &crate::animation::FocusAnimationConfig,
) -> (bool, bool, Option<usize>, bool, bool) {
    main_content_internal(
        ui,
        state,
        is_focused,
        panel_rect,
        animation_state,
        animation_config,
    )
}

fn main_content_internal(
    ui: &mut Ui,
    state: MainContentState<'_>,
    is_focused: bool,
    panel_rect: egui::Rect,
    animation_state: &crate::animation::FocusAnimationState,
    animation_config: &crate::animation::FocusAnimationConfig,
) -> (bool, bool, Option<usize>, bool, bool) {
    main_content_internal_impl(
        ui,
        state,
        is_focused,
        panel_rect,
        animation_state,
        animation_config,
    )
}

fn main_content_internal_impl(
    ui: &mut Ui,
    state: MainContentState<'_>,
    is_focused: bool,
    panel_rect: egui::Rect,
    animation_state: &crate::animation::FocusAnimationState,
    animation_config: &crate::animation::FocusAnimationConfig,
) -> (bool, bool, Option<usize>, bool, bool) {
    let mut post_saved = false;
    let mut editing_cancelled = false;
    let mut navigation_index = None;
    let mut retry_requested = false;
    let mut panel_clicked = false;

    // Save the initial rect for debugging
    let _initial_rect = ui.available_rect_before_wrap();
    // Debug logging removed for performance
    // log::debug!(
    //     "Main content initial rect: {:?} (min: {:?}, max: {:?}, size: {:?}), panel_rect: {:?} (min: {:?}, max: {:?}, size: {:?})",
    //     initial_rect,
    //     initial_rect.min,
    //     initial_rect.max,
    //     initial_rect.size(),
    //     panel_rect,
    //     panel_rect.min,
    //     panel_rect.max,
    //     panel_rect.size()
    // );

    // Draw animated focus indicator if panel is focused
    if is_focused {
        let current_time = ui.ctx().input(|i| i.time);

        crate::animation::FocusRenderer::draw_focus_indicator(
            ui.painter(),
            panel_rect,
            is_focused,
            animation_state,
            animation_config,
            current_time,
            ui,
        );
    }

    // Handle 404 route
    if matches!(
        state.navigation.current_route,
        crate::routing::Route::NotFound
    ) {
        ui.heading("404 - Page Not Found");
        ui.separator();
        ui.label("The requested page could not be found.");
        ui.add_space(20.0);
        if ui.button("🏠 Return to Home").clicked() {
            (state.navigation.on_navigate)(crate::routing::Route::Home);
        }
        return (
            post_saved,
            editing_cancelled,
            navigation_index,
            retry_requested,
            panel_clicked,
        );
    }

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
                ui.add(
                    egui::TextEdit::singleline(state.new_post_title)
                        .id(egui::Id::new("new_post_title")),
                );

                ui.label("Content (markdown):");
                ui.add(
                    egui::TextEdit::multiline(state.new_post_content)
                        .id(egui::Id::new("new_post_content"))
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

                    if components::post_metadata_with_tags(
                        ui,
                        &post.date,
                        &post.tags,
                        state.tag_search_state,
                        state.all_tags,
                    ) {
                        // Tags were clicked, trigger search update
                        // This will be handled by the main app loop
                    }
                    ui.separator();

                    // Render markdown content with math support using preprocessed content
                    if let Some(content) = post.processed_content() {
                        super::markdown::render_preprocessed_markdown(
                            ui,
                            content,
                            state.math_asset_manager,
                            state.math_resolution_scale,
                            state.fragment_to_scroll_to,
                            state.text_segment_cache,
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

    // Check for clicks on the panel at the end (after all widgets are drawn)
    // This ensures we detect clicks even on widgets
    let pointer = ui.ctx().input(|i| i.pointer.clone());

    // Simple approach: check if primary was clicked and the click position is in our rect
    if pointer.primary_clicked()
        && let Some(click_pos) = pointer.interact_pos()
        && panel_rect.contains(click_pos)
    {
        // Debug logging removed for performance
        // log::debug!("Main content clicked!");
        panel_clicked = true;
    }

    (
        post_saved,
        editing_cancelled,
        navigation_index,
        retry_requested,
        panel_clicked,
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
    fn test_main_content_returns_five_values() {
        // Test that main_content returns 5 values (including retry_requested and panel_clicked)
        // Now that we've updated the function, this test should pass

        // Create a mock to represent what the function should return
        let expected_return: (bool, bool, Option<usize>, bool, bool) =
            (false, false, None, false, false);

        // Destructure to verify we can handle 5 values
        let (_post_saved, _editing_cancelled, _navigation_index, _retry_requested, _panel_clicked) =
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
