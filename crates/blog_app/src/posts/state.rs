//! Post manager state tracking.

/// State of post loading operations.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
