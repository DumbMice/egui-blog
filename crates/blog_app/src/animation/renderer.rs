//! Drawing logic for panel focus animation.
//!
//! Renders animated focus indicators based on current animation state.

use egui::{Color32, Painter, Rect, Stroke, StrokeKind};

use crate::animation::{FocusAnimationConfig, FocusAnimationState};

/// Renders animated focus indicators for panels.
#[derive(Debug, Clone, Copy, Default)]
pub struct FocusRenderer;

impl FocusRenderer {
    /// Draw focus indicator for a panel.
    ///
    /// # Arguments
    /// * `painter` - Egui painter for drawing.
    /// * `rect` - Rectangle of the panel to highlight.
    /// * `is_focused` - Whether this panel is currently focused.
    /// * `animation_state` - Current animation state.
    /// * `config` - Animation configuration.
    /// * `current_time` - Current time in seconds from app start.
    /// * `accent_color` - Color to use for the focus indicator.
    pub fn draw_focus_indicator(
        painter: &Painter,
        rect: Rect,
        is_focused: bool,
        animation_state: &FocusAnimationState,
        config: &FocusAnimationConfig,
        current_time: f64,
        ui: &egui::Ui,
    ) {
        if !is_focused {
            return;
        }

        let (_bg_opacity, border_opacity) =
            animation_state.calculate_opacities(config, current_time);

        // Draw simple border flash if has opacity
        if border_opacity > 0.001 {
            // Use the theme's active widget background color (Catppuccin blue)
            // This follows the Catppuccin style guide for focus states
            let flash_color = ui.visuals().widgets.active.bg_fill;

            let border_color = Self::adjust_opacity(flash_color, border_opacity);

            painter.rect_stroke(
                rect,
                0.0,
                Stroke::new(config.border_thickness, border_color),
                StrokeKind::Outside,
            );
        }
    }

    /// Adjust color opacity by multiplying alpha channel.
    ///
    /// # Arguments
    /// * `color` - Original color.
    /// * `opacity` - Desired opacity (0.0 to 1.0).
    ///
    /// # Returns
    /// New color with adjusted opacity.
    fn adjust_opacity(color: Color32, opacity: f32) -> Color32 {
        let alpha = (color.a() as f32 * opacity).round() as u8;
        Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), alpha)
    }
}
