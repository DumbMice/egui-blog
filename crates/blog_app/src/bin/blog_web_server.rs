//! Development and production web server for the blog app.
//!
//! This binary provides a unified interface for:
//! - Development mode with hot reload (file watching)
//! - Production mode with optimized builds
//!
//! Usage:
//!   cargo run --bin `blog_web_server`                 # Development mode (default)
//!   cargo run --bin `blog_web_server` -- --serve-release  # Production mode
//!   cargo run --bin `blog_web_server` -- --port 9999     # Custom port
//!   cargo run --bin `blog_web_server` -- --build-only --serve-release  # Build only

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
}

#[expect(clippy::print_stdout)]
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
    println!("   Log level: {log_level}");

    if args.serve_release {
        run_release_mode(&args)
    } else {
        run_dev_mode(&args)
    }
}

#[expect(clippy::print_stdout, clippy::print_stderr)]
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
                eprintln!("⚠️  Failed to start file watcher:  {e}",);
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
        open_browser(args.port);
    }

    // 6. Wait for shutdown
    wait_for_shutdown();
}

#[expect(clippy::print_stdout)]
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
        open_browser(args.port);
    }

    // 5. Wait for shutdown
    wait_for_shutdown();
}

#[expect(clippy::print_stdout)]
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

#[expect(clippy::print_stdout, clippy::print_stderr)]
fn build_wasm(release: bool, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let build_mode = if release { "release" } else { "debug" };
    println!("🔨 Building WASM ({build_mode})...",);

    // Get current directory for absolute paths
    let current_dir =
        std::env::current_dir().map_err(|e| format!("Failed to get current directory: {e}",))?;

    // Create output directory relative to workspace root
    let output_relative_path = format!("web_blog/{output_dir}");
    let output_path = current_dir.join(&output_relative_path);
    fs::create_dir_all(&output_path)?;

    // Build command based on current build script
    let features = "web_app,wgpu,persistence"; // Using wgpu backend by default with persistence

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

    let wasm_relative_path = format!("target/wasm32-unknown-unknown/{build_mode}/blog_app.wasm");
    let wasm_path = current_dir.join(&wasm_relative_path);

    println!(
        "  Looking for WASM at: {} (absolute: {})",
        wasm_relative_path,
        wasm_path.display()
    );
    println!("  Output directory: {}", output_path.display());

    // Try to find wasm-bindgen - first check common cargo location
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_owned());
    let wasm_bindgen_path = std::path::PathBuf::from(&home).join(".cargo/bin/wasm-bindgen");

    if !wasm_bindgen_path.exists() {
        return Err(format!(
            "wasm-bindgen not found at {}. Install with: cargo install wasm-bindgen-cli",
            wasm_bindgen_path.display()
        )
        .into());
    }

    // Get canonical path
    let wasm_bindgen_canonical = match std::fs::canonicalize(&wasm_bindgen_path) {
        Ok(path) => path,
        Err(e) => {
            println!("  ⚠️  Failed to get canonical path for wasm-bindgen: {e}");
            wasm_bindgen_path.clone()
        }
    };

    println!(
        "  Using wasm-bindgen at: {} (canonical: {})",
        wasm_bindgen_path.display(),
        wasm_bindgen_canonical.display()
    );

    let wasm_path_str = wasm_path
        .to_str()
        .ok_or("WASM path contains invalid UTF-8 characters")?;

    let output_path_str = output_path
        .to_str()
        .ok_or("Output path contains invalid UTF-8 characters")?;

    let cmd_str = format!(
        "{} {} --out-dir {} --out-name blog_app --no-modules --no-typescript",
        wasm_bindgen_path.display(),
        wasm_path_str,
        output_path_str
    );
    println!("  Running command: {cmd_str}");

    // Convert path to string, handling potential UTF-8 issues
    let wasm_bindgen_str = wasm_bindgen_canonical
        .to_str()
        .ok_or("wasm-bindgen path contains invalid UTF-8 characters")?;

    // Execute wasm-bindgen
    let output = Command::new(wasm_bindgen_str)
        .arg(wasm_path_str)
        .arg("--out-dir")
        .arg(output_path_str)
        .arg("--out-name")
        .arg("blog_app")
        .arg("--no-modules")
        .arg("--no-typescript")
        .output()?;

    if !output.status.success() {
        eprintln!("❌ wasm-bindgen failed:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return Err("wasm-bindgen failed".into());
    }

    // Copy index.html
    fs::copy("web_blog/index.html", output_path.join("index.html"))?;

    // Optimize in release mode
    if release {
        println!("⚡ Optimizing WASM...");
        let wasm_file = output_path.join("blog_app_bg.wasm");
        // Use temporary file to avoid wasm-opt bug with in-place optimization
        let temp_file = wasm_file.with_extension("wasm.tmp");
        // Find wasm-opt in common cargo location
        let wasm_opt_path = std::path::PathBuf::from(&home).join(".cargo/bin/wasm-opt");
        if wasm_opt_path.exists() {
            let wasm_file_str = wasm_file
                .to_str()
                .ok_or("WASM file path contains invalid UTF-8 characters")?;
            let temp_file_str = temp_file
                .to_str()
                .ok_or("Temp file path contains invalid UTF-8 characters")?;

            let output = Command::new(wasm_opt_path.to_str().ok_or("wasm_opt_path exists.")?)
                .args([wasm_file_str, "-O1", "--fast-math", "-o", temp_file_str])
                .output();

            match output {
                Ok(output) if output.status.success() => {
                    // Move temp file back to original
                    if let Err(e) = fs::rename(&temp_file, &wasm_file) {
                        println!("⚠️  Failed to move optimized WASM: {e}");
                    } else {
                        println!("✅ WASM optimized with -O1");
                    }
                }
                Ok(output) => {
                    let status_str = output.status.to_string();
                    if status_str.contains("SIGSEGV") || status_str.contains("signal: 11") {
                        // SIGSEGV (segmentation fault) - wasm-opt bug
                        println!("⚠️  wasm-opt crashed (SIGSEGV) - known issue with wasm-opt");
                        println!("⚠️  WASM will be unoptimized but still functional");
                    } else {
                        println!("⚠️  wasm-opt failed with status: {status_str}");
                        if !output.stderr.is_empty() {
                            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                        }
                    }
                    // Clean up temp file if it exists
                    let _ = fs::remove_file(&temp_file);
                }
                Err(_) => {
                    println!("⚠️  wasm-opt not available (install with: cargo install wasm-opt)");
                }
            }
        } else {
            println!(
                "⚠️  wasm-opt not found at {}. Skipping optimization.",
                wasm_opt_path.display()
            );
        }
    }

    println!("✅ Build complete: {}", output_path.display());
    Ok(())
}

