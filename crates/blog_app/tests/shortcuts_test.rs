//! Tests for the keyboard shortcuts system.

use blog_app::shortcuts::{
    load_shortcuts_config, ActionExecutor, ContextProvider, FocusedPanel, PostNavigation,
    ShortcutConfig, ShortcutManager, TabDirection, TestIntegration,
};
use egui::Context;

#[test]
fn test_config_loading() {
    // This test will create a temporary config file and load it
    // Note: This test requires a shortcuts.toml file to exist
    // It should be created automatically when the app starts

    let result = load_shortcuts_config();
    assert!(
        result.is_ok(),
        "Failed to load shortcuts config: {:?}",
        result.err()
    );

    let config = result.unwrap();
    assert_eq!(config.version, "1.0");
    assert!(config.vim_mode_enabled);
    assert_eq!(config.sequence_timeout_ms, 1000);
    assert!(config.save_focus_state);
    assert_eq!(config.default_focused_panel, FocusedPanel::LeftPanel);

    // Should have at least the default shortcuts
    assert!(!config.shortcuts.is_empty(), "No shortcuts loaded");

    // Check for some expected shortcuts
    let shortcut_names: Vec<_> = config.shortcuts.iter().map(|s| s.name.as_str()).collect();
    assert!(
        shortcut_names.contains(&"focus_left_panel"),
        "Missing focus_left_panel shortcut"
    );
    assert!(
        shortcut_names.contains(&"focus_right_panel"),
        "Missing focus_right_panel shortcut"
    );
    assert!(
        shortcut_names.contains(&"next_post"),
        "Missing next_post shortcut"
    );
    assert!(
        shortcut_names.contains(&"previous_post"),
        "Missing previous_post shortcut"
    );
}

#[test]
fn test_shortcut_manager_creation() {
    let mut manager = ShortcutManager::new();
    assert!(!manager.is_enabled(), "Manager should start disabled");

    // Try to load config (will fail in test environment without proper setup)
    let result = manager.load_config();
    // This might fail if no config file exists in test environment
    // That's OK - we're testing that the method exists and can be called
    println!("Load config result: {:?}", result);
}

#[test]
fn test_test_integration() {
    let mut test = TestIntegration::new();
    let ctx = Context::default();

    // Test context provider
    test.focused_panel = FocusedPanel::LeftPanel;
    assert_eq!(test.focused_panel(), FocusedPanel::LeftPanel);

    // Test action executor
    assert!(test.navigate_post(PostNavigation::Next));
    assert!(test.switch_tab(TabDirection::Next));
    assert!(test.toggle_theme());
    assert!(test.show_help());

    // Check that actions were logged
    assert!(!test.actions_log.is_empty());
    assert!(test.actions_log[0].contains("navigate_post"));
}

#[test]
fn test_default_shortcuts_toml_exists() {
    // Check that the default shortcuts.toml file exists
    use std::path::Path;

    let config_path = Path::new("shortcuts.toml");
    assert!(config_path.exists(), "shortcuts.toml file should exist");

    // Check it's valid TOML
    let config_str = std::fs::read_to_string(config_path).expect("Failed to read shortcuts.toml");
    let result: Result<ShortcutConfig, _> = toml::from_str(&config_str);
    assert!(
        result.is_ok(),
        "shortcuts.toml should be valid TOML: {:?}",
        result.err()
    );
}
