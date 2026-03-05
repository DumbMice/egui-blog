//! Configuration structures for keyboard shortcuts.

use egui::{Key, KeyboardShortcut, Modifiers};
use std::collections::BTreeMap;

/// Context in which a shortcut is active
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ShortcutContext {
    /// Always active (regardless of focus)
    Global,
    /// When left panel (post list) has focus
    LeftPanel,
    /// When right panel (content area) has focus
    RightPanel,
    /// When search bar has focus
    Search,
    /// When editor has focus
    Editor,
    /// When find-in-content mode is active
    FindMode,
}

impl ShortcutContext {
    /// Get display name for the context
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Global => "Global",
            Self::LeftPanel => "Left Panel",
            Self::RightPanel => "Right Panel",
            Self::Search => "Search",
            Self::Editor => "Editor",
            Self::FindMode => "Find Mode",
        }
    }
}

/// Which panel currently has focus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FocusedPanel {
    /// Left panel (post list)
    LeftPanel,
    /// Right panel (content area)
    RightPanel,
    /// No panel focused
    #[default]
    None,
}

impl FocusedPanel {
    /// Convert to ShortcutContext
    pub fn to_context(&self) -> Option<ShortcutContext> {
        match self {
            Self::LeftPanel => Some(ShortcutContext::LeftPanel),
            Self::RightPanel => Some(ShortcutContext::RightPanel),
            Self::None => None,
        }
    }
}

/// Direction for post navigation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PostNavigation {
    /// Next post
    Next,
    /// Previous post
    Previous,
    /// First post
    First,
    /// Last post
    Last,
}

/// Direction for tab switching
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TabDirection {
    /// Next tab (right)
    Next,
    /// Previous tab (left)
    Previous,
}

/// Direction for scrolling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ScrollDirection {
    /// Scroll up
    Up,
    /// Scroll down
    Down,
}

/// A sequence of keys (for Vim-style sequences like "gg")
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "KeySequenceDeser", into = "KeySequenceDeser")
)]
pub enum KeySequence {
    /// Single key combination
    Single(KeyboardShortcut),
    /// Sequence of key presses (e.g., ["g", "g"])
    Sequence(Vec<KeyboardShortcut>),
}

/// Helper struct for deserializing KeySequence
#[cfg(feature = "serde")]
#[derive(serde::Deserialize, serde::Serialize)]
struct KeySequenceDeser {
    modifiers: Vec<String>,
    #[serde(default)]
    key: Option<String>,
    #[serde(default)]
    sequence: Vec<String>,
}

#[cfg(feature = "serde")]
impl TryFrom<KeySequenceDeser> for KeySequence {
    type Error = String;

    fn try_from(value: KeySequenceDeser) -> Result<Self, Self::Error> {
        // Parse modifiers
        let mut modifiers = Modifiers::NONE;
        for mod_str in &value.modifiers {
            match mod_str.as_str() {
                "Control" | "Ctrl" => modifiers |= Modifiers::CTRL,
                "Shift" => modifiers |= Modifiers::SHIFT,
                "Alt" => modifiers |= Modifiers::ALT,
                "Command" | "Cmd" | "Super" => modifiers |= Modifiers::COMMAND,
                _ => return Err(format!("Unknown modifier: {}", mod_str)),
            }
        }

        if let Some(key_str) = value.key {
            // Single key
            let key = parse_key(&key_str)?;
            Ok(KeySequence::Single(KeyboardShortcut::new(modifiers, key)))
        } else if !value.sequence.is_empty() {
            // Key sequence
            let mut sequence = Vec::new();
            for key_str in &value.sequence {
                let key = parse_key(key_str)?;
                // For sequences, typically no modifiers on individual keys
                sequence.push(KeyboardShortcut::new(Modifiers::NONE, key));
            }
            Ok(KeySequence::Sequence(sequence))
        } else {
            Err("KeySequence must have either 'key' or 'sequence' field".to_string())
        }
    }
}

