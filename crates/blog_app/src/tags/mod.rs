//! Tag system for organizing and discovering content.

use std::collections::HashMap;

use egui::Color32;

/// Get tag colors from the current Catppuccin theme
/// Uses Surface colors which are designed for subtle backgrounds
pub fn get_tag_colors(theme: &crate::ui::components::Theme) -> &'static [Color32] {
    use catppuccin::PALETTE;
    use std::sync::OnceLock;

    // Cache for each theme
    static LATTE_COLORS: OnceLock<Vec<Color32>> = OnceLock::new();
    static MACCHIATO_COLORS: OnceLock<Vec<Color32>> = OnceLock::new();

    match theme {
        crate::ui::components::Theme::CatppuccinLatte => {
            LATTE_COLORS.get_or_init(|| compute_tag_colors(&PALETTE.latte))
        }
        crate::ui::components::Theme::CatppuccinMacchiato => {
            MACCHIATO_COLORS.get_or_init(|| compute_tag_colors(&PALETTE.macchiato))
        }
    }
}

/// Compute tag colors for a specific flavor
fn compute_tag_colors(flavor: &catppuccin::Flavor) -> Vec<Color32> {
    // Convert Catppuccin colors to egui Color32
    fn to_color32(rgb: catppuccin::Rgb) -> Color32 {
        Color32::from_rgb(rgb.r, rgb.g, rgb.b)
    }

    // Use Surface and Overlay colors for tags - these are designed for backgrounds
    // and will automatically adapt to light/dark themes
    vec![
        // Surface colors (primary backgrounds)
        to_color32(flavor.colors.surface0.rgb),
        to_color32(flavor.colors.surface1.rgb),
        to_color32(flavor.colors.surface2.rgb),
        // Overlay colors (secondary backgrounds)
        to_color32(flavor.colors.overlay0.rgb),
        to_color32(flavor.colors.overlay1.rgb),
        to_color32(flavor.colors.overlay2.rgb),
        // Muted accent colors for variety (with reduced saturation)
        blend_with_surface(
            to_color32(flavor.colors.blue.rgb),
            to_color32(flavor.colors.surface1.rgb),
            0.3,
        ),
        blend_with_surface(
            to_color32(flavor.colors.green.rgb),
            to_color32(flavor.colors.surface1.rgb),
            0.3,
        ),
        blend_with_surface(
            to_color32(flavor.colors.yellow.rgb),
            to_color32(flavor.colors.surface1.rgb),
            0.3,
        ),
        blend_with_surface(
            to_color32(flavor.colors.red.rgb),
            to_color32(flavor.colors.surface1.rgb),
            0.3,
        ),
        blend_with_surface(
            to_color32(flavor.colors.mauve.rgb),
            to_color32(flavor.colors.surface1.rgb),
            0.3,
        ),
        blend_with_surface(
            to_color32(flavor.colors.peach.rgb),
            to_color32(flavor.colors.surface1.rgb),
            0.3,
        ),
    ]
}

/// Blend an accent color with a surface color to make it more subtle
fn blend_with_surface(accent: Color32, surface: Color32, accent_strength: f32) -> Color32 {
    let surface_strength = 1.0 - accent_strength;

    let r = (accent.r() as f32 * accent_strength + surface.r() as f32 * surface_strength) as u8;
    let g = (accent.g() as f32 * accent_strength + surface.g() as f32 * surface_strength) as u8;
    let b = (accent.b() as f32 * accent_strength + surface.b() as f32 * surface_strength) as u8;

    Color32::from_rgb(r, g, b)
}

/// Tag metadata
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
    /// Create a new tag with color assignment based on theme
    pub fn new(
        name: String,
        description: Option<String>,
        theme: &crate::ui::components::Theme,
    ) -> Self {
        let color = assign_tag_color(&name, theme);
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
#[derive(Default)]
pub struct TagSearchState {
    pub selected_tags: Vec<String>,
    pub tag_input: String,
    pub search_text: String,
    last_tag_input: String,
    pub suggestions: Vec<Tag>,
    pub show_suggestions: bool,
    pub in_tag_mode: bool,
    pub highlighted_index: usize,
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
    #[allow(dead_code)]
    pub fn remove_last_tag(&mut self) -> Option<String> {
        self.selected_tags.pop()
    }

    /// Clear all selected tags
    #[allow(dead_code)]
    pub fn clear_tags(&mut self) {
        self.selected_tags.clear();
    }

    /// Get the full search query including tags
    #[allow(dead_code)]
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

/// Assign a consistent color to a tag based on its hash and current theme
pub fn assign_tag_color(tag: &str, theme: &crate::ui::components::Theme) -> Color32 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash as _, Hasher as _};

    let mut hasher = DefaultHasher::new();
    tag.hash(&mut hasher);
    let hash = hasher.finish();

    let tag_colors = get_tag_colors(theme);
    let index = (hash as usize) % tag_colors.len();
    tag_colors[index]
}

/// Extract all unique tags from posts with theme-based colors
pub fn extract_all_tags(
    posts: &[crate::posts::BlogPost],
    theme: &crate::ui::components::Theme,
) -> HashMap<String, Tag> {
    let mut tag_map = HashMap::new();

    for post in posts {
        for tag_name in &post.tags {
            tag_map
                .entry(tag_name.clone())
                .and_modify(|tag: &mut Tag| tag.increment_count())
                .or_insert_with(|| Tag::new(tag_name.clone(), None, theme));
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
