//! Blog post data structures and management.

mod loader;
mod state;  // NEW

#[allow(unused_imports)]
pub use loader::{Frontmatter, LoadError, load_embedded_posts, load_post_from_file, load_posts_from_dir, parse_post_content};
pub use state::PostManagerState;  // NEW

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
}

impl BlogPost {
    /// Create a new blog post.
    pub fn new(id: usize, title: &str, content: &str, date: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            content: content.to_string(),
            date: date.to_string(),
            tags: Vec::new(),
        }
    }

    /// Create a new blog post with tags.
    pub fn with_tags(mut self, tags: &[&str]) -> Self {
        self.tags = tags.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Get a preview of the content (first 100 chars).
    pub fn preview(&self) -> String {
        let preview = self.content.chars().take(100).collect::<String>();
        if self.content.len() > 100 {
            format!("{}...", preview)
        } else {
            preview
        }
    }
}

/// Manages a collection of blog posts.
pub struct PostManager {
    posts: Vec<BlogPost>,
    next_id: usize,
    state: PostManagerState,  // NEW
}

impl Default for PostManager {
    fn default() -> Self {
        let mut manager = Self {
            posts: Vec::new(),
            next_id: 0,
            state: PostManagerState::Loading,  // NEW
        };

        // Load posts embedded at compile time
        match load_embedded_posts() {
            Ok(posts) => {
                for post in posts {
                    manager.add_post(post);
                }
                manager.state = if manager.posts.is_empty() {
                    PostManagerState::Empty
                } else {
                    PostManagerState::Loaded
                };
            }
            Err(err) => {
                eprintln!("Failed to load embedded posts: {}", err);
                manager.state = PostManagerState::Error(err.to_string());
                // Don't add example posts in error state - user can retry
                // Error state will be shown in UI with retry option
            }
        }
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

        self.add_post(BlogPost::new(
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
        ).with_tags(&["planning", "features"]));
    }

    /// Add a new post to the collection.
    pub fn add_post(&mut self, post: BlogPost) {
        self.posts.push(post);
        self.next_id += 1;
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
    pub fn search(&self, query: &str) -> Vec<&BlogPost> {
        if query.is_empty() {
            return self.posts.iter().collect();
        }

        let query_lower = query.to_lowercase();
        self.posts
            .iter()
            .filter(|post| {
                post.title.to_lowercase().contains(&query_lower) ||
                post.content.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get current loading state
    pub fn state(&self) -> &PostManagerState {
        &self.state
    }

    /// Reload posts from disk/embedded sources.
    pub fn reload(&mut self) -> Result<(), LoadError> {
        self.state = PostManagerState::Loading;

        // Clear existing posts
        self.posts.clear();
        self.next_id = 0;

        // Attempt to load embedded posts first
        match load_embedded_posts() {
            Ok(posts) => {
                for post in posts {
                    self.add_post(post);
                }
                self.state = if self.posts.is_empty() {
                    PostManagerState::Empty
                } else {
                    PostManagerState::Loaded
                };
                Ok(())
            }
            Err(err) => {
                self.state = PostManagerState::Error(err.to_string());
                Err(err)
            }
        }
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
        let result = manager.reload();

        // Should return Ok(()) since load_embedded_posts should succeed
        assert!(result.is_ok());

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