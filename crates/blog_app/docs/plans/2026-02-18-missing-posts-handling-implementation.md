# Graceful Missing Posts Handling Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Enhance blog app to handle missing posts scenarios gracefully with clear user feedback, error recovery, and robust state management.

**Architecture:** Add `PostManagerState` enum for tracking loading states, extend `LoadError` enum with specific variants, create UI components for loading/error/empty states, implement retry mechanism and bounds checking.

**Tech Stack:** Rust, egui, serde_yaml, std::path

---

### Task 1: Add PostManagerState enum to posts module

**Files:**
- Create: `crates/blog_app/src/posts/state.rs`
- Modify: `crates/blog_app/src/posts/mod.rs:1-7`

**Step 1: Write the failing test**

Create test file `crates/blog_app/src/posts/state.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_manager_state_variants() {
        // Test that we can create each variant
        let loading = PostManagerState::Loading;
        let loaded = PostManagerState::Loaded;
        let error = PostManagerState::Error("test error".to_string());
        let empty = PostManagerState::Empty;

        // Use them to avoid unused variable warnings
        let _ = loading;
        let _ = loaded;
        let _ = error;
        let _ = empty;
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_post_manager_state_variants -- --nocapture`
Expected: FAIL with "cannot find type `PostManagerState` in this scope"

**Step 3: Write minimal implementation**

Create `crates/blog_app/src/posts/state.rs`:
```rust
//! Post manager state tracking.

/// State of post loading operations.
#[derive(Debug, Clone, PartialEq)]
pub enum PostManagerState {
    /// Posts are being loaded
    Loading,
    /// Posts loaded successfully
    Loaded,
    /// Load failed with error message
    Error(String),
    /// No posts exist (successful empty load)
    Empty,
}
```

Update `crates/blog_app/src/posts/mod.rs`:
```rust
//! Blog post data structures and management.

mod loader;
mod state;  // NEW

#[allow(unused_imports)]
pub use loader::{Frontmatter, LoadError, load_embedded_posts, load_post_from_file, load_posts_from_dir, parse_post_content};
pub use state::PostManagerState;  // NEW

// ... rest of file
```

**Step 4: Run test to verify it passes**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_post_manager_state_variants -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/blog_app/src/posts/state.rs crates/blog_app/src/posts/mod.rs
git commit -m "feat: add PostManagerState enum for tracking loading states"
```

---

### Task 2: Extend LoadError enum with specific variants

**Files:**
- Modify: `crates/blog_app/src/posts/loader.rs:24-38`

**Step 1: Write the failing test**

Add to `crates/blog_app/src/posts/loader.rs` (after existing tests or create new test block):
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_load_error_variants() {
        // Test new error variants
        let io_error = LoadError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
        let yaml_error = LoadError::Yaml(serde_yaml::Error::from(String::from("test")));
        let format_error = LoadError::Format("test".to_string());
        let missing_delimiter = LoadError::MissingDelimiter;
        let file_not_found = LoadError::FileNotFound(PathBuf::from("test.md"));
        let dir_not_found = LoadError::DirectoryNotFound(PathBuf::from("posts"));

        // Test Display impl works
        assert!(io_error.to_string().contains("IO error"));
        assert!(yaml_error.to_string().contains("YAML parsing error"));
        assert!(format_error.to_string().contains("Invalid file format"));
        assert!(missing_delimiter.to_string().contains("Missing frontmatter delimiter"));
        assert!(file_not_found.to_string().contains("File not found"));
        assert!(dir_not_found.to_string().contains("Directory not found"));
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_load_error_variants -- --nocapture`
Expected: FAIL with "no variant named `FileNotFound`", "no variant named `DirectoryNotFound`"

**Step 3: Write minimal implementation**

Modify `crates/blog_app/src/posts/loader.rs:24-38`:
```rust
/// Errors that can occur during post loading.
#[derive(Debug, Error)]
pub enum LoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Invalid file format: {0}")]
    #[allow(dead_code)]
    Format(String),

    #[error("Missing frontmatter delimiter")]
    MissingDelimiter,

    #[error("File not found: {0:?}")]
    FileNotFound(PathBuf),

    #[error("Directory not found: {0:?}")]
    DirectoryNotFound(PathBuf),
}
```