#[cfg(feature = "notify")]
#[expect(clippy::print_stdout, clippy::print_stderr, clippy::mem_forget)]
fn start_file_watcher() -> Result<(), Box<dyn std::error::Error>> {
    use notify::{RecommendedWatcher, RecursiveMode, Watcher};
    use std::collections::HashSet;
    use std::sync::mpsc;
    use std::time::{Duration, Instant};

    println!("📁 Setting up file watcher...");

    // Channel from file watcher callback to file watcher thread
    let (event_tx, event_rx) = mpsc::channel();
    let event_tx_clone = event_tx.clone();

    let mut watcher: RecommendedWatcher = Watcher::new(
        move |res| match res {
            Ok(event) => {
                let _ = event_tx_clone.send(event);
            }
            Err(e) => eprintln!("📁 File watcher error: {e}"),
        },
        notify::Config::default(),
    )?;

    watcher.watch(Path::new("crates/blog_app"), RecursiveMode::Recursive)?;

    // Channel from file watcher thread to rebuild worker
    let (path_tx, path_rx) = mpsc::channel();

    // Spawn file watcher thread
    thread::Builder::new()
        .name("file-watcher".into())
        .spawn(move || {
            println!("📁 File watcher thread started");
            std::mem::forget(watcher); // Keep watcher alive

            for event in event_rx {
                // Filter by event kind - only care about modify/create/remove events
                let is_relevant_event = matches!(
                    event.kind,
                    notify::EventKind::Create(_)
                        | notify::EventKind::Modify(_)
                        | notify::EventKind::Remove(_)
                );

                if !is_relevant_event {
                    continue;
                }

                // Check if it's a relevant file change
                if let Some(path) = event.paths.first() {
                    let path_str = path.to_string_lossy();

                    // Check if it's a generated file we should ignore
                    let is_generated_file = path_str.contains("assets/math/")
                        || path_str.contains("src/math/embedded.rs")
                        || path_str.contains("target/");

                    if !is_generated_file {
                        // Check if it's a file we should rebuild for
                        let should_rebuild = {
                            // Only rebuild for Rust or Markdown files
                            if path_str.ends_with(".rs") || path_str.ends_with(".md") {
                                // Check for backup files and hidden files to ignore
                                let is_backup_file = path_str.ends_with('~')
                                    || path_str.ends_with(".bak")
                                    || path_str.ends_with(".tmp")
                                    || path_str.ends_with(".swp")
                                    || path_str.ends_with(".swx");

                                let is_hidden_file =
                                    path_str.contains("/.") || path_str.contains("\\."); // Windows paths

                                // Also ignore files in .git directory
                                let is_git_file =
                                    path_str.contains("/.git/") || path_str.contains("\\.git\\");

                                !is_backup_file && !is_hidden_file && !is_git_file
                            } else {
                                false
                            }
                        };

                        if should_rebuild {
                            // Send file path to rebuild worker
                            let _ = path_tx.send(path_str.to_string());
                        }
                    }
                }
            }
            println!("📁 File watcher thread exiting");
        })?;

    // Spawn rebuild worker thread
    thread::Builder::new()
        .name("blog-builder".into())
        .spawn(move || {
            println!("🔨 Rebuild worker thread started");

            let mut pending_files = HashSet::new();
            let mut last_rebuild_time = Instant::now();
            let debounce_delay = Duration::from_millis(800);
            let min_rebuild_interval = Duration::from_millis(2000);

            let mut debounce_timer: Option<Instant> = None;

            loop {
                // Calculate timeout for recv
                let timeout = if let Some(timer_deadline) = debounce_timer {
                    let now = Instant::now();
                    if timer_deadline > now {
                        timer_deadline - now
                    } else {
                        Duration::from_millis(0) // Timer expired
                    }
                } else {
                    Duration::from_millis(100) // Default timeout
                };

                // Check for file change events with calculated timeout
                match path_rx.recv_timeout(timeout) {
                    Ok(file_path) => {
                        // Only log if this is a new file in the set
                        if pending_files.insert(file_path.clone()) {
                            println!("📁 Detected change: {file_path}");
                        }

                        // Reset debounce timer
                        debounce_timer = Some(Instant::now() + debounce_delay);
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        // Check if debounce timer has expired
                        if let Some(timer_deadline) = debounce_timer {
                            let now = Instant::now();
                            if now >= timer_deadline {
                                // Timer expired, check if we should rebuild
                                let time_since_last_rebuild = now.duration_since(last_rebuild_time);

                                if !pending_files.is_empty()
                                    && time_since_last_rebuild >= min_rebuild_interval
                                {
                                    trigger_rebuild(&pending_files);
                                    pending_files.clear();
                                    last_rebuild_time = now;
                                } else if !pending_files.is_empty() {
                                    pending_files.clear();
                                }

                                debounce_timer = None;
                            }
                        }
                    }
                    Err(mpsc::RecvTimeoutError::Disconnected) => {
                        println!("🔨 Rebuild worker thread exiting");
                        break;
                    }
                }
            }
        })?;

    Ok(())
}

