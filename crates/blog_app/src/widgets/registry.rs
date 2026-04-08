//! Runtime widget registry and instance management.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::{Widget, WidgetConfig, WidgetError};
use uuid;

/// Type-erased widget constructor
type WidgetConstructor = Box<dyn Fn() -> Box<dyn Widget> + Send + Sync>;

/// Registry for widget types
pub struct WidgetRegistry {
    constructors: HashMap<String, WidgetConstructor>,
    instances: Mutex<HashMap<String, Box<dyn Widget>>>,
}

impl WidgetRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            constructors: HashMap::new(),
            instances: Mutex::new(HashMap::new()),
        }
    }

    /// Register a widget type
    pub fn register<W: Widget + Default + 'static>(&mut self, name: &str) {
        self.constructors
            .insert(name.to_string(), Box::new(|| Box::new(W::default())));
    }

    /// Register a widget with custom constructor
    pub fn register_with_constructor<F>(&mut self, name: &str, constructor: F)
    where
        F: Fn() -> Box<dyn Widget> + Send + Sync + 'static,
    {
        self.constructors
            .insert(name.to_string(), Box::new(constructor));
    }

    /// Create a widget instance
    pub fn create_instance(&self, name: &str) -> Result<Box<dyn Widget>, WidgetError> {
        let constructor = self
            .constructors
            .get(name)
            .ok_or_else(|| WidgetError::NotFound(name.to_string()))?;

        Ok(constructor())
    }

    /// Get or create a widget instance
    pub fn get_or_create_instance(&self, name: &str) -> Result<Box<dyn Widget>, WidgetError> {
        // For now, just create a new instance
        // TODO: Implement proper caching with configuration-based keys
        self.create_instance(name)
    }

    /// Check if a widget type is registered
    pub fn has_widget(&self, name: &str) -> bool {
        self.constructors.contains_key(name)
    }

    /// List all registered widget types
    pub fn list_widgets(&self) -> Vec<&str> {
        self.constructors.keys().map(|k| k.as_str()).collect()
    }

    /// Clear all widget instances
    pub fn clear_instances(&self) {
        self.instances.lock().unwrap().clear();
    }
}

/// Global widget registry
static WIDGET_REGISTRY: std::sync::OnceLock<Arc<WidgetRegistry>> = std::sync::OnceLock::new();

/// Get the global widget registry
pub fn global_registry() -> Arc<WidgetRegistry> {
    WIDGET_REGISTRY
        .get_or_init(|| {
            let mut registry = WidgetRegistry::new();

            // Register built-in widgets here
            use crate::widgets::examples::{ChartWidget, CounterWidget, EguiPlotWidgetSimple};
            registry.register::<CounterWidget>("counter");
            registry.register::<ChartWidget>("chart");
            registry.register::<EguiPlotWidgetSimple>("egui_plot_simple");

            Arc::new(registry)
        })
        .clone()
}

/// Widget instance with configuration
pub struct WidgetInstance {
    widget: Box<dyn Widget>,
    config: WidgetConfig,
    id: String,
}

impl std::fmt::Debug for WidgetInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WidgetInstance")
            .field("config", &self.config)
            .field("id", &self.id)
            .finish_non_exhaustive()
    }
}

impl WidgetInstance {
    /// Create a new widget instance
    pub fn new(name: &str, config: WidgetConfig) -> Result<Self, WidgetError> {
        let registry = global_registry();
        let widget = registry.create_instance(name)?;

        Ok(Self {
            widget,
            config,
            id: format!("{}-{}", name, uuid::Uuid::new_v4()),
        })
    }

    /// Render the widget
    pub fn render(&mut self, ui: &mut egui::Ui) -> egui::Vec2 {
        self.widget.render(ui, &self.config)
    }

    /// Update widget state
    pub fn update(&mut self, ctx: &egui::Context) {
        self.widget.update(ctx);
    }

    /// Get widget ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get widget configuration
    pub fn config(&self) -> &WidgetConfig {
        &self.config
    }

    /// Update widget configuration
    pub fn update_config(&mut self, config: WidgetConfig) {
        self.config = config;
    }

    /// Serialize widget state
    pub fn serialize_state(&self) -> serde_json::Value {
        self.widget.serialize_state()
    }

    /// Deserialize widget state
    pub fn deserialize_state(&mut self, state: serde_json::Value) {
        self.widget.deserialize_state(state);
    }
}
