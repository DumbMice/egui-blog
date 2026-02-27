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

## Priority 4: Enhanced Styling
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

✅ **Math Formula Rendering (2026-02-27)**
- Theme-aware SVG rendering with transparent backgrounds
- Paragraph accumulation for proper inline math layout
- Fixed horizontal spacing between text and math images
- Display math formulas centered with proper spacing
- All tests passing, web build successful

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

## Notes
- Server runs on port 8766 (`./scripts/start_server_blog.sh`)
- Build with `./scripts/build_blog_web.sh`
- Test changes via web interface at http://localhost:8766