#[cfg(feature = "serde")]
impl From<KeySequence> for KeySequenceDeser {
    fn from(value: KeySequence) -> Self {
        match value {
            KeySequence::Single(shortcut) => {
                let mut modifiers = Vec::new();
                if shortcut.modifiers.ctrl {
                    modifiers.push("Control".to_string());
                }
                if shortcut.modifiers.shift {
                    modifiers.push("Shift".to_string());
                }
                if shortcut.modifiers.alt {
                    modifiers.push("Alt".to_string());
                }
                if shortcut.modifiers.command {
                    modifiers.push("Command".to_string());
                }

                KeySequenceDeser {
                    modifiers,
                    key: Some(key_to_string(shortcut.logical_key)),
                    sequence: Vec::new(),
                }
            }
            KeySequence::Sequence(shortcuts) => {
                let sequence = shortcuts
                    .iter()
                    .map(|s| key_to_string(s.logical_key))
                    .collect();

                KeySequenceDeser {
                    modifiers: Vec::new(), // Sequences don't have global modifiers
                    key: None,
                    sequence,
                }
            }
        }
    }
}

#[cfg(feature = "serde")]
fn parse_key(key_str: &str) -> Result<Key, String> {
    use egui::Key;

    match key_str {
        "ArrowDown" => Ok(Key::ArrowDown),
        "ArrowUp" => Ok(Key::ArrowUp),
        "ArrowLeft" => Ok(Key::ArrowLeft),
        "ArrowRight" => Ok(Key::ArrowRight),
        "Home" => Ok(Key::Home),
        "End" => Ok(Key::End),
        "PageUp" => Ok(Key::PageUp),
        "PageDown" => Ok(Key::PageDown),
        "Backspace" => Ok(Key::Backspace),
        "Delete" => Ok(Key::Delete),
        "Insert" => Ok(Key::Insert),
        "Escape" => Ok(Key::Escape),
        "Enter" => Ok(Key::Enter),
        "Tab" => Ok(Key::Tab),
        "Space" => Ok(Key::Space),
        "F1" => Ok(Key::F1),
        "F2" => Ok(Key::F2),
        "F3" => Ok(Key::F3),
        "F4" => Ok(Key::F4),
        "F5" => Ok(Key::F5),
        "F6" => Ok(Key::F6),
        "F7" => Ok(Key::F7),
        "F8" => Ok(Key::F8),
        "F9" => Ok(Key::F9),
        "F10" => Ok(Key::F10),
        "F11" => Ok(Key::F11),
        "F12" => Ok(Key::F12),
        "A" | "a" => Ok(Key::A),
        "B" | "b" => Ok(Key::B),
        "C" | "c" => Ok(Key::C),
        "D" | "d" => Ok(Key::D),
        "E" | "e" => Ok(Key::E),
        "F" | "f" => Ok(Key::F),
        "G" | "g" => Ok(Key::G),
        "H" | "h" => Ok(Key::H),
        "I" | "i" => Ok(Key::I),
        "J" | "j" => Ok(Key::J),
        "K" | "k" => Ok(Key::K),
        "L" | "l" => Ok(Key::L),
        "M" | "m" => Ok(Key::M),
        "N" | "n" => Ok(Key::N),
        "O" | "o" => Ok(Key::O),
        "P" | "p" => Ok(Key::P),
        "Q" | "q" => Ok(Key::Q),
        "R" | "r" => Ok(Key::R),
        "S" | "s" => Ok(Key::S),
        "T" | "t" => Ok(Key::T),
        "U" | "u" => Ok(Key::U),
        "V" | "v" => Ok(Key::V),
        "W" | "w" => Ok(Key::W),
        "X" | "x" => Ok(Key::X),
        "Y" | "y" => Ok(Key::Y),
        "Z" | "z" => Ok(Key::Z),
        "0" => Ok(Key::Num0),
        "1" => Ok(Key::Num1),
        "2" => Ok(Key::Num2),
        "3" => Ok(Key::Num3),
        "4" => Ok(Key::Num4),
        "5" => Ok(Key::Num5),
        "6" => Ok(Key::Num6),
        "7" => Ok(Key::Num7),
        "8" => Ok(Key::Num8),
        "9" => Ok(Key::Num9),
        "/" => Ok(Key::Slash),
        "?" => Ok(Key::Questionmark),
        "[" => Ok(Key::OpenBracket),
        "]" => Ok(Key::CloseBracket),
        "\\" => Ok(Key::Backslash),
        ";" => Ok(Key::Semicolon),
        "'" => Ok(Key::Quote),
        "," => Ok(Key::Comma),
        "." => Ok(Key::Period),
        "`" => Ok(Key::Backtick),
        "-" => Ok(Key::Minus),
        "=" => Ok(Key::Equals),
        _ => Err(format!("Unknown key: {}", key_str)),
    }
}

