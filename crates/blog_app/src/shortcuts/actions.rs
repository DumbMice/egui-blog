//! Action execution for keyboard shortcuts.

use crate::shortcuts::config::{
    FocusedPanel, PostNavigation, ScrollAmount, ScrollDirection, ShortcutAction, TabDirection,
};

/// Trait for executing shortcut actions
pub trait ActionExecutor {
    /// Execute a shortcut action
    fn execute_action(&mut self, action: &ShortcutAction) -> bool;

    /// Navigate between posts
    fn navigate_post(&mut self, navigation: PostNavigation) -> bool;

    /// Switch content type tabs
    fn switch_tab(&mut self, direction: TabDirection) -> bool;

    /// Scroll content
    fn scroll(&mut self, direction: ScrollDirection, amount: ScrollAmount) -> bool;

    /// Focus a panel
    fn focus_panel(&mut self, panel: FocusedPanel) -> bool;

    /// Focus search bar
    fn focus_search(&mut self) -> bool;

    /// Find text in current post
    fn find_in_content(&mut self) -> bool;

    /// Navigate to next/previous find match
    fn find_next(&mut self) -> bool;
    fn find_previous(&mut self) -> bool;

    /// Toggle theme
    fn toggle_theme(&mut self) -> bool;

    /// Show help overlay
    fn show_help(&mut self) -> bool;

    /// Focus browser address bar (web only)
    fn browser_address(&mut self) -> bool;

    /// Execute custom action
    fn execute_custom(&mut self, action: &str) -> bool;
}

/// Default implementation for testing
pub struct TestExecutor {
    pub actions_log: Vec<String>,
}

impl TestExecutor {
    pub fn new() -> Self {
        Self {
            actions_log: Vec::new(),
        }
    }
}

impl ActionExecutor for TestExecutor {
    fn execute_action(&mut self, action: &ShortcutAction) -> bool {
        self.actions_log.push(format!("{:?}", action));
        true
    }

    fn navigate_post(&mut self, navigation: PostNavigation) -> bool {
        self.actions_log
            .push(format!("navigate_post: {:?}", navigation));
        true
    }

    fn switch_tab(&mut self, direction: TabDirection) -> bool {
        self.actions_log
            .push(format!("switch_tab: {:?}", direction));
        true
    }

    fn scroll(&mut self, direction: ScrollDirection, amount: ScrollAmount) -> bool {
        self.actions_log
            .push(format!("scroll: {:?} {:?}", direction, amount));
        true
    }

    fn focus_panel(&mut self, panel: FocusedPanel) -> bool {
        self.actions_log.push(format!("focus_panel: {:?}", panel));
        true
    }

    fn focus_search(&mut self) -> bool {
        self.actions_log.push("focus_search".to_string());
        true
    }

    fn find_in_content(&mut self) -> bool {
        self.actions_log.push("find_in_content".to_string());
        true
    }

    fn find_next(&mut self) -> bool {
        self.actions_log.push("find_next".to_string());
        true
    }

    fn find_previous(&mut self) -> bool {
        self.actions_log.push("find_previous".to_string());
        true
    }

    fn toggle_theme(&mut self) -> bool {
        self.actions_log.push("toggle_theme".to_string());
        true
    }

    fn show_help(&mut self) -> bool {
        self.actions_log.push("show_help".to_string());
        true
    }

    fn browser_address(&mut self) -> bool {
        self.actions_log.push("browser_address".to_string());
        true
    }

    fn execute_custom(&mut self, action: &str) -> bool {
        self.actions_log.push(format!("custom: {}", action));
        true
    }
}

/// Helper to execute actions with logging
pub struct LoggingExecutor<T: ActionExecutor> {
    inner: T,
    log: Vec<String>,
}

impl<T: ActionExecutor> LoggingExecutor<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            log: Vec::new(),
        }
    }

    pub fn log(&self) -> &[String] {
        &self.log
    }
}

impl<T: ActionExecutor> ActionExecutor for LoggingExecutor<T> {
    fn execute_action(&mut self, action: &ShortcutAction) -> bool {
        self.log.push(format!("Executing: {:?}", action));
        self.inner.execute_action(action)
    }

    fn navigate_post(&mut self, navigation: PostNavigation) -> bool {
        self.log.push(format!("Navigating post: {:?}", navigation));
        self.inner.navigate_post(navigation)
    }

    fn switch_tab(&mut self, direction: TabDirection) -> bool {
        self.log.push(format!("Switching tab: {:?}", direction));
        self.inner.switch_tab(direction)
    }

    fn scroll(&mut self, direction: ScrollDirection, amount: ScrollAmount) -> bool {
        self.log
            .push(format!("Scrolling: {:?} {:?}", direction, amount));
        self.inner.scroll(direction, amount)
    }

    fn focus_panel(&mut self, panel: FocusedPanel) -> bool {
        self.log.push(format!("Focusing panel: {:?}", panel));
        self.inner.focus_panel(panel)
    }

    fn focus_search(&mut self) -> bool {
        self.log.push("Focusing search".to_string());
        self.inner.focus_search()
    }

    fn find_in_content(&mut self) -> bool {
        self.log.push("Finding in content".to_string());
        self.inner.find_in_content()
    }

    fn find_next(&mut self) -> bool {
        self.log.push("Finding next match".to_string());
        self.inner.find_next()
    }

    fn find_previous(&mut self) -> bool {
        self.log.push("Finding previous match".to_string());
        self.inner.find_previous()
    }

    fn toggle_theme(&mut self) -> bool {
        self.log.push("Toggling theme".to_string());
        self.inner.toggle_theme()
    }

    fn show_help(&mut self) -> bool {
        self.log.push("Showing help".to_string());
        self.inner.show_help()
    }

    fn browser_address(&mut self) -> bool {
        self.log.push("Focusing browser address".to_string());
        self.inner.browser_address()
    }

    fn execute_custom(&mut self, action: &str) -> bool {
        self.log.push(format!("Executing custom: {}", action));
        self.inner.execute_custom(action)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_executor() {
        let mut executor = TestExecutor::new();

        assert!(executor.navigate_post(PostNavigation::Next));
        assert!(executor.switch_tab(TabDirection::Next));
        assert!(executor.scroll(ScrollDirection::Down, ScrollAmount::Small));
        assert!(executor.focus_panel(FocusedPanel::LeftPanel));
        assert!(executor.focus_search());
        assert!(executor.find_in_content());
        assert!(executor.find_next());
        assert!(executor.find_previous());
        assert!(executor.toggle_theme());
        assert!(executor.show_help());
        assert!(executor.browser_address());
        assert!(executor.execute_custom("test"));

        assert_eq!(executor.actions_log.len(), 12);
    }

    #[test]
    fn test_logging_executor() {
        let inner = TestExecutor::new();
        let mut executor = LoggingExecutor::new(inner);

        executor.navigate_post(PostNavigation::Next);
        executor.toggle_theme();

        assert_eq!(executor.log().len(), 2);
        assert!(executor.log()[0].contains("Navigating post"));
        assert!(executor.log()[1].contains("Toggling theme"));
    }
}
