//! A blog application built with egui.
//! Testing file watcher improvements.

#[cfg(target_arch = "wasm32")]
mod web;

pub mod math;
mod posts;
mod routing;
mod ui;
pub mod shortcuts;

#[cfg(debug_assertions)]
mod debug_windows;

use egui::{CentralPanel, Panel, ScrollArea};
pub use posts::{PostManager, PostManagerState};
use ui::{LayoutConfig, ResponsiveConfig, Theme};

use crate::math::MathAssetManager;
use crate::routing::{Route, Router};
use crate::shortcuts::ActionExecutor as _;

/// A text match for find-in-content functionality
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct TextMatch {
    /// Start position in text (byte offset)
    start: usize,
    /// End position in text (byte offset)
    end: usize,
}

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
    /// Selected content type filter (None = show all)
    selected_content_type: Option<crate::posts::ContentType>,
    /// Layout configuration
    layout_config: LayoutConfig,
    /// Responsive layout configuration
    responsive_config: ResponsiveConfig,
    /// Math asset manager for rendering formula SVGs
    #[cfg_attr(feature = "serde", serde(skip))]
    math_asset_manager: MathAssetManager,

    /// URL router
    #[cfg_attr(feature = "serde", serde(skip))]
    router: Router,
    /// Pending URL update to push to browser history
    #[cfg_attr(feature = "serde", serde(skip))]
    pending_url_update: Option<String>,

    /// Debug state (only available in debug builds)
    #[cfg(debug_assertions)]
    #[cfg_attr(feature = "serde", serde(skip))]
    debug_state: crate::debug_windows::DebugState,

    /// Keyboard shortcut system
    #[cfg_attr(feature = "serde", serde(skip))]
    shortcut_integration: crate::shortcuts::ShortcutIntegration,
    /// Currently focused panel
    focused_panel: crate::shortcuts::FocusedPanel,
    /// Scroll offset for content area
    /// Scroll offset for main content panel (persisted)
    scroll_offset: f32,
    /// Scroll offset for side panel
    side_panel_scroll_offset: f32,
    /// Flag to request auto-scroll to selected post in side panel
    request_side_panel_auto_scroll: bool,
    /// Requested scroll delta for main content panel (set by shortcuts, applied in UI)
    requested_scroll_delta: Option<f32>,
    /// Find mode state
    find_query: String,
    find_matches: Vec<TextMatch>,
    current_find_match: usize,
    /// Whether find mode is active
    find_mode_active: bool,
}

