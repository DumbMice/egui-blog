# Blog App Keyboard Shortcuts Analysis & Implementation Plan

## UI Interaction Patterns Analysis

### 1. Focus Contexts and Natural Interaction Areas:
- **Global Context**: App-wide shortcuts that should work anywhere
- **Search Bar Context**: When search input is focused (text entry mode)
- **Content Area Context**: When reading post content (text selection possible)
- **Editor Context**: When creating/editing posts (text input fields)
- **Post List Context**: When navigating post list (selection focus)

### 2. Conflict Analysis:
- **Search Bar**: Text input (`/` key) conflicts with vim search command
- **Editor Mode**: Text input conflicts with navigation shortcuts
- **Browser Integration**: Alt+D conflicts with browser shortcuts
- **Text Selection**: Arrow keys conflict with navigation when text is selected

### 3. User Workflow Considerations:
- **Primary Use Case**: Reading blog posts
- **Secondary Use Case**: Searching/filtering posts
- **Tertiary Use Case**: Creating posts (demo feature)
- **Navigation Patterns**: Linear (prev/next), random access (list), search-based

## Comprehensive Keyboard Shortcut Scheme

### Design Principles:
1. **Always-on Vim Mode**: No toggle, vim keys always active
2. **Context-Aware Activation**: Shortcuts adapt to current focus
3. **Progressive Disclosure**: Basic shortcuts first, power user shortcuts available
4. **Conflict Avoidance**: Text input contexts disable conflicting shortcuts
5. **Discoverability**: Logical, memorable key bindings

### Shortcut Categories:

#### Category 1: Global Navigation (Always Active)
| Shortcut | Action | Activation Mode | Focus Requirement | Conflict Considerations | Priority |
|----------|--------|----------------|-------------------|------------------------|----------|
| `j` / `↓` | Next post | Global | Anywhere except text input | Conflicts with text input | Must-have |
| `k` / `↑` | Previous post | Global | Anywhere except text input | Conflicts with text input | Must-have |
| `gg` | First post | Global | Anywhere except text input | Double-tap 'g' detection | Must-have |
| `G` | Last post | Global | Anywhere except text input | Shift+G detection | Must-have |
| `Home` | First post | Global | Anywhere | Browser/OS conflicts | Must-have |
| `End` | Last post | Global | Anywhere | Browser/OS conflicts | Must-have |
| `g` then `g` | First post (vim style) | Global | Anywhere except text input | Requires timing detection | Must-have |
| `Shift+G` | Last post (vim style) | Global | Anywhere except text input | None | Must-have |

#### Category 2: Search & Filter (Context-Sensitive)
| Shortcut | Action | Activation Mode | Focus Requirement | Conflict Considerations | Priority |
|----------|--------|----------------|-------------------|------------------------|----------|
| `/` | Focus search bar | Global | Anywhere except search bar | Conflicts with text input '/' | Must-have |
| `Esc` | Clear search/blur | Search context | Search bar focused | Standard vim behavior | Must-have |
| `Ctrl+F` | Find in page | Content context | Content area visible | Browser conflict (opens find dialog) | Nice-to-have |
| `n` | Next search result | Find context | After Ctrl+F | Standard vim behavior | Nice-to-have |
| `N` | Previous search result | Find context | After Ctrl+F | Standard vim behavior | Nice-to-have |

#### Category 3: Content Type Navigation
| Shortcut | Action | Activation Mode | Focus Requirement | Conflict Considerations | Priority |
|----------|--------|----------------|-------------------|------------------------|----------|
| `t` then `p` | Switch to Posts tab | Global | Anywhere except text input | Two-key sequence | Must-have |
| `t` then `n` | Switch to Notes tab | Global | Anywhere except text input | Two-key sequence | Must-have |
| `t` then `r` | Switch to Reviews tab | Global | Anywhere except text input | Two-key sequence | Must-have |
| `t` then `a` | Switch to All tab | Global | Anywhere except text input | Two-key sequence | Must-have |
| `Ctrl+1` | Posts tab | Global | Anywhere | Browser tab switching conflict | Nice-to-have |
| `Ctrl+2` | Notes tab | Global | Anywhere | Browser tab switching conflict | Nice-to-have |
| `Ctrl+3` | Reviews tab | Global | Anywhere | Browser tab switching conflict | Nice-to-have |
| `Ctrl+0` | All tab | Global | Anywhere | Browser tab switching conflict | Nice-to-have |

#### Category 4: Browser Integration
| Shortcut | Action | Activation Mode | Focus Requirement | Conflict Considerations | Priority |
|----------|--------|----------------|-------------------|------------------------|----------|
| `Alt+D` | Focus browser address bar | Global | Anywhere | Standard browser shortcut | Must-have |
| `Alt+←` | Browser back | Global | Anywhere | Standard browser shortcut | Nice-to-have |
| `Alt+→` | Browser forward | Global | Anywhere | Standard browser shortcut | Nice-to-have |
| `F5` / `Ctrl+R` | Reload page | Global | Anywhere | Standard browser shortcut | Nice-to-have |

#### Category 5: Theme & UI Controls
| Shortcut | Action | Activation Mode | Focus Requirement | Conflict Considerations | Priority |
|----------|--------|----------------|-------------------|------------------------|----------|
| `T` | Toggle theme | Global | Anywhere | None | Must-have |
| `?` | Show help | Global | Anywhere | None | Nice-to-have |
| `Esc` | Close help/panels | Help context | Help/panel open | Multi-use key | Nice-to-have |

