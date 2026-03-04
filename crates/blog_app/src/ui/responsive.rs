//! Responsive layout utilities for the blog app.

use egui::{Context, Ui};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Configuration for responsive layout behavior
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug)]
pub struct ResponsiveConfig {
    /// Optimal reading width in characters (80-100 chars)
    pub optimal_chars: usize,
    /// Minimum content width in pixels
    pub min_width: f32,
    /// Maximum content width in pixels
    pub max_width: f32,
    /// Side margins as percentage of available width
    pub margin_percent: f32,
    /// Breakpoint for mobile vs desktop (pixels)
    pub mobile_breakpoint: f32,
}

impl Default for ResponsiveConfig {
    fn default() -> Self {
        Self {
            optimal_chars: 90,        // Optimal reading width
            min_width: 300.0,         // Minimum readable width
            max_width: 800.0,         // Maximum comfortable width
            margin_percent: 0.05,     // 5% margins on each side
            mobile_breakpoint: 768.0, // Common mobile breakpoint
        }
    }
}

impl ResponsiveConfig {
    /// Calculate the ideal content width based on available space
    #[allow(dead_code)]
    pub fn calculate_content_width(&self, available_width: f32) -> f32 {
        // Start with optimal width based on character count
        // Approximate: average character width is ~8px for readable text
        let optimal_pixels = (self.optimal_chars as f32) * 8.0;

        // Apply min/max constraints
        let mut width = optimal_pixels.clamp(self.min_width, self.max_width);

        // If available width is less than optimal, use available width with margins
        if available_width < width {
            width = available_width * (1.0 - 2.0 * self.margin_percent);
            width = width.max(self.min_width);
        }

        width
    }

    /// Check if current screen size is mobile
    #[allow(dead_code)]
    pub fn is_mobile(&self, ctx: &Context) -> bool {
        let screen_size = ctx.content_rect().size();
        screen_size.x < self.mobile_breakpoint
    }

    /// Get responsive margins based on screen size
    pub fn get_margins(&self, available_width: f32) -> f32 {
        if available_width < self.mobile_breakpoint {
            // Smaller fixed margins on mobile
            16.0
        } else {
            // Percentage-based margins on desktop that adjust with zoom
            // Minimum margin of 20px, maximum of 10% of available width
            let min_margin = 20.0;
            let max_margin = available_width * 0.1;
            let margin = available_width * self.margin_percent;
            margin.clamp(min_margin, max_margin)
        }
    }

    /// Calculate content width that reduces when page width is less than optimal
    pub fn calculate_adaptive_width(&self, available_width: f32) -> f32 {
        let optimal_pixels = (self.optimal_chars as f32) * 8.0;

        if available_width >= optimal_pixels {
            // Enough space for optimal width
            optimal_pixels.clamp(self.min_width, self.max_width)
        } else {
            // Reduce width proportionally with available space
            let scale = available_width / optimal_pixels;
            let scaled_width = optimal_pixels * scale;
            scaled_width.clamp(self.min_width, available_width * 0.9)
        }
    }
}

/// Create a responsive container with optimal reading width
pub fn responsive_container<R>(
    ui: &mut Ui,
    config: &ResponsiveConfig,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    let available_width = ui.available_width();

    // Use adaptive width calculation that reduces when space is limited
    let content_width = config.calculate_adaptive_width(available_width);
    let margins = config.get_margins(available_width);

    // Calculate left margin to center content, but ensure minimum margin
    let left_margin = (available_width - content_width) / 2.0;
    let left_margin = left_margin.max(margins);

    // Create a horizontal layout with calculated width
    let response = ui.horizontal(|ui| {
        ui.add_space(left_margin);

        // Create vertical container with fixed width
        let response = ui.vertical(|ui| {
            ui.set_width(content_width);
            add_contents(ui)
        });

        response.inner
    });

    response.inner
}

/// Create a max-width container with auto-centering margins
#[expect(dead_code)]
pub fn max_width_container<R>(
    ui: &mut Ui,
    config: &ResponsiveConfig,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    let available_width = ui.available_width();
    let content_width = config.max_width.min(available_width);
    let margins = config.get_margins(available_width);

    // Calculate left margin to center content
    let left_margin = (available_width - content_width) / 2.0;
    let left_margin = left_margin.max(margins);

    // Create a horizontal layout with max width
    let response = ui.horizontal(|ui| {
        ui.add_space(left_margin);

        // Create vertical container with fixed width
        let response = ui.vertical(|ui| {
            ui.set_width(content_width);
            add_contents(ui)
        });

        response.inner
    });

    response.inner
}
