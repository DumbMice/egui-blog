//! Development and production web server for the blog app.
//!
//! This binary provides a unified interface for:
//! - Development mode with hot reload (file watching)
//! - Production mode with optimized builds
//!
//! Usage:
//!   cargo run --bin blog_web_server                 # Development mode (default)
//!   cargo run --bin blog_web_server -- --serve-release  # Production mode
//!   cargo run --bin blog_web_server -- --port 9999     # Custom port
//!   cargo run --bin blog_web_server -- --build-only --serve-release  # Build only

use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "Blog app web server with hot reload")]
struct Args {
    /// Serve release builds (optimized, no file watching)
    #[arg(long)]
    serve_release: bool,

    /// Port to serve on
    #[arg(long, default_value = "8766")]
    port: u16,

    /// Build only, don't start server
    #[arg(long)]
    build_only: bool,

    /// Open browser automatically
    #[arg(long)]
    open: bool,

    /// Log level: debug, info, warn, error
    #[arg(long, default_value = "debug")]
    log_level: String,

    /// Log file path (optional)
    #[arg(long)]
    log_file: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Set log level based on mode
    let log_level = if args.serve_release && args.log_level == "debug" {
        "info" // Default to info for release mode
    } else {
        &args.log_level
    };

    println!("🚀 Blog Web Server");
    println!(
        "   Mode: {}",
        if args.serve_release {
            "release"
        } else {
            "development"
        }
    );
    println!("   Port: {}", args.port);
    println!("   Log level: {}", log_level);

    if args.serve_release {
        run_release_mode(&args)
    } else {
        run_dev_mode(&args)
    }
}

fn run_dev_mode(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Starting development server...");

    // 1. Ensure tools are installed
    ensure_tools_installed()?;

    // 2. Build WASM in debug mode
    build_wasm(false, "dev")?;

    // 3. Start file watcher (if notify feature is available)
    #[cfg(feature = "notify")]
    {
        println!("👀 Watching for file changes in crates/blog_app/");
        match start_file_watcher() {
            Ok(_) => println!("✅ File watcher started successfully"),
            Err(e) => {
                eprintln!("⚠️  Failed to start file watcher: {}", e);
                eprintln!("   File changes won't trigger automatic rebuilds");
            }
        }
    }

    #[cfg(not(feature = "notify"))]
    {
        println!("👀 File watching disabled (notify feature not enabled)");
        println!("   Build with --features dev to enable file watching");
    }

    // 4. Start HTTP server
    start_http_server(args.port, "dev")?;

    // 5. Open browser if requested
    if args.open {
        open_browser(args.port)?;
    }

    // 6. Wait for shutdown
    wait_for_shutdown();

    println!("👋 Development server stopped");
    Ok(())
}

fn run_release_mode(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    println!("🏗️  Building release version...");

    // 1. Build WASM in release mode with optimization
    build_wasm(true, "release")?;

    // 2. If build-only, stop here
    if args.build_only {
        println!("✅ Release build complete");
        return Ok(());
    }

    // 3. Start HTTP server (no file watching)
    println!("🌐 Serving release build on port {}...", args.port);
    start_http_server(args.port, "release")?;

    // 4. Open browser if requested
    if args.open {
        open_browser(args.port)?;
    }

    // 5. Wait for shutdown
    wait_for_shutdown();

    println!("👋 Release server stopped");
    Ok(())
}

fn ensure_tools_installed() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Checking for required tools...");

    // Check for wasm-bindgen
    let has_wasm_bindgen = Command::new("wasm-bindgen")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok();

    if !has_wasm_bindgen {
        println!("📦 Installing wasm-bindgen-cli...");
        let status = Command::new("cargo")
            .args(["install", "wasm-bindgen-cli", "--version", "0.2.113"])
            .status()?;

        if !status.success() {
            return Err("Failed to install wasm-bindgen-cli".into());
        }
    }

    // Check for basic-http-server
    let has_basic_http_server = Command::new("basic-http-server")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok();

    if !has_basic_http_server {
        println!("📦 Installing basic-http-server...");
        let status = Command::new("cargo")
            .args(["install", "basic-http-server"])
            .status()?;

        if !status.success() {
            return Err("Failed to install basic-http-server".into());
        }
    }

    println!("✅ All tools installed");
    Ok(())
}