#[cfg(feature = "serde")]
fn key_to_string(key: Key) -> String {
    use egui::Key;

    match key {
        Key::ArrowDown => "ArrowDown".to_string(),
        Key::ArrowUp => "ArrowUp".to_string(),
        Key::ArrowLeft => "ArrowLeft".to_string(),
        Key::ArrowRight => "ArrowRight".to_string(),
        Key::Home => "Home".to_string(),
        Key::End => "End".to_string(),
        Key::PageUp => "PageUp".to_string(),
        Key::PageDown => "PageDown".to_string(),
        Key::Backspace => "Backspace".to_string(),
        Key::Delete => "Delete".to_string(),
        Key::Insert => "Insert".to_string(),
        Key::Escape => "Escape".to_string(),
        Key::Enter => "Enter".to_string(),
        Key::Tab => "Tab".to_string(),
        Key::Space => "Space".to_string(),
        Key::F1 => "F1".to_string(),
        Key::F2 => "F2".to_string(),
        Key::F3 => "F3".to_string(),
        Key::F4 => "F4".to_string(),
        Key::F5 => "F5".to_string(),
        Key::F6 => "F6".to_string(),
        Key::F7 => "F7".to_string(),
        Key::F8 => "F8".to_string(),
        Key::F9 => "F9".to_string(),
        Key::F10 => "F10".to_string(),
        Key::F11 => "F11".to_string(),
        Key::F12 => "F12".to_string(),
        Key::A => "A".to_string(),
        Key::B => "B".to_string(),
        Key::C => "C".to_string(),
        Key::D => "D".to_string(),
        Key::E => "E".to_string(),
        Key::F => "F".to_string(),
        Key::G => "G".to_string(),
        Key::H => "H".to_string(),
        Key::I => "I".to_string(),
        Key::J => "J".to_string(),
        Key::K => "K".to_string(),
        Key::L => "L".to_string(),
        Key::M => "M".to_string(),
        Key::N => "N".to_string(),
        Key::O => "O".to_string(),
        Key::P => "P".to_string(),
        Key::Q => "Q".to_string(),
        Key::R => "R".to_string(),
        Key::S => "S".to_string(),
        Key::T => "T".to_string(),
        Key::U => "U".to_string(),
        Key::V => "V".to_string(),
        Key::W => "W".to_string(),
        Key::X => "X".to_string(),
        Key::Y => "Y".to_string(),
        Key::Z => "Z".to_string(),
        Key::Num0 => "0".to_string(),
        Key::Num1 => "1".to_string(),
        Key::Num2 => "2".to_string(),
        Key::Num3 => "3".to_string(),
        Key::Num4 => "4".to_string(),
        Key::Num5 => "5".to_string(),
        Key::Num6 => "6".to_string(),
        Key::Num7 => "7".to_string(),
        Key::Num8 => "8".to_string(),
        Key::Num9 => "9".to_string(),
        Key::Slash => "/".to_string(),
        Key::Questionmark => "?".to_string(),
        Key::OpenBracket => "[".to_string(),
        Key::CloseBracket => "]".to_string(),
        Key::Backslash => "\\".to_string(),
        Key::Semicolon => ";".to_string(),
        Key::Quote => "'".to_string(),
        Key::Comma => ",".to_string(),
        Key::Period => ".".to_string(),
        Key::Backtick => "`".to_string(),
        Key::Minus => "-".to_string(),
        Key::Equals => "=".to_string(),
        _ => format!("{:?}", key),
    }
}

