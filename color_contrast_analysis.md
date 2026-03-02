# Color Contrast Analysis for Strong Text in Blog App

## Summary

The Catppuccin themes in the blog app have a critical accessibility issue: **strong (bold) text is completely invisible** because it uses the background color for text color, resulting in a 1:1 contrast ratio.

## Technical Analysis

### Root Cause
In `crates/blog_app/src/ui/components.rs`, the `catppuccin_visuals()` function sets:
```rust
visuals.widgets.active.fg_stroke.color = to_color32(flavor.colors.base.rgb);
```

This makes strong text (which uses `visuals.strong_text_color() = widgets.active.text_color()`) the same color as the background (`window_fill = base.rgb`).

### Contrast Ratio Results

**Catppuccin Latte (Light):**
- Normal text: `#4C4F69` on `#EFF1F5` = **7.06:1** (Good - exceeds WCAG 4.5:1)
- Strong text: `#EFF1F5` on `#EFF1F5` = **1.00:1** (Invisible!)
- Active button text: `#EFF1F5` on `#1E66F5` = **4.34:1** (Acceptable)

**Catppuccin Mocha (Dark):**
- Normal text: `#CDD6F4` on `#1E1E2E` = **11.34:1** (Excellent)
- Strong text: `#1E1E2E` on `#1E1E2E` = **1.00:1** (Invisible!)
- Active button text: `#1E1E2E` on `#89B4FA` = **7.79:1** (Good)

### Why This Happened

The code comment explains the intention:
```rust
// For active widget text (and strong text)
// Use base color (background) for better contrast with blue button background
```

This makes sense for **active buttons** (where text appears on a blue background), but breaks **strong text in markdown** (where text appears on the base background).

### How Default egui Themes Handle This

**Light theme:**
- Normal text: `#505050` (gray)
- Strong text: `#000000` (black) 
- Background: `#F8F8F8` (light gray)
- Contrast: ~15:1 for strong text

**Dark theme:**
- Normal text: `#8C8C8C` (light gray)
- Strong text: `#FFFFFF` (white)
- Background: `#1B1B1B` (dark gray)
- Contrast: ~15:1 for strong text

Default themes use maximum contrast colors (black/white) for strong text.

## Recommended Solutions

### Option 1: Fix Active Widget Text Color (Recommended)
Change line 134 in `components.rs` to use a color that contrasts well with both:
1. Blue button background (for active buttons)
2. Base background (for strong text)

For light themes: Use black or very dark color
For dark themes: Use white or very light color

### Option 2: Separate Strong Text from Active Widget Text
Override `strong_text_color()` separately instead of tying it to `widgets.active.text_color()`.

### Option 3: Use Catppuccin's Semantic Colors
Use appropriate accent colors from the palette:
- Light themes: Use `overlay2` (#6C6F85) or darker
- Dark themes: Use `text` (#CDD6F4) or lighter

### Option 4: Follow egui's Pattern
Simply use black for light themes and white for dark themes, like egui does.

## WCAG Compliance Impact

**Current state:**
- ✅ Normal text: Compliant (7.06:1 and 11.34:1)
- ❌ Strong text: **Critical failure** (1:1 - invisible)
- ✅ Active buttons: Compliant (4.34:1 and 7.79:1)

**Required fix:** Strong text must have at least 4.5:1 contrast ratio, and ideally should have **better** contrast than normal text.

## Implementation Priority

This is a **high-priority accessibility bug** that makes bold text in blog posts completely invisible for users with Catppuccin themes enabled.

## Testing Recommendation

After fixing, test with:
1. Markdown containing `**bold text**`
2. Headings (which may also be affected)
3. Active buttons to ensure they still look good
4. Both light and dark Catppuccin variants
