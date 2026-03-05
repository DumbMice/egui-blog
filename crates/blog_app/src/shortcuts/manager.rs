//! Shortcut manager for handling keyboard shortcuts.

use crate::shortcuts::actions::ActionExecutor;
use crate::shortcuts::config::{KeySequence, ShortcutConfig, ShortcutContext};
use crate::shortcuts::context::ContextDetector;
use crate::shortcuts::sequences::KeySequenceHandler;
use egui::Context;

/// Manages keyboard shortcuts and their execution
pub struct ShortcutManager {
    /// Loaded configuration
    config: Option<ShortcutConfig>,
    /// Current active context
    active_context: ShortcutContext,
    /// Handler for Vim-style key sequences
    sequence_handler: KeySequenceHandler,
    /// Whether shortcuts are enabled
    enabled: bool,
    /// Last error message (if any)
    last_error: Option<String>,
}

impl ShortcutManager {
    /// Create a new shortcut manager
    pub fn new() -> Self {
        Self {
            config: None,
            active_context: ShortcutContext::Global,
            sequence_handler: KeySequenceHandler::new(1000), // Default timeout
            enabled: false,                                  // Disabled until config is loaded
            last_error: None,
        }
    }

    /// Load configuration (tries file first, falls back to embedded)
    pub fn load_config(&mut self) -> Result<(), String> {
        // Try to load from file first (for native targets with custom config)
        #[cfg(not(target_arch = "wasm32"))]
        {
            use crate::shortcuts::loader::load_shortcuts_config;

            match load_shortcuts_config() {
                Ok(config) => {
                    self.config = Some(config.clone());
                    self.sequence_handler = KeySequenceHandler::new(config.sequence_timeout_ms);
                    self.enabled = true;
                    self.last_error = None;
                    log::info!(
                        "Shortcut manager loaded from file with {} shortcuts",
                        config.shortcuts.len()
                    );
                    return Ok(());
                }
                Err(err) => {
                    // File loading failed, log but continue to try embedded
                    log::warn!(
                        "Failed to load shortcuts from file: {}, falling back to embedded",
                        err
                    );
                }
            }
        }

        // Fall back to embedded config (for WASM or when file loading fails)
        self.load_embedded_config()
    }

    /// Load embedded configuration
    fn load_embedded_config(&mut self) -> Result<(), String> {
        // Load embedded shortcuts.toml at compile time
        const SHORTCUTS_TOML: &str = include_str!("../../shortcuts.toml");

        match toml::from_str(SHORTCUTS_TOML) {
            Ok(config) => {
                self.config = Some(config);
                if let Some(ref config) = self.config {
                    self.sequence_handler = KeySequenceHandler::new(config.sequence_timeout_ms);
                    self.enabled = true;
                    self.last_error = None;
                    log::info!(
                        "Shortcut manager enabled with {} embedded shortcuts",
                        config.shortcuts.len()
                    );
                }
                Ok(())
            }
            Err(err) => {
                let err_msg = format!("Failed to parse embedded shortcuts.toml: {}", err);
                self.last_error = Some(err_msg.clone());
                self.enabled = false;
                Err(err_msg)
            }
        }
    }

    /// Update the active context based on UI state
    pub fn update_context<A: ActionExecutor + crate::shortcuts::context::ContextProvider>(
        &mut self,
        ctx: &Context,
        app: &A,
    ) {
        if !self.enabled {
            return;
        }

        self.active_context = ContextDetector::detect(ctx, app);
    }

    /// Handle keyboard input and execute shortcuts
    pub fn handle_input<A: ActionExecutor>(&mut self, ctx: &Context, app: &mut A) -> bool {
        if !self.enabled {
            return false;
        }

        // Get config and shortcuts before mutable operations
        let (shortcuts, contexts_enabled, active_context) = {
            let config = match &self.config {
                Some(config) => config,
                None => return false,
            };

            // Extract what we need
            (
                config.shortcuts.clone(),
                config.contexts_enabled.clone(),
                self.active_context,
            )
        };

        // Update sequence handler
        self.sequence_handler.update(ctx);

        // Check each shortcut
        for shortcut in &shortcuts {
            // Check if shortcut is active in current context
            if !shortcut.contexts.contains(&active_context) {
                continue;
            }

            // Check if context is enabled
            if let Some(false) = contexts_enabled.get(&active_context) {
                continue;
            }

            // Check primary keys
            for key_seq in &shortcut.keys {
                if self.check_sequence(ctx, key_seq) {
                    log::debug!(
                        "Shortcut sequence matched: {} - {:?}",
                        shortcut.name,
                        key_seq
                    );
                    return self.execute_shortcut(app, shortcut);
                }
            }

            // Check alternate keys
            for key_seq in &shortcut.alternate_keys {
                if self.check_sequence(ctx, key_seq) {
                    return self.execute_shortcut(app, shortcut);
                }
            }
        }

        false
    }

    /// Check if a key sequence matches current input
    fn check_sequence(&mut self, ctx: &Context, sequence: &KeySequence) -> bool {
        match sequence {
            KeySequence::Single(shortcut) => ctx.input_mut(|i| i.consume_shortcut(shortcut)),
            KeySequence::Sequence(seq) => self.sequence_handler.check_sequence(ctx, seq),
        }
    }

    /// Execute a shortcut action
    fn execute_shortcut<A: ActionExecutor>(
        &self,
        app: &mut A,
        shortcut: &crate::shortcuts::config::ShortcutDefinition,
    ) -> bool {
        log::debug!("Executing shortcut: {}", shortcut.name);
        app.execute_action(&shortcut.action)
    }

    /// Get whether shortcuts are enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get the last error message (if any)
    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    /// Get the active context
    pub fn active_context(&self) -> ShortcutContext {
        self.active_context
    }

    /// Get the configuration (if loaded)
    pub fn config(&self) -> Option<&ShortcutConfig> {
        self.config.as_ref()
    }

    /// Enable or disable shortcuts
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl Default for ShortcutManager {
    fn default() -> Self {
        Self::new()
    }
}
