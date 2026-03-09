//! Tag system for organizing and discovering content.

use std::collections::HashMap;

use egui::Color32;

/// Catppuccin palette colors for tags
/// Using more subtle Surface colors instead of vibrant accent colors
/// Based on Catppuccin Latte (light) and Macchiato (dark) themes
pub const CATPPUCCIN_COLORS: [Color32; 12] = [
    // Surface 0 - Light: #ccd0da, Dark: #363a4f
    Color32::from_rgb(140, 143, 161), // Muted lavender-gray
    // Surface 1 - Light: #bcc0cc, Dark: #494d64
    Color32::from_rgb(120, 124, 147), // Slightly darker muted tone
    // Surface 2 - Light: #acb0be, Dark: #5b6078
    Color32::from_rgb(100, 104, 130), // Medium muted tone
    // Overlay 0 - Light: #8c8fa1, Dark: #6c7086
    Color32::from_rgb(110, 113, 138), // Gray-blue
    // Overlay 1 - Light: #828596, Dark: #7c7f93
    Color32::from_rgb(125, 128, 150), // Soft gray
    // Overlay 2 - Light: #737994, Dark: #8c8fa1
    Color32::from_rgb(130, 133, 160), // Light gray-blue
    // Subtext 0 - Light: #6c6f85, Dark: #a5adcb
    Color32::from_rgb(135, 138, 165), // Muted blue-gray
    // Subtext 1 - Light: #5c5f77, Dark: #b8c0e0
    Color32::from_rgb(140, 143, 175), // Soft blue
    // Muted accent colors (less saturated versions)
    Color32::from_rgb(100, 130, 180), // Muted blue
    Color32::from_rgb(100, 160, 120), // Muted green
    Color32::from_rgb(180, 140, 100), // Muted peach
    Color32::from_rgb(160, 120, 180), // Muted mauve
];

/// Tag metadata
#[derive(Debug, Clone)]
pub struct Tag {
    /// Tag name
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Assigned color from Catppuccin palette
    pub color: Color32,
    /// Number of posts with this tag
    pub post_count: usize,
}

impl Tag {
    /// Create a new tag with color assignment
    pub fn new(name: String, description: Option<String>) -> Self {
        let color = assign_tag_color(&name);
        Self {
            name,
            description,
            color,
            post_count: 1,
        }
    }

    /// Increment post count
    pub fn increment_count(&mut self) {
        self.post_count += 1;
    }
}

/// Tag search state
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TagSearchState {
    /// Currently selected tags
    pub selected_tags: Vec<String>,
    /// Text search query
    pub search_text: String,
    /// Whether we're currently in tag mode (typing after #)
    pub in_tag_mode: bool,
    /// Current tag input (when typing #tag)
    pub tag_input: String,
    /// Tag suggestions for autocomplete
    #[cfg_attr(feature = "serde", serde(skip))]
    pub suggestions: Vec<Tag>,
    /// Index of highlighted suggestion
    pub highlighted_index: usize,
}

impl Default for TagSearchState {
    fn default() -> Self {
        Self {
            selected_tags: Vec::new(),
            search_text: String::new(),
            in_tag_mode: false,
            tag_input: String::new(),
            suggestions: Vec::new(),
            highlighted_index: 0,
        }
    }
}

impl TagSearchState {
    /// Create a new tag search state
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if a tag is selected
    pub fn has_tag(&self, tag: &str) -> bool {
        self.selected_tags.iter().any(|t| t == tag)
    }

    /// Add a tag to selected tags
    pub fn add_tag(&mut self, tag: String) {
        if !self.has_tag(&tag) {
            self.selected_tags.push(tag);
        }
    }

    /// Remove a tag from selected tags
    pub fn remove_tag(&mut self, tag: &str) {
        self.selected_tags.retain(|t| t != tag);
    }

    /// Remove the last selected tag
    pub fn remove_last_tag(&mut self) -> Option<String> {
        self.selected_tags.pop()
    }

    /// Clear all selected tags
    pub fn clear_tags(&mut self) {
        self.selected_tags.clear();
    }

    /// Get the full search query including tags
    pub fn full_query(&self) -> String {
        let mut parts = Vec::new();

        // Add tags as #tag format
        for tag in &self.selected_tags {
            parts.push(format!("#{tag}"));
        }

        // Add text search
        if !self.search_text.is_empty() {
            parts.push(self.search_text.clone());
        }

        parts.join(" ")
    }

    /// Check if search is active (has tags or text)
    pub fn is_active(&self) -> bool {
        !self.selected_tags.is_empty() || !self.search_text.is_empty()
    }

    /// Reset search state
    pub fn reset(&mut self) {
        self.selected_tags.clear();
        self.search_text.clear();
        self.in_tag_mode = false;
        self.tag_input.clear();
        self.suggestions.clear();
        self.highlighted_index = 0;
    }
}

/// Assign a consistent color to a tag based on its hash
pub fn assign_tag_color(tag: &str) -> Color32 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    tag.hash(&mut hasher);
    let hash = hasher.finish();

    let index = (hash as usize) % CATPPUCCIN_COLORS.len();
    CATPPUCCIN_COLORS[index]
}

/// Extract all unique tags from posts
pub fn extract_all_tags(posts: &[crate::posts::BlogPost]) -> HashMap<String, Tag> {
    let mut tag_map = HashMap::new();

    for post in posts {
        for tag_name in &post.tags {
            tag_map
                .entry(tag_name.clone())
                .and_modify(|tag: &mut Tag| tag.increment_count())
                .or_insert_with(|| Tag::new(tag_name.clone(), None));
        }
    }

    tag_map
}

/// Filter posts by tags (AND logic)
pub fn filter_posts_by_tags(
    posts: &[crate::posts::BlogPost],
    selected_tags: &[String],
) -> Vec<crate::posts::BlogPost> {
    if selected_tags.is_empty() {
        return posts.to_vec();
    }

    posts
        .iter()
        .filter(|post| selected_tags.iter().all(|tag| post.tags.contains(tag)))
        .cloned()
        .collect()
}

/// Search posts with combined tag and text search (AND logic)
pub fn search_posts(
    posts: &[crate::posts::BlogPost],
    search_state: &TagSearchState,
) -> Vec<crate::posts::BlogPost> {
    let mut filtered = posts.to_vec();

    // Filter by tags (AND logic)
    if !search_state.selected_tags.is_empty() {
        filtered = filter_posts_by_tags(&filtered, &search_state.selected_tags);
    }

    // Filter by text search
    if !search_state.search_text.is_empty() {
        let query = search_state.search_text.to_lowercase();
        filtered.retain(|post| {
            post.title.to_lowercase().contains(&query)
                || post.content.to_lowercase().contains(&query)
        });
    }

    filtered
}
