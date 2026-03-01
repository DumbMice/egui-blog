//! Blog post data structures and management.

mod loader;
mod state; // NEW

#[expect(unused_imports)]
pub use loader::{
    Frontmatter, LoadError, load_embedded_posts, load_post_from_file, load_posts_from_dir,
    parse_post_content,
};
pub use state::PostManagerState; // NEW

/// A blog post.
#[derive(Clone, Debug)]
pub struct BlogPost {
    /// Unique identifier
    pub id: usize,
    /// Post title
    pub title: String,
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
    /// Create a new blog post.
    pub fn new(id: usize, title: &str, content: &str, date: &str) -> Self {
        let manifest = crate::math::load_manifest();
        let processed_content =
            crate::ui::markdown::extract_and_replace_math_formulas(content, manifest);

        Self {
            id,
            title: title.to_owned(),
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

    /// Get a preview of the content (first 100 chars).
    pub fn preview(&self) -> String {
        let preview = self.content.chars().take(100).collect::<String>();
        if self.content.len() > 100 {
            format!("{preview}...")
        } else {
            preview
        }
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
    /// Add example posts for demonstration.
    pub fn add_example_posts(&mut self) {
        self.add_post(BlogPost::new(
            self.next_id,
            "Welcome to My Blog",
            "This is my first blog post using egui! I'm excited to build a blog with Rust and WebAssembly.

## Features

- **Fast**: Compiled to WebAssembly
- **Simple**: No JavaScript framework
- **Rust**: Safety and performance

Stay tuned for more updates!",
            "2026-02-10",
        ).with_tags(&["welcome", "introduction"]));

        self.add_post(BlogPost::new(
            self.next_id,
            "Learning egui",
            "Today I learned about egui's immediate mode GUI. It's quite different from retained mode frameworks but very intuitive.

### What I like:
- Easy to get started
- Great documentation
- Cross-platform (native and web)

### Code snippet:
```rust
fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading(\"My App\");
        if ui.button(\"Click me\").clicked() {
            // handle click
        }
    });
}
```",
            "2026-02-11",
        ).with_tags(&["tutorial", "egui", "learning"]));

        self.add_post(
            BlogPost::new(
                self.next_id,
                "Future Plans",
                "I plan to add more features to this blog:

1. Markdown rendering
2. Code syntax highlighting
3. Dark/light theme toggle
4. Search functionality
5. Comments section

Let me know what you think!",
                "2026-02-12",
            )
            .with_tags(&["planning", "features"]),
        );
    }

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
