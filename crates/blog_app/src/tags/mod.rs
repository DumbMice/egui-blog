//! Tag system for organizing and discovering content.

use std::collections::HashMap;

use egui::Color32;

/// Catppuccin palette colors for tags
/// Based on Catppuccin Latte (light) and Macchiato (dark) themes
pub const CATPPUCCIN_COLORS: [Color32; 12] = [
    // Blue
    Color32::from_rgb(30, 102, 245), // Catppuccin Blue
    // Green
    Color32::from_rgb(64, 160, 43), // Catppuccin Green
    // Yellow
    Color32::from_rgb(223, 142, 29), // Catppuccin Yellow
    // Red
    Color32::from_rgb(210, 15, 57), // Catppuccin Red
    // Mauve
    Color32::from_rgb(136, 57, 239), // Catppuccin Mauve
    // Pink
    Color32::from_rgb(234, 118, 203), // Catppuccin Pink
    // Peach
    Color32::from_rgb(254, 100, 11), // Catppuccin Peach
    // Rosewater
    Color32::from_rgb(220, 138, 120), // Catppuccin Rosewater
    // Lavender
    Color32::from_rgb(183, 189, 248), // Catppuccin Lavender
    // Sky
    Color32::from_rgb(4, 165, 229), // Catppuccin Sky
    // Sapphire
    Color32::from_rgb(32, 159, 181), // Catppuccin Sapphire
    // Teal
    Color32::from_rgb(23, 146, 153), // Catppuccin Teal
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
