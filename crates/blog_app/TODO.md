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

## Priority 6: Enhanced Styling
- [ ] Design custom theme system
- [ ] Implement color customization
- [ ] Improve typography (fonts, spacing)
- [ ] Add responsive layout adaptations
- [ ] Implement smooth theme transitions
- [ ] Polish UI spacing and borders
- [ ] Add visual feedback for interactions

## Priority 7: Dynamic Content Loading (Low Priority)
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

## Git Checkpoints
- `fdd9f4ec` - Initial blog app with web and native support
- `6ace4f51` - Clean up blog_app crate warnings and unused code
- `d3dcb0d7` - WIP: Implement paragraph accumulation for inline math rendering
- `5b39f118` - Fix horizontal spacing for inline math formulas
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

### Performance Optimizations Needed
- **Math formula lookup optimization**: `find_hash` uses O(n) linear search through HashMap instead of O(1) hash lookup
  - **Current**: 9 formulas × 5 formulas per post × 60 FPS = ~2,700 comparisons/second
  - **Impact**: Minimal now (9 formulas) but scales poorly with more formulas
  - **Fix**: Add reverse index `HashMap<(formula, is_display), hash>` or cache processed markdown text
- **Markdown processing per frame**: `extract_and_replace_math_formulas` runs every frame on static content
  - **Current**: Same formula extraction and hash lookups repeated 60 times/second
  - **Fix**: Cache processed markdown text with `(hash.typ)` placeholders
- **HashMap iteration warnings**: Several `#[allow(clippy::iter_over_hash_type)]` attributes hide potential order-dependent bugs
  - **Build script**: Iterations where order doesn't matter (updating metadata, processing formulas)
  - **Runtime**: `find_hash` linear search through HashMap (design issue, not just iteration order)

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