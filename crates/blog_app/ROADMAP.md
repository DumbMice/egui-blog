# Blog App Development Roadmap

## Current Status (2026-02-27)
✅ **Foundation Complete**
- Basic blog UI with post listing, viewing, and creation
- Modular architecture (posts/ + ui/ modules)
- Dual-target support (native + wasm32)
- All reported layout and rendering issues fixed
- Code cleanup completed (no unused code/warnings)

✅ **Content Separation Complete**
- Posts loaded from external Markdown files with YAML frontmatter
- File watcher for live reload during development
- Compile-time embedding for production builds
- Graceful handling of missing posts

✅ **Markdown Rendering Complete**
- Full markdown support with pulldown-cmark
- Syntax highlighting for code blocks
- Tables, lists, blockquotes, horizontal rules
- Links, images, emphasis, strikethrough

✅ **Math Formula Rendering Complete**
- Typst math formula rendering to SVG
- Theme-aware colors (white in dark mode, black in light mode)
- Inline and display math support
- Proper paragraph accumulation and spacing
- All tests passing, production-ready

## Priority 1: Content Separation
**Problem:** Blog posts are hardcoded in Rust source code (`posts/mod.rs:add_example_posts()`)
**Goal:** Load posts from external Markdown files with frontmatter

### Implementation Steps:
1. **Define post file format** - Markdown with YAML frontmatter
2. **Create posts directory** - `posts/` with `.md` files
3. **Implement file loader** - Load and parse markdown files at runtime
4. **Add file watcher** - Live reload during development (`notify` crate)
5. **Compile-time embedding** - Use `include_str!` for production builds
6. **Update PostManager** - Replace hardcoded posts with file loading

### File Format Example:
```markdown
---
title: "Welcome to My Blog"
date: "2026-02-10"
tags: ["welcome", "introduction"]
---

This is my first blog post using egui!

## Features
- **Fast**: Compiled to WebAssembly
- **Simple**: No JavaScript framework
- **Rust**: Safety and performance
```

## Priority 2: Markdown Rendering
**Current:** Plain text display (`Label::new(&post.content).wrap()`)
**Goal:** Rich formatting with proper markdown rendering

### Implementation Steps:
1. **Evaluate renderer options** - `egui_markdown` vs custom implementation
2. **Integrate markdown parser** - `pulldown-cmark` or similar
3. **Implement basic rendering** - Headings, paragraphs, emphasis
4. **Add code block support** - Syntax highlighting
5. **Support links and images** - With proper egui widgets
6. **Advanced elements** - Lists, blockquotes, tables

## Priority 3: Math Formula Rendering (COMPLETED 2026-02-27)
**Problem:** Need to render mathematical formulas in blog posts
**Goal:** Support Typst math syntax with theme-aware rendering

### Implementation Steps (Completed):
1. **Typst integration** - Use Typst CLI to render formulas to SVG
2. **Build script** - Process math formulas during cargo build
3. **SVG processing** - Make backgrounds transparent, theme-aware colors
4. **Inline vs display math** - Support `$formula$` (inline) and `$$ formula $$` (display)
5. **Paragraph accumulation** - Proper inline rendering without line breaks
6. **Horizontal spacing fix** - Set `item_spacing.x = 0.0` for seamless text flow
7. **Theme adaptation** - Formulas use `text_color()` to match body text
8. **Caching system** - Manifest tracks rendered formulas for performance

### Key Features:
- ✅ Formulas appear white in dark mode, black in light mode
- ✅ Inline formulas flow seamlessly with text (no extra spacing)
- ✅ Display formulas are centered with proper vertical spacing
- ✅ Automatic rebuild when formulas change
- ✅ All tests passing, web build successful

## Priority 4: Enhanced Styling
**Opportunities:** Custom themes, typography, responsive layouts

### Implementation Steps:
1. **Custom theme system** - Beyond light/dark, allow color customization
2. **Typography improvements** - Font sizes, line heights, spacing
3. **Responsive layouts** - Adapt to different screen sizes
4. **Animation support** - Smooth theme transitions
5. **UI polish** - Better spacing, borders, shadows

## Priority 5: Advanced Features (Long-term)

### 4.1 Comments System
- Persistent storage (localStorage for web, files for native)
- Threaded comments with replies
- Moderation tools
- Markdown support in comments

### 4.2 Data Visualization
- Graphs and charts using `egui_plot`
- Integration with markdown via custom syntax
- Interactive plots with tooltips

### 4.3 3D Rendering
- `egui_wgpu` integration for embedded 3D views
- WebGL support for wasm target
- Simple scene editor

### 4.4 Enhanced Search
- Tag filtering
- Full-text search with indexing
- Search result highlighting
- Advanced query syntax

## Development Workflow
- **Git checkpoints** - Commit after each major feature
- **Testing** - Manual testing via web server (port 8766)
- **Build system** - Unified Rust binary with hot reload (`cargo blog`)
- **Server** - Development server with auto-rebuild on file changes

## Technical Considerations
- **WASM size** - Keep dependencies minimal
- **Performance** - Efficient rendering for long posts
- **Accessibility** - Screen reader support
- **SEO** - Server-side rendering for search engines (future)

## Dependencies to Evaluate
- `pulldown-cmark` - Markdown parsing
- `serde_yaml` - Frontmatter parsing
- `notify` - File watching for development
- `egui_markdown` - If available and maintained
- `syntect` or `tree-sitter` - Syntax highlighting
- `egui_plot` - Chart rendering