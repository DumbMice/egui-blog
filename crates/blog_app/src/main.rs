//! Blog app built with egui.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
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
        Box::new(|cc| Ok(Box::new(blog_app::BlogApp::default()))),
    )
}
