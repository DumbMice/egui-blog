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

use anyhow::{Context, Result, anyhow};
use chrono::Utc;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
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
                        formulas.push((formula.to_string(), true));
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
                        formulas.push((formula.to_string(), true));
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
                        formulas.push((formula.to_string(), false));
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

/// Create SVG using Typst CLI if available, otherwise use placeholder
fn create_typst_svg(formula: &str, is_display: bool) -> Result<String> {
    // Try to use Typst CLI if available
    if Command::new("typst").arg("--version").output().is_ok() {
        // Create a temporary Typst file
        let temp_dir = std::env::temp_dir();
        let typst_file = temp_dir.join(format!("formula_{}.typ", hash_formula(formula)));
        let svg_file = temp_dir.join(format!("formula_{}.svg", hash_formula(formula)));

        // Create Typst content
        let typst_content = if is_display {
            format!(
                r#"#set page(width: auto, height: auto, margin: 0pt)
#set text(size: 16pt, fill: white)
#show math.equation: set text(top-edge: "bounds", bottom-edge: "bounds")

$ {formula} $"#
            )
        } else {
            // For inline math, we need more vertical margin to accommodate fractions
            // and other elements that extend beyond the normal text bounds
            format!(
                r#"#set page(width: auto, height: auto, margin: 0pt)
#set text(size: 16pt, fill: white)
#show math.equation: set text(top-edge: "bounds", bottom-edge: "bounds")

${formula}$"#
            )
        };

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

            return Ok(svg_content);
        } else {
            let error_msg = if output.stderr.is_empty() {
                format!("Typst CLI failed with status: {}", output.status)
            } else {
                format!(
                    "Typst CLI failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                )
            };
            return Err(anyhow!(error_msg));
        }
    } else {
        return Err(anyhow!("Typst CLI not found"));
    }
}

/// Process SVG to make it theme-aware with transparent background
fn process_svg_for_theme(svg_content: &str) -> Result<String> {
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

        let mut processed_line = line.to_string();

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
                processed_line = format!(
                    "{} style=\"background-color: transparent;\"{}",
                    before, after
                );
            }
        }

        processed.push_str(&processed_line);
        processed.push('\n');
    }

    Ok(processed)
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
        r##"<?xml version="1.0" encoding="UTF-8"?>
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
</svg>"##
    )
}

/// Scan all markdown files in the posts directory
fn scan_markdown_files(posts_dir: &Path) -> Result<Vec<(PathBuf, String)>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(posts_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            match fs::read_to_string(path) {
                Ok(content) => files.push((path.to_path_buf(), content)),
                Err(e) => eprintln!("Warning: Failed to read {}: {}", path.display(), e),
            }
        }
    }

    Ok(files)
}

