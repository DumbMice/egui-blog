//! Configuration file loading for keyboard shortcuts.

use crate::shortcuts::config::{default_shortcuts, ShortcutConfig};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Error type for config loading failures
#[derive(Debug, thiserror::Error)]
pub enum ShortcutConfigError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("Config file not found: {0}")]
    NotFound(String),
    #[error("Invalid config: {0}")]
    Invalid(String),
}

/// Find the shortcuts.toml file by searching up from current directory
/// Stops at project root (where Cargo.toml is found)
pub fn find_shortcuts_toml() -> Result<PathBuf, ShortcutConfigError> {
    let mut current_dir = std::env::current_dir().map_err(ShortcutConfigError::Io)?;

    loop {
        let shortcuts_path = current_dir.join("shortcuts.toml");
        let cargo_toml_path = current_dir.join("Cargo.toml");

        // Check if we're at project root (has Cargo.toml)
        if cargo_toml_path.exists() {
            if shortcuts_path.exists() {
                return Ok(shortcuts_path);
            } else {
                // At project root but no shortcuts.toml - create default
                return create_default_config(&shortcuts_path);
            }
        }

        // Move up one directory
        if !current_dir.pop() {
            return Err(ShortcutConfigError::NotFound(
                "shortcuts.toml not found and no Cargo.toml to determine project root".to_string(),
            ));
        }
    }
}

/// Create a default shortcuts.toml file
fn create_default_config(path: &Path) -> Result<PathBuf, ShortcutConfigError> {
    let default_config = generate_default_toml();
    fs::write(path, default_config).map_err(ShortcutConfigError::Io)?;
    log::info!("Created default shortcuts.toml at {}", path.display());
    Ok(path.to_path_buf())
}

/// Generate default TOML config content
fn generate_default_toml() -> String {
    let config = ShortcutConfig {
        version: "1.0".to_string(),
        vim_mode_enabled: true,
        sequence_timeout_ms: 1000,
        save_focus_state: true,
        default_focused_panel: crate::shortcuts::config::FocusedPanel::LeftPanel,
        shortcuts: default_shortcuts(),
        contexts_enabled: std::collections::BTreeMap::new(),
    };

    // Convert to TOML
    toml::to_string_pretty(&config).unwrap_or_else(|e| {
        log::error!("Failed to generate default TOML: {}", e);
        fallback_default_toml()
    })
}

