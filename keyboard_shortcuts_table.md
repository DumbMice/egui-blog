# Blog App Keyboard Shortcuts - Implementation Table

## Phase 1: Core Navigation (Must-have)

### Post Navigation
| Shortcut | Action | Context | Notes |
|----------|--------|---------|-------|
| `j` or `↓` | Next post | Global (except text input) | Vim-style down |
| `k` or `↑` | Previous post | Global (except text input) | Vim-style up |
| `Home` | First post | Global | Standard navigation |
| `End` | Last post | Global | Standard navigation |
| `gg` | First post | Global (except text input) | Double-tap 'g' within 500ms |
| `G` | Last post | Global (except text input) | Shift+G |

### Search & UI
| Shortcut | Action | Context | Notes |
|----------|--------|---------|-------|
| `/` | Focus search bar | Global (except search bar) | Vim search command |
| `Esc` | Clear search/blur input | Search bar focused | Also cancels editor mode |
| `T` | Toggle theme | Global | Capital T for Theme |
| `Alt+D` | Focus browser address bar | Global (web only) | Standard browser shortcut |

### Editor Mode (Demo)
| Shortcut | Action | Context | Notes |
|----------|--------|---------|-------|
| `e` | Enter editor mode | Global (except text input) | Create new post |
| `Ctrl+S` | Save post | Editor mode | Standard save shortcut |
| `Esc` | Cancel editing | Editor mode | Returns to reading mode |

## Phase 2: Enhanced Navigation (Nice-to-have)

### Content Type Tabs
| Shortcut | Action | Context | Notes |
|----------|--------|---------|-------|
| `t` then `p` | Switch to Posts tab | Global (except text input) | Two-key sequence |
| `t` then `n` | Switch to Notes tab | Global (except text input) | Two-key sequence |
| `t` then `r` | Switch to Reviews tab | Global (except text input) | Two-key sequence |
| `t` then `a` | Switch to All tab | Global (except text input) | Two-key sequence |

### Find in Page
| Shortcut | Action | Context | Notes |
|----------|--------|---------|-------|
| `Ctrl+F` | Find in page | Content area visible | Opens find dialog |
| `n` | Next match | After Ctrl+F | Standard find behavior |
| `N` | Previous match | After Ctrl+F | Standard find behavior |

### Browser Navigation
| Shortcut | Action | Context | Notes |
|----------|--------|---------|-------|
| `Alt+←` | Browser back | Global (web only) | History navigation |
| `Alt+→` | Browser forward | Global (web only) | History navigation |

### Help
| Shortcut | Action | Context | Notes |
|----------|--------|---------|-------|
| `?` | Show keyboard shortcuts | Global | Help overlay |

## Context Rules & Conflict Resolution

### 1. Text Input Contexts (DISABLE navigation shortcuts):
- **Search bar focused**: Only `Esc` works
- **Editor fields focused**: Only text editing shortcuts work
- **Content area with text selected**: Arrow keys scroll instead of navigate posts

### 2. Global Context (ENABLE all shortcuts):
- Content area (no text selection)
- Post list area
- Empty states
- Error states

### 3. Special Cases:
- `/` key: If search bar not focused → focus it; if already focused → type '/'
- `Esc` key: Context-sensitive (clear search, cancel editor, close help)
- `Alt+D`: Web target only, passes focus to browser address bar

## Implementation Details

### Key Detection Strategy:
1. **Single keys**: Use `ctx.input(|i| i.key_pressed(Key::J))`
2. **Modified keys**: Check `i.modifiers` for Shift, Alt, Ctrl
3. **Multi-key sequences**: Track with `KeyboardState` and timers
4. **Focus detection**: `ui.memory(|mem| mem.focused())` for text inputs

### State Management:
```rust
struct KeyboardState {
    last_key_press: Option<(Key, Instant)>,
    sequence_buffer: Vec<Key>,
    current_mode: InputMode,
}

enum InputMode {
    Global,
    SearchFocused,
    EditorFocused,
    CommandMode,
}
```

### Focus IDs:
- Search bar: `"search_bar"`
- Editor title: `"editor_title"`
- Editor content: `"editor_content"`
- Content area: `"content_area"`

## Testing Checklist

### Phase 1 Tests:
- [ ] `j/k` navigates posts when no text input focused
- [ ] `j/k` does nothing when search bar focused
- [ ] `/` focuses search bar from anywhere
- [ ] `Esc` clears search and blurs input
- [ ] `Home/End` go to first/last post
- [ ] `gg` (double-tap) goes to first post
- [ ] `G` goes to last post
- [ ] `T` toggles theme
- [ ] `Alt+D` focuses browser address bar (web)
- [ ] `e` enters editor mode
- [ ] `Esc` cancels editor mode
- [ ] `Ctrl+S` saves post in editor

### Phase 2 Tests:
- [ ] `t` sequences switch content type tabs
- [ ] `Ctrl+F` opens find in page
- [ ] `n/N` navigate find results
- [ ] `Alt+←/→` browser navigation (web)
- [ ] `?` shows help overlay

## Notes for Developers

1. **Always-on Vim Mode**: No toggle switch, vim keys always work in global context
2. **Non-configurable**: Shortcuts are hardcoded as per requirements
3. **Progressive Enhancement**: Start with Phase 1, add Phase 2 later
4. **Error Handling**: Graceful fallback when shortcuts can't execute (e.g., no posts)
5. **Visual Feedback**: Consider subtle indicators for mode changes