/// Generate embedded.rs file with include_bytes! macros
fn generate_embedded_rs(assets_dir: &Path, manifest: &MathManifest) -> Result<String> {
    let mut code = String::new();

    // Header
    code.push_str("//! Embedded math assets generated by build script.\n");
    code.push_str("//! This file is auto-generated - DO NOT EDIT MANUALLY.\n\n");
    code.push_str("use serde::{Deserialize, Serialize};\n");
    code.push_str("use std::collections::HashMap;\n\n");

    // FormulaMetadata struct
    code.push_str("/// Metadata for a rendered formula\n");
    code.push_str("#[derive(Debug, Serialize, Deserialize, Clone)]\n");
    code.push_str("pub struct FormulaMetadata {\n");
    code.push_str("    /// The original formula text\n");
    code.push_str("    pub formula: String,\n");
    code.push_str("    /// Whether this is display math (true) or inline math (false)\n");
    code.push_str("    pub is_display: bool,\n");
    code.push_str("    /// When this formula was last rendered (ISO 8601)\n");
    code.push_str("    pub rendered_at: String,\n");
    code.push_str("    /// Size of the SVG file in bytes\n");
    code.push_str("    pub svg_size: usize,\n");
    code.push_str("    /// The hash used as filename\n");
    code.push_str("    pub hash: String,\n");
    code.push_str(
        "    /// Whether this is a placeholder SVG (true) or actual Typst rendering (false)\n",
    );
    code.push_str("    pub is_placeholder: bool,\n");
    code.push_str("    /// Whether this SVG has been processed for theme adaptation (true)\n");
    code.push_str("    #[serde(default = \"default_theme_processed\")]\n");
    code.push_str("    pub theme_processed: bool,\n");
    code.push_str("}\n\n");

    code.push_str("fn default_theme_processed() -> bool {\n");
    code.push_str("    true\n");
    code.push_str("}\n\n");

    // MathManifest struct
    code.push_str("/// Manifest tracking all rendered formulas\n");
    code.push_str("#[derive(Debug, Serialize, Deserialize)]\n");
    code.push_str("pub struct MathManifest {\n");
    code.push_str("    /// Map from formula hash to metadata\n");
    code.push_str("    pub formulas: HashMap<String, FormulaMetadata>,\n");
    code.push_str("    /// When this manifest was last updated\n");
    code.push_str("    pub updated_at: String,\n");
    code.push_str("}\n\n");

    // MathManifest implementation
    code.push_str("impl MathManifest {\n");
    code.push_str("    /// Find the hash for a given formula text and display type\n");
    code.push_str(
        "    pub fn find_hash(&self, formula: &str, is_display: bool) -> Option<&str> {\n",
    );
    code.push_str("        for (hash, metadata) in &self.formulas {\n");
    code.push_str(
        "            if metadata.formula == formula && metadata.is_display == is_display {\n",
    );
    code.push_str("                return Some(hash);\n");
    code.push_str("            }\n");
    code.push_str("        }\n");
    code.push_str("        None\n");
    code.push_str("    }\n");
    code.push_str("\n");
    code.push_str("    /// Get all formula hashes\n");
    code.push_str("    pub fn all_hashes(&self) -> Vec<&str> {\n");
    code.push_str("        self.formulas.keys().map(|k| k.as_str()).collect()\n");
    code.push_str("    }\n");
    code.push_str("\n");
    code.push_str("    /// Get metadata for a hash\n");
    code.push_str("    pub fn get_metadata(&self, hash: &str) -> Option<&FormulaMetadata> {\n");
    code.push_str("        self.formulas.get(hash)\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    // Manifest JSON
    let manifest_json =
        serde_json::to_string_pretty(manifest).context("Failed to serialize manifest")?;
    code.push_str("/// Load the embedded manifest\n");
    code.push_str("pub fn load_manifest() -> MathManifest {\n");
    code.push_str("    let manifest_json = r#\"");
    code.push_str(&manifest_json);
    code.push_str("\"#;\n");
    code.push_str("    serde_json::from_str(manifest_json).unwrap_or_else(|e| {\n");
    code.push_str("        eprintln!(\"Failed to parse embedded math manifest: {}\", e);\n");
    code.push_str("        MathManifest {\n");
    code.push_str("            formulas: HashMap::new(),\n");
    code.push_str("            updated_at: String::new(),\n");
    code.push_str("        }\n");
    code.push_str("    })\n");
    code.push_str("}\n\n");

    // SVG bytes lookup function
    code.push_str("/// Get SVG bytes for a formula hash\n");
    code.push_str("pub fn get_svg_bytes(hash: &str) -> Option<&'static [u8]> {\n");
    code.push_str("    match hash {\n");

    // Add match arms for each formula
    for (hash, _metadata) in &manifest.formulas {
        let svg_path = assets_dir.join(format!("{}.svg", hash));
        if svg_path.exists() {
            // Get relative path from embedded.rs (in src/math/) to SVG file (in assets/math/)
            // embedded.rs is at crate_root/src/math/embedded.rs
            // SVG files are at crate_root/assets/math/*.svg
            // So we need to go up two levels: ../../assets/math/*.svg
            let relative_path = format!("../../assets/math/{}.svg", hash);

            code.push_str(&format!(
                "        \"{}\" => Some(include_bytes!(\"{}\")),\n",
                hash, relative_path
            ));
        }
    }

    code.push_str("        _ => None,\n");
    code.push_str("    }\n");
    code.push_str("}\n");

    Ok(code)
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=posts/");
    println!("cargo:rerun-if-changed=assets/math/");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let crate_dir = Path::new(&manifest_dir);
    let posts_dir = crate_dir.join("posts");
    let assets_dir = crate_dir.join("assets").join("math");
    let manifest_path = assets_dir.join("manifest.json");
    let embedded_rs_path = crate_dir.join("src").join("math").join("embedded.rs");

    // Create assets directory if it doesn't exist
    fs::create_dir_all(&assets_dir).context("Failed to create assets directory")?;

    // Load existing manifest
    let mut manifest = if manifest_path.exists() {
        match fs::read_to_string(&manifest_path) {
            Ok(content) => {
                let mut manifest: MathManifest =
                    serde_json::from_str(&content).context("Failed to parse existing manifest")?;

                // Update old manifests to include missing fields
                for metadata in manifest.formulas.values_mut() {
                    // If is_placeholder field doesn't exist in JSON, it will default to false
                    // which is what we want for old manifests (they were actual renderings)

                    // Old SVGs need theme processing
                    metadata.theme_processed = false;
                }

                manifest
            }
            Err(e) => {
                eprintln!("Warning: Failed to load manifest: {}", e);
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

    // Scan all markdown files
    let markdown_files = scan_markdown_files(&posts_dir)?;

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

    println!(
        "cargo:warning=Found {} unique math formulas",
        unique_formulas.len()
    );

    // Track which formulas we're using
    let mut used_hashes = HashSet::new();

    // Process each unique formula
    for (formula, is_display) in unique_formulas {
        let hash = hash_formula(&formula);
        used_hashes.insert(hash.clone());

        let svg_path = assets_dir.join(format!("{}.svg", hash));

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
            println!("cargo:warning=Rendering formula: {}", formula);

            // Create SVG using Typst or placeholder
            match create_typst_svg(&formula, is_display) {
                Ok(svg_content) => {
                    // Process SVG for theme adaptation
                    let processed_svg = if svg_needs_processing(&svg_content) {
                        match process_svg_for_theme(&svg_content) {
                            Ok(processed) => {
                                println!("cargo:warning=Processed SVG for theme adaptation");
                                processed
                            }
                            Err(e) => {
                                eprintln!(
                                    "cargo:warning=Failed to process SVG: {}, using original",
                                    e
                                );
                                svg_content
                            }
                        }
                    } else {
                        svg_content
                    };

                    // Save processed SVG
                    fs::write(&svg_path, &processed_svg)
                        .with_context(|| format!("Failed to write SVG: {}", svg_path.display()))?;

                    // Update manifest
                    manifest.formulas.insert(
                        hash.clone(),
                        FormulaMetadata {
                            formula: formula.clone(),
                            is_display,
                            rendered_at: Utc::now().to_rfc3339(),
                            svg_size: processed_svg.len(),
                            hash: hash.clone(),
                            is_placeholder: false,
                            theme_processed: true,
                        },
                    );
                }
                Err(e) => {
                    eprintln!("cargo:warning=Error rendering formula '{}': {}", formula, e);
                    eprintln!(
                        "cargo:warning=Creating placeholder SVG for formula: {}",
                        formula
                    );

                    // Create placeholder SVG
                    let placeholder_svg = create_placeholder_svg(&formula, is_display);

                    // Process placeholder SVG for theme adaptation
                    let processed_placeholder = if svg_needs_processing(&placeholder_svg) {
                        match process_svg_for_theme(&placeholder_svg) {
                            Ok(processed) => {
                                println!(
                                    "cargo:warning=Processed placeholder SVG for theme adaptation"
                                );
                                processed
                            }
                            Err(e) => {
                                eprintln!(
                                    "cargo:warning=Failed to process placeholder SVG: {}, using original",
                                    e
                                );
                                placeholder_svg
                            }
                        }
                    } else {
                        placeholder_svg
                    };

                    // Save processed placeholder SVG
                    fs::write(&svg_path, &processed_placeholder).with_context(|| {
                        format!("Failed to write placeholder SVG: {}", svg_path.display())
                    })?;

                    // Update manifest with placeholder flag
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
                        },
                    );
                }
            }
        } else {
            println!("cargo:warning=Formula already rendered: {}", formula);

            // Process existing SVG for theme if needed
            if needs_theme_processing {
                println!(
                    "cargo:warning=Processing existing SVG for theme adaptation: {}",
                    formula
                );

                match fs::read_to_string(&svg_path) {
                    Ok(svg_content) => {
                        if svg_needs_processing(&svg_content) {
                            match process_svg_for_theme(&svg_content) {
                                Ok(processed_svg) => {
                                    // Save processed SVG
                                    fs::write(&svg_path, &processed_svg).with_context(|| {
                                        format!(
                                            "Failed to write processed SVG: {}",
                                            svg_path.display()
                                        )
                                    })?;

                                    // Update manifest to mark as processed
                                    if let Some(metadata) = manifest.formulas.get_mut(&hash) {
                                        metadata.theme_processed = true;
                                        metadata.svg_size = processed_svg.len();
                                        metadata.rendered_at = Utc::now().to_rfc3339();
                                        println!(
                                            "cargo:warning=Successfully processed SVG for theme adaptation"
                                        );
                                    }
                                }
                                Err(e) => {
                                    eprintln!(
                                        "cargo:warning=Failed to process existing SVG: {}, keeping original",
                                        e
                                    );
                                }
                            }
                        } else {
                            // SVG doesn't need processing, mark as processed
                            if let Some(metadata) = manifest.formulas.get_mut(&hash) {
                                metadata.theme_processed = true;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "cargo:warning=Failed to read existing SVG for processing: {}",
                            e
                        );
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

    for hash in unused_hashes {
        let svg_path = assets_dir.join(format!("{}.svg", hash));
        if svg_path.exists() {
            println!("cargo:warning=Removing unused formula: {}", hash);
            let _ = fs::remove_file(&svg_path);
        }
        manifest.formulas.remove(&hash);
    }

    // Update manifest timestamp
    manifest.updated_at = Utc::now().to_rfc3339();

    // Save updated manifest
    let manifest_json =
        serde_json::to_string_pretty(&manifest).context("Failed to serialize manifest")?;
    fs::write(&manifest_path, manifest_json).context("Failed to save manifest")?;

    println!(
        "cargo:warning=Manifest saved to: {}",
        manifest_path.display()
    );

    // Generate embedded.rs file
    let embedded_code = generate_embedded_rs(&assets_dir, &manifest)?;
    fs::write(&embedded_rs_path, embedded_code).context("Failed to write embedded.rs")?;

    println!(
        "cargo:warning=Generated embedded.rs at: {}",
        embedded_rs_path.display()
    );

    println!("cargo:warning=Math processing completed successfully");

    Ok(())
}
