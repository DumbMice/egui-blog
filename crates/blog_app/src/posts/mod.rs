//! Blog post data structures and management.

mod loader;
mod state; // NEW

#[expect(unused_imports)]
pub use loader::{
    load_embedded_posts, load_post_from_file, load_posts_from_dir, parse_post_content, Frontmatter,
    LoadError,
};
pub use state::PostManagerState; // NEW

/// Type of content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ContentType {
    /// Public blog posts
    Post,
    /// Private notes
    Note,
    /// Research reviews
    Review,
}

impl ContentType {
    /// Get the display name for this content type
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Post => "Posts",
            Self::Note => "Notes",
            Self::Review => "Reviews",
        }
    }

    /// Get the URL path prefix for this content type
    pub fn url_prefix(&self) -> &'static str {
        match self {
            Self::Post => "posts",
            Self::Note => "notes",
            Self::Review => "reviews",
        }
    }

    /// Get the directory name for this content type
    pub fn directory_name(&self) -> &'static str {
        match self {
            Self::Post => "posts",
            Self::Note => "notes",
            Self::Review => "reviews",
        }
    }

    /// Parse content type from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "post" | "posts" => Some(Self::Post),
            "note" | "notes" => Some(Self::Note),
            "review" | "reviews" => Some(Self::Review),
            _ => None,
        }
    }
}

impl Default for ContentType {
    fn default() -> Self {
        Self::Post
    }
}

/// A content item (post, note, or review).
#[derive(Clone, Debug)]
pub struct BlogPost {
    /// Unique identifier
    pub id: usize,
    /// Content type
    pub content_type: ContentType,
    /// Post title
    pub title: String,
    /// URL-friendly identifier (slug)
    pub slug: String,
    /// Post content (markdown format)
    pub content: String,
    /// Publication date
    pub date: String,
    /// Optional tags/categories
    pub tags: Vec<String>,
    /// Cached processed content with math placeholders
    cached_processed_content: Option<String>,
}

impl BlogPost {
    /// Create a new content item.
    pub fn new(
        id: usize,
        content_type: ContentType,
        title: &str,
        slug: &str,
        content: &str,
        date: &str,
    ) -> Self {
        let manifest = crate::math::load_manifest();
        let processed_content =
            crate::ui::markdown::extract_and_replace_math_formulas(content, manifest);

        // Ensure slug is URL-friendly
        let slug = if slug.is_empty() {
            Self::generate_slug(title)
        } else {
            slug.to_owned()
        };

        Self {
            id,
            content_type,
            title: title.to_owned(),
            slug,
            content: content.to_owned(),
            date: date.to_owned(),
            tags: Vec::new(),
            cached_processed_content: Some(processed_content),
        }
    }

    /// Create a new blog post with tags.
    pub fn with_tags(mut self, tags: &[&str]) -> Self {
        self.tags = tags.iter().map(ToString::to_string).collect();
        self
    }

    /// Generate a URL-friendly slug from a title.
    pub fn generate_slug(title: &str) -> String {
        let mut slug = String::new();
        let mut last_was_dash = false;

        for c in title.chars() {
            if c.is_alphanumeric() {
                slug.push(c.to_ascii_lowercase());
                last_was_dash = false;
            } else if (c.is_whitespace() || c == '-' || c == '_')
                && !last_was_dash
                && !slug.is_empty()
            {
                slug.push('-');
                last_was_dash = true;
            }
            // Skip other characters
        }

        // Trim trailing dash
        if slug.ends_with('-') {
            slug.pop();
        }

        // Ensure slug is not empty
        if slug.is_empty() {
            slug = "post".to_owned();
        }

        slug
    }

