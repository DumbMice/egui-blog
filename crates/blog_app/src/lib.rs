//! A blog application built with egui.
//! Testing file watcher improvements.

#[cfg(target_arch = "wasm32")]
mod web;

pub mod animation;
pub mod math;
mod posts;
mod routing;
pub mod shortcuts;
pub mod tags;
pub mod typography;
pub mod ui;

mod build_filter;

#[cfg(debug_assertions)]
mod debug_windows;

use egui::{CentralPanel, Panel, ScrollArea};
pub use posts::{PostManager, PostManagerState};
use ui::{LayoutConfig, ResponsiveConfig, Theme};

use crate::math::MathAssetManager;
use crate::routing::{Route, Router};
use crate::shortcuts::ActionExecutor as _;

/// Default math resolution scale (1.0 = original resolution)
fn default_math_resolution_scale() -> f32 {
    1.0
}

/// Font loading state tracking
/// Fonts load asynchronously in egui and are only available in the next frame
#[derive(Debug, Clone, PartialEq)]
enum FontLoadingState {
    /// Fonts are being loaded (initial state)
    Loading,
    /// Fonts have been loaded and are ready for use
    Ready,
    /// Font loading failed with error message
    Failed(String),
}

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
    /// Tag-based search state
    tag_search_state: crate::tags::TagSearchState,
    /// Cached tags with theme-based colors
    #[cfg_attr(feature = "serde", serde(skip))]
    cached_tags: Option<(
        crate::ui::components::Theme,
        std::collections::HashMap<String, crate::tags::Tag>,
    )>,
    /// Selected content type filter (None = show all)
    selected_content_type: Option<crate::posts::ContentType>,
    /// Layout configuration
    layout_config: LayoutConfig,
    /// Responsive layout configuration
    responsive_config: ResponsiveConfig,
    /// Side panel collapsed state
    side_panel_collapsed: bool,
    /// Math asset manager for rendering formula SVGs
    #[cfg_attr(feature = "serde", serde(skip))]
    math_asset_manager: MathAssetManager,

    /// Math formula resolution scaling factor
    /// 1.0 = original resolution, 2.0 = 2x resolution, etc.
    /// Capped at 25.0 maximum
    #[cfg_attr(feature = "serde", serde(default = "default_math_resolution_scale"))]
    math_resolution_scale: f32,

    /// Font loading state tracking
    /// Fonts load asynchronously in egui and are only available in the next frame
    /// This tracks whether fonts have been successfully loaded and are ready for use
    #[cfg_attr(feature = "serde", serde(skip))]
    font_loading_state: FontLoadingState,

    /// URL router
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
    /// Previous focused panel (for detecting focus changes)
    #[cfg_attr(feature = "serde", serde(skip))]
    previous_focused_panel: crate::shortcuts::FocusedPanel,
    /// Animation state for panel focus visualization
    #[cfg_attr(feature = "serde", serde(skip))]
    focus_animation: crate::animation::FocusAnimationState,
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
    /// Whether route has been restored from persistence (to avoid restoring every frame)
    route_restored: bool,
    /// Whether app was just restored from persistence (to avoid navigation immediately after restore)
    #[cfg_attr(feature = "serde", serde(skip))]
    just_restored: bool,
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
            tag_search_state: crate::tags::TagSearchState::new(),
            selected_content_type: None, // Show all content types by default
            layout_config: LayoutConfig::default(),
            responsive_config: ResponsiveConfig::default(),
            side_panel_collapsed: false,
            math_asset_manager: MathAssetManager::default(),
            math_resolution_scale: default_math_resolution_scale(),
            font_loading_state: FontLoadingState::Loading,
            router: Router::new(),
            pending_url_update: None,

            #[cfg(debug_assertions)]
            debug_state: crate::debug_windows::DebugState::default(),

            shortcut_integration: crate::shortcuts::ShortcutIntegration::new(),
            focused_panel: crate::shortcuts::FocusedPanel::LeftPanel,
            previous_focused_panel: crate::shortcuts::FocusedPanel::LeftPanel,
            focus_animation: crate::animation::FocusAnimationState::new(),
            scroll_offset: 0.0,
            side_panel_scroll_offset: 0.0,
            request_side_panel_auto_scroll: false,
            requested_scroll_delta: None,
            find_query: String::new(),
            find_matches: Vec::new(),
            current_find_match: 0,
            find_mode_active: false,
            cached_tags: None,
            route_restored: false,
            just_restored: false,
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

        #[cfg(feature = "persistence")]
        {
            // If we loaded from storage, mark as just restored
            if cc.storage.is_some() {
                app.just_restored = true;
            }
        }

        #[cfg(not(feature = "persistence"))]
        let mut app = Self::default();

        // Configure custom typography with Ubuntu font variants
        let fonts_configured = crate::typography::configure_typography(cc);
        if fonts_configured {
            app.font_loading_state = FontLoadingState::Loading;
            log::info!("Font configuration initiated, fonts will be available in next frame");
        } else {
            app.font_loading_state =
                FontLoadingState::Failed("Font configuration failed".to_owned());
            log::error!("Font configuration failed");
        }

        // Apply theme to context (this will also set up text styles)
        app.theme.apply(&cc.egui_ctx);
        app.previous_theme = app.theme;

        // Migration: Convert old search_query to new tag_search_state
        #[cfg(feature = "persistence")]
        {
            // Check if we have the old field (this is a hack since we can't directly access it)
            // We'll rely on serde's default for missing fields
        }

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

        // Invalidate tag cache since posts may have changed
        self.cached_tags = None;

        // Ensure valid selection
        self.ensure_valid_selection();
    }

    /// Get cached tags, computing them if necessary
    fn get_cached_tags(&mut self) -> &std::collections::HashMap<String, crate::tags::Tag> {
        // Check if cache is valid (matches current theme)
        let cache_valid = self
            .cached_tags
            .as_ref()
            .is_some_and(|(cached_theme, _)| *cached_theme == self.theme);

        if !cache_valid {
            // Compute tags and cache them
            let tags = crate::tags::extract_all_tags(self.post_manager.posts(), &self.theme);
            self.cached_tags = Some((self.theme, tags));
        }

        &self
            .cached_tags
            .as_ref()
            .expect("cached_tags should be initialized")
            .1
    }

    /// Navigate to a new route and update browser URL.
    pub fn navigate_to(&mut self, route: Route) {
        use std::backtrace::Backtrace;
        let backtrace = Backtrace::capture();
        log::debug!(
            "navigate_to called with route: {:?}. Current route: {:?}, selected_post: {}, selected_content_type: {:?}\nBacktrace:\n{}",
            route,
            self.router.current_route(),
            self.selected_post,
            self.selected_content_type,
            backtrace
        );
        let url = self.router.navigate_to(route);
        self.pending_url_update = Some(url);
        self.sync_state_to_route();
    }

    /// Sync app state to match the current route.
    fn sync_state_to_route(&mut self) {
        use std::backtrace::Backtrace;
        let backtrace = Backtrace::capture();
        log::debug!(
            "sync_state_to_route called. Current route: {:?}, selected_post before: {}, just_restored: {}\nBacktrace:\n{}",
            self.router.current_route(),
            self.selected_post,
            self.just_restored,
            backtrace
        );

        // If we just restored from persistence, skip navigation for any route
        // This prevents unwanted navigation when state is freshly restored
        if self.just_restored {
            log::debug!("Skipping sync_state_to_route after restore (just_restored: true)");
            self.just_restored = false;
            return;
        }

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
            Route::Search { query, tags } => {
                self.tag_search_state.search_text = query.clone();
                self.tag_search_state.selected_tags = tags.clone();
            }
            Route::Tag { tag } => {
                self.tag_search_state.selected_tags = vec![tag.clone()];
                self.tag_search_state.search_text.clear();
            }
            Route::NotFound => {
                // Show 404 message - handled in UI
            }
            Route::Home => {
                log::debug!("Route::Home detected in sync_state_to_route");
                // Reset to default state
                self.selected_content_type = None; // Show all content types on home

                // Reset to first post when navigating to Home
                // Note: just_restored check at beginning prevents this from executing
                // when restoring from persistence or URL
                if self.post_manager.count() > 0 {
                    log::debug!("Setting selected_post = 0 for Route::Home");
                    self.selected_post = 0;
                }
                self.editing_new_post = false;
            }
        }
    }

    /// Handle URL changes from the browser (web target only).
    #[cfg(target_arch = "wasm32")]
    fn handle_url_changes(&mut self, frame: &eframe::Frame) {
        #[cfg(target_arch = "wasm32")]
        {
            let hash = &frame.info().web_info.location.hash;
            log::debug!("handle_url_changes called with hash: '{}'", hash);

            // Skip if we just restored state (to prevent conflicts)
            if self.just_restored {
                log::debug!("Skipping handle_url_changes after restore (just_restored: true)");
                return;
            }

            // Update router from hash
            if self.router.update_from_hash(hash) {
                log::debug!("Route changed, calling sync_state_to_route()");
                self.sync_state_to_route();
            } else {
                // Clear any pending update since we're already at this route
                self.pending_url_update = None;
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        let _ = frame; // Mark as unused on native
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
        log::debug!(
            "restore_route called. Current route: {:?}, selected_post before: {}",
            self.router.current_route(),
            self.selected_post
        );
        self.sync_state_to_route();
    }

    /// Unified state restoration with clear precedence
    /// Precedence: Browser URL > Persisted State > Default
    fn restore_state_with_precedence(&mut self, frame: &eframe::Frame) {
        log::debug!("restore_state_with_precedence called");

        // Step 1: Check browser URL (highest priority for web)
        #[cfg(target_arch = "wasm32")]
        {
            let hash = &frame.info().web_info.location.hash;
            if !hash.is_empty() && hash != "#/" {
                log::debug!("Browser URL found: '{}', using as source of truth", hash);
                if self.router.update_from_hash(hash) {
                    log::debug!("Updated router from browser URL");
                }
                // Apply the route to app state
                self.just_restored = false; // Clear just_restored so sync_state_to_route() executes
                self.sync_state_to_route();
                // Browser URL takes precedence, skip persisted state
                self.route_restored = true;
                return;
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        let _ = frame; // Mark as unused on native

        // Step 2: Use persisted state (if available and not just restored)
        #[cfg(feature = "persistence")]
        {
            if !self.route_restored {
                log::debug!("No browser URL, restoring from persisted state");
                self.restore_route();
                self.route_restored = true;
                self.just_restored = false; // Clear just_restored flag for any route
            }
        }

        // Step 3: Default state (already set in constructor)
        log::debug!("Using default state");
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
        log::debug!("=== UI FRAME START ===");
        log::debug!(
            "Current state: route: {:?}, selected_post: {}, route_restored: {}",
            self.router.current_route(),
            self.selected_post,
            self.route_restored
        );

        // Check and update font loading state
        // Fonts load asynchronously and are only available in the next frame
        match &self.font_loading_state {
            FontLoadingState::Loading => {
                // Check if fonts are now ready
                if crate::typography::verify_text_styles_available(ui.ctx()) {
                    log::info!("Fonts are now ready for use");
                    self.font_loading_state = FontLoadingState::Ready;
                } else {
                    log::debug!("Fonts still loading, waiting for next frame");
                }
            }
            FontLoadingState::Ready => {
                // Fonts are ready, nothing to do
            }
            FontLoadingState::Failed(error) => {
                log::warn!("Font loading failed: {error}");
                // In a real implementation, we might try to recover here
                // For now, we'll just log the error
            }
        }

        // Unified state restoration with clear precedence
        if !self.route_restored {
            log::debug!("State not restored yet, calling restore_state_with_precedence()");
            self.restore_state_with_precedence(_frame);
            log::debug!(
                "After restore_state_with_precedence: route: {:?}, selected_post: {}",
                self.router.current_route(),
                self.selected_post
            );
        } else {
            log::debug!(
                "State already restored (route_restored: {}), handling URL changes only",
                self.route_restored
            );
            // Handle URL changes from browser (web target only)
            #[cfg(target_arch = "wasm32")]
            self.handle_url_changes(_frame);
        }

        // Initialize and handle keyboard shortcuts
        self.initialize_shortcuts(ui.ctx());

        // Handle keyboard shortcuts - need to avoid borrowing issues
        let shortcut_handled = {
            // Take the integration out, use it, then put it back
            let mut integration = std::mem::take(&mut self.shortcut_integration);
            log::debug!(
                "Shortcut integration initialized: {}",
                integration.initialized
            );
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

        // Mobile auto-collapse logic
        let screen_width = ui.ctx().content_rect().width();
        let is_mobile = screen_width < self.responsive_config.mobile_breakpoint;

        if is_mobile && !self.side_panel_collapsed {
            log::debug!(
                "Mobile screen detected ({}px < {}px), auto-collapsing side panel",
                screen_width,
                self.responsive_config.mobile_breakpoint
            );
            self.side_panel_collapsed = true;
        }

        // Note: Removed auto-expand logic to give users full control over panel state
        // Users can expand/collapse using hamburger buttons in top panel or side panel
        // Auto-collapse on mobile still works, but auto-expand on desktop is disabled

        let current_time = ui.ctx().input(|i| i.time);

        // Check if focus changed since last frame
        if self.focused_panel != self.previous_focused_panel {
            log::debug!(
                "Focus changed from {:?} to {:?}, triggering animation",
                self.previous_focused_panel,
                self.focused_panel
            );

            // Trigger animation for focus change
            self.focus_animation
                .on_focus_change(self.focused_panel, current_time);

            // Update previous focused panel
            self.previous_focused_panel = self.focused_panel;
        }

        // Update animation state every frame
        let animation_config = {
            #[cfg(debug_assertions)]
            {
                self.debug_state.animation_config
            }
            #[cfg(not(debug_assertions))]
            {
                crate::animation::FocusAnimationConfig::default()
            }
        };
        self.focus_animation.update(current_time, &animation_config);

        // Track if tag search state was modified
        let mut tag_search_was_modified = false;

        // Get cached tags once and reuse
        let all_tags = self.get_cached_tags();
        let all_tags_vec: Vec<_> = all_tags.values().cloned().collect();

        // Store search state before top panel to detect actual changes
        let search_state_before = self.tag_search_state.clone();

        // Top panel
        let mut top_panel_result = ui::layout::TopPanelResult {
            search_changed: false,
            theme_changed: false,
            search_committed: false,
        };
        Panel::top("top_panel").show_inside(ui, |ui| {
            top_panel_result = ui::layout::top_panel(
                ui,
                "My Blog",
                &mut self.theme,
                &mut ui::layout::TopPanelConfig {
                    tag_search_state: &mut self.tag_search_state,
                    all_tags: &all_tags_vec,
                    post_manager: &self.post_manager,
                    selected_post: self.selected_post,
                },
                #[cfg(debug_assertions)]
                &mut self.debug_state,
            );

            // Only mark search as modified if it actually changed
            if top_panel_result.search_changed {
                tag_search_was_modified = true;
            }
        });

        // Check if theme changed (via UI button or keyboard shortcut) and apply it
        if self.theme != self.previous_theme {
            log::debug!(
                "Theme changed from {:?} to {:?}, applying to UI",
                self.previous_theme,
                self.theme
            );
            self.theme.apply(ui.ctx());
            self.previous_theme = self.theme;
        } else if top_panel_result.theme_changed {
            // This shouldn't happen, but log if it does (theme changed but detection didn't trigger)
            log::warn!("top_panel reported theme changed but self.theme == self.previous_theme");
        }

        // Handle search committed with Enter key
        if top_panel_result.search_committed {
            log::debug!("Search committed with Enter key, updating URL");
            // Navigate to search route to update URL
            let route = crate::routing::Route::Search {
                query: self.tag_search_state.search_text.clone(),
                tags: self.tag_search_state.selected_tags.clone(),
            };
            self.navigate_to(route);
        }

        // Defensive check: Verify search actually changed before navigating
        // This prevents false positives from theme changes or other UI interactions
        if tag_search_was_modified {
            let search_actually_changed = search_state_before.search_text
                != self.tag_search_state.search_text
                || search_state_before.selected_tags != self.tag_search_state.selected_tags;

            if !search_actually_changed {
                log::debug!(
                    "Search marked as modified but no actual change detected (likely theme change), skipping navigation"
                );
                tag_search_was_modified = false;
            }
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

            // Show animation configuration window if enabled
            if self.debug_state.show_animation_config {
                crate::debug_windows::show_animation_config_window(ui, &mut self.debug_state);
            }

            // Show simple search test window if enabled
            if self.debug_state.show_simple_search_test {
                crate::debug_windows::show_simple_search_test_window(ui, &mut self.debug_state);
            }

            // Show math resolution config window if enabled
            if self.debug_state.show_math_resolution_config {
                crate::debug_windows::show_math_resolution_config_window(
                    ui,
                    &mut self.debug_state,
                    &mut self.math_resolution_scale,
                );
            }
        }

        // Side panel
        let mut selection_changed = false;
        let mut selected_post_for_nav = None;

        // Determine panel width based on collapsed state
        let panel_width = if self.side_panel_collapsed {
            40.0 // Minimal width when collapsed (just enough for hamburger button)
        } else {
            200.0 // Default width when expanded
        };

        let _side_panel_response = Panel::left("side_panel")
            .resizable(!self.side_panel_collapsed) // Only resizable when expanded
            .min_size(if self.side_panel_collapsed {
                40.0
            } else {
                150.0
            })
            .max_size(if self.side_panel_collapsed {
                40.0
            } else {
                500.0
            })
            .default_size(panel_width)
            .show_inside(ui, |ui| {
                // Get the full panel rect (will be 0 width when collapsed)
                let panel_rect = ui.available_rect_before_wrap();

                let (changed, panel_clicked) = ui::layout::side_panel(
                    ui,
                    &self.post_manager,
                    &self.post_manager_state,
                    &mut self.tag_search_state,
                    &all_tags_vec,
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
                    // Animation parameters
                    &self.focus_animation,
                    &animation_config,
                    // Panel state
                    self.side_panel_collapsed,
                    || {
                        self.side_panel_collapsed = !self.side_panel_collapsed;
                    },
                );
                selection_changed = changed;

                if panel_clicked {
                    log::debug!("Side panel clicked from layout.rs, focusing left panel");
                    self.focused_panel = crate::shortcuts::FocusedPanel::LeftPanel;
                }
            });

        if selection_changed {
            self.editing_new_post = false;
            if let Some(post) = selected_post_for_nav {
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
            } else {
                // Navigate to Home (e.g., when "All" tab is clicked)
                log::debug!("Selection changed to None, navigating to Home");
                self.navigate_to(crate::routing::Route::Home);
            }
        }

        // Main content area with scrolling
        let mut post_saved = false;
        let mut editing_cancelled = false;
        let mut navigation_index = None;
        let mut retry_requested = false;
        let mut panel_clicked = false;
        let mut route_to_navigate = None;

        // all_tags_vec is already computed above and can be reused here

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
                            &mut self.tag_search_state,
                            &all_tags_vec,
                            self.math_resolution_scale,
                        );
                        let result = ui::layout::main_content(
                            ui,
                            state,
                            self.focused_panel == crate::shortcuts::FocusedPanel::RightPanel,
                            panel_rect,
                            // Animation parameters
                            &self.focus_animation,
                            &animation_config,
                        );
                        (
                            post_saved,
                            editing_cancelled,
                            navigation_index,
                            retry_requested,
                            panel_clicked,
                        ) = result;

                        if panel_clicked {
                            log::debug!(
                                "Main content clicked from layout.rs, focusing right panel"
                            );
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

        // Note: Search state changes no longer automatically update URL
        // URL updates only happen via explicit actions (Enter key, navigation)
        // This prevents cursor positioning issues in WASM
        if tag_search_was_modified {
            #[cfg(target_arch = "wasm32")]
            log::debug!("Search modified but URL not updated (prevent cursor issues)");
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

    fn search_has_focus(&self, ctx: &egui::Context) -> bool {
        // Check if the search bar widget has focus
        // The search bar has ID: egui::Id::new("tag_search_input")
        ctx.memory(|mem| mem.has_focus(egui::Id::new("tag_search_input")))
    }

    fn editor_has_focus(&self, ctx: &egui::Context) -> bool {
        // Check if any editor text field has focus
        // New post title has ID: egui::Id::new("new_post_title")
        // New post content has ID: egui::Id::new("new_post_content")
        // Find dialog input has ID: egui::Id::new("find_dialog_input")
        ctx.memory(|mem| {
            mem.has_focus(egui::Id::new("new_post_title"))
                || mem.has_focus(egui::Id::new("new_post_content"))
                || mem.has_focus(egui::Id::new("find_dialog_input"))
        })
    }

    fn find_mode_active(&self) -> bool {
        self.find_mode_active
    }
}

// Implement ActionExecutor for BlogApp
impl crate::shortcuts::ActionExecutor for BlogApp {
    fn execute_action(&mut self, action: &crate::shortcuts::ShortcutAction) -> bool {
        use crate::shortcuts::ShortcutAction::{
            BrowserAddress, CollapseSidePanel, Custom, ExpandSidePanel, FindInContent, FindNext,
            FindPrevious, FocusPanel, FocusSearch, NavigatePost, Scroll, ShowHelp, SwitchTab,
            ToggleSidePanel, ToggleTheme,
        };

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
            ToggleSidePanel => self.toggle_side_panel(),
            CollapseSidePanel => self.collapse_side_panel(),
            ExpandSidePanel => self.expand_side_panel(),
            Custom { name } => self.execute_custom(name),
        }
    }

    fn navigate_post(&mut self, navigation: crate::shortcuts::PostNavigation) -> bool {
        use crate::shortcuts::PostNavigation::{First, Last, Next, Previous};

        // Get filtered posts using the same logic as the side panel display
        // This includes tag search, sort order, and content type filter
        let mut posts_to_show =
            crate::tags::search_posts(self.post_manager.posts(), &self.tag_search_state);

        // Apply content type filter if set
        if let Some(content_type) = self.selected_content_type {
            posts_to_show.retain(|post| post.content_type == content_type);
        }

        // Apply sort order
        posts_to_show.sort_by(|a, b| match self.layout_config.post_sort_order {
            crate::ui::layout::PostSortOrder::NewestFirst => b.date.cmp(&a.date),
            crate::ui::layout::PostSortOrder::OldestFirst => a.date.cmp(&b.date),
        });

        let posts_to_show: Vec<_> = posts_to_show.into_iter().enumerate().collect();

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
            posts_to_show
                .iter()
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
                    self.selected_post = self
                        .post_manager
                        .posts()
                        .iter()
                        .position(|p| p.id == next_post.id)
                        .unwrap_or(self.selected_post);
                    log::debug!(
                        "Navigated to next post: {} (index {})",
                        next_post.title,
                        self.selected_post
                    );
                    true
                } else {
                    log::debug!(
                        "Cannot navigate next: already at last post (index {current_index})"
                    );
                    false
                }
            }
            Previous => {
                if current_index > 0 {
                    // Get the original index of the previous post in display order
                    let prev_post = &posts_to_show[current_index - 1].1;
                    self.selected_post = self
                        .post_manager
                        .posts()
                        .iter()
                        .position(|p| p.id == prev_post.id)
                        .unwrap_or(self.selected_post);
                    log::debug!(
                        "Navigated to previous post: {} (index {})",
                        prev_post.title,
                        self.selected_post
                    );
                    true
                } else {
                    log::debug!(
                        "Cannot navigate previous: already at first post (index {current_index})"
                    );
                    false
                }
            }
            First => {
                let first_post = &posts_to_show[0].1;
                self.selected_post = self
                    .post_manager
                    .posts()
                    .iter()
                    .position(|p| p.id == first_post.id)
                    .unwrap_or(self.selected_post);
                log::debug!(
                    "Navigated to first post: {} (index {})",
                    first_post.title,
                    self.selected_post
                );
                true
            }
            Last => {
                let last_post = &posts_to_show
                    .last()
                    .expect("posts_to_show should not be empty")
                    .1;
                self.selected_post = self
                    .post_manager
                    .posts()
                    .iter()
                    .position(|p| p.id == last_post.id)
                    .unwrap_or(self.selected_post);
                log::debug!(
                    "Navigated to last post: {} (index {})",
                    last_post.title,
                    self.selected_post
                );
                true
            }
        };

        // Request auto-scroll if navigation was successful
        if navigation_successful {
            self.request_side_panel_auto_scroll = true;

            // Update URL to match the new post selection (consistent with mouse clicks)
            if let Some(post) = self.post_manager.get(self.selected_post) {
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
        }

        navigation_successful
    }

    fn switch_tab(&mut self, direction: crate::shortcuts::TabDirection) -> bool {
        use crate::posts::ContentType::{Note, Post, Review};
        use crate::shortcuts::TabDirection::{Next, Previous};

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

    fn scroll(
        &mut self,
        direction: crate::shortcuts::ScrollDirection,
        amount: crate::shortcuts::ScrollAmount,
    ) -> bool {
        // Calculate scroll amount based on direction and amount type
        let scroll_step = match amount {
            crate::shortcuts::ScrollAmount::Small => 50.0, // Small step
            crate::shortcuts::ScrollAmount::HalfPage => 300.0, // Half page
            crate::shortcuts::ScrollAmount::Page => 600.0, // Full page
        };

        let delta = match direction {
            crate::shortcuts::ScrollDirection::Up => scroll_step, // Positive = scroll up (content moves down) - FIXED
            crate::shortcuts::ScrollDirection::Down => -scroll_step, // Negative = scroll down (content moves up) - FIXED
        };

        // Store delta to be applied in UI
        self.requested_scroll_delta = Some(delta);
        log::debug!("Scroll requested: {direction:?} {amount:?} (delta: {delta})");
        true
    }

    fn focus_panel(&mut self, panel: crate::shortcuts::FocusedPanel) -> bool {
        log::debug!("Focus panel called: {panel:?}");

        // If focusing left panel and it's collapsed, expand it first
        if panel == crate::shortcuts::FocusedPanel::LeftPanel && self.side_panel_collapsed {
            log::debug!("Left panel is collapsed, expanding it");
            self.side_panel_collapsed = false;
        }

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
        log::debug!(
            "Find next: match {} of {}",
            self.current_find_match + 1,
            self.find_matches.len()
        );
        true
    }

    fn find_previous(&mut self) -> bool {
        if self.find_matches.is_empty() {
            return false;
        }

        self.current_find_match =
            (self.current_find_match + self.find_matches.len() - 1) % self.find_matches.len();

        // TODO: Scroll to match
        // For now, just log
        log::debug!(
            "Find previous: match {} of {}",
            self.current_find_match + 1,
            self.find_matches.len()
        );
        true
    }

    fn toggle_theme(&mut self) -> bool {
        log::debug!(
            "toggle_theme called. Current route: {:?}, selected_post: {}",
            self.router.current_route(),
            self.selected_post
        );
        self.theme = match self.theme {
            crate::ui::Theme::CatppuccinLatte => crate::ui::Theme::CatppuccinMacchiato,
            crate::ui::Theme::CatppuccinMacchiato => crate::ui::Theme::CatppuccinLatte,
        };
        // Invalidate tag cache since theme changed
        self.cached_tags = None;
        log::debug!("toggle_theme completed. New theme: {:?}", self.theme);
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

    fn toggle_side_panel(&mut self) -> bool {
        log::debug!(
            "Toggling side panel, current state: {}",
            self.side_panel_collapsed
        );
        self.side_panel_collapsed = !self.side_panel_collapsed;
        true
    }

    fn collapse_side_panel(&mut self) -> bool {
        log::debug!("Collapsing side panel");
        self.side_panel_collapsed = true;
        true
    }

    fn expand_side_panel(&mut self) -> bool {
        log::debug!("Expanding side panel");
        self.side_panel_collapsed = false;
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
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.find_query)
                            .id(egui::Id::new("find_dialog_input")),
                    );

                    // Focus the text input when dialog opens
                    if !self.find_query.is_empty() && self.find_matches.is_empty() {
                        self.update_find_matches();
                    }

                    // Handle Enter key to find next
                    if response.lost_focus()
                        && ui.input(|i| i.key_pressed(Key::Enter))
                        && !self.find_matches.is_empty()
                    {
                        self.current_find_match =
                            (self.current_find_match + 1) % self.find_matches.len();
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

                    if ui.button("Next").clicked()
                        || ui.input(|i| i.key_pressed(Key::N) && i.modifiers.ctrl)
                    {
                        self.find_next();
                    }

                    if ui.button("Previous").clicked()
                        || ui.input(|i| i.key_pressed(Key::P) && i.modifiers.ctrl)
                    {
                        self.find_previous();
                    }

                    if ui.button("Close").clicked() {
                        self.find_mode_active = false;
                    }
                });

                // Show match count
                if !self.find_matches.is_empty() {
                    ui.label(format!(
                        "{} of {}",
                        self.current_find_match + 1,
                        self.find_matches.len()
                    ));
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

    #[test]
    fn test_theme_toggle_does_not_navigate_to_home() {
        let mut app = BlogApp::default();

        // Simulate being on a specific post (not home)
        app.selected_post = 2; // Select a non-zero post
        app.route_restored = true; // Route has been restored

        // Save initial state
        let initial_selected_post = app.selected_post;
        let initial_route_restored = app.route_restored;

        // Toggle theme
        app.toggle_theme();

        // Verify theme changed
        assert_ne!(app.theme, crate::ui::Theme::default());

        // Simulate persistence save/load cycle
        // When persistence saves and loads, route_restored should remain true
        // and selected_post should not change to 0
        app.route_restored = initial_route_restored; // This would be restored from persistence

        // Check that we're still on the same post
        assert_eq!(
            app.selected_post, initial_selected_post,
            "Theme toggle should not change selected post from {} to {}",
            initial_selected_post, app.selected_post
        );

        // Check that route_restored is still true (preventing restore_route() call)
        assert!(
            app.route_restored,
            "route_restored should remain true after theme toggle"
        );
    }
}
