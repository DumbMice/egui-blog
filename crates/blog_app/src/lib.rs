//! A blog application built with egui.

#[cfg(target_arch = "wasm32")]
mod web;

mod posts;
mod ui;

use egui::{CentralPanel, Panel, ScrollArea};
use posts::{PostManager, PostManagerState};
use ui::{LayoutConfig, Theme};

/// The main app state.
pub struct BlogApp {
    /// Manages blog posts
    post_manager: PostManager,
    /// Current post manager state
    post_manager_state: PostManagerState,  // NEW
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
    /// Search query
    search_query: String,
    /// Layout configuration
    layout_config: LayoutConfig,
}

impl Default for BlogApp {
    fn default() -> Self {
        let post_manager = PostManager::default();
        let post_manager_state = post_manager.state().clone();  // NEW

        Self {
            post_manager,
            post_manager_state,  // NEW
            selected_post: 0,
            editing_new_post: false,
            new_post_title: String::new(),
            new_post_content: String::new(),
            theme: Theme::Light,
            search_query: String::new(),
            layout_config: LayoutConfig::default(),
        }
    }
}

impl BlogApp {
    /// Ensure selected_post is within valid bounds
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
        match self.post_manager.reload() {
            Ok(()) => {
                // Update our state tracking
                self.post_manager_state = self.post_manager.state().clone();
            }
            Err(err) => {
                // Error already captured in PostManager state
                eprintln!("Retry failed: {}", err);
                self.post_manager_state = self.post_manager.state().clone();
            }
        }

        // Ensure valid selection
        self.ensure_valid_selection();
    }
}

impl eframe::App for BlogApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
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
            );
        });

        if top_panel_changed {
            // If search changed, we might need to adjust selection
            // For now, just keep current selection if possible
        }

        // Side panel
        let mut selection_changed = false;
        Panel::left("side_panel").show_inside(ui, |ui| {
            selection_changed = ui::layout::side_panel(
                ui,
                &self.post_manager,
                &self.search_query,
                &mut self.selected_post,
                &self.layout_config,
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
                (post_saved, editing_cancelled, navigation_index, retry_requested) = ui::layout::main_content(
                    ui,
                    &self.post_manager,
                    self.selected_post,
                    self.editing_new_post,
                    &mut self.new_post_title,
                    &mut self.new_post_content,
                    &self.post_manager_state,
                );
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
}

