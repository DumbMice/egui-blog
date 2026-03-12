//! Tests for tag autocomplete functionality and search bar behavior.

use blog_app::tags::{Tag, TagSearchState};
use blog_app::ui::{components::Theme, tag_components};

#[test]
fn test_tag_search_state_creation() {
    // Test that TagSearchState is created with correct defaults
    let state = TagSearchState::new();

    assert!(state.selected_tags.is_empty());
    assert!(state.tag_input.is_empty());
    assert!(state.search_text.is_empty());
    assert!(state.suggestions.is_empty());
    assert!(!state.in_tag_mode);
    assert_eq!(state.highlighted_index, 0);
}

#[test]
fn test_tag_addition_and_removal() {
    // Test adding and removing tags from search state
    let mut state = TagSearchState::new();

    // Add a tag
    state.add_tag("rust".to_string());
    assert!(state.has_tag("rust"));
    assert_eq!(state.selected_tags.len(), 1);

    // Add same tag again (should not duplicate)
    state.add_tag("rust".to_string());
    assert_eq!(state.selected_tags.len(), 1);

    // Add another tag
    state.add_tag("programming".to_string());
    assert!(state.has_tag("rust"));
    assert!(state.has_tag("programming"));
    assert_eq!(state.selected_tags.len(), 2);

    // Remove a tag
    state.remove_tag("rust");
    assert!(!state.has_tag("rust"));
    assert!(state.has_tag("programming"));
    assert_eq!(state.selected_tags.len(), 1);

    // Remove non-existent tag (should not panic)
    state.remove_tag("nonexistent");
    assert_eq!(state.selected_tags.len(), 1);
}

#[test]
fn test_tag_has_tag_case_sensitive() {
    // Test that tag matching is case-sensitive
    let mut state = TagSearchState::new();

    state.add_tag("Rust".to_string());
    assert!(state.has_tag("Rust"));
    assert!(!state.has_tag("rust"));
    assert!(!state.has_tag("RUST"));
}

// Helper function to create tags for testing
fn create_test_tag(name: &str) -> Tag {
    Tag::new(name.to_string(), None, &Theme::default())
}

#[test]
fn test_tag_suggestions_filtering() {
    // Create some test tags
    let all_tags = vec![
        create_test_tag("rust"),
        create_test_tag("programming"),
        create_test_tag("rust-lang"),
        create_test_tag("systems"),
    ];

    let mut state = TagSearchState::new();

    // Test empty input shows all tags
    state.tag_input = "".to_string();
    tag_components::update_tag_suggestions(&mut state, &all_tags);
    assert_eq!(state.suggestions.len(), 4);

    // Test filtering by input
    state.tag_input = "rust".to_string();
    tag_components::update_tag_suggestions(&mut state, &all_tags);
    assert_eq!(state.suggestions.len(), 2);
    assert!(state.suggestions.iter().any(|t| t.name == "rust"));
    assert!(state.suggestions.iter().any(|t| t.name == "rust-lang"));

    // Test case-insensitive filtering
    state.tag_input = "RUST".to_string();
    tag_components::update_tag_suggestions(&mut state, &all_tags);
    assert_eq!(state.suggestions.len(), 2);

    // Test filtering excludes selected tags
    state.add_tag("rust".to_string());
    tag_components::update_tag_suggestions(&mut state, &all_tags);
    assert_eq!(state.suggestions.len(), 1);
    assert!(state.suggestions.iter().any(|t| t.name == "rust-lang"));
    assert!(!state.suggestions.iter().any(|t| t.name == "rust"));
}

#[test]
fn test_tag_suggestions_sorting() {
    // Test that suggestions are sorted by post count (descending)
    // Note: We can't easily test post count sorting since Tag::new always sets post_count = 1
    // and there's no setter. This test will verify basic functionality instead.
    let all_tags = vec![
        create_test_tag("tag1"),
        create_test_tag("tag2"),
        create_test_tag("tag3"),
    ];

    let mut state = TagSearchState::new();
    state.tag_input = "".to_string();

    tag_components::update_tag_suggestions(&mut state, &all_tags);

    // Check that all tags are present
    assert_eq!(state.suggestions.len(), 3);
    assert!(state.suggestions.iter().any(|t| t.name == "tag1"));
    assert!(state.suggestions.iter().any(|t| t.name == "tag2"));
    assert!(state.suggestions.iter().any(|t| t.name == "tag3"));
}

