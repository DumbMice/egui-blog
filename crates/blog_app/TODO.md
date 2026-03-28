# Blog App TODO List

## Recent Fixes (2026-03-15)
✅ **Priority 21: Panel Focus and Scroll Position Persistence**
- Fixed RON serialization by adding `#[serde(skip)]` to `route_restored` field
- Added `Serialize`/`Deserialize` derives to `FocusAnimationState` and `AnimationPhase`
- Changed default focused panel from `LeftPanel` to `RightPanel` as requested
- Scroll position persistence now uses egui's `id_salt()` with dynamic post-based IDs
- Improved main content click detection with multiple methods
- Handle corrupted LocalStorage data gracefully with fallback to defaults
- Fixed all clippy warnings (replace `println!` with `log::debug!`)
- All tests pass

## Recent Fixes (2026-03-12)
✅ **Priority 18: Search Bar Crash with '#' Character and Tag Icon Bug**
- Fixed WASM crash when typing '#' in search bar (get_dropdown_position with fallback)
- Fixed cross icon from ✕ to ❌ in selected tags chips
- Fixed cursor positioning by removing URL updates during typing
- Added stable widget IDs for focus detection
- Fixed state persistence across refreshes
- Fixed inconsistent URL updates for keyboard navigation
- Fixed tab switching behavior (now only filters)
- Implemented search URL updates on Enter key
- Clear search text after selecting tag from autocomplete
- Prevent single-character shortcuts when typing in text fields
- All tests pass

## Recent Fixes (2026-03-10)
✅ **Theme Toggle Navigation Bug Fix**
- Fixed bug where toggling theme caused navigation to home page
- Root cause: Theme changes incorrectly marked as search modifications  
- Solution: Added `TopPanelResult` struct to separate search vs theme changes
- Added defensive check comparing search state before/after top panel
- Fixed router serialization with custom deserialization for graceful fallback
- Updated state restoration precedence: Browser URL > Persisted State > Default

## Priority 1: Content Separation
- [x] Define post file format (Markdown + YAML frontmatter)
- [x] Create posts directory structure
- [x] Implement markdown file loader
- [x] Add frontmatter parser (YAML)
- [x] Update PostManager to use file loading
- [x] Add file watcher for live reload (development)
- [x] Implement compile-time embedding (production)
- [x] Create example post files
- [x] Test loading and display
- [x] Update UI to handle missing posts gracefully

## Priority 2: Markdown Rendering
- [x] Evaluate markdown rendering options
- [x] Choose and integrate markdown parser
- [x] Implement basic text rendering (headings, paragraphs)
- [x] Add emphasis rendering (bold, italic)
- [x] Implement code block support
- [x] Add language labels to code blocks
- [x] Add syntax highlighting
- [x] Support links and images (basic image placeholder)
- [x] Add strikethrough support
- [x] Add list rendering (ordered/unordered)
- [x] Improve list spacing and visual markers
- [x] Implement blockquotes and horizontal rules
- [x] Add table support (optional)

## Priority 3: Math Formula Rendering
- [x] Implement Typst math formula rendering system
- [x] Add build script for processing math formulas
- [x] Create SVG generation and processing pipeline
- [x] Implement theme-aware SVG rendering (white in dark mode, black in light mode)
- [x] Add paragraph accumulation for inline math rendering
- [x] Fix horizontal spacing for inline formulas (item_spacing.x = 0.0)
- [x] Support both inline ($formula$) and display ($$ formula $$) math
- [x] Add manifest system for formula caching and tracking