impl KeySequence {
    /// Create a single key shortcut
    pub fn single(modifiers: Modifiers, key: Key) -> Self {
        Self::Single(KeyboardShortcut::new(modifiers, key))
    }

    /// Create a key sequence
    pub fn sequence(keys: Vec<(Modifiers, Key)>) -> Self {
        Self::Sequence(
            keys.into_iter()
                .map(|(modifiers, key)| KeyboardShortcut::new(modifiers, key))
                .collect(),
        )
    }
}

/// Action to perform when shortcut is triggered
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum ShortcutAction {
    /// Navigate between posts
    NavigatePost {
        direction: PostNavigation,
    },
    /// Switch content type tabs
    SwitchTab {
        direction: TabDirection,
    },
    /// Scroll content
    Scroll {
        direction: ScrollDirection,
        #[cfg_attr(feature = "serde", serde(default))]
        amount: ScrollAmount,
    },
    /// Focus a panel
    FocusPanel {
        panel: FocusedPanel,
    },
    /// Focus search bar
    FocusSearch,
    /// Find text in current post
    FindInContent,
    /// Navigate to next/previous find match
    FindNext,
    FindPrevious,
    /// Toggle theme
    ToggleTheme,
    /// Show help overlay
    ShowHelp,
    /// Focus browser address bar (web only)
    BrowserAddress,
    /// Custom action (for extensibility)
    Custom {
        name: String,
    },
}

/// Amount to scroll
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ScrollAmount {
    #[default]
    Small,
    HalfPage,
    Page,
}

/// Definition of a keyboard shortcut
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ShortcutDefinition {
    /// Unique identifier
    pub name: String,
    /// User-facing description
    pub description: String,
    /// Contexts where shortcut is active
    pub contexts: Vec<ShortcutContext>,
    /// Primary key sequence(s)
    pub keys: Vec<KeySequence>,
    /// Alternate key bindings
    #[cfg_attr(feature = "serde", serde(default))]
    pub alternate_keys: Vec<KeySequence>,
    /// Action to perform
    pub action: ShortcutAction,
}

/// Configuration for the shortcut system
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct ShortcutConfig {
    /// Version of the config format
    pub version: String,
    /// Whether Vim mode is always enabled
    pub vim_mode_enabled: bool,
    /// Timeout for key sequences in milliseconds
    pub sequence_timeout_ms: u64,
    /// Whether to save focus state across sessions
    pub save_focus_state: bool,
    /// Default focused panel on startup
    pub default_focused_panel: FocusedPanel,
    /// All shortcut definitions
    pub shortcuts: Vec<ShortcutDefinition>,
    /// Which contexts are enabled
    #[cfg_attr(feature = "serde", serde(default))]
    pub contexts_enabled: BTreeMap<ShortcutContext, bool>,
}

impl Default for ShortcutConfig {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            vim_mode_enabled: true,
            sequence_timeout_ms: 1000,
            save_focus_state: true,
            default_focused_panel: FocusedPanel::LeftPanel,
            shortcuts: Vec::new(),
            contexts_enabled: BTreeMap::new(),
        }
    }
}