/// Fallback default TOML if serialization fails
fn fallback_default_toml() -> String {
    r#"# Keyboard Shortcuts Configuration
# Required file - no shortcuts work without this file
version = "1.0"
vim_mode_enabled = true
sequence_timeout_ms = 1000
save_focus_state = true
default_focused_panel = "LeftPanel"

# Shortcut definitions
[[shortcuts]]
name = "focus_left_panel"
description = "Focus left panel (post list)"
contexts = ["Global"]
keys = [{ modifiers = ["Control"], key = "h" }]
action = { type = "FocusPanel", panel = "LeftPanel" }

[[shortcuts]]
name = "focus_right_panel"
description = "Focus right panel (content)"
contexts = ["Global"]
keys = [{ modifiers = ["Control"], key = "l" }]
action = { type = "FocusPanel", panel = "RightPanel" }

[[shortcuts]]
name = "next_post"
description = "Select next post"
contexts = ["LeftPanel"]
keys = [
    { modifiers = [], key = "ArrowDown" },
    { modifiers = [], key = "j" }
]
action = { type = "NavigatePost", direction = "Next" }

[[shortcuts]]
name = "previous_post"
description = "Select previous post"
contexts = ["LeftPanel"]
keys = [
    { modifiers = [], key = "ArrowUp" },
    { modifiers = [], key = "k" }
]
action = { type = "NavigatePost", direction = "Previous" }

[[shortcuts]]
name = "first_post"
description = "Navigate to first post"
contexts = ["LeftPanel", "RightPanel"]
keys = [
    { modifiers = [], key = "Home" },
    { modifiers = [], sequence = ["g", "g"] }
]
action = { type = "NavigatePost", direction = "First" }

[[shortcuts]]
name = "last_post"
description = "Navigate to last post"
contexts = ["LeftPanel", "RightPanel"]
keys = [
    { modifiers = [], key = "End" },
    { modifiers = ["Shift"], key = "g" }
]
action = { type = "NavigatePost", direction = "Last" }

[[shortcuts]]
name = "switch_tab_left"
description = "Switch to previous content tab"
contexts = ["LeftPanel"]
keys = [
    { modifiers = [], key = "ArrowLeft" },
    { modifiers = [], key = "h" }
]
action = { type = "SwitchTab", direction = "Previous" }

[[shortcuts]]
name = "switch_tab_right"
description = "Switch to next content tab"
contexts = ["LeftPanel"]
keys = [
    { modifiers = [], key = "ArrowRight" },
    { modifiers = [], key = "l" }
]
action = { type = "SwitchTab", direction = "Next" }

[[shortcuts]]
name = "scroll_down"
description = "Scroll down small step"
contexts = ["RightPanel"]
keys = [{ modifiers = [], key = "j" }]
action = { type = "Scroll", direction = "Down", amount = "Small" }

[[shortcuts]]
name = "scroll_up"
description = "Scroll up small step"
contexts = ["RightPanel"]
keys = [{ modifiers = [], key = "k" }]
action = { type = "Scroll", direction = "Up", amount = "Small" }

[[shortcuts]]
name = "scroll_half_page_down"
description = "Scroll down half page"
contexts = ["RightPanel"]
keys = [{ modifiers = ["Control"], key = "d" }]
action = { type = "Scroll", direction = "Down", amount = "HalfPage" }

[[shortcuts]]
name = "scroll_half_page_up"
description = "Scroll up half page"
contexts = ["RightPanel"]
keys = [{ modifiers = ["Control"], key = "u" }]
action = { type = "Scroll", direction = "Up", amount = "HalfPage" }

[[shortcuts]]
name = "focus_search"
description = "Focus search bar"
contexts = ["Global", "LeftPanel"]
keys = [
    { modifiers = ["Control"], key = "k" },
    { modifiers = [], key = "/" }
]
action = { type = "FocusSearch" }

[[shortcuts]]
name = "find_in_content"
description = "Find text in current post"
contexts = ["RightPanel"]
keys = [
    { modifiers = ["Control"], key = "f" },
    { modifiers = [], key = "/" }
]
action = { type = "FindInContent" }

[[shortcuts]]
name = "next_match"
description = "Next find match"
contexts = ["FindMode"]
keys = [{ modifiers = [], key = "n" }]
action = { type = "FindNext" }

[[shortcuts]]
name = "previous_match"
description = "Previous find match"
contexts = ["FindMode"]
keys = [{ modifiers = ["Shift"], key = "n" }]
action = { type = "FindPrevious" }

[[shortcuts]]
name = "toggle_theme"
description = "Toggle between light and dark themes"
contexts = ["Global"]
keys = [{ modifiers = ["Control"], key = "t" }]
action = { type = "ToggleTheme" }

[[shortcuts]]
name = "show_help"
description = "Show keyboard shortcuts help"
contexts = ["Global"]
keys = [{ modifiers = [], key = "?" }]
action = { type = "ShowHelp" }

[[shortcuts]]
name = "browser_address"
description = "Focus browser address bar (web only)"
contexts = ["Global"]
keys = [{ modifiers = ["Alt"], key = "d" }]
action = { type = "BrowserAddress" }
"#
    .to_string()
}

/// Load shortcuts configuration from file
pub fn load_shortcuts_config() -> Result<ShortcutConfig, ShortcutConfigError> {
    let config_path = find_shortcuts_toml()?;

    log::info!("Loading shortcuts config from {}", config_path.display());

    let config_str = fs::read_to_string(&config_path).map_err(ShortcutConfigError::Io)?;
    let mut config: ShortcutConfig =
        toml::from_str(&config_str).map_err(ShortcutConfigError::Toml)?;

    // Validate config
    validate_config(&config)?;

    // Ensure all contexts are enabled by default
    use crate::shortcuts::config::ShortcutContext::*;
    let all_contexts = [Global, LeftPanel, RightPanel, Search, Editor, FindMode];
    for context in all_contexts {
        config.contexts_enabled.entry(context).or_insert(true);
    }

    log::info!("Loaded {} shortcuts", config.shortcuts.len());
    Ok(config)
}

