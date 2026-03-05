//! Integration helpers for `BlogApp`.

use crate::shortcuts::{
    ActionExecutor, ContextProvider, FocusedPanel, HelpOverlay, PostNavigation, ScrollAmount,
    ScrollDirection, ShortcutConfig, ShortcutContext, ShortcutManager, TabDirection,
};

/// Integration helper for `BlogApp`
pub struct ShortcutIntegration {
    /// Shortcut manager
    pub manager: ShortcutManager,
    /// Help overlay
    pub help_overlay: HelpOverlay,
    /// Whether shortcuts are initialized
    pub initialized: bool,
}

impl ShortcutIntegration {
    /// Create new integration
    pub fn new() -> Self {
        Self {
            manager: ShortcutManager::new(),
            help_overlay: HelpOverlay::new(),
            initialized: false,
        }
    }

    /// Initialize shortcuts (load config)
    ///
    /// # Errors
    /// Returns an error string if shortcut configuration fails to load
    pub fn initialize(&mut self) -> Result<(), String> {
        if self.initialized {
            return Ok(());
        }

        match self.manager.load_config() {
            Ok(()) => {
                self.initialized = true;
                log::info!("Shortcuts initialized successfully");
                Ok(())
            }
            Err(err) => {
                log::error!("Failed to initialize shortcuts: {err}");
                Err(err)
            }
        }
    }

    /// Update context and handle input
    pub fn update<A>(&mut self, ctx: &egui::Context, app: &mut A) -> bool
    where
        A: ActionExecutor + ContextProvider,
    {
        if !self.initialized {
            return false;
        }

        // Update context
        self.manager.update_context(ctx, app);

        // Handle input
        let handled = self.manager.handle_input(ctx, app);

        // Draw help overlay if needed
        if self.help_overlay.is_visible()
            && let Some(config) = self.manager.config() {
                self.help_overlay.draw(ctx, config);
            }

        handled
    }

    /// Show help overlay
    pub fn show_help(&mut self) {
        self.help_overlay.show();
    }

    /// Get whether shortcuts are enabled
    pub fn is_enabled(&self) -> bool {
        self.manager.is_enabled()
    }

    /// Get the active context
    pub fn active_context(&self) -> ShortcutContext {
        self.manager.active_context()
    }

    /// Get the configuration (if loaded)
    pub fn config(&self) -> Option<&ShortcutConfig> {
        self.manager.config()
    }
}

impl Default for ShortcutIntegration {
    fn default() -> Self {
        Self::new()
    }
}

/// Default implementation for testing
pub struct TestIntegration {
    pub integration: ShortcutIntegration,
    pub focused_panel: FocusedPanel,
    pub search_focus: bool,
    pub editor_focus: bool,
    pub find_mode: bool,
    pub actions_log: Vec<String>,
}

impl Default for TestIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl TestIntegration {
    pub fn new() -> Self {
        Self {
            integration: ShortcutIntegration::new(),
            focused_panel: FocusedPanel::None,
            search_focus: false,
            editor_focus: false,
            find_mode: false,
            actions_log: Vec::new(),
        }
    }
}

impl ContextProvider for TestIntegration {
    fn focused_panel(&self) -> FocusedPanel {
        self.focused_panel
    }

    fn search_has_focus(&self, _ctx: &egui::Context) -> bool {
        self.search_focus
    }

    fn editor_has_focus(&self, _ctx: &egui::Context) -> bool {
        self.editor_focus
    }

    fn find_mode_active(&self) -> bool {
        self.find_mode
    }
}

impl ActionExecutor for TestIntegration {
    fn execute_action(&mut self, action: &crate::shortcuts::config::ShortcutAction) -> bool {
        self.actions_log
            .push(format!("execute_action: {action:?}"));
        true
    }

    fn navigate_post(&mut self, navigation: PostNavigation) -> bool {
        self.actions_log
            .push(format!("navigate_post: {navigation:?}"));
        true
    }

    fn switch_tab(&mut self, direction: TabDirection) -> bool {
        self.actions_log
            .push(format!("switch_tab: {direction:?}"));
        true
    }

    fn scroll(&mut self, direction: ScrollDirection, amount: ScrollAmount) -> bool {
        self.actions_log
            .push(format!("scroll: {direction:?} {amount:?}"));
        true
    }

    fn focus_panel(&mut self, panel: FocusedPanel) -> bool {
        self.actions_log.push(format!("focus_panel: {panel:?}"));
        true
    }

    fn focus_search(&mut self) -> bool {
        self.actions_log.push("focus_search".to_owned());
        true
    }

    fn find_in_content(&mut self) -> bool {
        self.actions_log.push("find_in_content".to_owned());
        true
    }

    fn find_next(&mut self) -> bool {
        self.actions_log.push("find_next".to_owned());
        true
    }

    fn find_previous(&mut self) -> bool {
        self.actions_log.push("find_previous".to_owned());
        true
    }

    fn toggle_theme(&mut self) -> bool {
        self.actions_log.push("toggle_theme".to_owned());
        true
    }

    fn show_help(&mut self) -> bool {
        self.actions_log.push("show_help".to_owned());
        self.integration.show_help();
        true
    }

    fn browser_address(&mut self) -> bool {
        self.actions_log.push("browser_address".to_owned());
        true
    }

    fn execute_custom(&mut self, action: &str) -> bool {
        self.actions_log.push(format!("execute_custom: {action}"));
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_creation() {
        let integration = ShortcutIntegration::new();
        assert!(!integration.initialized);
        assert!(!integration.is_enabled());
    }

    #[test]
    fn test_test_integration() {
        let mut test = TestIntegration::new();
        let ctx = egui::Context::default();

        // Test context provider
        test.focused_panel = FocusedPanel::LeftPanel;
        assert_eq!(test.focused_panel(), FocusedPanel::LeftPanel);

        // Test action executor
        assert!(test.navigate_post(PostNavigation::Next));
        assert_eq!(test.actions_log.len(), 1);
        assert!(test.actions_log[0].contains("navigate_post"));
    }
}
