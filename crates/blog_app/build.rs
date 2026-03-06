//! Build script for processing math formulas in blog posts.
//! This script extracts Typst math formulas from markdown files,
//! renders them to SVG using Typst, and generates embedded Rust code.
//!
//! ## Typst Math Syntax Requirements
//!
//! 1. **Inline math**: `$formula$` (no spaces around formula)
//! 2. **Display math**: `$ formula $` (spaces around formula) or `$$ formula $$`
//! 3. **Valid Typst math syntax** (not LaTeX syntax):
//!    - Use `sum_(n=1)^infinity` not `sum from n=1 to infinity`
//!    - Use `sqrt(x)` not `\sqrt{x}`
//!    - Use `mat(1, 2; 3, 4)` for matrices
//!    - Use spaces around operators: `4 a c` not `4ac`
//! 4. **Common issues**:
//!    - Fractions: `1/2` works, but complex fractions may need parentheses
//!    - Subscripts: `x_i` not `x_{i}`
//!    - Superscripts: `x^2` not `x^{2}`
//!
//! ## Error Handling
//!
//! - If Typst CLI fails to render a formula, a placeholder SVG is generated
//! - Placeholder SVGs are marked with `is_placeholder: true` in the manifest
//! - Build warnings are emitted for failed formulas
//!

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Context as _, Result};
use chrono::Utc;

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use walkdir::WalkDir;

/// Metadata for a rendered formula
#[derive(Debug, Serialize, Deserialize)]
struct FormulaMetadata {
    /// The original formula text
    formula: String,
    /// Whether this is display math (true) or inline math (false)
    is_display: bool,
    /// When this formula was last rendered (ISO 8601)
    rendered_at: String,
    /// Size of the SVG file in bytes
    svg_size: usize,
    /// The hash used as filename
    hash: String,
    /// Whether this is a placeholder SVG (true) or actual Typst rendering (false)
    is_placeholder: bool,
    /// Whether this SVG has been processed for theme adaptation (true)
    #[serde(default = "default_theme_processed")]
    theme_processed: bool,
    /// Baseline position from top of SVG (in SVG units)
    /// Only meaningful for inline math (`is_display` = false)
    #[serde(default)]
    baseline_from_top: Option<f32>,
    /// SVG height for reference (in SVG units)
    #[serde(default)]
    svg_height: Option<f32>,
}

fn default_theme_processed() -> bool {
    true // New SVGs are processed by default
}

/// Manifest tracking all rendered formulas
#[derive(Debug, Serialize, Deserialize)]
struct MathManifest {
    /// Map from formula hash to metadata
    formulas: HashMap<String, FormulaMetadata>,
    /// When this manifest was last updated
    updated_at: String,
}

/// Extract Typst math formulas from markdown text
fn extract_formulas_from_markdown(content: &str) -> Vec<(String, bool)> {
    let mut formulas = Vec::new();

    // Remove YAML frontmatter first
    let lines: Vec<&str> = content.lines().collect();
    let mut in_frontmatter = false;
    let mut content_lines = Vec::new();

    for line in lines {
        if line.trim() == "---" {
            in_frontmatter = !in_frontmatter;
            continue;
        }
        if !in_frontmatter {
            content_lines.push(line);
        }
    }

    let content_no_frontmatter = content_lines.join("\n");

    // Simple state machine to extract formulas
    // Handles: $formula$ (inline), $ formula $ (display), $$ formula $$ (LaTeX display)
    let mut i = 0;
    let chars: Vec<char> = content_no_frontmatter.chars().collect();

    while i < chars.len() {
        if chars[i] == '$' {
            // Check what type of formula this is
            if i + 1 < chars.len() && chars[i + 1] == '$' {
                // LaTeX display math: $$ formula $$
                let mut j = i + 2;
                while j < chars.len()
                    && !(chars[j] == '$' && j + 1 < chars.len() && chars[j + 1] == '$')
                {
                    j += 1;
                }

                if j + 1 < chars.len() && chars[j] == '$' && chars[j + 1] == '$' {
                    let formula: String = chars[i + 2..j].iter().collect();
                    let formula = formula.trim();
                    if !formula.is_empty() && !formula.starts_with('#') {
                        formulas.push((formula.to_owned(), true));
                    }
                    i = j + 2;
                    continue;
                }
            } else if i + 1 < chars.len() && chars[i + 1] == ' ' {
                // Typst display math: $ formula $
                let mut j = i + 2;
                while j < chars.len()
                    && !(chars[j] == ' ' && j + 1 < chars.len() && chars[j + 1] == '$')
                {
                    j += 1;
                }

                if j + 1 < chars.len() && chars[j] == ' ' && chars[j + 1] == '$' {
                    let formula: String = chars[i + 2..j].iter().collect();
                    let formula = formula.trim();
                    if !formula.is_empty() && !formula.starts_with('#') {
                        formulas.push((formula.to_owned(), true));
                    }
                    i = j + 2;
                    continue;
                }
            } else {
                // Typst inline math: $formula$
                let mut j = i + 1;
                while j < chars.len() && chars[j] != '$' {
                    j += 1;
                }

                if j < chars.len() && chars[j] == '$' {
                    let formula: String = chars[i + 1..j].iter().collect();
                    let formula = formula.trim();
                    if !formula.is_empty() && !formula.starts_with('#') {
                        formulas.push((formula.to_owned(), false));
                    }
                    i = j + 1;
                    continue;
                }
            }
        }
        i += 1;
    }

    formulas
}

