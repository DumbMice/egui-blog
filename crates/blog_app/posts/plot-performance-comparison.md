---
title: "Egui Plot Simple Performance Test"
date: 2026-04-07
tags: [widgets, plots, performance, test]
---

# Egui Plot Simple - Performance Test

This post tests the performance of the simplified `egui_plot_simple` widget in isolation.

## Test Setup
- Enable frame rate debug window: Press `Ctrl+D` or enable in settings
- Monitor FPS in the "Frame Rate" window
- Scroll through widgets to test animation behavior

## About This Widget

The `egui_plot_simple` widget:
- Uses `egui_plot` library directly
- Has **no custom visibility checks** (relies on egui's built-in optimizations)
- Has **no custom animation throttling** (simple fixed delta)
- Has **no manual repaint requests** (relies on egui_plot's built-in optimizations)
- **Fixed ID clash issue** with separate `state_id` and `plot_id`

## Test Widgets

### Sine Wave (Default)
![Sine Wave](egui_plot_simple.rs)

### Higher Frequency
![Fast Sine](egui_plot_simple.rs?{"type":"sine","frequency":2.0})

### Cosine Wave
![Cosine](egui_plot_simple.rs?{"type":"cosine","amplitude":1.5})

### Random Data
![Random](egui_plot_simple.rs?{"type":"random","points":80})

### Dense Points
![Dense Sine](egui_plot_simple.rs?{"type":"sine","points":200})

### High Amplitude
![Large Cosine](egui_plot_simple.rs?{"type":"cosine","amplitude":3.0})

## Performance Test Scenarios

### Scenario 1: Viewing This Post
When viewing this post with all 6 widgets visible:
- **Expected**: Smooth animation, stable FPS
- **Target**: ≥50 FPS stable
- **Animation**: All sine/cosine widgets should animate smoothly

### Scenario 2: Scrolling (Partial Visibility)
Scroll so only some widgets are visible:
- **Expected**: Partial animation (only visible widgets animate)
- **Test**: FPS should remain stable

### Scenario 3: Not Viewing This Post
Navigate to a different post (not containing plot widgets):
- **Critical Test**: FPS should remain high
- **Expected**: ≥50 FPS (widgets should not impact performance when not visible)

## How to Measure Performance

1. **Enable Frame Rate Monitor**: `Ctrl+D` or Settings → Debug → Show Frame Rate
2. **Check Current FPS**: Displayed in frame rate window
3. **Performance Categories**:
   - ≥ 60 FPS: Excellent
   - ≥ 50 FPS: Good (our target)
   - ≥ 30 FPS: Fair
   - < 30 FPS: Poor

## Widget Configuration

Available configuration options for `egui_plot_simple`:
- `type`: `"sine"`, `"cosine"`, or `"random"`
- `points`: Number of data points (default: 50)
- `amplitude`: Wave amplitude (default: 1.0)
- `frequency`: Wave frequency multiplier (default: 1.0)
- `width`: Widget width in pixels
- `height`: Widget height in pixels

Example:
```markdown
![My Plot](egui_plot_simple.rs?{"type":"sine","frequency":2.0,"points":100})
```
