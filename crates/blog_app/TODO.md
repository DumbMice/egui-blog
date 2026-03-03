# Blog App TODO List

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

## Priority 9: Multiple Content Types & Tabs
- [ ] Support different content types: blog posts, private notes, research reviews
- [ ] Implement tab-based navigation between content types
- [ ] Shared search database across all content types
- [ ] Same Markdown format with type-specific frontmatter
- [ ] Different directories: `posts/`, `notes/`, `reviews/`
- [ ] Filterable views or separate navigation sections

**Note**: Single WASM app with multiple content sections. Tabs as starting navigation pattern.

## Priority 10: Complete Label/Tag System
- [ ] Make labels interactive (click to search)
- [ ] Assign colors from Catppuccin palette
- [ ] Implement tag autocomplete in search bar
- [ ] Support multiple tag selection with visual chips
- [ ] Allow backspace to remove selected tags
- [ ] Combine tag search with text search (AND logic)
- [ ] Optional tag descriptions (show on hover)
- [ ] Tag filtering in post lists

**Note**: Complex but powerful feature for content discovery. See [TAG_SYSTEM.md](TAG_SYSTEM.md) for detailed specification.

## Priority 11: Keyboard Shortcuts & Vim Navigation
- [ ] Basic navigation (arrow keys, Home/End)
- [ ] Vim navigation (j/k for up/down, gg/G for top/bottom)
- [ ] `/` for page content search (not post search)
- [ ] Alt+D to focus browser address bar
- [ ] Always-on vim mode (not toggleable)
- [ ] Non-configurable shortcuts (hardcoded)

**Note**: Power user features for efficient navigation. Vim mode always enabled.

## Priority 12: Improved Responsive Layout
- [ ] Central content width: 80-100 characters optimal
- [ ] Responsive margins that adjust with zoom level
- [ ] Max-width content container with auto margins
- [ ] Breakpoint for mobile vs desktop layouts
- [ ] Content width reduces when page width < 80-100 chars
- [ ] Desktop-only responsive behavior

**Note**: Improves readability with proper typographic margins.

## Priority 13: Collapsible & Resizable Side Panel
- [ ] Add toggle button on left panel or top-left of content
- [ ] Persist panel state across sessions
- [ ] Keep current resizable behavior
- [ ] Auto-hide on small screens
- [ ] No special keyboard shortcut for toggling (for now)
- [ ] Adjust layout when panel is collapsed

**Note**: Quick UX improvement for more screen space when needed.

## Priority 14: Advanced Typography (Future Enhancement)
- [ ] Add support for real bold fonts (font weight changes, not just color)
- [ ] Research egui font loading and font family support
- [ ] Implement proper font weight variations (light, regular, bold, etc.)
- [ ] Add italic font support if not already available
- [ ] Consider adding custom font loading for better typography
- [ ] Test font rendering performance and WASM size impact
- [ ] Ensure font licensing compliance for any bundled fonts

**Note**: Currently `.strong()` only changes text color due to egui limitations. Real bold fonts would require proper font loading and font family support.

## Priority 15: Dynamic Content Loading (Low Priority)
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

## Git Checkpoints
- `fdd9f4ec` - Initial blog app with web and native support
- `6ace4f51` - Clean up blog_app crate warnings and unused code
- `d3dcb0d7` - WIP: Implement paragraph accumulation for inline math rendering
- `5b39f118` - Fix horizontal spacing for inline math formulas
- `66d90429` - Performance optimizations: manifest caching, reverse index, markdown cache
- `a0b6c22e` - Fix Catppuccin style guide compliance and strong text visibility
- `54a14fe3` - Improve theme toggle to single button
- `f12f4fb4e` - Implement URL routing with Router encapsulation (Priority 8)
- *Add checkpoint after each priority completion*

## Minor Issues for Future Improvement

### Inline Formula Vertical Alignment
- **Issue**: When an inline formula SVG has significant height, the text following it appears lower than text before the formula
- **Cause**: The image widget's height increases the line height, and text is vertically centered within that line
- **Current behavior**: Text before and after tall inline formulas may appear at different vertical positions
- **Potential fix**: Adjust vertical alignment of text or images to maintain consistent baseline
- **Priority**: Low - functional but visually imperfect

### Other Minor Issues
- **Formula size consistency**: Some formulas appear slightly larger/smaller than others
- **SVG baseline alignment**: Could improve vertical positioning of formula glyphs
- **Performance optimization**: Formula caching could be more intelligent
- **Accessibility**: Screen reader support for math formulas
- **Strong text contrast**: ✅ FIXED (2026-03-02) - Now uses high-contrast colors (Sapphire/Peach) for visibility
- **Real bold fonts**: Currently `.strong()` only changes color, not font weight (see Priority 8)

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