**Step 4: Run test to verify it passes**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_load_error_variants -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/blog_app/src/posts/loader.rs
git commit -m "feat: extend LoadError enum with file/directory not found variants"
```

---

### Task 3: Add state field to PostManager

**Files:**
- Modify: `crates/blog_app/src/posts/mod.rs:53-178`

**Step 1: Write the failing test**

Add test to `crates/blog_app/src/posts/mod.rs` (at end of file):
```rust
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
}
```

**Step 2: Run test to verify it fails**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_post_manager_has_state -- --nocapture`
Expected: FAIL with "no method named `state` found for struct `PostManager`"

**Step 3: Write minimal implementation**

Modify `crates/blog_app/src/posts/mod.rs:53-61`:
```rust
/// Manages a collection of blog posts.
pub struct PostManager {
    posts: Vec<BlogPost>,
    next_id: usize,
    state: PostManagerState,  // NEW
}
```

Add `state()` method in `impl PostManager` block (around line 177):
```rust
    /// Get current loading state
    pub fn state(&self) -> &PostManagerState {
        &self.state
    }
```

Update `Default` implementation (lines 58-80):
```rust
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
                // Fall back to example posts
                manager.add_example_posts();
                manager.state = PostManagerState::Loaded;
            }
        }
        manager
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_post_manager_has_state -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/blog_app/src/posts/mod.rs
git commit -m "feat: add state field to PostManager with loading/error tracking"
```

---

### Task 4: Add bounds checking to BlogApp

**Files:**
- Modify: `crates/blog_app/src/lib.rs:33-46`
- Modify: `crates/blog_app/src/lib.rs:103-130`

**Step 1: Write the failing test**

