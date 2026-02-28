//! Shared utilities for file embedding macros.

use std::env;
use std::path::Path;

use anyhow::{Context, Result};
use walkdir::WalkDir;

/// Information about a file to be embedded.
#[derive(Debug, Clone)]
pub struct FileInfo {
    /// Filename (with extension)
    pub filename: String,
    /// Basename (filename without extension)
    pub basename: String,
}

/// Scan a directory for files matching a glob pattern.
///
/// # Arguments
/// * `relative_dir` - Relative path from source file (e.g., "../../assets/math/")
/// * `pattern` - Glob pattern (e.g., "*.svg")
///
/// # Returns
/// Vector of file information sorted by filename.
pub fn scan_directory(relative_dir: &str, pattern: &str) -> Result<Vec<FileInfo>> {
    // Get the crate root directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .context("CARGO_MANIFEST_DIR environment variable not set")?;

    // Convert source-relative path to crate-relative path.
    // The user provides a path relative to the source file (e.g., "../../assets/math/")
    // but we need to find files from the crate root.
    //
    // We need to handle this properly with platform-agnostic path handling.
    let crate_relative_dir = {
        // Parse the relative path
        let rel_path = Path::new(relative_dir);

        // Count how many ".." components at the start
        let components = rel_path.components();
        let mut parent_count = 0;

        for component in components.clone() {
            match component {
                std::path::Component::ParentDir => parent_count += 1,
                _ => break,
            }
        }

        // If we have parent dir components, we need to remove them
        // to get the crate-relative path
        if parent_count > 0 {
            // Skip the parent dir components
            let new_components: Vec<_> = rel_path.components().skip(parent_count).collect();

            // Reconstruct path
            if new_components.is_empty() {
                Path::new("").to_path_buf()
            } else {
                new_components.iter().collect()
            }
        } else {
            rel_path.to_path_buf()
        }
    };

    // Convert to absolute path
    let target_dir = Path::new(&manifest_dir).join(crate_relative_dir);

    // Validate directory exists
    if !target_dir.exists() {
        anyhow::bail!("Directory does not exist: {}", target_dir.display());
    }

    if !target_dir.is_dir() {
        anyhow::bail!("Path is not a directory: {}", target_dir.display());
    }

    // Compile glob pattern
    let glob_pattern = glob::Pattern::new(pattern)
        .with_context(|| format!("Invalid glob pattern: {}", pattern))?;

    let mut files = Vec::new();

    // Walk directory (non-recursive for now)
    for entry in WalkDir::new(&target_dir)
        .max_depth(1) // Only immediate files, not subdirectories
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Skip directories
        if !path.is_file() {
            continue;
        }

        // Get filename
        let filename = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => continue, // Skip files with non-UTF-8 names
        };

        // Apply glob pattern
        if !glob_pattern.matches(filename) {
            continue;
        }

        // Extract basename (filename without extension)
        let basename = match Path::new(filename).file_stem().and_then(|s| s.to_str()) {
            Some(stem) => stem.to_string(),
            None => filename.to_string(),
        };

        files.push(FileInfo {
            filename: filename.to_string(),
            basename,
        });
    }

    // Sort by filename for deterministic output
    files.sort_by(|a, b| a.filename.cmp(&b.filename));

    Ok(files)
}

/// Generate the include path for a file.
///
/// This returns the path that should be used with `include_bytes!()` or `include_str!()`.
/// It's the relative path from the source file to the target file.
///
/// # Arguments
/// * `relative_dir` - Relative directory path (e.g., "../../assets/math/")
/// * `filename` - Filename (e.g., "hash.svg")
pub fn include_path(relative_dir: &str, filename: &str) -> String {
    // Use Path to join paths in a platform-agnostic way
    let path = Path::new(relative_dir).join(filename);

    // Convert to string, using forward slashes for consistency
    // (include_str!/include_bytes! expect forward slashes even on Windows)
    path.to_string_lossy().replace('\\', "/")
}

/// Validate that a relative directory path is reasonable.
pub fn validate_relative_dir(relative_dir: &str) -> Result<()> {
    if relative_dir.is_empty() {
        anyhow::bail!("Relative directory path cannot be empty");
    }

    // Check for common issues using Path
    let path = Path::new(relative_dir);

    if path.is_absolute() {
        anyhow::bail!("Relative directory path should not be absolute. Use a relative path like '../../assets/math/'");
    }

    // Check for parent directory components not at the start
    let components: Vec<_> = path.components().collect();
    let mut found_non_parent = false;

    for (_i, component) in components.iter().enumerate() {
        match component {
            std::path::Component::ParentDir => {
                if found_non_parent {
                    eprintln!(
                        "Warning: Relative path '{}' contains '..' after other path components",
                        relative_dir
                    );
                    break;
                }
            }
            std::path::Component::CurDir => {
                // Current dir is fine
            }
            _ => {
                found_non_parent = true;
            }
        }
    }

    Ok(())
}