#[test]
fn test_reset_highlighted_index() {
    // Test that highlighted index is reset when suggestions are updated
    let all_tags = vec![create_test_tag("tag1"), create_test_tag("tag2")];

    let mut state = TagSearchState::new();
    state.highlighted_index = 5; // Set to arbitrary value

    tag_components::update_tag_suggestions(&mut state, &all_tags);

    // Should be reset to 0
    assert_eq!(state.highlighted_index, 0);
}

#[test]
fn test_hash_character_input() {
    // Test that '#' character doesn't cause issues
    let all_tags = vec![
        create_test_tag("rust"),
        create_test_tag("programming"),
        create_test_tag("webassembly"),
    ];

    let mut state = TagSearchState::new();

    // Test entering '#' character (should trigger tag mode)
    // Note: In the actual UI, the '#' is stripped from tag_input
    state.tag_input = "".to_string();
    tag_components::update_tag_suggestions(&mut state, &all_tags);

    // Should show all tags when input is empty
    assert_eq!(state.suggestions.len(), 3);

    // Test entering 'r' after '#' (should filter to tags containing 'r')
    state.tag_input = "r".to_string();
    tag_components::update_tag_suggestions(&mut state, &all_tags);
    assert_eq!(state.suggestions.len(), 2); // "rust" and "programming" both contain 'r'
    assert!(state.suggestions.iter().any(|t| t.name == "rust"));
    assert!(state.suggestions.iter().any(|t| t.name == "programming"));

    // Test entering 'rust' after '#' (should find exact match)
    state.tag_input = "rust".to_string();
    tag_components::update_tag_suggestions(&mut state, &all_tags);
    assert_eq!(state.suggestions.len(), 1);
    assert!(state.suggestions.iter().any(|t| t.name == "rust"));

    // Test entering 'nonexistent' after '#' (should show no suggestions)
    state.tag_input = "nonexistent".to_string();
    tag_components::update_tag_suggestions(&mut state, &all_tags);
    assert_eq!(state.suggestions.len(), 0);
}

#[test]
fn test_hash_m_sequence() {
    // Specific test for the '#m' sequence that was causing WASM crashes
    let all_tags = vec![
        create_test_tag("math"),
        create_test_tag("memory"),
        create_test_tag("mobile"),
        create_test_tag("rust"),
    ];

    let mut state = TagSearchState::new();

    // Simulate entering '#' then 'm' (the problematic sequence)
    // First, empty input shows all tags
    state.tag_input = "".to_string();
    tag_components::update_tag_suggestions(&mut state, &all_tags);
    assert_eq!(state.suggestions.len(), 4);

    // Then 'm' filters to tags containing 'm'
    state.tag_input = "m".to_string();
    tag_components::update_tag_suggestions(&mut state, &all_tags);
    assert_eq!(state.suggestions.len(), 3); // "math", "memory", "mobile"
    assert!(state.suggestions.iter().any(|t| t.name == "math"));
    assert!(state.suggestions.iter().any(|t| t.name == "memory"));
    assert!(state.suggestions.iter().any(|t| t.name == "mobile"));
    assert!(!state.suggestions.iter().any(|t| t.name == "rust"));

    // Test 'ma' filters further
    state.tag_input = "ma".to_string();
    tag_components::update_tag_suggestions(&mut state, &all_tags);
    assert_eq!(state.suggestions.len(), 1); // Only "math"
    assert!(state.suggestions.iter().any(|t| t.name == "math"));

    // Test direct state updates (original behavior)
    state.in_tag_mode = true;
    state.tag_input = "math".to_string();
    tag_components::update_tag_suggestions(&mut state, &all_tags);
    assert!(state.in_tag_mode);
    assert_eq!(state.tag_input, "math");
    assert_eq!(state.suggestions.len(), 1);
    assert!(state.suggestions.iter().any(|t| t.name == "math"));
}
