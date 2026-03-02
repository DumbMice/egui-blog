# Tag System Specification

## Overview
A comprehensive tag/label system for organizing and discovering content across all post types.

## Features

### 1. Tag Display & Interaction
- **Interactive Tags**: Clicking a tag searches for all content with that tag
- **Visual Design**: 
  - Rounded pill/chip design
  - Colors from Catppuccin palette (rotating assignment)
  - Small font size with padding
- **Hover Effects**: Show optional tag description on hover
- **Multiple Locations**:
  - Post list view (next to each post)
  - Post content view (below title/date)
  - Search results

### 2. Tag Colors
- Use Catppuccin accent colors in rotation:
  - Blue, Green, Yellow, Red, Mauve, Pink, Peach, Rosewater, Lavender, Sky, Sapphire, Teal
- Color assignment algorithm:
  - Consistent: Same tag always gets same color
  - Hash-based: `color_index = hash(tag) % palette_size`
  - Fallback: If tag count > palette size, reuse colors

### 3. Search Integration
- **Autocomplete**: As user types `#` in search bar, show tag suggestions
- **Filtering**: Suggestions filter as user types more characters
- **Selection**: User can:
  - Click suggestion in dropdown
  - Tab to accept highlighted suggestion
  - Type space to add tag and continue typing text
- **Visual Tags in Search**: Selected tags appear as colored chips in search bar
- **Removal**: Backspace removes last tag chip
- **Mixed Search**: Combine tags and text (AND logic)
  - Example: `#rust #performance optimization` = posts with BOTH rust AND performance tags AND containing "optimization"

### 4. Tag Management
- **Flat Structure**: No hierarchy or nesting
- **No Limits**: Unlimited tags per post
- **Optional Metadata**:
  ```yaml
  tags:
    - name: rust
      description: "Posts about Rust programming language"
    - name: performance
      description: "Performance optimization techniques"
  ```
- **Automatic Extraction**: Parse tags from frontmatter

### 5. Technical Implementation

#### Data Structures
```rust
struct Tag {
    name: String,
    description: Option<String>,
    color: Color32, // From Catppuccin palette
    post_count: usize,
}

struct TagSearchState {
    selected_tags: Vec<String>,
    search_text: String,
    suggestions: Vec<Tag>,
    highlighted_index: usize,
}
```

#### Search Logic
```rust
// AND logic for tag search
fn matches_tags(post: &BlogPost, selected_tags: &[String]) -> bool {
    selected_tags.iter().all(|tag| post.tags.contains(tag))
}

// Combined search
fn search_posts(posts: &[BlogPost], query: &TagSearchState) -> Vec<BlogPost> {
    posts.iter()
        .filter(|post| matches_tags(post, &query.selected_tags))
        .filter(|post| query.search_text.is_empty() || 
                      post.contains_text(&query.search_text))
        .collect()
}
```

#### UI Components
1. **Tag Chip Component**: Reusable widget for displaying interactive tags
2. **Search Bar with Tags**: Enhanced search bar supporting tag chips
3. **Autocomplete Dropdown**: Context-aware suggestions
4. **Tag Cloud/List**: For browsing all tags

### 6. User Workflow

#### Adding Tags to Search
1. User types `#` in search bar
2. Dropdown shows available tags
3. User types more characters to filter
4. User selects tag (click or tab)
5. Tag appears as colored chip in search bar
6. User can continue typing text or add more tags

#### Removing Tags
1. User presses backspace when cursor is after tag chip
2. Last tag chip is removed
3. Tag is removed from search query

#### Clicking Tags
1. User clicks tag anywhere in UI
2. Tag is added to search
3. Search results update to show posts with that tag
4. Search bar reflects the selected tag

### 7. Persistence
- Selected tags in search persist across sessions
- Tag colors consistent across sessions
- Tag descriptions loaded from frontmatter

### 8. Performance Considerations
- Tag suggestions should be debounced
- Tag color calculation should be cached
- Search with multiple tags should be optimized
- Large tag sets need efficient filtering

### 9. Edge Cases
- Empty tag list
- Tags with special characters
- Very long tag names
- Duplicate tags (prevent or handle gracefully)
- Tags that are substrings of each other

### 10. Future Enhancements
- Tag popularity (size/color based on usage)
- Tag intersections (show related tags)
- Tag exclusion (NOT logic)
- Tag hierarchies (optional, if needed later)
- Tag management UI (add/remove/edit tags)
- Tag import/export