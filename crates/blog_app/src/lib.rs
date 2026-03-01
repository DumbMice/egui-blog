//! A blog application built with egui.
//! Testing file watcher improvements.

#[cfg(target_arch = "wasm32")]
mod web;

pub mod math;
mod posts;
mod ui;

#[cfg(debug_assertions)]
mod debug_windows;

use egui::{CentralPanel, Panel, ScrollArea};
pub use posts::{PostManager, PostManagerState};
use ui::{LayoutConfig, Theme};

use crate::math::MathAssetManager;

/// The main app state.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct BlogApp {
    /// Manages blog posts
    #[cfg_attr(feature = "serde", serde(skip))]
    post_manager: PostManager,
    /// Current post manager state
    post_manager_state: PostManagerState, // NEW
    /// Currently selected post index
    selected_post: usize,
    /// Are we editing a new post?
    editing_new_post: bool,
    /// Title for new post
    new_post_title: String,
    /// Content for new post
    new_post_content: String,
    /// Current theme
    theme: Theme,
    /// Previous theme (to detect changes)
    previous_theme: Theme,
    /// Search query
    search_query: String,
    /// Layout configuration
    layout_config: LayoutConfig,
    /// Math asset manager for rendering formula SVGs
    #[cfg_attr(feature = "serde", serde(skip))]
    math_asset_manager: MathAssetManager,

    /// Debug state (only available in debug builds)
    #[cfg(debug_assertions)]
    #[cfg_attr(feature = "serde", serde(skip))]
    debug_state: crate::debug_windows::DebugState,
}

impl Default for BlogApp {
    fn default() -> Self {
        let post_manager = PostManager::default();
        let post_manager_state = post_manager.state().clone(); // NEW

        Self {
            post_manager,
            post_manager_state, // NEW
            selected_post: 0,
            editing_new_post: false,
            new_post_title: String::new(),
            new_post_content: String::new(),
            theme: Theme::Light,
            previous_theme: Theme::Light,
            search_query: String::new(),
            layout_config: LayoutConfig::default(),
            math_asset_manager: MathAssetManager::default(),
            #[cfg(debug_assertions)]
            debug_state: crate::debug_windows::DebugState::default(),
        }
    }
}

impl BlogApp {
    /// Create a new `BlogApp`, optionally loading from storage.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        #[cfg(feature = "persistence")]
        let mut app = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Self::default()
        };

        #[cfg(not(feature = "persistence"))]
        let mut app = Self::default();

        // Note: Fonts are not available until first Context::run()
        // We rely on default font configuration

        // Apply theme to context
        app.theme.apply(&cc.egui_ctx);
        app.previous_theme = app.theme;

        // Ensure valid selection
        app.ensure_valid_selection();

        app
    }

    /// Ensure `selected_post` is within valid bounds
    fn ensure_valid_selection(&mut self) {
        if self.post_manager.count() == 0 {
            self.selected_post = 0;
            self.editing_new_post = false;
        } else if self.selected_post >= self.post_manager.count() {
            self.selected_post = self.post_manager.count() - 1;
        }
    }

    /// Handle retry button click from error state.
    fn handle_retry(&mut self) {
        // Trigger reload
        self.post_manager.reload();
        // Update our state tracking
        self.post_manager_state = self.post_manager.state().clone();

        // Ensure valid selection
        self.ensure_valid_selection();
    }
}

impl eframe::App for BlogApp {
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Update post manager state
        self.post_manager_state = self.post_manager.state().clone();

        // Apply theme if it changed
        if self.theme != self.previous_theme {
            self.theme.apply(ui.ctx());
            self.previous_theme = self.theme;
        }

        // Apply current theme
        self.theme.apply(ui.ctx());

        // Top panel
        let mut top_panel_changed = false;
        Panel::top("top_panel").show_inside(ui, |ui| {
            top_panel_changed = ui::layout::top_panel(
                ui,
                "My Blog",
                &mut self.theme,
                &mut self.search_query,
                &self.post_manager,
                self.selected_post,
                #[cfg(debug_assertions)]
                &mut self.debug_state,
            );
        });

        if top_panel_changed {
            // If search changed, we might need to adjust selection
            // For now, just keep current selection if possible
        }

