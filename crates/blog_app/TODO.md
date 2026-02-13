# Blog App TODO List

## Priority 1: Content Separation
- [x] Define post file format (Markdown + YAML frontmatter)
- [x] Create posts directory structure
- [x] Implement markdown file loader
- [x] Add frontmatter parser (YAML)
- [x] Update PostManager to use file loading
- [ ] Add file watcher for live reload (development)
- [x] Implement compile-time embedding (production)
- [x] Create example post files
- [x] Test loading and display
- [ ] Update UI to handle missing posts gracefully

## Priority 2: Markdown Rendering
- [x] Evaluate markdown rendering options
- [x] Choose and integrate markdown parser
- [x] Implement basic text rendering (headings, paragraphs)
- [x] Add emphasis rendering (bold, italic)
- [x] Implement code block support
- [x] Add language labels to code blocks
- [ ] Add syntax highlighting
- [x] Support links and images (basic image placeholder)
- [x] Add strikethrough support
- [x] Add list rendering (ordered/unordered)
- [x] Improve list spacing and visual markers
- [x] Implement blockquotes and horizontal rules
- [ ] Add table support (optional)

## Priority 3: Enhanced Styling
- [ ] Design custom theme system
- [ ] Implement color customization
- [ ] Improve typography (fonts, spacing)
- [ ] Add responsive layout adaptations
- [ ] Implement smooth theme transitions
- [ ] Polish UI spacing and borders
- [ ] Add visual feedback for interactions

## Completed Tasks
✅ **Foundation (2026-02-13)**
- Basic blog UI with panels and navigation
- Modular architecture (posts/ + ui/ modules)
- Dual-target compilation (native + wasm32)
- Fixed layout container issues
- Resolved emoji rendering problems
- Cleaned up unused code and warnings
- Created build and server scripts

## Git Checkpoints
- `fdd9f4ec` - Initial blog app with web and native support
- `6ace4f51` - Clean up blog_app crate warnings and unused code
- *Add checkpoint after each priority completion*

## Notes
- Server runs on port 8766 (`./scripts/start_server_blog.sh`)
- Build with `./scripts/build_blog_web.sh`
- Test changes via web interface at http://localhost:8766