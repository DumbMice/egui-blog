//! A blog application built with egui.

#[cfg(target_arch = "wasm32")]
mod web;

mod posts;
mod ui;

use egui::{CentralPanel, Panel, ScrollArea};
use posts::PostManager;
use ui::{LayoutConfig, Theme};

/// The main app state.
pub struct BlogApp {
    /// Manages blog posts
    post_manager: PostManager,
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
        Self {
            post_manager: PostManager::default(),
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
        CentralPanel::default().show_inside(ui, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                (post_saved, editing_cancelled, navigation_index) = ui::layout::main_content(
                    ui,
                    &self.post_manager,
                    self.selected_post,
                    self.editing_new_post,
                    &mut self.new_post_title,
                    &mut self.new_post_content,
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

        // Bottom panel
        Panel::bottom("bottom_panel").show_inside(ui, |ui| {
            ui::layout::bottom_panel(ui);
        });
    }
}