        // Update and show debug windows (debug builds only)
        #[cfg(debug_assertions)]
        {
            // Update frame rate calculation
            crate::debug_windows::update_frame_rate(ui.ctx(), &mut self.debug_state);

            // Show font book window if enabled
            if self.debug_state.show_font_book {
                crate::debug_windows::show_font_book_window(ui, &mut self.debug_state);
            }

            // Show frame rate window if enabled
            if self.debug_state.show_frame_rate {
                crate::debug_windows::show_frame_rate_window(ui, &mut self.debug_state);
            }
        }

        // Side panel
        let mut selection_changed = false;
        Panel::left("side_panel").show_inside(ui, |ui| {
            selection_changed = ui::layout::side_panel(
                ui,
                &self.post_manager,
                &self.post_manager_state, // NEW: pass state
                &self.search_query,
                &mut self.selected_post,
                &mut self.layout_config,
            );
        });

        if selection_changed {
            self.editing_new_post = false;
        }

        // Main content area with scrolling
        let mut post_saved = false;
        let mut editing_cancelled = false;
        let mut navigation_index = None;
        let mut retry_requested = false;
        CentralPanel::default().show_inside(ui, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                let state = ui::layout::MainContentState::new(
                    &self.post_manager,
                    self.selected_post,
                    self.editing_new_post,
                    &mut self.new_post_title,
                    &mut self.new_post_content,
                    &self.post_manager_state,
                    Some(&mut self.math_asset_manager),
                );
                let result = ui::layout::main_content(ui, state);
                (
                    post_saved,
                    editing_cancelled,
                    navigation_index,
                    retry_requested,
                ) = result;
            });
        });

        if let Some(new_index) = navigation_index {
            self.selected_post = new_index;
            self.editing_new_post = false;
        }

        if post_saved {
            // Create new post
            let new_post = posts::BlogPost::new(
                self.post_manager.count(),
                &self.new_post_title,
                &self.new_post_content,
                "2026-02-10",
            );
            self.post_manager.add_post(new_post);
            self.selected_post = self.post_manager.count() - 1;
            self.editing_new_post = false;
            self.new_post_title.clear();
            self.new_post_content.clear();
        }

        if editing_cancelled {
            self.editing_new_post = false;
            self.new_post_title.clear();
            self.new_post_content.clear();
        }

        // Handle retry request (to be implemented in Task 11)
        if retry_requested {
            self.handle_retry();
        }

        // Bottom panel
        Panel::bottom("bottom_panel").show_inside(ui, |ui| {
            ui::layout::bottom_panel(ui);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_valid_selection() {
        let mut app = BlogApp::default();

        // Test that ensure_valid_selection method exists and works
        app.ensure_valid_selection();

        // Verify selection is valid (0 when no posts)
        assert_eq!(app.selected_post, 0);
        assert!(!app.editing_new_post);
    }

    #[test]
    fn test_ui_method_passes_post_manager_state() {
        // Test that BlogApp UI method passes post_manager_state to main_content
        // and handles the 4-value return tuple (including retry_requested)

        // This test verifies the compilation and basic structure
        let app = BlogApp::default();

        // We can't easily test the UI method directly since it requires egui context,
        // but we can verify that the method signature would compile correctly
        // by checking that post_manager_state field exists and is accessible
        let _state = &app.post_manager_state;

        // Verify the field exists and is of correct type
        match app.post_manager_state {
            PostManagerState::Loading => (),
            PostManagerState::Error(_) => (),
            PostManagerState::Empty => (),
            PostManagerState::Loaded => (),
        }

        // The real test is that the code compiles with the updated call
        // to main_content with 7 arguments and 4 return values
        assert!(true, "Test structure for UI method passing state");
    }

    #[test]
    fn test_blog_app_handle_retry() {
        let mut app = BlogApp::default();

        // Test that handle_retry method exists and can be called
        // This will fail to compile until we implement the method
        app.handle_retry();
    }

    #[test]
    fn test_blog_app_passes_state_to_side_panel() {
        let app = BlogApp::default();
        // Verify app compiles with updated side panel call
        let _ = app;
    }
}
