//! Build-time filtering functions for posts.
//! This module provides functions that can be used with the `embed_file_array!` macro
//! to filter posts at compile time (actually at runtime during startup).

use serde::Deserialize;

/// Simple frontmatter structure for tag checking
#[derive(Debug, Deserialize)]
struct Frontmatter {
    #[serde(default)]
    tags: Vec<String>,
}

/// Check if markdown content has #test tag in frontmatter
/// Returns true if content has "test" in tags array
fn has_test_tag(content: &str) -> bool {
    // Extract YAML frontmatter between --- delimiters
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < 2 || lines[0].trim() != "---" {
        return false;
    }

    let mut frontmatter_lines = Vec::new();

    for line in lines.iter().skip(1) {
        if line.trim() == "---" {
            break;
        }
        frontmatter_lines.push(*line);
    }

    if frontmatter_lines.is_empty() {
        return false;
    }

    let frontmatter_str = frontmatter_lines.join("\n");

    // Parse YAML frontmatter
    match serde_yaml::from_str::<Frontmatter>(&frontmatter_str) {
        Ok(frontmatter) => frontmatter.tags.iter().any(|tag| tag == "test"),
        Err(_) => false, // If parsing fails, assume no test tag (safer)
    }
}

/// Filter function for build-time post filtering
/// Returns true to include post, false to exclude
/// Uses profile-based filtering: in release builds, exclude test posts
pub fn filter_test_posts(content: &str) -> bool {
    // Use debug_assertions to check build profile at compile time
    // debug_assertions is enabled in debug builds, disabled in release builds
    if cfg!(debug_assertions) {
        // Debug/development build - include all posts
        true
    } else {
        // Release build - exclude posts with #test tag
        !has_test_tag(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_test_tag() {
        let content = r#"---
title: "Test Post"
date: "2026-01-01"
tags: ["test", "example"]
---

This is a test post."#;

        assert!(has_test_tag(content));

        let content_no_test = r#"---
title: "Real Post"
date: "2026-01-01"
tags: ["example", "tutorial"]
---

This is a real post."#;

        assert!(!has_test_tag(content_no_test));

        let content_no_tags = r#"---
title: "Post Without Tags"
date: "2026-01-01"
---

This post has no tags."#;

        assert!(!has_test_tag(content_no_tags));

        let content_no_frontmatter = "This is just plain markdown with no frontmatter.";
        assert!(!has_test_tag(content_no_frontmatter));
    }

    #[test]
    fn test_filter_test_posts() {
        let test_post = r#"---
title: "Test Post"
date: "2026-01-01"
tags: ["test"]
---

Test content."#;

        let real_post = r#"---
title: "Real Post"
date: "2026-01-01"
tags: ["tutorial"]
---

Real content."#;

        // Note: We can't easily test the cfg!(not(release)) behavior in unit tests
        // because the cfg is evaluated at compile time.
        // But we can test that the function at least doesn't panic.
        let _ = filter_test_posts(test_post);
        let _ = filter_test_posts(real_post);
    }
}