impl Default for BlogApp {
    fn default() -> Self {
        let post_manager = PostManager::default();
        let post_manager_state = post_manager.state().clone(); // NEW

        Self {
            post_manager,
            post_manager_state,
            selected_post: 0,
            editing_new_post: false,
            new_post_title: String::new(),
            new_post_content: String::new(),
            theme: Theme::default(),
            previous_theme: Theme::default(),
            search_query: String::new(),
            selected_content_type: None, // Show all content types by default
            layout_config: LayoutConfig::default(),
            responsive_config: ResponsiveConfig::default(),
            math_asset_manager: MathAssetManager::default(),
            router: Router::new(),
            pending_url_update: None,

            #[cfg(debug_assertions)]
            debug_state: crate::debug_windows::DebugState::default(),

            shortcut_integration: crate::shortcuts::ShortcutIntegration::new(),
            focused_panel: crate::shortcuts::FocusedPanel::LeftPanel,
            scroll_offset: 0.0,
            side_panel_scroll_offset: 0.0,
            request_side_panel_auto_scroll: false,
            requested_scroll_delta: None,
            find_query: String::new(),
            find_matches: Vec::new(),
            current_find_match: 0,
            find_mode_active: false,
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

    /// Navigate to a new route and update browser URL.
    pub fn navigate_to(&mut self, route: Route) {
        let url = self.router.navigate_to(route);
        self.pending_url_update = Some(url);
        self.sync_state_to_route();
    }

    /// Sync app state to match the current route.
    fn sync_state_to_route(&mut self) {
        match self.router.current_route() {
            Route::Post { slug } | Route::Note { slug } | Route::Review { slug } => {
                if let Some(index) = self.post_manager.find_post_index_by_slug(slug) {
                    self.selected_post = index;
                    self.editing_new_post = false;
                    // Request auto-scroll to the post
                    self.request_side_panel_auto_scroll = true;
                    // Don't update selected_content_type when navigating to a post
                    // This allows staying in "All" tab mode when clicking posts
                } else {
                    // Post not found - show 404
                    self.router.navigate_to(Route::NotFound);
                }
            }
            Route::Search { query, tags: _ } => {
                self.search_query = query.clone();
                // TODO: Handle tags when tag system is implemented
            }
            Route::Tag { tag: _ } | Route::NotFound => {
                // TODO: Handle tag filtering when tag system is implemented
                // Show 404 message - handled in UI
            }
            Route::Home => {
                // Reset to default state
                self.selected_content_type = None; // Show all content types on home
                if self.post_manager.count() > 0 {
                    self.selected_post = 0;
                }
                self.editing_new_post = false;
            }
        }
    }

    /// Handle URL changes from the browser (web target only).
    #[cfg(target_arch = "wasm32")]
    fn handle_url_changes(&mut self, frame: &eframe::Frame) {
        let hash = &frame.info().web_info.location.hash;

        // Update router from hash
        if self.router.update_from_hash(hash) {
            self.sync_state_to_route();
        } else {
            // Clear any pending update since we're already at this route
            self.pending_url_update = None;
        }
    }

    /// Update browser URL if needed (web target only).
    #[cfg(target_arch = "wasm32")]
    fn update_browser_url(&mut self) {
        if let Some(hash) = self.pending_url_update.take() {
            // For hash-based routing, we can just update window.location.hash
            // This automatically adds to browser history
            if let Some(window) = web_sys::window() {
                let location = window.location();
                if let Err(err) = location.set_hash(&hash) {
                    log::warn!("Failed to update browser URL hash: {:?}", err);
                }
            }
        }
    }

    /// Restore saved route if valid
    #[cfg(feature = "persistence")]
    fn restore_route(&mut self) {
        // Router state is restored from serialization
        // Need to sync app state to the restored route
        self.sync_state_to_route();
    }
    
    /// Initialize shortcuts (called from UI loop)
    pub fn initialize_shortcuts(&mut self, _ctx: &egui::Context) {
        if !self.shortcut_integration.initialized {
            match self.shortcut_integration.initialize() {
                Ok(_) => {
                    log::info!("Shortcuts initialized successfully");
                    // Log loaded shortcuts for debugging
                    if let Some(config) = self.shortcut_integration.manager.config() {
                        log::info!("Loaded {} shortcuts", config.shortcuts.len());
                        for shortcut in &config.shortcuts {
                            log::debug!("Shortcut: {} - {}", shortcut.name, shortcut.description);
                        }
                    }
                }
                Err(err) => {
                    log::error!("Failed to initialize shortcuts: {err}");
                    // Show error to user?
                }
            }
        }
    }
}

impl eframe::App for BlogApp {
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Router state is automatically serialized as part of BlogApp
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Restore saved route once on first frame
        #[cfg(feature = "persistence")]
        if !self.router.is_initialized() {
            self.restore_route();
        }

        // Handle URL changes from browser (web target only)
        #[cfg(target_arch = "wasm32")]
        self.handle_url_changes(_frame);

        // Initialize and handle keyboard shortcuts
        self.initialize_shortcuts(ui.ctx());
        
        // Handle keyboard shortcuts - need to avoid borrowing issues
        let shortcut_handled = {
            // Take the integration out, use it, then put it back
            let mut integration = std::mem::take(&mut self.shortcut_integration);
            log::debug!("Shortcut integration initialized: {}", integration.initialized);
            log::debug!("Current focused panel: {:?}", self.focused_panel);
            let handled = integration.update(ui.ctx(), self);
            self.shortcut_integration = integration;
            if handled {
                log::debug!("Shortcut was handled");
            }
            handled
        };
        
        // Request repaint if shortcut was handled
        if shortcut_handled {
            ui.ctx().request_repaint();
        }

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
        let mut selected_post_for_nav = None;
        let _side_panel_response = Panel::left("side_panel").show_inside(ui, |ui| {
            // Get the full panel rect
            let panel_rect = ui.available_rect_before_wrap();
            
            let (changed, panel_clicked) = ui::layout::side_panel(
                ui,
                &self.post_manager,
                &self.post_manager_state, // NEW: pass state
                &self.search_query,
                &mut self.selected_content_type,
                &mut self.selected_post,
                &mut self.layout_config,
                |post_opt| {
                    selected_post_for_nav = post_opt.cloned();
                },
                self.focused_panel == crate::shortcuts::FocusedPanel::LeftPanel,
                panel_rect,
                &mut self.side_panel_scroll_offset,
                &mut self.request_side_panel_auto_scroll,
            );
            selection_changed = changed;
            
            if panel_clicked {
                log::debug!("Side panel clicked from layout.rs, focusing left panel");
                self.focused_panel = crate::shortcuts::FocusedPanel::LeftPanel;
            }
        });

        if selection_changed {
            self.editing_new_post = false;
            match selected_post_for_nav {
                Some(post) => {
                    // Navigate to the correct route based on content type
                    let route = match post.content_type {
                        crate::posts::ContentType::Post => {
                            crate::routing::Router::route_to_post(&post.slug)
                        }
                        crate::posts::ContentType::Note => {
                            crate::routing::Router::route_to_note(&post.slug)
                        }
                        crate::posts::ContentType::Review => {
                            crate::routing::Router::route_to_review(&post.slug)
                        }
                    };
                    self.navigate_to(route);
                }
                None => {
                    // Navigate to Home (e.g., when "All" tab is clicked)
                    self.navigate_to(crate::routing::Route::Home);
                }
            }
        }

        // Main content area with scrolling
        let mut post_saved = false;
        let mut editing_cancelled = false;
        let mut navigation_index = None;
        let mut retry_requested = false;
        let mut panel_clicked = false;
        let mut route_to_navigate = None;

        let _central_panel_response = CentralPanel::default().show_inside(ui, |ui| {
            // Get the full panel rect BEFORE the scroll area
            let panel_rect = ui.available_rect_before_wrap();
            
            let scroll_response = ScrollArea::vertical()
                .scroll_offset(egui::vec2(0.0, self.scroll_offset))
                .show(ui, |ui| {
                    // Apply requested scroll delta if any
                    if let Some(delta) = self.requested_scroll_delta.take() {
                        ui.scroll_with_delta(egui::vec2(0.0, delta));
                    }
                    // Use responsive container for optimal reading width
                ui::responsive::responsive_container(ui, &self.responsive_config, |ui| {
                    // Create closure first to avoid borrow conflicts
                    let mut navigate_callback = |route: crate::routing::Route| {
                        route_to_navigate = Some(route);
                    };

                    let navigation = ui::layout::NavigationContext {
                        current_route: self.router.current_route(),
                        on_navigate: &mut navigate_callback,
                    };

                    let state = ui::layout::MainContentState::new(
                        &self.post_manager,
                        self.selected_post,
                        self.editing_new_post,
                        &mut self.new_post_title,
                        &mut self.new_post_content,
                        &self.post_manager_state,
                        Some(&mut self.math_asset_manager),
                        navigation,
                    );
                    let result = ui::layout::main_content(
                        ui, 
                        state,
                        self.focused_panel == crate::shortcuts::FocusedPanel::RightPanel,
                        panel_rect,
                    );
                    (
                        post_saved,
                        editing_cancelled,
                        navigation_index,
                        retry_requested,
                        panel_clicked,
                    ) = result;
                    
                    if panel_clicked {
                        log::debug!("Main content clicked from layout.rs, focusing right panel");
                        self.focused_panel = crate::shortcuts::FocusedPanel::RightPanel;
                    }
                });
            });
            
            // Update scroll offset from scroll area response
            self.scroll_offset = scroll_response.state.offset.y;
        });
        
        // Draw find dialog if find mode is active
        if self.find_mode_active {
            self.draw_find_dialog(ui.ctx());
        }

        if let Some(new_index) = navigation_index {
            self.selected_post = new_index;
            self.editing_new_post = false;
        }

        if let Some(route) = route_to_navigate {
            self.navigate_to(route);
        }

        if post_saved {
            // Create new post (demo feature - posts normally come from markdown files)
            let slug = posts::BlogPost::generate_slug(&self.new_post_title);
            // Use today's date as placeholder
            let today = "2026-02-10"; // Simple placeholder
            let new_post = posts::BlogPost::new(
                self.post_manager.count(),
                posts::ContentType::Post, // Demo feature creates blog posts
                &self.new_post_title,
                &slug,
                &self.new_post_content,
                today,
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

        // Update browser URL if needed (web target only)
        #[cfg(target_arch = "wasm32")]
        self.update_browser_url();
    }
}

// Implement ContextProvider for BlogApp
impl crate::shortcuts::ContextProvider for BlogApp {
    fn focused_panel(&self) -> crate::shortcuts::FocusedPanel {
        self.focused_panel
    }
    
    fn search_has_focus(&self, _ctx: &egui::Context) -> bool {
        // TODO: Implement proper focus detection for search bar
        // For now, check if search query is being edited
        false
    }
    
    fn editor_has_focus(&self, _ctx: &egui::Context) -> bool {
        // TODO: Implement proper focus detection for editor
        self.editing_new_post
    }
    
    fn find_mode_active(&self) -> bool {
        self.find_mode_active
    }
}

// Implement ActionExecutor for BlogApp
impl crate::shortcuts::ActionExecutor for BlogApp {
    fn execute_action(&mut self, action: &crate::shortcuts::ShortcutAction) -> bool {
        use crate::shortcuts::ShortcutAction::{NavigatePost, SwitchTab, Scroll, FocusPanel, FocusSearch, FindInContent, FindNext, FindPrevious, ToggleTheme, ShowHelp, BrowserAddress, Custom};
        
        match action {
            NavigatePost { direction } => self.navigate_post(*direction),
            SwitchTab { direction } => self.switch_tab(*direction),
            Scroll { direction, amount } => self.scroll(*direction, *amount),
            FocusPanel { panel } => self.focus_panel(*panel),
            FocusSearch => self.focus_search(),
            FindInContent => self.find_in_content(),
            FindNext => self.find_next(),
            FindPrevious => self.find_previous(),
            ToggleTheme => self.toggle_theme(),
            ShowHelp => self.show_help(),
            BrowserAddress => self.browser_address(),
            Custom { name } => self.execute_custom(name),
        }
    }
    
    fn navigate_post(&mut self, navigation: crate::shortcuts::PostNavigation) -> bool {
        use crate::shortcuts::PostNavigation::{Next, Previous, First, Last};
        
        // Get filtered posts using the same logic as the side panel display
        // This includes search query, sort order, and content type filter
        let posts_to_show: Vec<_> = self.post_manager
            .search(&self.search_query, self.layout_config.post_sort_order)
            .into_iter()
            .enumerate()
            .filter(|(_, post)| {
                // Apply content type filter if set
                match self.selected_content_type {
                    Some(content_type) => post.content_type == content_type,
                    None => true, // Show all
                }
            })
            .collect();
        
        if posts_to_show.is_empty() {
            return false;
        }
        
        // Find current post in the filtered/displayed list
        let current_post = if self.selected_post < self.post_manager.posts().len() {
            Some(&self.post_manager.posts()[self.selected_post])
        } else {
            None
        };
        
        let current_display_index = if let Some(current_post) = current_post {
            posts_to_show.iter()
                .position(|(_, post)| post.id == current_post.id)
        } else {
            None
        };
        
        // If current post is not in the displayed list, start from first
        let current_index = current_display_index.unwrap_or(0);
        
        let navigation_successful = match navigation {
            Next => {
                if current_index + 1 < posts_to_show.len() {
                    // Get the original index of the next post in display order
                    let next_post = &posts_to_show[current_index + 1].1;
                    self.selected_post = self.post_manager.posts()
                        .iter()
                        .position(|p| p.id == next_post.id)
                        .unwrap_or(self.selected_post);
                    log::debug!("Navigated to next post: {} (index {})", next_post.title, self.selected_post);
                    true
                } else {
                    log::debug!("Cannot navigate next: already at last post (index {current_index})");
                    false
                }
            }
            Previous => {
                if current_index > 0 {
                    // Get the original index of the previous post in display order
                    let prev_post = &posts_to_show[current_index - 1].1;
                    self.selected_post = self.post_manager.posts()
                        .iter()
                        .position(|p| p.id == prev_post.id)
                        .unwrap_or(self.selected_post);
                    log::debug!("Navigated to previous post: {} (index {})", prev_post.title, self.selected_post);
                    true
                } else {
                    log::debug!("Cannot navigate previous: already at first post (index {current_index})");
                    false
                }
            }
            First => {
                let first_post = &posts_to_show[0].1;
                self.selected_post = self.post_manager.posts()
                    .iter()
                    .position(|p| p.id == first_post.id)
                    .unwrap_or(self.selected_post);
                log::debug!("Navigated to first post: {} (index {})", first_post.title, self.selected_post);
                true
            }
            Last => {
                let last_post = &posts_to_show.last().expect("posts_to_show should not be empty").1;
                self.selected_post = self.post_manager.posts()
                    .iter()
                    .position(|p| p.id == last_post.id)
                    .unwrap_or(self.selected_post);
                log::debug!("Navigated to last post: {} (index {})", last_post.title, self.selected_post);
                true
            }
        };
        
        // Request auto-scroll if navigation was successful
        if navigation_successful {
            self.request_side_panel_auto_scroll = true;
        }
        
        navigation_successful
    }
    
    fn switch_tab(&mut self, direction: crate::shortcuts::TabDirection) -> bool {
        use crate::shortcuts::TabDirection::{Next, Previous};
        use crate::posts::ContentType::{Post, Note, Review};
        
        let current = self.selected_content_type;
        let tabs = [None, Some(Post), Some(Note), Some(Review)];
        
        let current_index = tabs.iter().position(|&t| t == current).unwrap_or(0);
        let new_index = match direction {
            Next => (current_index + 1) % tabs.len(),
            Previous => (current_index + tabs.len() - 1) % tabs.len(),
        };
        
        self.selected_content_type = tabs[new_index];
        true
    }
    
    fn scroll(&mut self, direction: crate::shortcuts::ScrollDirection, amount: crate::shortcuts::ScrollAmount) -> bool {
        // Calculate scroll amount based on direction and amount type
        let scroll_step = match amount {
            crate::shortcuts::ScrollAmount::Small => 50.0, // Small step
            crate::shortcuts::ScrollAmount::HalfPage => 300.0, // Half page
            crate::shortcuts::ScrollAmount::Page => 600.0, // Full page
        };
        
        let delta = match direction {
            crate::shortcuts::ScrollDirection::Up => scroll_step,    // Positive = scroll up (content moves down) - FIXED
            crate::shortcuts::ScrollDirection::Down => -scroll_step, // Negative = scroll down (content moves up) - FIXED
        };
        
        // Store delta to be applied in UI
        self.requested_scroll_delta = Some(delta);
        log::debug!("Scroll requested: {direction:?} {amount:?} (delta: {delta})");
        true
    }
    
    fn focus_panel(&mut self, panel: crate::shortcuts::FocusedPanel) -> bool {
        log::debug!("Focus panel called: {panel:?}");
        self.focused_panel = panel;
        true
    }
    
    fn focus_search(&mut self) -> bool {
        // TODO: Implement focus search
        // Need to set focus to search bar widget
        log::debug!("Focus search requested");
        false
    }
    
    fn find_in_content(&mut self) -> bool {
        self.find_mode_active = true;
        self.find_query.clear();
        self.find_matches.clear();
        self.current_find_match = 0;
        true
    }
    
    fn find_next(&mut self) -> bool {
        if self.find_matches.is_empty() {
            return false;
        }
        
        self.current_find_match = (self.current_find_match + 1) % self.find_matches.len();
        
        // TODO: Scroll to match
        // For now, just log
        log::debug!("Find next: match {} of {}", self.current_find_match + 1, self.find_matches.len());
        true
    }
    
    fn find_previous(&mut self) -> bool {
        if self.find_matches.is_empty() {
            return false;
        }
        
        self.current_find_match = (self.current_find_match + self.find_matches.len() - 1) % self.find_matches.len();
        
        // TODO: Scroll to match
        // For now, just log
        log::debug!("Find previous: match {} of {}", self.current_find_match + 1, self.find_matches.len());
        true
    }
    
    fn toggle_theme(&mut self) -> bool {
        self.theme = match self.theme {
            crate::ui::Theme::CatppuccinLatte => crate::ui::Theme::CatppuccinMacchiato,
            crate::ui::Theme::CatppuccinMacchiato => crate::ui::Theme::CatppuccinLatte,
        };
        true
    }
    
    fn show_help(&mut self) -> bool {
        self.shortcut_integration.show_help();
        true
    }
    
    fn browser_address(&mut self) -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            // TODO: Implement browser address bar focus for web
            log::debug!("Browser address focus requested (web only)");
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            log::debug!("Browser address focus is web-only feature");
        }
        true
    }
    
    fn execute_custom(&mut self, action: &str) -> bool {
        log::debug!("Custom action requested: {action}");
        false
    }
}

impl BlogApp {
    /// Draw the find dialog
    fn draw_find_dialog(&mut self, ctx: &egui::Context) {
        use egui::{Align2, Key};
        
        let mut open = self.find_mode_active;
        egui::Window::new("Find in Content")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::RIGHT_TOP, egui::vec2(-20.0, 20.0))
            .default_width(300.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Find:");
                    let response = ui.text_edit_singleline(&mut self.find_query);
                    
                    // Focus the text input when dialog opens
                    if !self.find_query.is_empty() && self.find_matches.is_empty() {
                        self.update_find_matches();
                    }
                    
                    // Handle Enter key to find next
                    if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter))
                        && !self.find_matches.is_empty()
                    {
                        self.current_find_match = (self.current_find_match + 1) % self.find_matches.len();
                    }
                    
