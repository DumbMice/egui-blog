//! Native desktop application for the blog app.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// Native desktop application entry point
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("My Blog"),
        ..Default::default()
    };

    eframe::run_native(
        "My Blog",
        options,
        Box::new(|cc| {
            // Install image loaders for SVG support
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(blog_app::BlogApp::default()))
        }),
    )
}

// WASM target - this binary is not built for WASM
#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("This binary is for native desktop only. Use the library target for WASM.");
}
