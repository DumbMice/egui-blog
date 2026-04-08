//! Test binary for typography configuration.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Blog Typography Test"),
        ..Default::default()
    };

    eframe::run_native(
        "Blog Typography Test",
        options,
        Box::new(|cc| {
            // Install image loaders for SVG support
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // Note: Typography is configured in BlogApp::new()
            Ok(Box::new(blog_app::BlogApp::new(cc)))
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // This binary is not intended for WASM
    println!("typography_test is a native-only binary");
}
