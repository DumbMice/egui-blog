//! Markdown file loading and parsing for blog posts.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use thiserror::Error;

use crate::posts::BlogPost;

/// Frontmatter metadata for a blog post.
#[derive(Debug, Clone, Deserialize)]
pub struct Frontmatter {
    /// Post title
    pub title: String,
    /// Publication date (YYYY-MM-DD format)
    pub date: String,
    /// Optional tags/categories
    #[serde(default)]
    pub tags: Vec<String>,
    /// Optional URL-friendly slug (auto-generated from title if not provided)
    #[serde(default)]
    pub slug: Option<String>,
    /// Optional content type (post, note, review)
    #[serde(default)]
    pub content_type: Option<String>,
}

/// Errors that can occur during post loading.
#[derive(Debug, Error)]
pub enum LoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Invalid file format: {0}")]
    #[cfg(test)]
    Format(String),

    #[error("Missing frontmatter delimiter")]
    MissingDelimiter,

    #[error("File not found: {0:?}")]
    #[cfg(test)]
    FileNotFound(PathBuf),

    #[error("Directory not found: {0:?}")]
    #[cfg(test)]
    DirectoryNotFound(PathBuf),
}

/// Load a content item from a markdown file.
///
/// File format:
/// ```markdown
/// ---
/// title: "Post Title"
/// date: "2026-02-10"
/// tags: ["tag1", "tag2"]
/// type: "post" # optional: "post", "note", or "review"
/// ---
///
/// Content here...
/// ```
pub fn parse_post_content(
    content: &str,
    id: usize,
    default_content_type: crate::posts::ContentType,
) -> Result<BlogPost, LoadError> {
    // Split by frontmatter delimiter
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(LoadError::MissingDelimiter);
    }

    let frontmatter_yaml = parts[1].trim();
    let markdown_content = parts[2].trim();

    // Parse frontmatter
    let frontmatter: Frontmatter = serde_yaml::from_str(frontmatter_yaml)?;

    // Determine content type from frontmatter or use default
    let content_type = if let Some(type_str) = &frontmatter.content_type {
        crate::posts::ContentType::from_str(type_str).unwrap_or(default_content_type)
    } else {
        default_content_type
    };

    // Generate slug from title if not provided in frontmatter
    let slug = frontmatter
        .slug
        .unwrap_or_else(|| crate::posts::BlogPost::generate_slug(&frontmatter.title));

    // Create blog post with preprocessed content
    let manifest = crate::math::load_manifest();
    let processed_content =
        crate::ui::markdown::extract_and_replace_math_formulas(markdown_content, manifest);

    Ok(BlogPost {
        id,
        content_type,
        title: frontmatter.title,
        slug,
        content: markdown_content.to_owned(),
        date: frontmatter.date,
        tags: frontmatter.tags,
        cached_processed_content: Some(processed_content),
    })
}

/// Load a post from a file.
pub fn load_post_from_file(
    path: &Path,
    id: usize,
    default_content_type: crate::posts::ContentType,
) -> Result<BlogPost, LoadError> {
    let content = fs::read_to_string(path)?;
    parse_post_content(&content, id, default_content_type)
}

/// Load all posts from a directory.
#[expect(unused)]
pub fn load_posts_from_dir(
    dir: &Path,
    default_content_type: crate::posts::ContentType,
) -> Result<Vec<BlogPost>, LoadError> {
    let mut posts = Vec::new();

    if !dir.exists() {
        return Ok(posts); // Empty directory is ok
    }

    let mut entries: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| {
            path.extension()
                .is_some_and(|ext| ext == "md" || ext == "markdown")
        })
        .collect();

    // Sort by filename for consistent ordering
    entries.sort();

    for (idx, path) in entries.iter().enumerate() {
        match load_post_from_file(path, idx, default_content_type) {
            Ok(post) => posts.push(post),
            Err(err) => log::warn!("Failed to load {}: {}", path.display(), err),
        }
    }

    Ok(posts)
}