/// Compute SHA-256 hash of a formula
fn hash_formula(formula: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(formula.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Parse SVG height from viewBox or width/height attributes
fn parse_svg_height(svg_content: &str) -> Result<f32> {
    // Try to parse viewBox first (most reliable)
    if let Some(viewbox_start) = svg_content.find("viewBox=\"") {
        let viewbox_content_start = viewbox_start + "viewBox=\"".len();
        let viewbox_content_end = svg_content[viewbox_content_start..]
            .find('"')
            .context("Failed to find closing quote for viewBox")?;
        let viewbox_str =
            &svg_content[viewbox_content_start..viewbox_content_start + viewbox_content_end];

        let parts: Vec<&str> = viewbox_str.split_whitespace().collect();
        if parts.len() >= 4 {
            let height = parts[3]
                .parse::<f32>()
                .context("Failed to parse viewBox height")?;
            return Ok(height);
        }
    }

    // Fall back to height attribute
    if let Some(height_start) = svg_content.find("height=\"") {
        let height_content_start = height_start + "height=\"".len();
        let height_content_end = svg_content[height_content_start..]
            .find('"')
            .context("Failed to find closing quote for height")?;
        let height_str =
            &svg_content[height_content_start..height_content_start + height_content_end];

        // Parse number, removing units like pt, px, etc.
        let number_str = height_str
            .replace("pt", "")
            .replace("px", "")
            .replace("em", "")
            .replace("rem", "")
            .replace("in", "")
            .replace("cm", "")
            .replace("mm", "");

        let height = number_str
            .parse::<f32>()
            .context("Failed to parse height attribute")?;
        return Ok(height);
    }

    Err(anyhow!(
        "Could not extract SVG height - no viewBox or height attribute found"
    ))
}

/// Create Typst content with specific edge configuration
fn create_typst_content(
    formula: &str,
    is_display: bool,
    top_edge: &str,
    bottom_edge: &str,
) -> String {
    if is_display {
        format!(
            r#"#set page(width: auto, height: auto, margin: 0pt)
#set text(size: 16pt, fill: white)
#show math.equation: set text(top-edge: "{top_edge}", bottom-edge: "{bottom_edge}")

$ {formula} $"#
        )
    } else {
        format!(
            r#"#set page(width: auto, height: auto, margin: 0pt)
#set text(size: 16pt, fill: white)
#show math.equation: set text(top-edge: "{top_edge}", bottom-edge: "{bottom_edge}")

${formula}$"#
        )
    }
}

/// Create SVG using Typst CLI with specific edge configuration
fn create_typst_svg_with_config(
    formula: &str,
    is_display: bool,
    top_edge: &str,
    bottom_edge: &str,
) -> Result<String> {
    // Try to use Typst CLI if available
    if Command::new("typst").arg("--version").output().is_ok() {
        // Create a temporary Typst file
        let temp_dir = tempfile::tempdir().context("Failed to create temp directory")?;
        let typst_file = temp_dir
            .path()
            .join(format!("formula_{}.typ", hash_formula(formula)));
        let svg_file = temp_dir
            .path()
            .join(format!("formula_{}.svg", hash_formula(formula)));

        // Create Typst content with specified edge configuration
        let typst_content = create_typst_content(formula, is_display, top_edge, bottom_edge);

        fs::write(&typst_file, typst_content).context("Failed to write temporary Typst file")?;

        // Run typst CLI
        let output = Command::new("typst")
            .arg("compile")
            .arg(&typst_file)
            .arg(&svg_file)
            .output()
            .context("Failed to run Typst CLI")?;

        if output.status.success() && svg_file.exists() {
            let svg_content =
                fs::read_to_string(&svg_file).context("Failed to read generated SVG")?;

            // Clean up temp files
            let _ = fs::remove_file(typst_file);
            let _ = fs::remove_file(svg_file);

            Ok(svg_content)
        } else {
            let error_msg = if output.stderr.is_empty() {
                format!("Typst CLI failed with status: {}", output.status)
            } else {
                format!(
                    "Typst CLI failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                )
            };
            Err(anyhow!(error_msg))
        }
    } else {
        Err(anyhow!("Typst CLI not found"))
    }
}

/// Extract baseline position by generating two SVGs and measuring heights
fn extract_baseline_position(formula: &str, is_display: bool) -> Result<(f32, f32)> {
    if is_display {
        // Display math doesn't need baseline alignment
        return Ok((0.0, 0.0));
    }

    // Generate top SVG: bounds to baseline
    let top_svg = create_typst_svg_with_config(
        formula, false, // Always inline for baseline measurement
        "bounds", "baseline",
    )?;

    // Generate bottom SVG: baseline to bounds
    let bottom_svg = create_typst_svg_with_config(
        formula, false, // Always inline for baseline measurement
        "baseline", "bounds",
    )?;

    // Parse SVG heights
    let top_height = parse_svg_height(&top_svg)?;
    let bottom_height = parse_svg_height(&bottom_svg)?;

    // Calculate baseline position
    // baseline_from_top = top_height
    // total_height = top_height + bottom_height
    Ok((top_height, top_height + bottom_height))
}

/// Create SVG using Typst CLI if available, otherwise use placeholder
/// Returns (`svg_content`, `baseline_from_top`, `svg_height`)
fn create_typst_svg(formula: &str, is_display: bool) -> Result<(String, Option<f32>, Option<f32>)> {
    // Extract baseline position for inline math
    let (baseline_from_top, svg_height) = if !is_display {
        match extract_baseline_position(formula, false) {
            Ok((baseline, height)) => (Some(baseline), Some(height)),
            Err(e) => {
                println!("cargo:warning=Failed to extract baseline for formula '{formula}': {e}");
                (None, None)
            }
        }
    } else {
        (None, None)
    };

    // Generate the actual SVG for rendering (bounds-to-bounds)
    let svg_content = create_typst_svg_with_config(formula, is_display, "bounds", "bounds")?;

    Ok((svg_content, baseline_from_top, svg_height))
}

/// Process SVG to make it theme-aware with transparent background
fn process_svg_for_theme(svg_content: &str) -> String {
    // Simple string processing to:
    // 1. Remove white background paths (Typst may add them)
    // 2. Ensure transparent background
    // 3. Handle any remaining black strokes (Typst should set them to white, but be safe)

    let mut processed = String::with_capacity(svg_content.len());
    let lines: Vec<&str> = svg_content.lines().collect();

    for line in lines {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            processed.push_str(line);
            processed.push('\n');
            continue;
        }

        // Remove white background rectangles/paths
        // Typst with fill: white should generate white text, but may still add white backgrounds
        if (trimmed.contains("fill=\"#ffffff\"") || trimmed.contains("fill='#ffffff'"))
            && (trimmed.contains("<rect") || trimmed.contains("<path"))
        {
            // Skip this line (removes the background)
            continue;
        }

        let mut processed_line = line.to_owned();

        // Handle any remaining black strokes (Typst should set them to white with fill: white)
        // But we keep this as a safety measure
        processed_line = processed_line.replace("stroke=\"#000000\"", "stroke=\"#ffffff\"");
        processed_line = processed_line.replace("stroke='#000000'", "stroke=\"#ffffff\"");
        processed_line = processed_line.replace("stroke=\"#000\"", "stroke=\"#ffffff\"");
        processed_line = processed_line.replace("stroke='#000'", "stroke=\"#ffffff\"");

        // Ensure SVG has transparent background
        if trimmed.starts_with("<svg") && !trimmed.contains("style=") {
            // Add style for transparency if not present
            if let Some(pos) = processed_line.find('>') {
                let before = &processed_line[..pos];
                let after = &processed_line[pos..];
                processed_line =
                    format!("{before} style=\"background-color: transparent;\"{after}");
            }
        }

        processed.push_str(&processed_line);
        processed.push('\n');
    }

    processed
}

/// Check if an SVG needs processing (has white background, black strokes, or missing transparent background)
fn svg_needs_processing(svg_content: &str) -> bool {
    // Check for white background rectangles/paths (Typst may add them)
    let has_white_background = (svg_content.contains("fill=\"#ffffff\"")
        || svg_content.contains("fill='#ffffff'"))
        && (svg_content.contains("<rect") || svg_content.contains("<path"));

    // Check for black strokes (Typst should set them to white with fill: white, but be safe)
    let has_black_strokes = svg_content.contains("stroke=\"#000000\"")
        || svg_content.contains("stroke='#000000'")
        || svg_content.contains("stroke=\"#000\"")
        || svg_content.contains("stroke='#000'");

    // Check if SVG has transparent background style
    let has_transparent_bg = svg_content.contains("style=\"background-color: transparent;\"");

    has_white_background || has_black_strokes || !has_transparent_bg
}

/// Create a placeholder SVG for testing
fn create_placeholder_svg(formula: &str, is_display: bool) -> String {
    let width = if is_display { 400 } else { 200 };
    let height = if is_display { 80 } else { 40 };
    let math_type = if is_display { "display" } else { "inline" };

    // Escape special characters in formula for XML
    let _escaped_formula = formula
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;");

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<svg width="{width}" height="{height}" viewBox="0 0 {width} {height}" xmlns="http://www.w3.org/2000/svg" style="background-color: transparent;">
  <!-- Transparent background - no fill rect -->
  <!-- Border to show it's a math area -->
  <rect x="5" y="5" width="190" height="30" fill="none" stroke="currentColor" stroke-width="2" stroke-dasharray="5,5" opacity="0.5"/>
  <!-- Simple math symbol using circle and plus -->
  <circle cx="30" cy="20" r="12" fill="currentColor" opacity="0.3"/>
  <rect x="55" y="15" width="20" height="10" fill="currentColor" opacity="0.3"/>
  <rect x="60" y="10" width="10" height="20" fill="currentColor" opacity="0.3"/>
  <!-- Display type indicator -->
  <rect x="100" y="10" width="80" height="20" fill="currentColor" opacity="0.2" rx="5"/>
  <text x="140" y="25" font-family="Arial" font-size="12" text-anchor="middle" fill="currentColor" opacity="0.5">{math_type}</text>
</svg>"#
    )
}

/// Scan all markdown files in multiple content directories
fn scan_markdown_files(content_dirs: &[&Path]) -> Vec<(PathBuf, String)> {
    let mut files = Vec::new();

    for dir in content_dirs {
        for entry in WalkDir::new(dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
                match fs::read_to_string(path) {
                    Ok(content) => files.push((path.to_path_buf(), content)),
                    Err(e) => println!("cargo:warning=Failed to read {}: {}", path.display(), e),
                }
            }
        }
    }

    files
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=posts/");
    println!("cargo:rerun-if-changed=notes/");
    println!("cargo:rerun-if-changed=reviews/");
    println!("cargo:rerun-if-changed=assets/math/");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_owned());
    let crate_dir = Path::new(&manifest_dir);
    let posts_dir = crate_dir.join("posts");
    let notes_dir = crate_dir.join("notes");
    let reviews_dir = crate_dir.join("reviews");
    let assets_dir = crate_dir.join("assets").join("math");
    let manifest_path = assets_dir.join("manifest.json");

    // Create assets directory if it doesn't exist
    fs::create_dir_all(&assets_dir).context("Failed to create assets directory")?;

    // Load existing manifest
    let mut manifest = if manifest_path.exists() {
        match fs::read_to_string(&manifest_path) {
            Ok(content) => {
                let mut manifest: MathManifest =
                    serde_json::from_str(&content).context("Failed to parse existing manifest")?;

                // Update old manifests to include missing fields
                #[expect(clippy::iter_over_hash_type)]
                for metadata in manifest.formulas.values_mut() {
                    // If is_placeholder field doesn't exist in JSON, it will default to false
                    // which is what we want for old manifests (they were actual renderings)

                    // Old SVGs need theme processing
                    metadata.theme_processed = false;
                }

                manifest
            }
            Err(e) => {
                println!("cargo:warning=Failed to load manifest: {e}");
                MathManifest {
                    formulas: HashMap::new(),
                    updated_at: String::new(),
                }
            }
        }
    } else {
        MathManifest {
            formulas: HashMap::new(),
            updated_at: String::new(),
        }
    };

    // Scan all markdown files from all content directories
    let content_dirs: [&Path; 3] = [&posts_dir, &notes_dir, &reviews_dir];
    let markdown_files = scan_markdown_files(&content_dirs);

    // Extract all formulas from markdown files
    let mut all_formulas = Vec::new();
    for (_file_path, content) in markdown_files {
        let formulas = extract_formulas_from_markdown(&content);
        for (formula, is_display) in formulas {
            all_formulas.push((formula, is_display));
        }
    }

    // Deduplicate formulas
    let unique_formulas: HashSet<_> = all_formulas.into_iter().collect();

    // Track which formulas we're using
    let mut used_hashes = HashSet::new();

    // Counters for summary
    let mut rendered_count = 0;
    let mut placeholder_count = 0;
    let mut error_count = 0;
    let mut processed_count = 0;
    let mut baseline_extracted_count = 0;
    let mut skipped_count = 0;

    // Process each unique formula
    #[expect(clippy::iter_over_hash_type)]
    for (formula, is_display) in unique_formulas {
        let hash = hash_formula(&formula);
        used_hashes.insert(hash.clone());

        let svg_path = assets_dir.join(format!("{hash}.svg"));

        // Check if we need to render or reprocess this formula
        let needs_rendering = if let Some(metadata) = manifest.formulas.get(&hash) {
            !(metadata.formula == formula && metadata.is_display == is_display && svg_path.exists())
        } else {
            true
        };

        // Check if existing SVG needs theme processing
        let needs_theme_processing = if let Some(metadata) = manifest.formulas.get(&hash) {
            !metadata.theme_processed && svg_path.exists()
        } else {
            false
        };

        if needs_rendering {
            // Create SVG using Typst or placeholder
            match create_typst_svg(&formula, is_display) {
                Ok((svg_content, baseline_from_top, svg_height)) => {
                    rendered_count += 1;

                    // Process SVG for theme adaptation
                    let processed_svg = if svg_needs_processing(&svg_content) {
                        process_svg_for_theme(&svg_content)
                    } else {
                        svg_content.clone()
                    };

                    // Save processed SVG
                    fs::write(&svg_path, &processed_svg)
                        .with_context(|| format!("Failed to write SVG: {}", svg_path.display()))?;

                    // Update manifest with baseline data
                    manifest.formulas.insert(
                        hash.clone(),
                        FormulaMetadata {
                            formula: formula.clone(),
                            is_display,
                            rendered_at: Utc::now().to_rfc3339(),
                            svg_size: processed_svg.len(),
                            hash: hash.clone(),
                            is_placeholder: false,
                            theme_processed: !svg_needs_processing(&svg_content),
                            baseline_from_top,
                            svg_height,
                        },
                    );
                }
                Err(e) => {
                    error_count += 1;
                    placeholder_count += 1;
                    println!("cargo:warning=Error rendering formula '{formula}': {e}");
                    println!("cargo:warning=Creating placeholder SVG for formula: {formula}");

                    // Create placeholder SVG
                    let placeholder_svg = create_placeholder_svg(&formula, is_display);
                    let processed_placeholder = process_svg_for_theme(&placeholder_svg);

                    fs::write(&svg_path, &processed_placeholder).with_context(|| {
                        format!("Failed to write placeholder SVG: {}", svg_path.display())
                    })?;

                    // Update manifest with placeholder info (no baseline data for placeholders)
                    manifest.formulas.insert(
                        hash.clone(),
                        FormulaMetadata {
                            formula: formula.clone(),
                            is_display,
                            rendered_at: Utc::now().to_rfc3339(),
                            svg_size: processed_placeholder.len(),
                            hash: hash.clone(),
                            is_placeholder: true,
                            theme_processed: true,
                            baseline_from_top: None,
                            svg_height: None,
                        },
                    );
                }
            }
        } else {
            skipped_count += 1;

            // Check if we need to extract baseline data for existing formulas
            let needs_baseline_data = if let Some(metadata) = manifest.formulas.get(&hash) {
                metadata.baseline_from_top.is_none()
                    && !metadata.is_display
                    && !metadata.is_placeholder
            } else {
                false
            };

            // Extract baseline data if missing for inline math
            if needs_baseline_data {
                match extract_baseline_position(&formula, false) {
                    Ok((baseline, height)) => {
                        if let Some(metadata) = manifest.formulas.get_mut(&hash) {
                            metadata.baseline_from_top = Some(baseline);
                            metadata.svg_height = Some(height);
                            baseline_extracted_count += 1;
                            println!("cargo:warning=Extracted baseline data for existing formula: {formula}");
                        }
                    }
                    Err(e) => {
                        println!("cargo:warning=Failed to extract baseline for existing formula '{formula}': {e}");
                    }
                }
            }

            // Process existing SVG for theme if needed
            if needs_theme_processing {
                processed_count += 1;

                match fs::read_to_string(&svg_path) {
                    Ok(svg_content) => {
                        if svg_needs_processing(&svg_content) {
                            let processed_svg = process_svg_for_theme(&svg_content);

                            // Save processed SVG
                            fs::write(&svg_path, &processed_svg).with_context(|| {
                                format!("Failed to write processed SVG: {}", svg_path.display())
                            })?;

                            // Update manifest to mark as theme processed
                            if let Some(metadata) = manifest.formulas.get_mut(&hash) {
                                metadata.theme_processed = true;
                            }
                        }
                    }
                    Err(e) => {
                        println!("cargo:warning=Failed to read existing SVG for processing: {e}");
                    }
                }
            }
        }
    }

    // Clean up unused formulas
    let unused_hashes: Vec<_> = manifest
        .formulas
        .keys()
        .filter(|hash| !used_hashes.contains(*hash))
        .cloned()
        .collect();

    for hash in &unused_hashes {
        let svg_path = assets_dir.join(format!("{hash}.svg"));
        if svg_path.exists() {
            let _ = fs::remove_file(&svg_path);
        }
        manifest.formulas.remove(hash);
    }

    // Update manifest timestamp
    manifest.updated_at = Utc::now().to_rfc3339();

    // Save updated manifest
    let manifest_json =
        serde_json::to_string_pretty(&manifest).context("Failed to serialize manifest")?;
    fs::write(&manifest_path, manifest_json).context("Failed to save manifest")?;

    // Output summary
    let total_formulas = rendered_count + placeholder_count + skipped_count;
    println!("cargo:warning=Found {total_formulas} unique math formulas");

    if rendered_count > 0 {
        println!("cargo:warning=  • Rendered {rendered_count} new formulas");
    }

    if placeholder_count > 0 {
        println!(
            "cargo:warning=  • Created {placeholder_count} placeholder formulas (Typst errors)"
        );
    }

    if error_count > 0 {
        println!("cargo:warning=  • Encountered {error_count} Typst rendering errors");
    }

    if processed_count > 0 {
        println!(
            "cargo:warning=  • Processed {processed_count} existing SVGs for theme adaptation"
        );
    }

    if baseline_extracted_count > 0 {
        println!(
            "cargo:warning=  • Extracted baseline data for {baseline_extracted_count} existing formulas"
        );
    }

    if skipped_count > 0 {
        println!("cargo:warning=  • Skipped {skipped_count} already rendered formulas");
    }

    let unused_count = unused_hashes.len();
    if unused_count > 0 {
        println!("cargo:warning=  • Removed {unused_count} unused formula SVGs");
    }

    println!("cargo:warning=Math processing completed successfully");

    Ok(())
}