    /// Get the first paragraph of the content as a description.
    /// Returns the first paragraph text if it exists and is plain text.
    /// Returns None if the first content after frontmatter is not a paragraph
    /// (e.g., heading, table, code block, formula, etc.).
    pub fn first_paragraph(&self) -> Option<String> {
        use pulldown_cmark::{Event, Parser, Tag};

        // Use preprocessed content (with math placeholders) if available
        let content = self.processed_content().unwrap_or(&self.content);

        let parser = Parser::new(content);
        let mut in_paragraph = false;
        let mut paragraph_text = String::new();
        let mut found_non_paragraph = false;

        for event in parser {
            match event {
                Event::Start(Tag::Paragraph) => {
                    if !found_non_paragraph {
                        in_paragraph = true;
                        paragraph_text.clear();
                    }
                }
                Event::End(Tag::Paragraph) => {
                    if in_paragraph && !paragraph_text.is_empty() {
                        // Found a paragraph with content
                        // Clean up math placeholders (replace (hash.typ) with [formula])
                        let cleaned = Self::clean_math_placeholders(&paragraph_text);
                        return Some(cleaned.trim().to_owned());
                    }
                    in_paragraph = false;
                }
                Event::Text(text) => {
                    if in_paragraph {
                        paragraph_text.push_str(&text);
                    } else if !found_non_paragraph {
                        // If we encounter text outside a paragraph before finding a paragraph,
                        // it means the first content is not a paragraph
                        found_non_paragraph = true;
                    }
                }
                Event::Code(_) => {
                    if in_paragraph {
                        // Code within paragraph is OK, but we might want to skip it
                        // For simplicity, we'll include it
                    } else if !found_non_paragraph {
                        // First content is code (inline or block)
                        found_non_paragraph = true;
                    }
                }
                Event::Start(tag) => {
                    if !matches!(tag, Tag::Paragraph) && !found_non_paragraph {
                        // First content is not a paragraph (heading, list, blockquote, etc.)
                        found_non_paragraph = true;
                    }
                }
                Event::SoftBreak => {
                    if in_paragraph {
                        paragraph_text.push(' ');
                    }
                }
                Event::HardBreak => {
                    if in_paragraph {
                        paragraph_text.push('\n');
                    }
                }
                _ => {
                    // Other events (HTML, footnote, etc.)
                    if !found_non_paragraph && !in_paragraph {
                        found_non_paragraph = true;
                    }
                }
            }

            // If we've already found non-paragraph content and we're not in a paragraph,
            // we can stop searching
            if found_non_paragraph && !in_paragraph && paragraph_text.is_empty() {
                break;
            }
        }

        None
    }

    /// Clean math placeholders from text, replacing (hash.typ) with [formula]
    fn clean_math_placeholders(text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let chars = text.chars().collect::<Vec<_>>();
        let mut i = 0;

        while i < chars.len() {
            if chars[i] == '(' && i + 1 < chars.len() {
                // Check if this might be a math placeholder
                let mut j = i + 1;
                while j < chars.len() && chars[j] != ')' {
                    j += 1;
                }

                if j < chars.len() && chars[j] == ')' {
                    let placeholder: String = chars[i..=j].iter().collect();
                    if placeholder.ends_with(".typ)") && placeholder.len() > 6 {
                        // Replace math placeholder with [formula]
                        result.push_str("[formula]");
                        i = j + 1;
                        continue;
                    }
                }
            }

            result.push(chars[i]);
            i += 1;
        }

        result
    }

    /// Get processed content with math formulas replaced by placeholders.
    /// Content is preprocessed when the post is created.
    pub fn processed_content(&self) -> Option<&str> {
        self.cached_processed_content.as_deref()
    }
}

/// Manages a collection of blog posts.
pub struct PostManager {
    posts: Vec<BlogPost>,
    next_id: usize,
    state: PostManagerState, // NEW
    // Cache for sorted posts to avoid sorting every frame
    sorted_posts_newest_first: Vec<BlogPost>,
    sorted_posts_oldest_first: Vec<BlogPost>,
}

impl Default for PostManager {
    fn default() -> Self {
        let mut manager = Self {
            posts: Vec::new(),
            next_id: 0,
            state: PostManagerState::Loading, // NEW
            sorted_posts_newest_first: Vec::new(),
            sorted_posts_oldest_first: Vec::new(),
        };

        // Load posts embedded at compile time
        let posts = load_embedded_posts();
        for post in posts {
            manager.add_post(post);
        }
        manager.state = if manager.posts.is_empty() {
            PostManagerState::Empty
        } else {
            PostManagerState::Loaded
        };
        manager
    }
}