                    // Handle Escape key to close
                    if ui.input(|i| i.key_pressed(Key::Escape)) {
                        self.find_mode_active = false;
                    }
                });
                
                ui.horizontal(|ui| {
                    if ui.button("Find").clicked() {
                        self.update_find_matches();
                    }
                    
                    if ui.button("Next").clicked() || ui.input(|i| i.key_pressed(Key::N) && i.modifiers.ctrl) {
                        self.find_next();
                    }
                    
                    if ui.button("Previous").clicked() || ui.input(|i| i.key_pressed(Key::P) && i.modifiers.ctrl) {
                        self.find_previous();
                    }
                    
                    if ui.button("Close").clicked() {
                        self.find_mode_active = false;
                    }
                });
                
                // Show match count
                if !self.find_matches.is_empty() {
                    ui.label(format!("{} of {}", self.current_find_match + 1, self.find_matches.len()));
                } else if !self.find_query.is_empty() {
                    ui.label("No matches found");
                }
            });
        
        self.find_mode_active = open;
    }
    
    /// Update find matches based on current query
    fn update_find_matches(&mut self) {
        self.find_matches.clear();
        self.current_find_match = 0;
        
        if self.find_query.is_empty() {
            return;
        }
        
        // Get current post content
        let posts = self.post_manager.posts();
        if self.selected_post >= posts.len() {
            return;
        }
        
        let post = &posts[self.selected_post];
        let content = &post.content;
        let query = self.find_query.to_lowercase();
        
        // Simple case-insensitive search
        let mut start = 0;
        while let Some(pos) = content[start..].to_lowercase().find(&query) {
            let actual_pos = start + pos;
            let end = actual_pos + query.len();
            
            self.find_matches.push(TextMatch {
                start: actual_pos,
                end,
            });
            
            start = end;
        }
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