fn build_wasm(release: bool, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let build_mode = if release { "release" } else { "debug" };
    println!("🔨 Building WASM ({})...", build_mode);

    // Create output directory relative to workspace root
    let output_path = format!("web_blog/{}", output_dir);
    fs::create_dir_all(&output_path)?;

    // Build command based on current build script
    let features = "web_app,wgpu"; // Using wgpu backend by default

    let mut cmd = Command::new("cargo");
    cmd.current_dir("crates/blog_app")
        .arg("build")
        .arg("--lib")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--no-default-features")
        .arg("--features")
        .arg(features);

    if release {
        cmd.arg("--release");
    }

    let output = cmd.output()?;

    if !output.status.success() {
        // Print build errors to help user debug
        eprintln!("❌ WASM build failed with errors:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return Err("WASM build failed".into());
    }

    // Generate JS bindings
    println!("🔗 Generating JS bindings...");
    let wasm_path = format!("target/wasm32-unknown-unknown/{}/blog_app.wasm", build_mode);
    let output = Command::new("wasm-bindgen")
        .args([
            &wasm_path,
            "--out-dir",
            &output_path,
            "--out-name",
            "blog_app",
            "--no-modules",
            "--no-typescript",
        ])
        .output()?;

    if !output.status.success() {
        eprintln!("❌ wasm-bindgen failed:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return Err("wasm-bindgen failed".into());
    }

    // Copy index.html
    fs::copy("web_blog/index.html", format!("{}/index.html", output_path))?;

    // Optimize in release mode
    if release {
        println!("⚡ Optimizing WASM...");
        let wasm_file = format!("{}/blog_app_bg.wasm", output_path);
        let status = Command::new("wasm-opt")
            .args([&wasm_file, "-O2", "--fast-math", "-o", &wasm_file])
            .status();

        match status {
            Ok(exit_status) if exit_status.success() => {
                println!("✅ WASM optimized");
            }
            _ => {
                println!("⚠️  wasm-opt not available (install with: cargo install wasm-opt)");
            }
        }
    }

    println!("✅ Build complete: {}", output_path);
    Ok(())
}

#[cfg(feature = "notify")]
fn start_file_watcher() -> Result<(), Box<dyn std::error::Error>> {
    use notify::{RecommendedWatcher, RecursiveMode, Watcher};
    use std::sync::mpsc;

    println!("📁 Setting up file watcher...");

    let (tx, rx) = mpsc::channel();

    let mut watcher: RecommendedWatcher = Watcher::new(
        move |res| match res {
            Ok(event) => {
                println!("📁 File watcher event: {:?}", event);
                let _ = tx.send(event);
            }
            Err(e) => eprintln!("📁 File watcher error: {}", e),
        },
        notify::Config::default(),
    )?;

    watcher.watch(Path::new("crates/blog_app"), RecursiveMode::Recursive)?;

    // Spawn a thread to handle file changes and trigger rebuilds
    thread::spawn(move || {
        println!("📁 File watcher thread started");

        // Keep the watcher alive by not dropping it
        std::mem::forget(watcher);

        for event in rx {
            println!("📁 File watcher received event: {:?}", event);

            // Check if it's a relevant file change
            if let Some(path) = event.paths.first() {
                let path_str = path.to_string_lossy();
                println!("📁 Processing file change: {}", path_str);

                // Check if it's a generated file we should ignore
                let is_generated_file = path_str.contains("assets/math/")
                    || path_str.contains("src/math/embedded.rs")
                    || path_str.contains("target/");

                if !is_generated_file {
                    // Only rebuild for Rust, Markdown, or asset files
                    if path_str.ends_with(".rs")
                        || path_str.ends_with(".md")
                        || path_str.contains("posts/")
                    {
                        println!("📁 Relevant file changed: {}", path_str);
                        println!("🔨 Triggering rebuild...");

                        // Trigger rebuild
                        println!("🔨 Starting rebuild...");
                        match rebuild_wasm(false) {
                            Ok(_) => {
                                println!("✅ Rebuild successful!");
                                println!("   Server is now serving updated files");
                                println!("   Refresh browser to see changes");
                            }
                            Err(e) => {
                                // Print to stderr so user sees errors even when output is redirected
                                eprintln!("═══════════════════════════════════════════════════");
                                eprintln!("❌ REBUILD FAILED!");
                                eprintln!("═══════════════════════════════════════════════════");
                                eprintln!("{}", e);
                                eprintln!("═══════════════════════════════════════════════════");
                                eprintln!("💡 Fix the error and save again to retry");
                                eprintln!("🌐 Server continues serving previous working version");
                                eprintln!("═══════════════════════════════════════════════════");
                            }
                        }
                    } else {
                        println!("📁 Ignoring non-relevant file: {}", path_str);
                    }
                } else {
                    println!("📁 Ignoring generated file: {}", path_str);
                }
            }
        }
        println!("📁 File watcher thread exiting");
    });

    Ok(())
}

#[cfg(feature = "notify")]
fn rebuild_wasm(release: bool) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = if release { "release" } else { "dev" };
    build_wasm(release, output_dir)
}

#[cfg(not(feature = "notify"))]
fn start_file_watcher() -> Result<(), Box<dyn std::error::Error>> {
    println!("📁 File watching requires --features dev");
    Ok(())
}

fn start_http_server(port: u16, mode: &str) -> Result<(), Box<dyn std::error::Error>> {
    let serve_dir = format!("web_blog/{}", mode);

    // Check if port is in use
    let port_check = Command::new("bash")
        .args([
            "-c",
            &format!("lsof -Pi :{} -sTCP:LISTEN -t 2>/dev/null", port),
        ])
        .output();

    if let Ok(output) = port_check {
        if !output.stdout.is_empty() {
            return Err(format!("Port {} already in use", port).into());
        }
    }

    println!("🌐 Starting HTTP server...");
    println!("   Serving from: {}", serve_dir);
    println!("   URL: http://localhost:{}", port);

    // Start server in background thread
    thread::spawn(move || {
        println!("🌐 HTTP server thread started, serving from: {}", serve_dir);

        let mut cmd = Command::new("basic-http-server");
        cmd.current_dir(&serve_dir)
            .args(["--addr", &format!("0.0.0.0:{}", port), "."]);

        println!("🌐 Starting HTTP server command: {:?}", cmd);

        match cmd.spawn() {
            Ok(mut child) => {
                println!("🌐 HTTP server started with PID: {}", child.id());

                // Wait for server to exit
                match child.wait() {
                    Ok(exit_status) if exit_status.success() => {
                        println!("✅ HTTP server stopped cleanly");
                    }
                    Ok(exit_status) => {
                        eprintln!("❌ HTTP server exited with error: {:?}", exit_status);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to wait for HTTP server: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to start HTTP server: {}", e);
            }
        }
    });

    // Give server time to start
    thread::sleep(Duration::from_millis(100));
    Ok(())
}

fn open_browser(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://localhost:{}", port);
    println!("🌐 Opening browser: {}", url);

    #[cfg(target_os = "linux")]
    let status = Command::new("xdg-open").arg(&url).status();

    #[cfg(target_os = "macos")]
    let status = Command::new("open").arg(&url).status();

    #[cfg(target_os = "windows")]
    let status = Command::new("cmd").args(["/C", "start", &url]).status();

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    let status: Result<std::process::ExitStatus, std::io::Error> = Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "Unsupported OS",
    ));

    match status {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("⚠️  Failed to open browser: {}", e);
            Ok(())
        }
    }
}

fn wait_for_shutdown() {
    println!();
    println!("Press Ctrl+C to stop the server");
    println!();

    // Simple wait for Ctrl+C
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
