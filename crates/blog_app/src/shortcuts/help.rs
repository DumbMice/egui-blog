//! Help overlay for keyboard shortcuts.

use crate::shortcuts::config::{ShortcutConfig, ShortcutContext};
use egui::{Color32, Context, RichText, Ui, Window};

/// Help overlay for displaying keyboard shortcuts
pub struct HelpOverlay {
    /// Whether the overlay is visible
    visible: bool,
    /// Filter by context
    filter_context: Option<ShortcutContext>,
}

impl HelpOverlay {
    /// Create a new help overlay
    pub fn new() -> Self {
        Self {
            visible: false,
            filter_context: None,
        }
    }

    /// Show the help overlay
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Hide the help overlay
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Toggle the help overlay visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Check if overlay is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Set filter context
    pub fn set_filter_context(&mut self, context: Option<ShortcutContext>) {
        self.filter_context = context;
    }

    /// Draw the help overlay
    pub fn draw(&mut self, ctx: &Context, config: &ShortcutConfig) {
        if !self.visible {
            return;
        }

        let mut visible = self.visible;
        Window::new("Keyboard Shortcuts Help")
            .open(&mut visible)
            .collapsible(false)
            .resizable(true)
            .default_width(600.0)
            .default_height(400.0)
            .show(ctx, |ui| {
                self.draw_content(ui, config);
            });
        self.visible = visible;
    }

    /// Draw the help content
    fn draw_content(&mut self, ui: &mut Ui, config: &ShortcutConfig) {
        // Header
        ui.heading("Keyboard Shortcuts");
        ui.separator();

        // Configuration info
        ui.horizontal(|ui| {
            ui.label("Configuration:");
            ui.monospace("shortcuts.toml");
            if config.vim_mode_enabled {
                ui.label(RichText::new("(Vim mode enabled)").color(Color32::GREEN));
            }
        });

        ui.separator();

        // Context filter buttons
        ui.horizontal_wrapped(|ui| {
            ui.label("Filter by context:");

            let all_selected = self.filter_context.is_none();
            if ui.selectable_label(all_selected, "All").clicked() {
                self.filter_context = None;
            }

            for context in [
                ShortcutContext::Global,
                ShortcutContext::LeftPanel,
                ShortcutContext::RightPanel,
                ShortcutContext::Search,
                ShortcutContext::Editor,
                ShortcutContext::FindMode,
            ] {
                let is_selected = self.filter_context == Some(context);
                if ui
                    .selectable_label(is_selected, context.display_name())
                    .clicked()
                {
                    self.filter_context = Some(context);
                }
            }
        });

        ui.separator();

        // Shortcuts table
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("shortcuts_grid")
                .num_columns(3)
                .striped(true)
                .spacing([10.0, 4.0])
                .show(ui, |ui| {
                    // Header
                    ui.strong("Shortcut");
                    ui.strong("Description");
                    ui.strong("Context");
                    ui.end_row();

                    // Shortcut rows
                    for shortcut in &config.shortcuts {
                        // Apply filter
                        if let Some(filter) = self.filter_context {
                            if !shortcut.contexts.contains(&filter) {
                                continue;
                            }
                        }

                        // Format keys
                        let keys_text = self.format_keys(&shortcut.keys);

                        // Format contexts
                        let contexts_text = shortcut
                            .contexts
                            .iter()
                            .map(|c| c.display_name())
                            .collect::<Vec<_>>()
                            .join(", ");

                        // Display row
                        ui.monospace(keys_text);
                        ui.label(&shortcut.description);
                        ui.label(contexts_text);
                        ui.end_row();
                    }
                });
        });

        ui.separator();

        // Footer
        ui.horizontal(|ui| {
            ui.label("Press ? to toggle this help");
            if ui.button("Close").clicked() {
                self.hide();
            }
        });
    }

    /// Format key sequences for display
    fn format_keys(&self, keys: &[crate::shortcuts::config::KeySequence]) -> String {
        use crate::shortcuts::config::KeySequence;

        let mut parts = Vec::new();

        for key_seq in keys {
            match key_seq {
                KeySequence::Single(shortcut) => {
                    parts.push(format_shortcut(shortcut));
                }
                KeySequence::Sequence(seq) => {
                    let seq_parts: Vec<_> = seq.iter().map(format_shortcut).collect();
                    parts.push(seq_parts.join(" then "));
                }
            }
        }

        parts.join(" or ")
    }
}