#[cfg(feature = "notify")]
#[expect(
    clippy::print_stdout,
    clippy::print_stderr,
    clippy::iter_over_hash_type
)]
fn trigger_rebuild(pending_files: &std::collections::HashSet<String>) {
    if pending_files.is_empty() {
        return;
    }

    // Log all changed files
    println!("📁 Files changed:");
    for file in pending_files {
        println!("   - {file}");
    }
    println!("🔨 Starting rebuild...");

    // Trigger rebuild
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
            eprintln!("{e}");
            eprintln!("═══════════════════════════════════════════════════");
            eprintln!("💡 Fix the error and save again to retry");
            eprintln!("🌐 Server continues serving previous working version");
            eprintln!("═══════════════════════════════════════════════════");
        }
    }
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

#[expect(clippy::print_stderr, clippy::print_stdout)]
fn start_http_server(port: u16, mode: &str) -> Result<(), Box<dyn std::error::Error>> {
    let serve_dir = format!("web_blog/{mode}");

    // Check if port is in use
    let port_check = Command::new("bash")
        .args([
            "-c",
            &format!("lsof -Pi :{port} -sTCP:LISTEN -t 2>/dev/null"),
        ])
        .output();

    if let Ok(output) = port_check
        && !output.stdout.is_empty()
    {
        return Err(format!("Port {port} already in use").into());
    }

    println!("🌐 Starting HTTP server...");
    println!("   Serving from: {serve_dir}");
    println!("   URL: http://localhost:{port}");

    // Start server in background thread
    thread::Builder::new()
        .name("http-server".to_owned())
        .spawn(move || {
            println!("🌐 HTTP server thread started, serving from: {serve_dir}");

            let mut cmd = Command::new("basic-http-server");
            cmd.current_dir(&serve_dir)
                .args(["--addr", &format!("0.0.0.0:{port}"), "."]);

            println!("🌐 Starting HTTP server command: {cmd:?}");

            match cmd.spawn() {
                Ok(mut child) => {
                    println!("🌐 HTTP server started with PID: {}", child.id());

                    // Wait for server to exit
                    match child.wait() {
                        Ok(exit_status) if exit_status.success() => {
                            println!("✅ HTTP server stopped cleanly");
                        }
                        Ok(exit_status) => {
                            eprintln!("❌ HTTP server exited with error: {exit_status:?}");
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to wait for HTTP server: {e}");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to start HTTP server: {e}");
                }
            }
        })?;

    // Give server time to start
    thread::sleep(Duration::from_millis(100));
    Ok(())
}

#[expect(clippy::print_stdout, clippy::print_stderr)]
fn open_browser(port: u16) {
    let url = format!("http://localhost:{port}");
    println!("🌐 Opening browser: {url}");
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
    if let Err(e) = status {
        eprintln!("⚠️  Failed to open browser: {e}");
    }
}

#[expect(clippy::print_stdout)]
fn wait_for_shutdown() -> ! {
    println!();
    println!("Press Ctrl+C to stop the server");
    println!();

    // Simple wait for Ctrl+C
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
