//! Blog app built with egui.
//!
//! This crate provides multiple binaries:
//! - `blog_native`: Native desktop application
//! - `blog_web_server`: Development/production web server with hot reload
//!
//! The default binary (`blog_app`) shows this help message.
//! Use `cargo run --bin <binary_name>` to run a specific binary.

#[expect(clippy::print_stderr)]
fn main() {
    eprintln!("Blog App - Available binaries:");
    eprintln!();
    eprintln!("  cargo run --bin blog_native");
    eprintln!("    ↳ Native desktop application");
    eprintln!();
    eprintln!("  cargo run --bin blog_web_server");
    eprintln!("    ↳ Web server with hot reload (development mode)");
    eprintln!("    ↳ Use --serve-release for production builds");
    eprintln!();
    eprintln!("  cargo run");
    eprintln!("    ↳ Defaults to blog_web_server (development mode)");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  cargo run --bin blog_web_server -- --serve-release");
    eprintln!("  cargo run --bin blog_web_server -- --port 9999");
    eprintln!("  cargo run --bin blog_web_server -- --open");
    eprintln!();
    eprintln!("For more information, see IMPLEMENTATION_PLAN.md");

    std::process::exit(1);
}