/// Helper to create default shortcuts
pub fn default_shortcuts() -> Vec<ShortcutDefinition> {
    use Key as K;
    use Modifiers as M;

    vec![
        // Panel switching
        ShortcutDefinition {
            name: "focus_left_panel".to_string(),
            description: "Focus left panel (post list)".to_string(),
            contexts: vec![ShortcutContext::Global],
            keys: vec![KeySequence::single(M::CTRL, K::H)],
            alternate_keys: vec![],
            action: ShortcutAction::FocusPanel {
                panel: FocusedPanel::LeftPanel,
            },
        },
        ShortcutDefinition {
            name: "focus_right_panel".to_string(),
            description: "Focus right panel (content)".to_string(),
            contexts: vec![ShortcutContext::Global],
            keys: vec![KeySequence::single(M::CTRL, K::L)],
            alternate_keys: vec![],
            action: ShortcutAction::FocusPanel {
                panel: FocusedPanel::RightPanel,
            },
        },
        // Post navigation in left panel
        ShortcutDefinition {
            name: "next_post".to_string(),
            description: "Select next post".to_string(),
            contexts: vec![ShortcutContext::LeftPanel],
            keys: vec![
                KeySequence::single(M::NONE, K::ArrowDown),
                KeySequence::single(M::NONE, K::J),
            ],
            alternate_keys: vec![],
            action: ShortcutAction::NavigatePost {
                direction: PostNavigation::Next,
            },
        },
        ShortcutDefinition {
            name: "previous_post".to_string(),
            description: "Select previous post".to_string(),
            contexts: vec![ShortcutContext::LeftPanel],
            keys: vec![
                KeySequence::single(M::NONE, K::ArrowUp),
                KeySequence::single(M::NONE, K::K),
            ],
            alternate_keys: vec![],
            action: ShortcutAction::NavigatePost {
                direction: PostNavigation::Previous,
            },
        },
        ShortcutDefinition {
            name: "first_post".to_string(),
            description: "Navigate to first post".to_string(),
            contexts: vec![ShortcutContext::LeftPanel, ShortcutContext::RightPanel],
            keys: vec![
                KeySequence::single(M::NONE, K::Home),
                KeySequence::sequence(vec![(M::NONE, K::G), (M::NONE, K::G)]),
            ],
            alternate_keys: vec![],
            action: ShortcutAction::NavigatePost {
                direction: PostNavigation::First,
            },
        },
        ShortcutDefinition {
            name: "last_post".to_string(),
            description: "Navigate to last post".to_string(),
            contexts: vec![ShortcutContext::LeftPanel, ShortcutContext::RightPanel],
            keys: vec![
                KeySequence::single(M::NONE, K::End),
                KeySequence::single(M::SHIFT, K::G),
            ],
            alternate_keys: vec![],
            action: ShortcutAction::NavigatePost {
                direction: PostNavigation::Last,
            },
        },
        // Tab switching
        ShortcutDefinition {
            name: "switch_tab_left".to_string(),
            description: "Switch to previous content tab".to_string(),
            contexts: vec![ShortcutContext::LeftPanel],
            keys: vec![
                KeySequence::single(M::NONE, K::ArrowLeft),
                KeySequence::single(M::NONE, K::H),
            ],
            alternate_keys: vec![],
            action: ShortcutAction::SwitchTab {
                direction: TabDirection::Previous,
            },
        },
        ShortcutDefinition {
            name: "switch_tab_right".to_string(),
            description: "Switch to next content tab".to_string(),
            contexts: vec![ShortcutContext::LeftPanel],
            keys: vec![
                KeySequence::single(M::NONE, K::ArrowRight),
                KeySequence::single(M::NONE, K::L),
            ],
            alternate_keys: vec![],
            action: ShortcutAction::SwitchTab {
                direction: TabDirection::Next,
            },
        },
        // Content scrolling
        ShortcutDefinition {
            name: "scroll_down".to_string(),
            description: "Scroll down small step".to_string(),
            contexts: vec![ShortcutContext::RightPanel],
            keys: vec![KeySequence::single(M::NONE, K::J)],
            alternate_keys: vec![],
            action: ShortcutAction::Scroll {
                direction: ScrollDirection::Down,
                amount: ScrollAmount::Small,
            },
        },
        ShortcutDefinition {
            name: "scroll_up".to_string(),
            description: "Scroll up small step".to_string(),
            contexts: vec![ShortcutContext::RightPanel],
            keys: vec![KeySequence::single(M::NONE, K::K)],
            alternate_keys: vec![],
            action: ShortcutAction::Scroll {
                direction: ScrollDirection::Up,
                amount: ScrollAmount::Small,
            },
        },
        ShortcutDefinition {
            name: "scroll_half_page_down".to_string(),
            description: "Scroll down half page".to_string(),
            contexts: vec![ShortcutContext::RightPanel],
            keys: vec![KeySequence::single(M::CTRL, K::D)],
            alternate_keys: vec![],
            action: ShortcutAction::Scroll {
                direction: ScrollDirection::Down,
                amount: ScrollAmount::HalfPage,
            },
        },
        ShortcutDefinition {
            name: "scroll_half_page_up".to_string(),
            description: "Scroll up half page".to_string(),
            contexts: vec![ShortcutContext::RightPanel],
            keys: vec![KeySequence::single(M::CTRL, K::U)],
            alternate_keys: vec![],
            action: ShortcutAction::Scroll {
                direction: ScrollDirection::Up,
                amount: ScrollAmount::HalfPage,
            },
        },
        // Search and find
        ShortcutDefinition {
            name: "focus_search".to_string(),
            description: "Focus search bar".to_string(),
            contexts: vec![ShortcutContext::Global, ShortcutContext::LeftPanel],
            keys: vec![
                KeySequence::single(M::CTRL, K::K),
                KeySequence::single(M::NONE, K::Slash),
            ],
            alternate_keys: vec![],
            action: ShortcutAction::FocusSearch,
        },
        ShortcutDefinition {
            name: "find_in_content".to_string(),
            description: "Find text in current post".to_string(),
            contexts: vec![ShortcutContext::RightPanel],
            keys: vec![
                KeySequence::single(M::CTRL, K::F),
                KeySequence::single(M::NONE, K::Slash),
            ],
            alternate_keys: vec![],
            action: ShortcutAction::FindInContent,
        },
        ShortcutDefinition {
            name: "next_match".to_string(),
            description: "Next find match".to_string(),
            contexts: vec![ShortcutContext::FindMode],
            keys: vec![KeySequence::single(M::NONE, K::N)],
            alternate_keys: vec![],
            action: ShortcutAction::FindNext,
        },
        ShortcutDefinition {
            name: "previous_match".to_string(),
            description: "Previous find match".to_string(),
            contexts: vec![ShortcutContext::FindMode],
            keys: vec![KeySequence::single(M::SHIFT, K::N)],
            alternate_keys: vec![],
            action: ShortcutAction::FindPrevious,
        },
        // Global shortcuts
        ShortcutDefinition {
            name: "toggle_theme".to_string(),
            description: "Toggle between light and dark themes".to_string(),
            contexts: vec![ShortcutContext::Global],
            keys: vec![KeySequence::single(M::CTRL, K::T)],
            alternate_keys: vec![],
            action: ShortcutAction::ToggleTheme,
        },
        ShortcutDefinition {
            name: "show_help".to_string(),
            description: "Show keyboard shortcuts help".to_string(),
            contexts: vec![ShortcutContext::Global],
            keys: vec![KeySequence::single(M::NONE, K::Questionmark)],
            alternate_keys: vec![],
            action: ShortcutAction::ShowHelp,
        },
        ShortcutDefinition {
            name: "browser_address".to_string(),
            description: "Focus browser address bar (web only)".to_string(),
            contexts: vec![ShortcutContext::Global],
            keys: vec![KeySequence::single(M::ALT, K::D)],
            alternate_keys: vec![],
            action: ShortcutAction::BrowserAddress,
        },
    ]
}
