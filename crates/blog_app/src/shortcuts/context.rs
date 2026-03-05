//! Context detection for keyboard shortcuts.

use crate::shortcuts::config::{FocusedPanel, ShortcutContext};
use egui::Context;

/// Trait for apps that can provide context information
pub trait ContextProvider {
    /// Get the currently focused panel
    fn focused_panel(&self) -> FocusedPanel;

    /// Check if search bar has focus
    fn search_has_focus(&self, ctx: &Context) -> bool;

    /// Check if editor has focus
    fn editor_has_focus(&self, ctx: &Context) -> bool;

    /// Check if find mode is active
    fn find_mode_active(&self) -> bool;
}

/// Detects the current shortcut context
pub struct ContextDetector;

impl ContextDetector {
    /// Detect the current shortcut context
    pub fn detect<P: ContextProvider>(ctx: &Context, provider: &P) -> ShortcutContext {
        // Check for special modes first
        if provider.find_mode_active() {
            return ShortcutContext::FindMode;
        }

        if provider.search_has_focus(ctx) {
            return ShortcutContext::Search;
        }

        if provider.editor_has_focus(ctx) {
            return ShortcutContext::Editor;
        }

        // Check focused panel
        match provider.focused_panel() {
            FocusedPanel::LeftPanel => ShortcutContext::LeftPanel,
            FocusedPanel::RightPanel => ShortcutContext::RightPanel,
            FocusedPanel::None => ShortcutContext::Global,
        }
    }

    /// Simple detection based only on focused panel
    pub fn detect_from_panel(panel: FocusedPanel) -> ShortcutContext {
        match panel {
            FocusedPanel::LeftPanel => ShortcutContext::LeftPanel,
            FocusedPanel::RightPanel => ShortcutContext::RightPanel,
            FocusedPanel::None => ShortcutContext::Global,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestProvider {
        panel: FocusedPanel,
        search_focus: bool,
        editor_focus: bool,
        find_mode: bool,
    }

    impl ContextProvider for TestProvider {
        fn focused_panel(&self) -> FocusedPanel {
            self.panel
        }

        fn search_has_focus(&self, _ctx: &Context) -> bool {
            self.search_focus
        }

        fn editor_has_focus(&self, _ctx: &Context) -> bool {
            self.editor_focus
        }

        fn find_mode_active(&self) -> bool {
            self.find_mode
        }
    }

    #[test]
    fn test_detect_find_mode() {
        let ctx = Context::default();
        let provider = TestProvider {
            panel: FocusedPanel::LeftPanel,
            search_focus: false,
            editor_focus: false,
            find_mode: true,
        };

        let context = ContextDetector::detect(&ctx, &provider);
        assert_eq!(context, ShortcutContext::FindMode);
    }

    #[test]
    fn test_detect_search() {
        let ctx = Context::default();
        let provider = TestProvider {
            panel: FocusedPanel::LeftPanel,
            search_focus: true,
            editor_focus: false,
            find_mode: false,
        };

        let context = ContextDetector::detect(&ctx, &provider);
        assert_eq!(context, ShortcutContext::Search);
    }

    #[test]
    fn test_detect_left_panel() {
        let ctx = Context::default();
        let provider = TestProvider {
            panel: FocusedPanel::LeftPanel,
            search_focus: false,
            editor_focus: false,
            find_mode: false,
        };

        let context = ContextDetector::detect(&ctx, &provider);
        assert_eq!(context, ShortcutContext::LeftPanel);
    }

    #[test]
    fn test_detect_right_panel() {
        let ctx = Context::default();
        let provider = TestProvider {
            panel: FocusedPanel::RightPanel,
            search_focus: false,
            editor_focus: false,
            find_mode: false,
        };

        let context = ContextDetector::detect(&ctx, &provider);
        assert_eq!(context, ShortcutContext::RightPanel);
    }

    #[test]
    fn test_detect_global() {
        let ctx = Context::default();
        let provider = TestProvider {
            panel: FocusedPanel::None,
            search_focus: false,
            editor_focus: false,
            find_mode: false,
        };

        let context = ContextDetector::detect(&ctx, &provider);
        assert_eq!(context, ShortcutContext::Global);
    }
}