## Priority 4: Build System Restructuring ✅ COMPLETED 2026-02-27
- [x] Implement three-target architecture (native, WASM, web server)
- [x] Fix winit dependency issues for native builds
- [x] Create unified development server with hot reload
- [x] Separate dev/release build outputs
- [x] Replace shell scripts with Rust-based workflow
- [x] Update documentation and cargo aliases
- [x] Implement file watching with auto-rebuild
- [x] Add proper error handling for build failures
- [x] See [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for implementation details

## Priority 5: State Persistence ✅ COMPLETED 2026-02-27
- [x] Add persistence feature to Cargo.toml (enabled by default)
- [x] Make BlogApp and dependent types serializable with serde
- [x] Implement save() method for app state serialization
- [x] Implement BlogApp::new() constructor with storage loading
- [x] Add persist_egui_memory() and auto_save_interval() methods
- [x] Update entry points to use new constructor
- [x] Test persistence across browser refreshes
- [x] Document persistence implementation in AGENTS.md

### New Development Workflow
```bash
# Development server with hot reload (default)
cargo run --bin blog_web_server --features dev

# Production server with optimized builds
cargo run --bin blog_web_server --features dev -- --serve-release

# Native desktop app
cargo run --bin blog_native

# Build only (no server)
cargo run --bin blog_web_server --features dev -- --build-only --serve-release

# Using cargo aliases
cargo blog          # Development server (hot reload)
cargo blog-release  # Production server (optimized)
cargo blog-native   # Native desktop app
cargo blog-wasm     # Build WASM library only
```

### Key Features
- **Hot reload**: File changes trigger automatic WASM rebuild
- **Error handling**: Build failures show compiler errors, server continues running
- **Multiple modes**: Development (debug) vs Production (optimized with wasm-opt)
- **Auto tool installation**: wasm-bindgen, basic-http-server installed if missing
- **Port management**: Checks for port conflicts, supports custom ports
- **Browser auto-open**: `--open` flag opens browser automatically
- **Logging control**: `--log-level` option for verbosity control

## Priority 6: Performance Optimizations ✅ COMPLETED 2026-03-01
- [x] Math manifest caching with `OnceLock` (3752× faster loading)
- [x] Formula reverse index for O(1) lookup instead of O(n) linear search
- [x] Markdown processing cache to avoid reprocessing static content every frame
- [x] Remove unused code and clean up function hierarchy
- [x] Add benchmark tests to measure performance improvements
- [x] Fix all clippy warnings and code quality issues

### Implementation Details:
1. **Math Manifest Caching**: Added `static MANIFEST_CACHE: OnceLock<MathManifest>` to cache JSON parsing
2. **Reverse Index**: Added `reverse_index: HashMap<(String, bool), String>` to `MathManifest` for O(1) formula lookup
3. **Markdown Cache**: Added `cached_processed_content: Option<String>` to `BlogPost` with preprocessing at creation
4. **API Updates**: Created `render_preprocessed_markdown()` function and updated rendering to use cached content
5. **Benchmarks**: Added performance benchmark tests showing 3752× faster manifest loading and O(1) formula lookup

## Priority 7: Enhanced Styling ✅ COMPLETED 2026-03-02
- [x] Design custom theme system (simplified to 2 Catppuccin themes)
- [x] Implement color customization (Catppuccin Latte/Macchiato)
- [x] Improve typography (fonts, spacing)
- [x] Add responsive layout adaptations
- [x] Implement smooth theme transitions
- [x] Polish UI spacing and borders
- [x] Add visual feedback for interactions
- [x] Fix strong text contrast in Catppuccin themes
- [x] Implement single-button theme toggle
- [x] Ensure Catppuccin style guide compliance

## Priority 8: Multi-Page Navigation & URL Routing ✅ COMPLETED 2026-03-03
- [x] Implement URL routing for direct post access (`/post/slug`)
- [x] Add URL routing for different page types (`/notes/id`, `/reviews/id`)
- [x] Support browser history navigation (back/forward)
- [x] Make URLs bookmarkable and shareable
- [x] Handle hash-based routing for SPA navigation
- [x] Sync app state with URL parameters

**Note**: Essential for sharing/bookmarking content. Uses egui's `ctx.input().raw` for URL changes. See [URL_ROUTING.md](URL_ROUTING.md) for detailed specification.

### Implementation Details:
1. **Router Encapsulation**: Created `Router` struct to encapsulate routing state and logic
2. **Hash-based Routing**: Supports `#/post/slug`, `#/search?q=query`, `#/tags/tag`, `#/` (home)
3. **Slug Generation**: Auto-generates URL-friendly slugs from post titles
4. **Browser Integration**: Handles back/forward navigation and URL persistence
5. **Navigation Context**: `NavigationContext` struct for UI components with route and callback
6. **404 Handling**: Shows error page with "Return to Home" navigation
7. **Query Parameters**: Basic support for search queries and tags (extensible)
8. **Persistence**: Router state saved across browser refreshes using serde serialization
9. **Comprehensive Testing**: 8 routing-specific tests added, all existing tests pass

## Priority 9: Multiple Content Types & Tabs ✅ COMPLETED 2026-03-03
- [x] Support different content types: blog posts, private notes, research reviews
- [x] Implement tab-based navigation between content types
- [x] Shared search database across all content types
- [x] Same Markdown format with type-specific frontmatter
- [x] Different directories: `posts/`, `notes/`, `reviews/`
- [x] Filterable views or separate navigation sections
- [x] Fix tab navigation bugs and UI issues
- [x] Add dates to post list display
- [x] Fix panel resizing with wrapping text labels

**Note**: Single WASM app with multiple content sections. Tabs as starting navigation pattern.

### Implementation Details:
1. **ContentType Enum**: Created `ContentType` enum with `Post`, `Note`, `Review` variants
2. **Directory Structure**: Created `notes/` and `reviews/` directories alongside existing `posts/`
3. **Example Content**: Added example markdown files with type-specific frontmatter:
   - `notes/2026-03-01-research-ideas.md` (with `type: "note"`, `status: "draft"`)
   - `notes/2026-02-28-meeting-notes.md` (with `type: "note"`, `status: "active"`)
   - `reviews/2026-02-25-deep-learning-book-review.md` (with `type: "review"`, `rating: 5`)
   - `reviews/2026-02-20-rust-programming-language-review.md` (with `type: "review"`, `rating: 4`)
4. **Frontmatter Extension**: Added `content_type` field to `Frontmatter` struct for YAML parsing
5. **Backward Compatibility**: When `type` is not specified in frontmatter, defaults to `ContentType::Post`
6. **Build System Update**: Modified `build.rs` to scan all three directories for math formulas
7. **Content Loading**: Updated `load_embedded_content()` to load from all three directories using `embed_file_array!` macro
8. **Testing**: Added comprehensive tests for content type parsing and loading verification
9. **Verification**: Confirmed loading of 6 posts, 2 notes, and 2 reviews in integration test

### Tab-based UI Navigation:
1. **State Management**: Added `selected_content_type` state to `BlogApp` struct
2. **Tab Implementation**: Updated `side_panel` function to include content type tabs ("All", "Posts", "Notes", "Reviews")
3. **Filtering Logic**: Posts are filtered based on selected content type with proper dereferencing fix
4. **Navigation Integration**: Tabs automatically select first post of type and trigger navigation
5. **Visual Indicators**: Added icons (📝, 📓, ⭐) for content types in post list
6. **Date Display**: Added publication dates to post list with proper styling

### Routing Extension:
1. **Route Enum**: Updated `Route` enum to support `Post`, `Note`, and `Review` routes
2. **URL Parsing**: Updated URL parsing to support `#/posts/slug`, `#/notes/slug`, `#/reviews/slug`
3. **Backward Compatibility**: Maintained backward compatibility with old `#/post/slug` URLs
4. **Navigation Logic**: Updated navigation logic to generate correct URLs based on content type
5. **State Sync**: Updated `sync_state_to_route` to handle new route types and update content type filter

### UI Improvements:
1. **Wrapping Labels**: Created `selectable_label_wrapping()` function to support text wrapping in post titles
2. **Panel Resizing**: Fixed left panel resizing issue by replacing `selectable_label` with wrapping version
3. **Date Display**: Added publication dates to post list with small, faded text styling
4. **Visual Feedback**: Maintained proper selection and hover states in custom wrapping labels

### Bug Fixes:
1. **Tab Comparison Bug**: Fixed `*selected_content_type == Some(content_type)` comparison (was comparing references)
2. **"All" Tab Logic**: Fixed logic for selecting first post when switching to "All" tab
3. **Navigation State**: Fixed content type filter persistence when navigating between posts
4. **Post Selection**: Fixed post selection click handler in `post_preview` function
5. **Theme Toggle Navigation Bug**: Fixed bug where toggling theme caused navigation to home page
   - Root cause: Theme changes incorrectly marked as search modifications
   - Solution: Added `TopPanelResult` struct to separate search vs theme changes
   - Added defensive check comparing search state before/after top panel
   - Fixed router serialization with custom deserialization for graceful fallback
   - Updated state restoration precedence: Browser URL > Persisted State > Default

### Testing:
- All 29 existing tests pass
- Comprehensive content type parsing tests
- Routing and navigation tests
- UI component tests

## Priority 10: Improved Responsive Layout ✅ COMPLETED 2026-03-03
- [x] Central content width: 80-100 characters optimal
- [x] Responsive margins that adjust with zoom level
- [x] Max-width content container with auto margins
- [x] Breakpoint for mobile vs desktop layouts
- [x] Content width reduces when page width < 80-100 chars
- [x] Desktop-only responsive behavior

**Note**: Improves readability with proper typographic margins.

### Implementation Details:
1. **ResponsiveConfig struct**: Created configuration for responsive behavior with optimal reading width (90 chars), min/max widths, margin percentages, and mobile breakpoint (768px)
2. **Adaptive width calculation**: `calculate_adaptive_width()` reduces content width proportionally when screen width is less than optimal
3. **Responsive margins**: `get_margins()` provides smaller fixed margins on mobile (16px) and percentage-based margins on desktop (5% min 20px, max 10%)
4. **Container utilities**: `responsive_container()` and `max_width_container()` create centered containers with optimal reading width
5. **Mobile detection**: `is_mobile()` function checks if screen width is below breakpoint
6. **Integration**: Updated main app to wrap content in responsive container using `ui::responsive::responsive_container()`
7. **Testing**: All 29 existing tests pass, WASM build successful

### Key Features:
- **Optimal reading width**: 80-100 characters (720-800px) for comfortable reading
- **Adaptive behavior**: Content width reduces when screen width is limited
- **Centered layout**: Auto-centering margins maintain visual balance
- **Mobile-friendly**: Smaller margins and adaptive width on mobile screens
- **Zoom-aware**: Percentage-based margins adjust with zoom level
- **Backward compatible**: All existing functionality preserved

## Priority 11: Keyboard Shortcuts & Vim Navigation ✅ COMPLETED 2026-03-05
- [x] Basic navigation (arrow keys, Home/End) - via configurable shortcuts
- [x] Vim navigation (j/k for up/down) - via configurable shortcuts
- [x] `/` for page content search (not post search) - Ctrl+F or `/` for find in content
- [x] Alt+D to focus browser address bar (web only) - via configurable shortcuts
- [x] Always-on vim mode (not toggleable) - vim_mode_enabled = true in config
- [x] Configurable via TOML file (shortcuts.toml) - more flexible than hardcoded
- [ ] **BUG**: `gg` and `G` shortcuts don't work (see Priority 25)

**Note**: Modular keyboard shortcut system with:
- Panel-based navigation (Ctrl+H/L to switch between left/right panels)
- Vim-style shortcuts (j/k for scrolling, h/l for tab switching)
- Configurable via TOML file (required, no defaults)
- Always-on vim mode
- Help overlay with `?` shortcut
- Focus state persistence across sessions
- Find-in-content functionality with dialog
- **Missing**: `gg` and `G` shortcuts for top/bottom navigation (to be implemented in Priority 25)

## Priority 12: Animated Focus Indicators ✅ COMPLETED 2026-03-05
- [x] Replace ugly blue border with animated focus indicator
- [x] Flash-only animation (no pulse/breathing effects)
- [x] Very short duration (100ms default) - just enough to see focus change
- [x] No persistent tint - panel returns to normal after flash
- [x] Uses Catppuccin blue color from theme (widgets.active.bg_fill)
- [x] Simple border flash without background tint or glow effects
- [x] Debug-configurable parameters (intensity, duration, thickness)
- [x] Animation triggers automatically when focus changes between panels

**Note**: Clean, minimal flash effect that:
- Shows focus change with quick blue border flash
- Doesn't distract with very short duration
- Follows Catppuccin style guide using theme colors
- Returns to normal after flash (no persistent visual effects)
- Configurable via debug window for tuning

### Implementation Details:
1. **Animation Module**: Created `src/animation/` with `config.rs`, `state.rs`, `renderer.rs`, `mod.rs`
2. **Flash-only State Machine**: `AnimationPhase` with only `Idle` and `Flash` variants
3. **Theme Integration**: Uses `ui.visuals().widgets.active.bg_fill` (Catppuccin blue)
4. **Focus Detection**: Tracks previous focused panel to trigger animation on change
5. **Debug Configuration**: Window with sliders for intensity, duration, border thickness
6. **UI Integration**: Updated `side_panel()` and `main_content()` to use animation rendering

## Priority 13: Fix Blockquote Rendering and Math Alignment ✅ COMPLETED 2026-03-06
- [x] Fix blockquote rendering issues (duplicate code, border implementation)
- [x] Fix blockquote vertical alignment (border was higher than text)
- [x] Fix blockquote multi-line scaling (border now scales with text height)
- [x] Fix inline math vertical alignment for short formulas
- [x] Use Typst configuration with `top-edge: "bounds"` and `bottom-edge: "baseline"` for baseline extraction
- [x] Extract baseline position from SVG using two-pass Typst rendering
- [x] Store baseline data in FormulaMetadata (baseline_from_top, svg_height)
- [x] Implement baseline-aligned rendering with accurate offset calculation
- [x] Account for horizontal_wrapped vertical centering in offset formula
- [x] Calibrated ascent ratio to 0.76 for perfect visual alignment
- [x] Enhanced debug visualization (7 colors) for verification
- [x] Set DEBUG_BASELINE = false after successful calibration
- [x] Implement tall SVG handling (discard offset + scaling for height > text height)
- [x] Fix all clippy warnings (removed `#[allow(clippy::clone_on_copy)]`)
- [x] Create comprehensive test post with various blockquote examples
- [x] Update documentation and commit history

**Note**: Math formula baseline alignment is now perfect! Uses Typst's baseline extraction with two-pass rendering to get baseline position, then aligns SVG baseline with text baseline accounting for layout centering. Tall SVGs automatically have baseline offset discarded and are scaled if needed to prevent line spacing disruption. Blockquote rendering now uses GitHub-style 4px solid border with proper padding (one row height horizontal, half row height vertical) and scales correctly with multi-line content.

## Priority 14: Collapsible & Resizable Side Panel ✅ COMPLETED 2026-03-09
- [x] Add toggle button on left panel or top-left of content
- [x] Persist panel state across sessions
- [x] Keep current resizable behavior
- [x] Auto-hide on small screens
- [x] No special keyboard shortcut for toggling (for now)
- [x] Adjust layout when panel is collapsed

**Note**: Quick UX improvement for more screen space when needed.

### Implementation Details:
1. **State extension**: Added `side_panel_collapsed: bool` field to `BlogApp` struct with serialization
2. **Panel configuration**: Custom width control (200px expanded, 40px collapsed) with `default_size()`
3. **Button redesign**: Changed from ☰ to directional arrows «/» positioned after date sort button
4. **Vim shortcuts**: Added `ToggleSidePanel`, `CollapseSidePanel`, `ExpandSidePanel` actions to `ShortcutAction` enum
5. **Shortcut configuration**: Added `zi`/`zm`/`zo` shortcuts to `shortcuts.toml`
6. **Enhanced `Ctrl+←`**: Updated `focus_panel()` to expand collapsed panel before focusing
7. **Mobile auto-collapse**: Added logic to collapse panel when screen width < 768px
8. **Trait updates**: Updated all `ActionExecutor` implementations
9. **Theme toggler fix**: Added theme application logic in `ui()` method
10. **Click detection fix**: Prevent panel focus when interactive elements clicked
11. **Auto-expand removal**: Removed conflicting auto-expand logic
12. **Code quality**: Fixed all clippy warnings and unused variables
13. **Testing**: All 42 existing tests pass, doctests updated

## Priority 15: Complete Label/Tag System ✅ COMPLETED 2026-03-09
- [x] Make labels interactive (click to search)
- [x] Assign colors from Catppuccin palette
- [x] Implement tag autocomplete in search bar
- [x] Support multiple tag selection with visual chips
- [x] Allow backspace to remove selected tags
- [x] Combine tag search with text search (AND logic)
- [x] Optional tag descriptions (show on hover)
- [x] Tag filtering in post lists

**Note**: Complex but powerful feature for content discovery. See [TAG_SYSTEM.md](TAG_SYSTEM.md) for detailed specification.

### Implementation Details:
1. **Tag module**: Created `src/tags/mod.rs` with `Tag`, `TagSearchState` structs and utility functions
2. **Catppuccin colors**: 12-color palette with hash-based assignment for consistent tag colors
3. **Interactive tag chips**: Clickable colored pills with hover tooltips showing post counts
4. **Enhanced search bar**: `tag_search_bar()` component with:
   - Tag autocomplete when typing `#`
   - Visual tag chips in search bar
   - Backspace to remove tags
   - Clear search button
5. **Tag components**: `src/ui/tag_components.rs` with reusable UI widgets
6. **Search integration**: Combined AND logic for tags + text search
7. **URL routing**: Tag searches update browser URL and are bookmarkable
8. **State persistence**: Tag search state saved across sessions
9. **Performance optimizations** (2026-03-09):
   - Cached tag color palettes per theme using `OnceLock`
   - Reduced duplicate tag computations (once per frame instead of twice)
   - Explicit cache invalidation when theme changes
   - Fixed frame spikes during theme toggling
   - All tag colors change synchronously with other UI elements
 10. **UI integration**: Tags are interactive in post lists, post metadata, and search results

## Priority 16: Fix Math Formula Rendering in Blockquotes, Lists, and Parentheses ✅ COMPLETED 2026-03-10
- [x] Fix math formulas not rendering in blockquotes (shows `(****.typ)` instead of SVG)
- [x] Fix math formulas not rendering in lists (shows `(****.typ)` instead of SVG)
- [x] Fix math formulas surrounded by parentheses: `($x$)` transforms to `((xxxx.typ))` instead of SVG
- [x] Root cause: Search and replace logic in SVG rendering fails in nested markdown contexts
- [x] Investigate markdown parser context handling for blockquotes and lists
- [x] Fix regex or replacement logic for formulas with surrounding parentheses
- [x] Test formulas in various nested markdown contexts
- [x] Ensure all math formulas render correctly regardless of surrounding syntax

**Note**: Math formulas now render correctly in all markdown contexts. The fix involved:
1. **Created utility functions**: `process_text_with_math()` and enhanced `render_paragraph_content_vec()` to handle math placeholders
2. **Fixed parentheses issue**: Updated `extract_and_replace_math_formulas()` to detect when formulas are inside parentheses
3. **Updated all markdown contexts**: Blockquotes, lists, headings, bold, italic, strikethrough, and links now process math placeholders
4. **Added test post**: `test_math_contexts.md` with comprehensive examples
5. **All tests pass**: 47 tests pass with no regressions

### Implementation Details:
- **Utility functions**: Created reusable `process_text_with_math()` that converts text with `(hash.typ)` placeholders to `Vec<ParagraphContent>`
- **Context fixes**: Updated 12 markdown contexts to use the new utility functions instead of raw text rendering
- **Parentheses handling**: Formulas inside `($x$)` now render as `(hash.typ)` instead of `((hash.typ))`
- **Performance**: Maintained existing caching and performance optimizations
- **Backward compatibility**: All existing functionality preserved

## Priority 17: Advanced Typography ✅ COMPLETED 2026-03-11
- [x] Add support for real bold fonts (font weight changes, not just color)
- [x] Research egui font loading and font family support
- [x] Implement proper font weight variations (light, regular, medium, bold, italic)
- [x] Add italic font support with Ubuntu-Italic font
- [x] Add custom font loading for better typography
- [x] Test font rendering performance and WASM size impact
- [x] Ensure font licensing compliance (Ubuntu fonts are freely licensed)

**Note**: Implemented true bold/italic font support with Ubuntu font variants. Fixed WASM heading color issue by removing `.strong()` which only changes color and has timing issues in WASM.

### Implementation Details:
1. **Font Variants**: Added Ubuntu font variants (Light-300, Regular-400, Medium-500, Bold-700, Italic) to `epaint_default_fonts/fonts/`
2. **Typography Module**: Created comprehensive `typography.rs` module with:
   - `configure_typography()` - Proper font loading with `entry().or_default().insert()` pattern
   - `content_text_styles()` - 16 custom text styles for content with proper weight progression
   - `ui_text_styles()` - Standard UI text styles
   - `apply_text_styles()` - Using `ctx.all_styles_mut()` pattern
   - `verify_text_styles_available()` - Font loading verification
3. **Font Weight Progression**:
   - Normal text: `Ubuntu-Light` (300 weight) via "Content" family
   - Regular text: `Ubuntu-Regular` (400 weight) via "ContentRegular" family  
   - Medium text: `Ubuntu-Medium` (500 weight) via "ContentMedium" family
   - Bold text: `Ubuntu-Bold` (700 weight) via "ContentBold" family
   - Italic text: `Ubuntu-Italic` via "ContentItalic" family
4. **Markdown Rendering Fix**: Updated to use true bold/italic fonts instead of `.strong()`/`.italics()`:
   - Added `bold_text_style()` and `italic_text_style()` helper functions
   - Removed `.strong()` from headings and bold text (caused WASM timing issues)
   - Proper heading hierarchy (H1: 36px, H2: 28px, H3: 24px, H4: 20px, etc.)
5. **Font Loading State Tracking**: Added `FontLoadingState` enum (`Loading`, `Ready`, `Failed`) to handle asynchronous font loading in WASM (fix from egui discussion #4449)
6. **WASM Heading Color Fix**: Fixed issue where heading color was black initially in WASM by removing `.strong()` which uses `visuals.widgets.active.fg_stroke.color` that has timing issues in WASM
7. **Testing**: Created comprehensive tests for typography and font weight progression, all tests pass

### Key Features:
- **True bold fonts**: Uses separate font families for different weights, not just color changes
- **WASM compatibility**: Fixed timing issues with font loading and `.strong()` color
- **Font loading robustness**: State tracking prevents crashes from asynchronous font loading
- **Consistent typography**: Follows existing code conventions and patterns
- **No defensive fallbacks**: System panics if fonts/text styles aren't available (as requested)

## Priority 18: Fix Search Bar Crash with '#' Character and Tag Icon Bug ✅ COMPLETED 2026-03-12
- [x] Fix WASM crash when typing '#' in search bar
- [x] Root cause: Autocomplete menu with tags causing NaN rect in `advance_cursor_after_rect`
- [x] Error: `panicked at crates/egui/src/ui.rs:1407:9: rect is nan in advance_cursor_after_rect`
- [x] Investigate tag autocomplete menu rendering logic
- [x] Fix NaN values in UI layout calculations
- [x] Add defensive checks for invalid rect dimensions
- [x] Test with various '#' input scenarios
- [x] Ensure tag autocomplete works without crashes
- [x] **Bug report**: Wrong cross icon in search bar tag chips
  - When a tag is selected in the search bar, it shows as a colored chip with text `#tagname ✕`
  - Expected: Should show `#tagname ❌` where `❌` is the correct removal icon
  - Current: Shows `#tagname ✕` where `✕` is the wrong removal icon
  - Location: `selected_tags_chips()` function in `src/ui/tag_components.rs:84`
  - Impact: Minor visual inconsistency, easy to fix
  - Fix: Update `RichText::new(format!("#{tag_name} ✕"))` to `RichText::new(format!("#{tag_name} ❌"))`

**Note**: Critical bug causing app crash when typing '#' for tag autocomplete. Fixed with comprehensive solution addressing multiple related issues.

### Implementation Details:
1. **Fixed WASM crash**: Created `get_dropdown_position()` function with fallback to prevent NaN coordinates in WASM
2. **Fixed cross icon**: Changed from `✕` to `❌` in selected tags chips (`src/ui/tag_components.rs:112`)
3. **Fixed cursor positioning**: Removed automatic URL updates during typing (caused focus/cursor corruption in WASM)
4. **Added stable widget IDs**: Added IDs to all text fields for proper focus detection:
   - Search bar: `egui::Id::new("tag_search_input")`
   - New post title: `egui::Id::new("new_post_title")`
   - New post content: `egui::Id::new("new_post_content")`
   - Find dialog: `egui::Id::new("find_dialog_input")`
5. **Fixed state persistence**: Added `sync_state_to_route()` call in browser URL branch of `restore_state_with_precedence()` to fix refresh navigation
6. **Fixed inconsistent URL updates**: `navigate_post()` now calls `navigate_to()` after successful keyboard navigation
7. **Fixed tab switching behavior**: Tab switching now only filters, doesn't navigate or select posts
8. **Implemented search URL updates**: Enter key commits search to URL (`#/search?q=...&tags=...`)
9. **Fixed Bug 1**: Clear search text after selecting tag from autocomplete menu
10. **Fixed Bug 2**: Prevent single-character shortcuts when typing in text fields with proper focus detection
11. **Created test case**: `simple_search_test.rs` to isolate cursor positioning issues without tag functionality
12. **All tests pass**: 8 tag autocomplete tests + all existing tests pass

### Key Fixes:
- **WASM crash prevention**: `get_dropdown_position()` with fallback prevents NaN rects
- **Cursor positioning**: No URL updates during typing prevents focus corruption
- **State persistence**: Browser URL takes precedence over persisted state on refresh
- **Keyboard navigation**: Consistent URL updates for both mouse and keyboard navigation
- **Shortcut blocking**: Single-character shortcuts (`i`, `j`, `k`, `h`, `l`) don't trigger when typing in text fields
- **Visual consistency**: Cross icon fixed from `✕` to `❌` in tag chips



## Priority 19: Build-time Filtering of Posts with Tag #test ✅ COMPLETED 2026-03-13
- [x] Extend `blog_macros::embed_file_array!` macro to accept optional filter callback
- [x] Add frontmatter parsing in build script (`build.rs`) to detect `#test` tag
- [x] Implement conditional filtering based on `#[cfg(debug_assertions)]` (debug vs release)
- [x] Update `load_embedded_content()` to handle filtered post arrays
- [x] Add build configuration via profile-based filtering (no extra features needed)
- [x] Test both dev and release builds to ensure proper filtering
- [x] Ensure backward compatibility (no filtering by default in debug builds)
- [x] Add fallback to include post if parsing fails (safer approach)

**Note**: Posts tagged with `#test` are excluded from release builds to keep production content clean. Uses runtime filtering with zero runtime overhead for unfiltered case. Implementation includes YAML frontmatter parsing and profile-based conditional compilation.

### Implementation Details:
1. **Extended `embed_file_array!` macro**: Added optional `filter` parameter that accepts a function `Fn(&str) -> bool`
2. **Created `build_filter` module**: Contains `filter_test_posts()` function that uses `cfg!(debug_assertions)` to determine filtering behavior
3. **Profile-based filtering**: 
   - Debug builds (`cargo build`): Include all posts (including test posts)
   - Release builds (`cargo build --release`): Exclude posts with `#test` tag
4. **Frontmatter parsing**: Uses `serde_yaml` to parse YAML frontmatter and check `tags` array for `"test"`
5. **Safe fallback**: If YAML parsing fails, post is included (prevents accidental exclusion)
6. **Runtime filtering**: Applied once at startup when posts are loaded, minimal overhead

### Key Features:
- **Profile-aware**: Automatic filtering based on build profile
- **Zero config**: No environment variables or feature flags needed
- **Backward compatible**: Debug builds behave exactly as before
- **Safe**: Parsing failures don't cause posts to be excluded
- **Tested**: All existing tests pass, new unit tests for filtering logic

## Priority 20: Table of Contents in Collapsible Right Panel ✅ COMPLETED 2026-03-15
- [x] Add TOC data structure to `BlogPost` with hierarchical heading support
- [x] Parse headings during post loading in `parse_post_content()` with URL-friendly ID generation
- [x] Extend `Route` enum to support heading fragments: `Post { slug: String, fragment: Option<String> }`
- [x] Update router to parse `#/posts/slug#heading-id` format and handle fragment navigation
- [x] Add `right_panel_collapsed: bool` to `BlogApp` with persistence support
- [x] Create `right_panel()` function in `layout.rs` with TOC rendering
- [x] Implement click-to-scroll navigation using `ui.scroll_to_cursor()`
- [x] Add collapsible sections with `CollapsingHeader` for nested headings
- [x] Handle URL/LocalStorage conflicts (fragment IDs vs SPA routing hash)
- [x] Test TOC with various heading structures and nesting levels

**Note**: Interactive table of contents with heading navigation and URL fragment support. Shows all headings with nested indentation and collapsible sub-levels. Clicking TOC items scrolls to corresponding heading and updates URL with fragment. Basic functionality includes TOC generation, right panel UI, and fragment navigation. Advanced features (scroll tracking, keyboard shortcuts) are optional enhancements.

## Priority 21: Fix Panel Focus and Scroll Position Persistence ✅ COMPLETED 2026-03-15
- [x] Fix RON serialization by adding `#[serde(skip)]` to `route_restored` field
- [x] Add `Serialize`/`Deserialize` derives to `FocusAnimationState` and `AnimationPhase`
- [x] Change default focused panel from `LeftPanel` to `RightPanel` as requested
- [x] Update `test_focused_panel_persistence` test to expect `RightPanel` default
- [x] Scroll position persistence now uses egui's `id_salt()` with dynamic post-based IDs
- [x] Improved main content click detection with multiple methods (`interact_pos`, `press_origin`, `latest_pos`)
- [x] Handle corrupted LocalStorage data gracefully with fallback to defaults
- [x] Remove unused `save_current_scroll_position` and `restore_current_scroll_position` methods
- [x] Fix all clippy warnings (replace `println!` with `log::debug!`, suppress intentional `eprintln!`)

**Note**: Fixed urgent bug where panel focus and scroll position were not restored after browser refresh. Scroll position persistence now uses egui's built-in `id_salt()` mechanism with dynamic IDs per post (`main_content_scroll_{post_key}`). Panel focus persistence fixed by correcting RON serialization issues. When corrupted LocalStorage data exists, app falls back to defaults and overwrites with correct data on next save.

## Priority 22: Fix Theme Persistence Bug
- [ ] Selected theme is not persisted over refresh - page always goes to bright theme after refresh
- [ ] Investigate why theme state is not being saved/restored correctly
- [ ] Check serialization/deserialization of Theme enum
- [ ] Verify LocalStorage save/restore logic for theme preference
- [ ] Test theme persistence across browser refreshes

**Note**: Users report that after refreshing the page, the theme always resets to bright theme instead of preserving the selected theme.

## Priority 23: Fix Math Formula Replacement Logic for Parentheses
- [ ] Typst math formulas originally wrapped inside parentheses are not replaced with rendered SVG
- [ ] Root cause: Replacement logic error - should detect innermost `(xxxx.typ)` and replace with SVG image
- [ ] Current issue: `($x$)` transforms to `((xxxx.typ))` instead of `(xxxx.typ)` → SVG
- [ ] Fix regex or replacement logic to handle parentheses correctly
- [ ] Test formulas in various contexts with parentheses
- [ ] Ensure all math formulas render correctly regardless of surrounding syntax

**Note**: Math formulas inside parentheses show as `(xxxx.typ)` placeholders instead of rendered SVGs due to incorrect replacement logic.

## Priority 24: Fix Main Content Panel Scroll Area Margins
- [ ] The scroll area in the main panel/content panel should include the margins on both sides
- [ ] Current issue: Scroll area doesn't include responsive margins, causing content to appear cut off
- [ ] Investigate ScrollArea configuration and container nesting
- [ ] Ensure responsive margins are applied within the scrollable region
- [ ] Test scrolling behavior with various content widths
- [ ] Fix layout so content scrolls with proper margins on both sides

**Note**: The main content scroll area doesn't include the responsive margins, making content appear incorrectly positioned during scrolling.

## Priority 25: Implement gg and G Keyboard Shortcuts
- [ ] Add vim-like `gg` and `G` keyboard shortcuts for scrolling/navigation
- [ ] Context-sensitive behavior:
  - `gg` in LeftPanel (post list) → navigate to first post
  - `gg` in RightPanel (content) → scroll to top of current post
  - `G` (Shift+g) in LeftPanel → navigate to last post  
  - `G` (Shift+g) in RightPanel → scroll to bottom of current post
- [ ] Keep existing `Home`/`End` keys for post navigation
- [ ] Ensure shortcuts work without requiring mouse movement (app should repaint to check for keyboard input)
- [ ] Test both web (WASM) and native targets
- [ ] Add to shortcuts.toml configuration

**Note**: Vim-style navigation shortcuts for quick top/bottom navigation and scrolling. Different behavior based on which panel has focus.

## Priority 26: Dynamic Content Loading (Low Priority)
- [ ] Evaluate dynamic loading benefits vs complexity
- [ ] Research HTTP fetching with ehttp crate
- [ ] Design async loading architecture
- [ ] Implement post loading from server/API
- [ ] Implement SVG/PNG fetching for math formulas
- [ ] Add loading states and error handling
- [ ] Implement client-side caching
- [ ] Test offline fallback behavior

**Note**: Dynamic loading would reduce WASM size by moving posts (~10KB) and SVGs (~105KB) out of binary, but requires server infrastructure and adds network complexity. The SVG rendering stack (~3MB) would still be needed unless formulas are pre-rasterized server-side.

## Completed Tasks
✅ **Foundation (2026-02-13)**
- Basic blog UI with panels and navigation
- Modular architecture (posts/ + ui/ modules)
- Dual-target compilation (native + wasm32)
- Fixed layout container issues
- Resolved emoji rendering problems
- Cleaned up unused code and warnings
- Created build and server scripts

✅ **Math Formula Rendering (2026-02-27)**
- Theme-aware SVG rendering with transparent backgrounds
- Paragraph accumulation for proper inline math layout
- Fixed horizontal spacing between text and math images
- Display math formulas centered with proper spacing
- All tests passing, web build successful

✅ **State Persistence (2026-02-27)**
- App state saved across browser refreshes using LocalStorage
- Selected post, theme, search, layout preserved
- Auto-save every 30 seconds
- Post content and math SVGs reloaded from source (not serialized)
- Follows egui demo app persistence pattern

✅ **Performance Optimizations (2026-03-01)**
- Math manifest caching with `OnceLock` (3752× faster loading)
- Formula reverse index for O(1) lookup instead of O(n) linear search  
- Markdown processing cache to avoid reprocessing static content every frame
- Removed unused code and cleaned up function hierarchy
- Added benchmark tests showing significant performance improvements
- Fixed all clippy warnings and code quality issues

✅ **Enhanced Styling & Theme System (2026-03-02)**
- Simplified theme system to only 2 Catppuccin themes (Latte light / Macchiato dark)
- Fixed critical strong text contrast bug in Catppuccin themes
- Implemented Catppuccin style guide compliance:
  - "On Accent" text uses Base color (for buttons)
  - Selection uses Overlay 2 with 25% opacity
  - Links use Blue color (not Sapphire)
  - Semantic colors follow style guide (Yellow=warnings, Red=errors)
- Implemented high-contrast strong text colors:
  - Light theme: Sapphire (distinct but not aggressive)
  - Dark theme: Peach (good contrast on dark backgrounds)
- Improved theme toggle to single button (shows opposite theme icon)
- Cleaned up UI by removing "Theme:" label and theme name display
- All tests pass, no clippy warnings

✅ **URL Routing & Navigation (2026-03-03)**
- Implemented Router struct to encapsulate routing state and logic
- Hash-based URL routing (`#/post/slug`, `#/search?q=query`, `#/tags/tag`, `#/`)
- Auto-generated URL-friendly slugs from post titles
- Browser history navigation (back/forward) with route persistence
- NavigationContext for UI components with route and callback
- 404 error pages with "Return to Home" navigation
- Basic query parameter support for search queries and tags
- Router state saved across browser refreshes using serde serialization
- 8 routing-specific tests added, all existing tests pass

✅ **Animated Focus Indicators (2026-03-05)**
- Replaced static blue border with animated flash indicator
- Flash-only animation (100ms duration) shows focus change clearly
- Uses Catppuccin blue color from theme (widgets.active.bg_fill)
- No persistent tint - panel returns to normal after flash
- Debug-configurable parameters (intensity, duration, thickness)
- Animation triggers automatically when focus changes between panels
- Follows Catppuccin style guide for focus states
- All clippy warnings fixed, code compiles cleanly

✅ **Blockquote Rendering & Math Alignment (2026-03-06)**
- Fixed blockquote vertical alignment (border now aligns with text)
- Fixed blockquote multi-line scaling (border scales with text height)
- Fixed all clippy warnings (removed `#[allow(clippy::clone_on_copy)]`)
- Created comprehensive test post with various blockquote examples
- Perfect math formula baseline alignment with 0.76 ascent ratio
- Tall SVG handling prevents line spacing disruption
- GitHub-style 4px solid border with proper padding (row height horizontal, half row height vertical)

 ## Git Checkpoints
- `fdd9f4ec` - Initial blog app with web and native support
- `6ace4f51` - Clean up blog_app crate warnings and unused code
- `d3dcb0d7` - WIP: Implement paragraph accumulation for inline math rendering
- `5b39f118` - Fix horizontal spacing for inline math formulas
- `66d90429` - Performance optimizations: manifest caching, reverse index, markdown cache
- `a0b6c22e` - Fix Catppuccin style guide compliance and strong text visibility
- `54a14fe3` - Improve theme toggle to single button
- `f12f4fb4e` - Implement URL routing with Router encapsulation (Priority 8)
- `e3c3e42a7` - WIP: Simplified focus animation system - flash-only with Catppuccin blue (Priority 12)
- `569ec6f63` - WIP: Implement baseline alignment for math formulas (Priority 13)
- `25eec7e04` - WIP: Implement tall SVG fix for math formulas
- `1aeb1b58c` - WIP: Fix blockquote rendering issues
- `710c59889` - fix: Complete blockquote vertical alignment and multi-line support
- `e03221867` - feat: Fix theme toggle navigation bug (Priority 9 fix)
- `a715e2ec6` - wip: Fix math formula rendering in all markdown contexts (Priority 17)
- `870e08098` - feat: Complete math formula rendering fix for all markdown contexts (Priority 17)
- `ef5976c40` - docs: Update TODO.md with tag icon bug report in Priority 16
- `b75920e02` - docs: Add two new priorities to TODO.md
- *Add checkpoint after each priority completion*

## Minor Issues for Future Improvement

### Inline Formula Vertical Alignment ✅ COMPLETED 2026-03-05
- **Issue**: When an inline formula SVG has significant height, the text following it appears lower than text before the formula
- **Cause**: The image widget's height increases the line height, and text is vertically centered within that line
- **Solution**: Implemented baseline alignment using Typst baseline extraction and accurate offset calculation
- **Result**: Perfect baseline alignment with calibrated 0.76 ascent ratio
- **Status**: ✅ Fixed - All formulas now align perfectly with text baseline

### Other Minor Issues
- **Formula size consistency**: Some formulas appear slightly larger/smaller than others
- **SVG baseline alignment**: ✅ FIXED (2026-03-05) - Perfect baseline alignment with 0.76 ascent ratio
- **Tall SVG handling**: ✅ FIXED (2026-03-06) - Baseline offset discarded and scaling for SVGs taller than text height
- **Performance optimization**: Formula caching could be more intelligent
- **Accessibility**: Screen reader support for math formulas
- **Strong text contrast**: ✅ FIXED (2026-03-02) - Now uses high-contrast colors (Sapphire/Peach) for visibility
- **Real bold fonts**: Currently `.strong()` only changes color, not font weight (see Priority 15)
- **Blockquote rendering**: ✅ FIXED (2026-03-06) - Now uses GitHub-style 4px solid border with proper padding and correct vertical alignment

### Performance Optimizations ✅ COMPLETED 2026-03-01
- **Math formula lookup optimization**: `find_hash` now uses O(1) reverse index lookup instead of O(n) linear search
  - **Before**: 9 formulas × 5 formulas per post × 60 FPS = ~2,700 comparisons/second
  - **After**: O(1) hash map lookup with reverse index `HashMap<(formula, is_display), hash>`
  - **Result**: 313ns average lookup time, scales efficiently with more formulas
- **Math manifest caching**: `load_manifest()` now cached with `OnceLock` instead of parsing JSON every frame
  - **Before**: JSON parsing 60 times/second
  - **After**: One-time initialization with static reference
  - **Result**: 3752× faster (75µs cold → 20ns average)
- **Markdown processing caching**: `extract_and_replace_math_formulas` now preprocessed at load time
  - **Before**: Same formula extraction repeated 60 times/second on static content
  - **After**: Content preprocessed when `BlogPost` is created, cached with `(hash.typ)` placeholders
  - **Result**: Eliminates reprocessing of static markdown every frame
- **HashMap iteration warnings**: Fixed by sorting entries for deterministic iteration
  - **Build script**: Now sorts by hash before building reverse index
  - **Runtime**: All `#[allow(clippy::iter_over_hash_type)]` attributes removed

## Notes
### New Development Workflow (2026-02-27)
- **Development server**: `cargo run --bin blog_web_server --features dev` (port 8766)
  - File watching with auto-rebuild on changes
  - Compiler errors shown in terminal
  - Server continues running after rebuild failures
- **Production server**: `cargo run --bin blog_web_server --features dev -- --serve-release`
  - Optimized builds with wasm-opt
  - No file watching
- **Native desktop**: `cargo run --bin blog_native`
- **Using aliases**: `cargo blog`, `cargo blog-release`, `cargo blog-native`
- **Build outputs**: `web_blog/dev/` (development), `web_blog/release/` (production)

### Command Options
```bash
--port 9999           # Custom port (default: 8766)
--open                # Open browser automatically
--build-only          # Build only, don't start server
--log-level info      # Control log verbosity (debug, info, warn, error)
```

### Legacy Scripts (Deprecated)
- `./scripts/start_server_blog.sh` - Use `blog_web_server` binary instead
- `./scripts/build_blog_web.sh` - Use `blog_web_server --build-only --serve-release`
- `./scripts/watch_blog.sh` - Use `blog_web_server` (development mode)
- `./scripts/setup_web.sh` - Tools auto-installed by `blog_web_server`

Test changes via web interface at http://localhost:8766