/// Format a single keyboard shortcut
fn format_shortcut(shortcut: &egui::KeyboardShortcut) -> String {
    use egui::Key;

    let mut parts = Vec::new();

    // Modifiers
    if shortcut.modifiers.ctrl || shortcut.modifiers.command {
        parts.push("Ctrl".to_string());
    }
    if shortcut.modifiers.shift {
        parts.push("Shift".to_string());
    }
    if shortcut.modifiers.alt {
        parts.push("Alt".to_string());
    }

    // Key
    let key_str = match shortcut.logical_key {
        Key::ArrowDown => "↓",
        Key::ArrowUp => "↑",
        Key::ArrowLeft => "←",
        Key::ArrowRight => "→",
        Key::Escape => "Esc",
        Key::Tab => "Tab",
        Key::Backspace => "Backspace",
        Key::Enter => "Enter",
        Key::Space => "Space",
        Key::Home => "Home",
        Key::End => "End",
        Key::PageUp => "PageUp",
        Key::PageDown => "PageDown",
        Key::Insert => "Insert",
        Key::Delete => "Delete",
        Key::F1 => "F1",
        Key::F2 => "F2",
        Key::F3 => "F3",
        Key::F4 => "F4",
        Key::F5 => "F5",
        Key::F6 => "F6",
        Key::F7 => "F7",
        Key::F8 => "F8",
        Key::F9 => "F9",
        Key::F10 => "F10",
        Key::F11 => "F11",
        Key::F12 => "F12",
        Key::Num0 => "0",
        Key::Num1 => "1",
        Key::Num2 => "2",
        Key::Num3 => "3",
        Key::Num4 => "4",
        Key::Num5 => "5",
        Key::Num6 => "6",
        Key::Num7 => "7",
        Key::Num8 => "8",
        Key::Num9 => "9",
        Key::A => "A",
        Key::B => "B",
        Key::C => "C",
        Key::D => "D",
        Key::E => "E",
        Key::F => "F",
        Key::G => "G",
        Key::H => "H",
        Key::I => "I",
        Key::J => "J",
        Key::K => "K",
        Key::L => "L",
        Key::M => "M",
        Key::N => "N",
        Key::O => "O",
        Key::P => "P",
        Key::Q => "Q",
        Key::R => "R",
        Key::S => "S",
        Key::T => "T",
        Key::U => "U",
        Key::V => "V",
        Key::W => "W",
        Key::X => "X",
        Key::Y => "Y",
        Key::Z => "Z",
        Key::Plus => "+",
        Key::Minus => "-",
        Key::Equals => "=",
        Key::OpenBracket => "[",
        Key::CloseBracket => "]",
        Key::Backslash => "\\",
        Key::Semicolon => ";",
        Key::Quote => "'",
        Key::Backtick => "`",
        Key::Comma => ",",
        Key::Period => ".",
        Key::Slash => "/",
        Key::Questionmark => "?",
        _ => {
            // Fallback for unknown keys
            "?"
        }
    };

    parts.push(key_str.to_string());
    parts.join("+")
}

impl Default for HelpOverlay {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Key, Modifiers};

    #[test]
    fn test_format_shortcut() {
        let shortcut = egui::KeyboardShortcut::new(Modifiers::CTRL, Key::S);
        assert_eq!(format_shortcut(&shortcut), "Ctrl+S");

        let shortcut = egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, Key::T);
        assert_eq!(format_shortcut(&shortcut), "Ctrl+Shift+T");

        let shortcut = egui::KeyboardShortcut::new(Modifiers::NONE, Key::ArrowDown);
        assert_eq!(format_shortcut(&shortcut), "↓");

        let shortcut = egui::KeyboardShortcut::new(Modifiers::NONE, Key::Slash);
        assert_eq!(format_shortcut(&shortcut), "/");
    }

    #[test]
    fn test_help_overlay_toggle() {
        let mut overlay = HelpOverlay::new();

        assert!(!overlay.is_visible());

        overlay.show();
        assert!(overlay.is_visible());

        overlay.hide();
        assert!(!overlay.is_visible());

        overlay.toggle();
        assert!(overlay.is_visible());

        overlay.toggle();
        assert!(!overlay.is_visible());
    }
}