/// Validate the loaded configuration
fn validate_config(config: &ShortcutConfig) -> Result<(), ShortcutConfigError> {
    // Check version
    if config.version != "1.0" {
        return Err(ShortcutConfigError::Invalid(format!(
            "Unsupported config version: {}",
            config.version
        )));
    }

    // Check sequence timeout
    if config.sequence_timeout_ms == 0 {
        return Err(ShortcutConfigError::Invalid(
            "sequence_timeout_ms must be greater than 0".to_string(),
        ));
    }

    // Check for duplicate shortcut names
    let mut names = std::collections::HashSet::new();
    for shortcut in &config.shortcuts {
        if !names.insert(&shortcut.name) {
            return Err(ShortcutConfigError::Invalid(format!(
                "Duplicate shortcut name: {}",
                shortcut.name
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    #[ignore = "Changes global current directory, causes issues with parallel test execution"]
    fn test_find_shortcuts_toml_creates_default() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        // Create Cargo.toml to simulate project root
        let cargo_toml = temp_dir.path().join("Cargo.toml");
        fs::write(&cargo_toml, "[package]\nname = \"test\"").unwrap();

        // Should create default shortcuts.toml
        let result = find_shortcuts_toml();
        assert!(result.is_ok());

        let shortcuts_path = result.unwrap();
        assert!(shortcuts_path.exists());

        // Verify it's valid TOML
        let config_str = fs::read_to_string(&shortcuts_path).unwrap();
        let config: ShortcutConfig = toml::from_str(&config_str).unwrap();
        assert_eq!(config.version, "1.0");
        assert!(config.vim_mode_enabled);
    }

    #[test]
    #[ignore = "Changes global current directory, causes issues with parallel test execution"]
    fn test_load_shortcuts_config_valid() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        // Create Cargo.toml and valid shortcuts.toml
        let cargo_toml = temp_dir.path().join("Cargo.toml");
        fs::write(&cargo_toml, "[package]\nname = \"test\"").unwrap();

        let shortcuts_toml = temp_dir.path().join("shortcuts.toml");
        let valid_config = r#"
version = "1.0"
vim_mode_enabled = true
sequence_timeout_ms = 1000
save_focus_state = true
default_focused_panel = "LeftPanel"

[[shortcuts]]
name = "test_shortcut"
description = "Test shortcut"
contexts = ["Global"]
keys = [{ modifiers = ["Control"], key = "a" }]
action = { type = "Custom", name = "test" }
"#;
        fs::write(&shortcuts_toml, valid_config).unwrap();

        let result = load_shortcuts_config();
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.version, "1.0");
        assert_eq!(config.shortcuts.len(), 1);
        assert_eq!(config.shortcuts[0].name, "test_shortcut");
    }

    #[test]
    #[ignore = "Changes global current directory, causes issues with parallel test execution"]
    fn test_load_shortcuts_config_invalid_version() {
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        let cargo_toml = temp_dir.path().join("Cargo.toml");
        fs::write(&cargo_toml, "[package]\nname = \"test\"").unwrap();

        let shortcuts_toml = temp_dir.path().join("shortcuts.toml");
        let invalid_config = r#"
version = "2.0"  # Invalid version
vim_mode_enabled = true
sequence_timeout_ms = 1000
save_focus_state = true
default_focused_panel = "LeftPanel"
shortcuts = []
"#;
        fs::write(&shortcuts_toml, invalid_config).unwrap();

        let result = load_shortcuts_config();
        assert!(result.is_err());

        if let Err(ShortcutConfigError::Invalid(msg)) = result {
            assert!(msg.contains("Unsupported config version"));
        } else {
            panic!("Expected Invalid error");
        }
    }
}