#### Category 6: Editor Mode (Demo Feature)
| Shortcut | Action | Activation Mode | Focus Requirement | Conflict Considerations | Priority |
|----------|--------|----------------|-------------------|------------------------|----------|
| `e` | Enter editor mode | Global | Anywhere except text input | Conflicts with text input 'e' | Must-have |
| `Esc` | Cancel editing | Editor context | Editor active | Multi-use key | Must-have |
| `Ctrl+S` | Save post | Editor context | Editor active | Browser save conflict | Must-have |
| `Tab` | Next field | Editor context | Editor field focused | Standard behavior | Must-have |
| `Shift+Tab` | Previous field | Editor context | Editor field focused | Standard behavior | Must-have |

#### Category 7: Advanced Navigation
| Shortcut | Action | Activation Mode | Focus Requirement | Conflict Considerations | Priority |
|----------|--------|----------------|-------------------|------------------------|----------|
| `:` then number | Go to post number | Global | Anywhere except text input | Vim command mode style | Nice-to-have |
| `/` then query | Search posts | Global | Anywhere except text input | Requires command mode | Future |
| `*` | Search for word under cursor | Content context | Content area with selection | Requires text selection | Future |
| `#` | Search backward for word | Content context | Content area with selection | Requires text selection | Future |

## Activation Mode Definitions

### 1. **Global Mode**:
- Active everywhere in the app
- Disabled when text input is focused (search bar, editor fields)
- Always-on vim navigation keys

### 2. **Search Context**:
- Activated when search bar is focused
- `Esc` clears search and returns focus to content
- `/` key is disabled (already in search)

### 3. **Content Context**:
- Activated when content area is visible
- Supports text selection and find-in-page
- Arrow keys scroll content when text not selected

### 4. **Editor Context**:
- Activated when creating/editing posts
- Standard text editor shortcuts take precedence
- Navigation shortcuts disabled

### 5. **Command Mode** (Future):
- Activated by `:` key
- For advanced commands (go to line, search, etc.)
- Similar to vim command mode

## Conflict Resolution Strategy

### 1. **Text Input Priority**:
- When any text input is focused (search bar, editor fields):
  - Disable: `j`, `k`, `gg`, `G`, `t` sequences, `e`
  - Enable: Standard text editing shortcuts
  - Special: `Esc` to blur/clear input

### 2. **Browser Shortcut Conflicts**:
- `Alt+D`: Override browser behavior to focus address bar
- `Ctrl+1-9`: Use with caution (browser tab switching)
- `F5`/`Ctrl+R`: Allow browser default (app state persists)

### 3. **Multi-Key Sequence Handling**:
- `gg`: Detect double-tap 'g' within 500ms
- `t` sequences: Wait for second key with timeout
- `:` commands: Enter command mode, wait for input

### 4. **Focus Management**:
- `/` focuses search bar if not already focused
- `Esc` blurs current focus
- Tab cycling between interactive elements

## Implementation Priority Phasing

### Phase 1: Core Navigation (Must-have)
1. Basic post navigation: `j/k`, `Home/End`, `gg/G`
2. Search focus: `/` and `Esc`
3. Theme toggle: `T`
4. Browser address bar: `Alt+D`
5. Editor mode: `e` and `Esc`

### Phase 2: Enhanced Navigation (Nice-to-have)
1. Content type tabs: `t` sequences
2. Find in page: `Ctrl+F`, `n/N`
3. Browser navigation: `Alt+←/→`
4. Help system: `?`

### Phase 3: Advanced Features (Future)
1. Command mode: `:` for advanced commands
2. Word search: `*` and `#`
3. Numbered navigation: `:N` to go to post N
4. Custom shortcut configuration

## Technical Implementation Notes

### 1. **egui Input Handling**:
- Use `ctx.input(|i| i.key_pressed(Key::ArrowDown))` for key detection
- Track focus state with `ui.memory(|mem| mem.focused())`
- Handle multi-key sequences with timing and state tracking

### 2. **State Management**:
- Add `KeyboardState` to `BlogApp` struct
- Track: last key press time, sequence buffer, current mode
- Reset state on context changes

### 3. **Focus Detection**:
- Check if search bar is focused before enabling shortcuts
- Detect editor mode activation
- Track text selection in content area

### 4. **Browser Integration**:
- Web: Use `web_sys` for `Alt+D` implementation
- Native: Handle differently or disable browser-specific shortcuts

### 5. **Accessibility Considerations**:
- Maintain mouse/touch compatibility
- Don't break screen reader navigation
- Provide visual feedback for mode changes

## Testing Strategy

### 1. **Unit Tests**:
- Key sequence detection logic
- Focus state management
- Conflict resolution rules

### 2. **Integration Tests**:
- Shortcut functionality in different contexts
- Browser integration (web target)
- Persistence across navigation

### 3. **User Testing**:
- Vim users for familiarity
- Non-vim users for discoverability
- Accessibility testing

## Conclusion

This keyboard shortcut scheme provides a comprehensive, user-friendly implementation that follows the "always-on vim mode" requirement while avoiding frustrating conflicts. The phased approach allows for incremental implementation starting with the most essential navigation shortcuts.

The design balances power user efficiency with accessibility, ensuring that the app remains usable for all users while providing vim-style navigation for those who prefer it.