/// Load all content embedded at compile time.
pub fn load_embedded_content() -> Vec<BlogPost> {
    use blog_macros::embed_file_array;

    let mut all_content = Vec::new();
    let mut posts = Vec::new();

    // Load posts from posts directory
    let post_contents = embed_file_array!("../../posts/", pattern = "*.md");
    for (id, content) in post_contents.iter().enumerate() {
        all_content.push((content, id, crate::posts::ContentType::Post));
    }

    // Load notes from notes directory
    let note_contents = embed_file_array!("../../notes/", pattern = "*.md");
    for (id, content) in note_contents.iter().enumerate() {
        all_content.push((
            content,
            id + post_contents.len(),
            crate::posts::ContentType::Note,
        ));
    }

    // Load reviews from reviews directory
    let review_contents = embed_file_array!("../../reviews/", pattern = "*.md");
    for (id, content) in review_contents.iter().enumerate() {
        all_content.push((
            content,
            id + post_contents.len() + note_contents.len(),
            crate::posts::ContentType::Review,
        ));
    }

    // Parse all content
    for (content, id, content_type) in all_content {
        match parse_post_content(content, id, content_type) {
            Ok(post) => {
                posts.push(post);
            }
            Err(err) => log::warn!("Failed to parse embedded content {id}: {err}"),
        }
    }

    // Sort posts by date in reverse chronological order (newest first)
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    posts
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_load_error_variants() {
        // Test new error variants
        let io_error = LoadError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
        let yaml_error =
            LoadError::Yaml(serde_yaml::from_str::<Frontmatter>("invalid: yaml: [").unwrap_err());
        let format_error = LoadError::Format("test".to_string());
        let missing_delimiter = LoadError::MissingDelimiter;
        let file_not_found = LoadError::FileNotFound(PathBuf::from("test.md"));
        let dir_not_found = LoadError::DirectoryNotFound(PathBuf::from("posts"));

        // Test Display impl works
        assert!(io_error.to_string().contains("IO error"));
        assert!(yaml_error.to_string().contains("YAML parsing error"));
        assert!(format_error.to_string().contains("Invalid file format"));
        assert!(missing_delimiter
            .to_string()
            .contains("Missing frontmatter delimiter"));
        assert!(file_not_found.to_string().contains("File not found"));
        assert!(dir_not_found.to_string().contains("Directory not found"));
    }

    #[test]
    fn test_content_type_parsing() {
        // Test post content type
        let post_content = r#"---
title: Test Post
date: 2026-03-01
tags: [test]
type: "post"
---

This is a test post."#;

        let result = parse_post_content(post_content, 0, crate::posts::ContentType::Post);
        assert!(result.is_ok());
        let post = result.unwrap();
        assert_eq!(post.content_type, crate::posts::ContentType::Post);
        assert_eq!(post.title, "Test Post");

        // Test note content type
        let note_content = r#"---
title: Test Note
date: 2026-03-01
tags: [test]
type: "note"
status: "draft"
---

This is a test note."#;

        let result = parse_post_content(note_content, 1, crate::posts::ContentType::Note);
        assert!(result.is_ok());
        let note = result.unwrap();
        assert_eq!(note.content_type, crate::posts::ContentType::Note);
        assert_eq!(note.title, "Test Note");

        // Test review content type
        let review_content = r#"---
title: Test Review
date: 2026-03-01
tags: [test]
type: "review"
rating: 5
---

This is a test review."#;

        let result = parse_post_content(review_content, 2, crate::posts::ContentType::Review);
        assert!(result.is_ok());
        let review = result.unwrap();
        assert_eq!(review.content_type, crate::posts::ContentType::Review);
        assert_eq!(review.title, "Test Review");

        // Test default content type (when type is not specified)
        let default_content = r#"---
title: Default Content
date: 2026-03-01
tags: [test]
---

This is default content."#;

        let result = parse_post_content(default_content, 3, crate::posts::ContentType::Post);
        assert!(result.is_ok());
        let default_post = result.unwrap();
        assert_eq!(default_post.content_type, crate::posts::ContentType::Post);
        assert_eq!(default_post.title, "Default Content");
    }
}
