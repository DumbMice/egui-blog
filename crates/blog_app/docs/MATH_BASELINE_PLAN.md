# Math Baseline Alignment Implementation Plan

## Overview
Fix inline math formula vertical alignment by extracting baseline position from Typst SVGs and aligning with text baseline in egui.

## Problem Statement
Inline math formulas currently have incorrect vertical alignment. When an inline formula SVG has significant height, the text following it appears lower than text before the formula. This happens because the image widget's height increases the line height, and text is vertically centered within that line.

## Solution Approach
1. **Build-time**: Generate two SVGs per formula to measure baseline position
2. **Metadata**: Store baseline position in formula manifest
3. **Runtime**: Use baseline position to align SVG with text baseline during rendering

## Phase 1: Build System Modifications ✅ In Progress

### 1.1 Extend FormulaMetadata struct with baseline fields ✅ COMPLETED
- Added `baseline_from_top: Option<f32>` - baseline position from top of SVG (SVG units)
- Added `svg_height: Option<f32>` - total SVG height for reference
- Updated both `build.rs` and `embedded.rs` for consistency

### 1.2 Add SVG height parsing utility ✅ COMPLETED
```rust
fn parse_svg_height(svg_content: &str) -> Result<f32>
```
Extract height from viewBox or width/height attributes.

### 1.3 Implement baseline extraction function
```rust
fn extract_baseline_position(formula: &str, is_display: bool) -> Result<(f32, f32)>
```
Generate two SVGs:
- Top: `top-edge: "bounds", bottom-edge: "baseline"`
- Bottom: `top-edge: "baseline", bottom-edge: "bounds"`

Calculate: `baseline_from_top = top_height`, `total_height = top_height + bottom_height`

### 1.4 Modify `create_typst_svg` to extract baseline
- For inline math: extract baseline, store in metadata
- For display math: no baseline needed (centered)
- Keep original SVG generation for rendering

### 1.5 Regenerate all formulas with baseline data
- Clear `assets/math/` directory
- Regenerate all formulas
- Update manifest with baseline metadata

## Phase 2: Asset Loading Updates

### 2.1 Update MathAssetManager
- Add `get_baseline_position()` method
- Add `get_svg_size_with_baseline()` method
- Return `None` for display math, `Some(baseline)` for inline math

## Phase 3: Rendering Logic Updates

### 3.1 Implement baseline-aligned image rendering
**Approach**: Custom painting with `paint_at()` and translation
```rust
fn render_baseline_aligned_image(ui: &mut Ui, image_source, size, baseline_from_top)
```
Calculate offset: `offset_y = metrics.ascent - baseline_from_top`

### 3.2 Update markdown rendering
- Add `baseline_from_top` field to `ParagraphContent::MathImage`
- Use baseline-aligned rendering for inline math
- Keep centered rendering for display math

## Phase 4: Testing

### 4.1 Create test post `test_math_alignment.md`
- Simple formulas: `$x^2$`
- Complex fractions: `$frac{a}{b}$`
- Tall formulas: `$sum_{i=1}^n i$`
- Mixed with text styles
- Multiple inline formulas
- Display math (should remain centered)

## Phase 5: Debug Visualization

### 5.1 Add baseline markers
- Red line: text baseline
- Green line: SVG baseline
- Toggle via `DEBUG_BASELINE` constant
- Remove after stabilization

## Technical Details

### Baseline Calculation
1. Generate top SVG with `top-edge: "bounds", bottom-edge: "baseline"`
2. Generate bottom SVG with `top-edge: "baseline", bottom-edge: "bounds"`
3. Parse heights: `top_height`, `bottom_height`
4. Calculate: `baseline_from_top = top_height`, `total_height = top_height + bottom_height`

### Text Baseline in egui
- Font metrics: `ascent` (distance from baseline to top), `descent` (distance from baseline to bottom)
- Text baseline position: `text_top + metrics.ascent`
- Need to align SVG baseline with text baseline

### Rendering Approach
```rust
// Calculate vertical offset
let offset_y = metrics.ascent - svg_baseline_from_top;

// Render image with offset
let translated_rect = rect.translate(egui::Vec2::new(0.0, offset_y));
image.paint_at(ui, translated_rect);
```

## Implementation Status
- [x] Phase 1.1: Extend FormulaMetadata struct
- [x] Phase 1.2: Add SVG height parsing utility
- [ ] Phase 1.3: Implement baseline extraction function
- [ ] Phase 1.4: Modify create_typst_svg
- [ ] Phase 1.5: Regenerate formulas
- [ ] Phase 2: Asset loading updates
- [ ] Phase 3: Rendering updates
- [ ] Phase 4: Testing
- [ ] Phase 5: Debug visualization

## Notes
- Display math doesn't need baseline alignment (centered)
- Backward compatibility: regenerate all formulas
- Debug visualization helps verify alignment
- Performance: generating two SVGs per formula doubles build time for math