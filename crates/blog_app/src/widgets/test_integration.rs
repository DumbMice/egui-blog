//! Integration test for widget system.

#[cfg(test)]
mod tests {
    use super::super::examples::EguiPlotWidgetSimple;
    use super::super::{ChartWidget, CounterWidget, Widget, WidgetConfig};

    #[test]
    fn test_counter_widget_basics() {
        let widget = CounterWidget::default();

        // Test basic properties
        assert_eq!(widget.name(), "counter");
        assert_eq!(widget.display_name(), "Counter");

        // Test serialization/deserialization
        let state = widget.serialize_state();
        assert!(state.is_object());

        let mut widget2 = CounterWidget::default();
        widget2.deserialize_state(state.clone());

        // Initial state should match
        let state2 = widget2.serialize_state();
        assert_eq!(state, state2);
    }

    #[test]
    fn test_chart_widget_basics() {
        let widget = ChartWidget::default();

        assert_eq!(widget.name(), "chart");
        assert_eq!(widget.display_name(), "Chart");

        let state = widget.serialize_state();
        assert!(state.is_object());

        let mut widget2 = ChartWidget::default();
        widget2.deserialize_state(state.clone());

        let state2 = widget2.serialize_state();
        assert_eq!(state, state2);
    }

    #[test]
    fn test_widget_config() {
        let config = WidgetConfig {
            config: serde_json::json!({
                "label": "Test",
                "value": 42,
            }),
            width: Some(200.0),
            height: Some(100.0),
            interactive: true,
        };

        // Test serialization round-trip
        let json = serde_json::to_string(&config).unwrap();
        let config2: WidgetConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.width, config2.width);
        assert_eq!(config.height, config2.height);
        assert_eq!(config.interactive, config2.interactive);
    }

    #[test]
    fn test_egui_plot_widget_simple_basics() {
        let widget = EguiPlotWidgetSimple::default();

        assert_eq!(widget.name(), "egui_plot_simple");
        assert_eq!(widget.display_name(), "Egui Plot (Simple)");

        let state = widget.serialize_state();
        assert!(state.is_object());

        let mut widget2 = EguiPlotWidgetSimple::default();
        widget2.deserialize_state(state.clone());

        let state2 = widget2.serialize_state();
        assert_eq!(state, state2);
    }
}