impl PostManager {
    /// Add a new post to the collection.
    pub fn add_post(&mut self, post: BlogPost) {
        self.posts.push(post.clone());
        // Update sorted caches
        self.sorted_posts_newest_first.push(post.clone());
        self.sorted_posts_oldest_first.push(post);
        self.next_id += 1;

        // Sort the caches
        self.sorted_posts_newest_first
            .sort_by(|a, b| b.date.cmp(&a.date));
        self.sorted_posts_oldest_first
            .sort_by(|a, b| a.date.cmp(&b.date));
    }

    /// Get all posts.
    pub fn posts(&self) -> &[BlogPost] {
        &self.posts
    }

    /// Get a post by index.
    pub fn get(&self, index: usize) -> Option<&BlogPost> {
        self.posts.get(index)
    }

    /// Get the number of posts.
    pub fn count(&self) -> usize {
        self.posts.len()
    }

    /// Find posts containing text in title or content.
    pub fn search(
        &self,
        query: &str,
        sort_order: crate::ui::layout::PostSortOrder,
    ) -> Vec<&BlogPost> {
        if query.is_empty() {
            return self.sorted_posts(sort_order);
        }

        let query_lower = query.to_lowercase();
        // Filter from the appropriate sorted cache
        let source = match sort_order {
            crate::ui::layout::PostSortOrder::NewestFirst => &self.sorted_posts_newest_first,
            crate::ui::layout::PostSortOrder::OldestFirst => &self.sorted_posts_oldest_first,
        };

        source
            .iter()
            .filter(|post| {
                post.title.to_lowercase().contains(&query_lower)
                    || post.content.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get posts sorted by date (uses cache).
    pub fn sorted_posts(&self, sort_order: crate::ui::layout::PostSortOrder) -> Vec<&BlogPost> {
        match sort_order {
            crate::ui::layout::PostSortOrder::NewestFirst => {
                self.sorted_posts_newest_first.iter().collect()
            }
            crate::ui::layout::PostSortOrder::OldestFirst => {
                self.sorted_posts_oldest_first.iter().collect()
            }
        }
    }

    /// Get current loading state
    pub fn state(&self) -> &PostManagerState {
        &self.state
    }

    /// Find a post by its slug (URL-friendly identifier).
    pub fn find_post_by_slug(&self, slug: &str) -> Option<&BlogPost> {
        self.posts.iter().find(|post| post.slug == slug)
    }

    /// Find the index of a post by its slug.
    pub fn find_post_index_by_slug(&self, slug: &str) -> Option<usize> {
        self.posts.iter().position(|post| post.slug == slug)
    }

    /// Reload posts from disk/embedded sources.
    pub fn reload(&mut self) {
        // Clear existing posts
        self.posts.clear();
        self.sorted_posts_newest_first.clear();
        self.sorted_posts_oldest_first.clear();
        self.next_id = 0;
        self.state = PostManagerState::Loading;

        // Load posts embedded at compile time
        let posts = load_embedded_posts();
        for post in posts {
            self.add_post(post);
        }

        // Update state based on whether we loaded any posts
        self.state = if self.posts.is_empty() {
            PostManagerState::Empty
        } else {
            PostManagerState::Loaded
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_manager_has_state() {
        let manager = PostManager::default();

        // Test that state() method exists and returns a PostManagerState
        let state = manager.state();

        // Should start in Loading state (after we implement it)
        // For now just verify we can call it
        let _ = state;
    }

    #[test]
    fn test_post_manager_reload_method() {
        let mut manager = PostManager::default();

        // Test that reload method exists and can be called
        manager.reload();

        // After reload, state should be Loaded (if posts exist) or Empty
        let state = manager.state();
        match state {
            PostManagerState::Loaded => {
                // Should have some posts
                assert!(manager.count() > 0);
            }
            PostManagerState::Empty => {
                // No posts loaded
                assert_eq!(manager.count(), 0);
            }
            _ => panic!("Unexpected state after reload: {:?}", state),
        }
    }
}
