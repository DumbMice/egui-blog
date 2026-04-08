# Embeddable UI Widgets for Blog Posts

This document describes how to use embeddable interactive UI widgets in egui blog posts.

## Overview

The widget system allows embedding interactive UI components in markdown posts using image syntax: `![Widget](widget-name.rs)`. Widgets are Rust components that implement the `Widget` trait and are registered in the global widget registry.

## Usage in Markdown

To embed a widget in a markdown post:

```markdown
Here's a counter widget: ![Counter](counter.rs)

You can specify size: ![Big Plot](plot.rs?width=300&height=200)
```

The syntax follows standard markdown image syntax:
- `![alt text](widget-name.rs)` - basic widget
- `![alt text](widget-name.rs?width=300&height=200)` - widget with size constraints

## Available Widgets

The following widgets are built-in:

### Counter Widget
**Name**: `counter`
A simple interactive counter with increment/decrement buttons.

### Plot Widget  
**Name**: `plot`
A mathematical plot widget showing a sine wave.

### Chart Widget
**Name**: `chart`
A bar chart widget with sample data.

## Creating Custom Widgets

To create a custom widget:

1. Create a new Rust file in `src/widgets/` (e.g., `src/widgets/my_widget.rs`)
2. Implement the `Widget` trait
3. Register the widget in the global registry

### Example Widget

```rust
// src/widgets/my_widget.rs
use crate::widgets::{Widget, WidgetConfig};
use egui::{Color32, Ui};

#[derive(Default)]
pub struct MyWidget {
    value: f32,
}

impl Widget for MyWidget {
    fn name(&self) -> &'static str {
        "my_widget"
    }

    fn display_name(&self) -> &'static str {
        "My Custom Widget"
    }

    fn render(&mut self, ui: &mut Ui, config: &WidgetConfig) -> egui::Vec2 {
        ui.label("My Custom Widget");
        ui.add(egui::Slider::new(&mut self.value, 0.0..=100.0).text("Value"));
        
        // Return rendered size
        ui.available_size()
    }

    fn update(&mut self, ctx: &egui::Context) {
        // Update widget state each frame
    }

    fn serialize_state(&self) -> serde_json::Value {
        serde_json::json!({ "value": self.value })
    }

    fn deserialize_state(&mut self, state: serde_json::Value) {
        if let Some(value) = state.get("value").and_then(|v| v.as_f64()) {
            self.value = value as f32;
        }
    }
}
```

### Registering the Widget

Add to `src/widgets/registry.rs` in the `global_registry()` function:

```rust
registry.register::<MyWidget>("my_widget");
```

## Widget Configuration

Widgets can be configured via the `WidgetConfig` struct:

- `config`: JSON configuration specific to the widget
- `width`: Optional width constraint
- `height`: Optional height constraint  
- `interactive`: Whether the widget responds to user input

## Architecture

### Components

1. **Widget Trait** (`src/widgets/mod.rs`): Interface for all widgets
2. **Widget Registry** (`src/widgets/registry.rs`): Runtime registry of available widgets
3. **Widget Instance** (`src/widgets/registry.rs`): Managed widget instance with config
4. **Markdown Integration** (`src/ui/markdown.rs`): Parses `![Widget](widget.rs)` syntax
5. **Example Widgets** (`src/widgets/examples/`): Built-in widget implementations

### Build Process

Widgets are compiled into the main binary. The build system:
- Compiles all widget Rust modules
- Registers them in the global registry
- Validates widget references in markdown files

## Performance Considerations

- Widgets add to WASM binary size (currently ~15MB with glow backend)
- Each widget instance maintains its own state
- Widget rendering happens each frame (immediate mode GUI)

## Limitations

- Widget source files (.rs) must be part of the crate (not external files)
- Widget state is not persisted across page reloads (unless implemented)
- Complex widgets may impact performance

## Future Enhancements

1. **Dynamic Loading**: Load widget WASM modules at runtime
2. **Build-time Compilation**: Compile `.rs` files from `assets/widgets/` directory
3. **State Persistence**: Save widget state across sessions
4. **Widget Gallery**: UI for browsing available widgets
5. **Configuration UI**: Visual configuration of widget parameters