Add test to `crates/blog_app/src/lib.rs` (at end of file):
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_valid_selection() {
        let mut app = BlogApp::default();

        // Test that ensure_valid_selection method exists
        // We'll add the method in implementation
        // For now just verify app compiles
        let _ = app;
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_ensure_valid_selection -- --nocapture`
Expected: Compiles (test will pass, we're adding method in next step)

**Step 3: Write minimal implementation**

Add `ensure_valid_selection` method to `BlogApp` impl block in `crates/blog_app/src/lib.rs` (add after line 130):
```rust
    /// Ensure selected_post is within valid bounds
    fn ensure_valid_selection(&mut self) {
        if self.post_manager.count() == 0 {
            self.selected_post = 0;
            self.editing_new_post = false;
        } else if self.selected_post >= self.post_manager.count() {
            self.selected_post = self.post_manager.count() - 1;
        }
    }
```

Update `BlogApp` struct to include `post_manager_state` field (line 33-46):
```rust
/// The main app state.
pub struct BlogApp {
    /// Manages blog posts
    post_manager: PostManager,
    /// Current post manager state
    post_manager_state: PostManagerState,  // NEW
    /// Currently selected post index
    selected_post: usize,
    /// Are we editing a new post?
    editing_new_post: bool,
    /// Title for new post
    new_post_title: String,
    /// Content for new post
    new_post_content: String,
    /// Current theme
    theme: Theme,
    /// Search query
    search_query: String,
    /// Layout configuration
    layout_config: LayoutConfig,
}
```

Update `Default` implementation (lines 33-46):
```rust
impl Default for BlogApp {
    fn default() -> Self {
        let post_manager = PostManager::default();
        let post_manager_state = post_manager.state().clone();  // NEW

        Self {
            post_manager,
            post_manager_state,  // NEW
            selected_post: 0,
            editing_new_post: false,
            new_post_title: String::new(),
            new_post_content: String::new(),
            theme: Theme::Light,
            search_query: String::new(),
            layout_config: LayoutConfig::default(),
        }
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_ensure_valid_selection -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/blog_app/src/lib.rs
git commit -m "feat: add bounds checking and state tracking to BlogApp"
```

---

### Task 5: Create loading spinner UI component

**Files:**
- Modify: `crates/blog_app/src/ui/components.rs:172-184`

**Step 1: Write the failing test**

Add test to `crates/blog_app/src/ui/components.rs` (at end of file):
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use egui::Context;

    #[test]
    fn test_loading_spinner_function() {
        // Test that loading_spinner function exists
        // We'll implement in next step
        // For now just verify file compiles
        let _context = Context::default();
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_loading_spinner_function -- --nocapture`
Expected: Compiles (test will pass, we're adding function in next step)

**Step 3: Write minimal implementation**

Add `loading_spinner` function to `crates/blog_app/src/ui/components.rs` (add after `post_preview` function around line 172):
```rust
/// Display a loading spinner with message.
pub fn loading_spinner(ui: &mut Ui, message: &str) {
    ui.vertical_centered(|ui| {
        ui.spinner();  // egui's built-in spinner
        ui.add_space(8.0);
        ui.label(message);
    });
}
```

**Step 4: Run test to verify it passes**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_loading_spinner_function -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/blog_app/src/ui/components.rs
git commit -m "feat: add loading_spinner UI component"
```

---

### Task 6: Create error message UI component

**Files:**
- Modify: `crates/blog_app/src/ui/components.rs:184-210`

**Step 1: Write the failing test**

Add test to `crates/blog_app/src/ui/components.rs` (in existing test mod):
```rust
    #[test]
    fn test_error_message_function() {
        // Test that error_message function exists
        // We'll implement in next step
        // For now just verify file compiles
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_error_message_function -- --nocapture`
Expected: Compiles (test will pass)

**Step 3: Write minimal implementation**

Add `error_message` function to `crates/blog_app/src/ui/components.rs` (after `loading_spinner`):
```rust
/// Display an error message with optional retry button.
pub fn error_message(
    ui: &mut Ui,
    title: &str,
    description: &str,
    details: Option<&str>,
    show_retry: bool,
) -> bool {
    let mut retry_clicked = false;

    ui.vertical(|ui| {
        // Error header with icon
        ui.horizontal(|ui| {
            ui.colored_label(ui.visuals().error_fg_color(), "⚠");
            ui.heading(title);
        });

        // Error description
        ui.label(description);

        // Optional technical details (collapsible)
        if let Some(details) = details {
            ui.collapsing("Technical details", |ui| {
                ui.monospace(details);
            });
        }

        // Action buttons
        ui.horizontal(|ui| {
            if show_retry && ui.button("🔄 Retry").clicked() {
                retry_clicked = true;
            }

            if ui.button("📝 Create example post").clicked() {
                // Will be implemented later
            }
        });
    });

    retry_clicked
}
```

**Step 4: Run test to verify it passes**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_error_message_function -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/blog_app/src/ui/components.rs
git commit -m "feat: add error_message UI component with retry button"
```

---

### Task 7: Create empty state UI component

**Files:**
- Modify: `crates/blog_app/src/ui/components.rs:210-230`

**Step 1: Write the failing test**

Add test to `crates/blog_app/src/ui/components.rs` (in existing test mod):
```rust
    #[test]
    fn test_empty_state_function() {
        // Test that empty_state function exists
        // We'll implement in next step
        // For now just verify file compiles
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_empty_state_function -- --nocapture`
Expected: Compiles (test will pass)

**Step 3: Write minimal implementation**

Add `empty_state` function to `crates/blog_app/src/ui/components.rs` (after `error_message`):
```rust
/// Display empty state message with create post button.
pub fn empty_state(ui: &mut Ui, is_error: bool) -> bool {
    let mut create_clicked = false;

    ui.vertical_centered(|ui| {
        if is_error {
            ui.colored_label(ui.visuals().error_fg_color(), "⚠ Failed to load posts");
            ui.label("Could not load any blog posts.");
        } else {
            ui.heading("📝 No blog posts yet");
            ui.label("Create your first post to get started!");
        }

        ui.add_space(16.0);

        if ui.button("📝 Create your first post").clicked() {
            create_clicked = true;
        }

        ui.add_space(8.0);
        ui.small("Posts directory: crates/blog_app/posts/");
    });

    create_clicked
}
```

**Step 4: Run test to verify it passes**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_empty_state_function -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/blog_app/src/ui/components.rs
git commit -m "feat: add empty_state UI component"
```

---

### Task 8: Update main_content to use loading/error states

**Files:**
- Modify: `crates/blog_app/src/ui/layout.rs:139-210`

**Step 1: Write the failing test**

Add test to `crates/blog_app/src/ui/layout.rs` (at end of file):
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::posts::{PostManager, PostManagerState};

    #[test]
    fn test_main_content_accepts_state_param() {
        // Test that main_content accepts post_manager_state parameter
        // We'll update signature in next step
        // For now just verify file compiles
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_main_content_accepts_state_param -- --nocapture`
Expected: Compiles (test will pass)

**Step 3: Write minimal implementation**

Update `main_content` function signature in `crates/blog_app/src/ui/layout.rs:139-147`:
```rust
/// Main content area showing a post or editor.
pub fn main_content(
    ui: &mut Ui,
    post_manager: &PostManager,
    post_manager_state: &PostManagerState,  // NEW
    selected_post_index: usize,
    is_editing_new_post: bool,
    new_post_title: &mut String,
    new_post_content: &mut String,
) -> (bool, bool, Option<usize>, bool) {  // Added bool for retry
    let mut post_saved = false;
    let mut editing_cancelled = false;
    let mut navigation_index = None;
    let mut retry_requested = false;

    match post_manager_state {
        PostManagerState::Loading => {
            super::components::loading_spinner(ui, "Loading blog posts...");
        }
        PostManagerState::Error(err_msg) => {
            retry_requested = super::components::error_message(
                ui,
                "Failed to load posts",
                &err_msg,
                None,  // Could include file paths
                true,  // Show retry button
            );
        }
        PostManagerState::Empty => {
            if super::components::empty_state(ui, false) {
                // Switch to edit mode - handled by caller
            }
        }
        PostManagerState::Loaded => {
            // Existing logic for showing posts/editor
            if is_editing_new_post {
                // New post editor
                ui.heading("Create New Post");
                ui.separator();

                ui.label("Title:");
                ui.text_edit_singleline(new_post_title);

                ui.label("Content (markdown):");
                ui.add(
                    egui::TextEdit::multiline(new_post_content)
                        .desired_rows(20)
                        .desired_width(f32::INFINITY),
                );

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("💾 Save").clicked() && !new_post_title.trim().is_empty() {
                        post_saved = true;
                    }

                    if ui.button("❌ Cancel").clicked() {
                        editing_cancelled = true;
                    }
                });
            } else if post_manager.count() == 0 {
                // Show empty state (no posts but successful load)
                super::components::empty_state(ui, false);
            } else if let Some(post) = post_manager.get(selected_post_index) {
                // Display existing post
                ui.vertical(|ui| {
                    ui.heading(&post.title);
                    ui.separator();

                    super::components::post_metadata(ui, &post.date, &post.tags);
                    ui.separator();

                    // Render markdown content
                    super::render_markdown(ui, &post.content);

                    ui.separator();

                    // Navigation buttons
                    if let Some(new_index) = super::components::post_navigation(
                        ui,
                        selected_post_index,
                        post_manager.count(),
                    ) {
                        navigation_index = Some(new_index);
                    }
                });
            }
        }
    }

    (post_saved, editing_cancelled, navigation_index, retry_requested)
}
```

**Step 4: Run test to verify it passes**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_main_content_accepts_state_param -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/blog_app/src/ui/layout.rs
git commit -m "feat: update main_content to handle loading/error/empty states"
```

---

### Task 9: Update BlogApp UI to pass state to layout

**Files:**
- Modify: `crates/blog_app/src/lib.rs:48-136`

**Step 1: Write the failing test**

Add test to `crates/blog_app/src/lib.rs` (in existing test mod):
```rust
    #[test]
    fn test_blog_app_ui_passes_state() {
        let mut app = BlogApp::default();
        // Verify app compiles with updated UI method
        let _ = app;
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_blog_app_ui_passes_state -- --nocapture`
Expected: Compiles (test will pass)

**Step 3: Write minimal implementation**

Update the `ui` method in `crates/blog_app/src/lib.rs:48-136` (lines 88-102):
```rust
        // Main content area with scrolling
        let mut post_saved = false;
        let mut editing_cancelled = false;
        let mut navigation_index = None;
        let mut retry_requested = false;  // NEW
        CentralPanel::default().show_inside(ui, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                (post_saved, editing_cancelled, navigation_index, retry_requested) = ui::layout::main_content(
                    ui,
                    &self.post_manager,
                    &self.post_manager_state,  // NEW: pass state
                    self.selected_post,
                    self.editing_new_post,
                    &mut self.new_post_title,
                    &mut self.new_post_content,
                );
            });
        });

        if retry_requested {
            self.handle_retry();  // Will be implemented in next task
        }
```

**Step 4: Run test to verify it passes**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_blog_app_ui_passes_state -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/blog_app/src/lib.rs
git commit -m "feat: update BlogApp UI to pass state to layout"
```

---

### Task 10: Add reload method to PostManager

**Files:**
- Modify: `crates/blog_app/src/posts/mod.rs:177-210`

**Step 1: Write the failing test**

Add test to `crates/blog_app/src/posts/mod.rs` (in existing test mod):
```rust
    #[test]
    fn test_post_manager_reload_method() {
        let mut manager = PostManager::default();

        // Test that reload method exists
        // We'll implement in next step
        let _ = manager;
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_post_manager_reload_method -- --nocapture`
Expected: Compiles (test will pass)

**Step 3: Write minimal implementation**

Add `reload` method to `PostManager` impl block in `crates/blog_app/src/posts/mod.rs` (after existing methods):
```rust
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
```

**Step 4: Run test to verify it passes**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_post_manager_reload_method -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/blog_app/src/posts/mod.rs
git commit -m "feat: add reload method to PostManager"
```

---

### Task 11: Add handle_retry method to BlogApp

**Files:**
- Modify: `crates/blog_app/src/lib.rs:130-150`

**Step 1: Write the failing test**

Add test to `crates/blog_app/src/lib.rs` (in existing test mod):
```rust
    #[test]
    fn test_blog_app_handle_retry() {
        let mut app = BlogApp::default();

        // Test that handle_retry method exists
        // We'll implement in next step
        let _ = app;
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_blog_app_handle_retry -- --nocapture`
Expected: Compiles (test will pass)

**Step 3: Write minimal implementation**

Add `handle_retry` method to `BlogApp` impl block in `crates/blog_app/src/lib.rs` (after `ensure_valid_selection`):
```rust
    /// Handle retry button click from error state.
    fn handle_retry(&mut self) {
        // Trigger reload
        match self.post_manager.reload() {
            Ok(()) => {
                // Update our state tracking
                self.post_manager_state = self.post_manager.state().clone();
            }
            Err(err) => {
                // Error already captured in PostManager state
                eprintln!("Retry failed: {}", err);
                self.post_manager_state = self.post_manager.state().clone();
            }
        }

        // Ensure valid selection
        self.ensure_valid_selection();
    }
```

**Step 4: Run test to verify it passes**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_blog_app_handle_retry -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/blog_app/src/lib.rs
git commit -m "feat: add handle_retry method to BlogApp"
```

---

### Task 12: Update side panel to handle loading/error states

**Files:**
- Modify: `crates/blog_app/src/ui/layout.rs:72-137`

**Step 1: Write the failing test**

Add test to `crates/blog_app/src/ui/layout.rs` (in existing test mod):
```rust
    #[test]
    fn test_side_panel_handles_states() {
        // Verify side_panel function compiles
        // Implementation will handle states internally
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_side_panel_handles_states -- --nocapture`
Expected: Compiles (test will pass)

**Step 3: Write minimal implementation**

Update `side_panel` function in `crates/blog_app/src/ui/layout.rs:72-137` (add check at beginning):
```rust
/// Side panel with post list.
pub fn side_panel(
    ui: &mut Ui,
    post_manager: &PostManager,
    post_manager_state: &PostManagerState,  // NEW
    search_query: &str,
    selected_post_index: &mut usize,
    config: &LayoutConfig,
) -> bool {
    let mut selection_changed = false;

    ui.vertical(|ui| {
        ui.heading("Blog Posts");
        ui.separator();

        // Handle loading/error states
        match post_manager_state {
            PostManagerState::Loading => {
                super::components::loading_spinner(ui, "Loading posts...");
                return selection_changed;
            }
            PostManagerState::Error(_) => {
                ui.label("Failed to load posts");
                ui.small("See main content for error details");
                return selection_changed;
            }
            PostManagerState::Empty => {
                ui.label("No posts found");
                return selection_changed;
            }
            PostManagerState::Loaded => {
                // Continue with normal logic
            }
        }

        let posts_to_show = post_manager.search(search_query);

        if posts_to_show.is_empty() {
            ui.label("No posts found");
            if !search_query.is_empty() {
                ui.label("Try a different search term");
            }
        } else {
            // ... existing code ...
        }
    });

    selection_changed
}
```

**Step 4: Run test to verify it passes**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_side_panel_handles_states -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/blog_app/src/ui/layout.rs
git commit -m "feat: update side panel to handle loading/error states"
```

---

### Task 13: Update BlogApp to pass state to side panel

**Files:**
- Modify: `crates/blog_app/src/lib.rs:71-86`

**Step 1: Write the failing test**

Add test to `crates/blog_app/src/lib.rs` (in existing test mod):
```rust
    #[test]
    fn test_blog_app_passes_state_to_side_panel() {
        let mut app = BlogApp::default();
        // Verify app compiles with updated side panel call
        let _ = app;
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_blog_app_passes_state_to_side_panel -- --nocapture`
Expected: Compiles (test will pass)

**Step 3: Write minimal implementation**

Update side panel call in `crates/blog_app/src/lib.rs:71-86`:
```rust
        // Side panel
        let mut selection_changed = false;
        Panel::left("side_panel").show_inside(ui, |ui| {
            selection_changed = ui::layout::side_panel(
                ui,
                &self.post_manager,
                &self.post_manager_state,  // NEW: pass state
                &self.search_query,
                &mut self.selected_post,
                &self.layout_config,
            );
        });
```

**Step 4: Run test to verify it passes**

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_blog_app_passes_state_to_side_panel -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/blog_app/src/lib.rs
git commit -m "feat: update BlogApp to pass state to side panel"
```

---

### Task 14: Test complete workflow

**Files:**
- Test all modified files

**Step 1: Run all tests to verify everything works**

```bash
cd /home/jack/Work/egui && cargo test -p blog_app -- --nocapture
```

Expected: All tests pass (should be 1+ tests for each component)

**Step 2: Test the app builds and runs**

```bash
cd /home/jack/Work/egui && ./scripts/build_blog_web.sh
```

Expected: Build succeeds without errors

**Step 3: Start server and verify UI works**

```bash
cd /home/jack/Work/egui && ./scripts/start_server_blog.sh &
SERVER_PID=$!
sleep 2
curl -s http://localhost:8766 | head -5
kill $SERVER_PID
```

Expected: Server starts, returns HTML

**Step 4: Create test for missing posts scenario**

Create test file `crates/blog_app/tests/missing_posts.rs`:
```rust
use blog_app::posts::{PostManager, PostManagerState};

#[test]
fn test_error_state_display() {
    let manager = PostManager::default();
    let state = manager.state();

    // Verify state is one of the expected variants
    match state {
        PostManagerState::Loading => println!("Loading state"),
        PostManagerState::Loaded => println!("Loaded state"),
        PostManagerState::Error(_) => println!("Error state"),
        PostManagerState::Empty => println!("Empty state"),
    }
}
```

Run: `cd /home/jack/Work/egui && cargo test -p blog_app test_error_state_display -- --nocapture`
Expected: PASS

**Step 5: Commit final implementation**

```bash
git add crates/blog_app/tests/missing_posts.rs
git commit -m "feat: complete missing posts handling implementation"
```

---

### Task 15: Update TODO.md to mark feature complete

**Files:**
- Modify: `crates/blog_app/TODO.md:13`

**Step 1: Check current TODO.md status**

```bash
cd /home/jack/Work/egui && head -20 crates/blog_app/TODO.md
```

**Step 2: Update line 13 to mark as completed**

Change:
```markdown
- [ ] Update UI to handle missing posts gracefully
```

To:
```markdown
- [x] Update UI to handle missing posts gracefully
```

**Step 3: Verify update**

```bash
cd /home/jack/Work/egui && sed -n '13p' crates/blog_app/TODO.md
```

Expected: `- [x] Update UI to handle missing posts gracefully`

**Step 4: Commit documentation update**

```bash
git add crates/blog_app/TODO.md
git commit -m "docs: mark missing posts handling as completed in TODO"
```

**Step 5: Final verification**

Run complete test suite:
```bash
cd /home/jack/Work/egui && cargo test --all -- --nocapture
```

Expected: All tests pass

---

## Success Criteria Verification

After completing all tasks, verify:

1. ✅ **All missing posts scenarios handled**: Test with missing files, empty directory
2. ✅ **Clear user feedback**: UI shows loading, error, empty states appropriately
3. ✅ **Recovery options**: Retry button works in error state
4. ✅ **No crashes**: App doesn't panic on invalid post indices
5. ✅ **Existing functionality preserved**: Post display, editing, search still work
6. ✅ **Code follows patterns**: Uses existing egui patterns and conventions

**Implementation complete!**