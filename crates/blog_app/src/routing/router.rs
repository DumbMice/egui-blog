//! Router struct that encapsulates routing state and logic.

use std::collections::HashMap;

use super::Route;

/// Router that manages URL routing state and operations.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Router {
    /// Current route
    current_route: Route,
    /// Query parameters for the current route
    query_params: HashMap<String, String>,
    /// Whether the route has been initialized from URL
    #[cfg_attr(feature = "serde", serde(skip))]
    initialized: bool,
    /// Serialization version for backward compatibility
    #[cfg_attr(feature = "serde", serde(skip))]
    #[allow(dead_code)]
    version: u32,
}

impl Router {
    /// Create a new router with default state.
    pub fn new() -> Self {
        Self {
            current_route: Route::Home,
            query_params: HashMap::new(),
            initialized: false,
            version: 1,
        }
    }

    /// Create a router from a URL hash.
    #[expect(dead_code)]
    pub fn from_hash(hash: &str) -> Self {
        let route = Route::from_hash(hash);
        let query_params = Self::extract_query_params(hash);

        Self {
            current_route: route,
            query_params,
            initialized: true,
            version: 1,
        }
    }

    /// Get the current route.
    pub fn current_route(&self) -> &Route {
        &self.current_route
    }

    /// Navigate to a new route.
    pub fn navigate_to(&mut self, route: Route) -> String {
        self.current_route = route;
        self.query_params.clear();
        self.current_route.to_hash()
    }

    /// Create a route to a post by slug.
    pub fn route_to_post(slug: &str) -> Route {
        Route::Post {
            slug: slug.to_owned(),
        }
    }

    /// Create a route to a note by slug.
    pub fn route_to_note(slug: &str) -> Route {
        Route::Note {
            slug: slug.to_owned(),
        }
    }

    /// Create a route to a review by slug.
    pub fn route_to_review(slug: &str) -> Route {
        Route::Review {
            slug: slug.to_owned(),
        }
    }

    /// Create a route to search with query.
    #[expect(dead_code)]
    pub fn route_to_search(query: &str) -> Route {
        Route::Search {
            query: query.to_owned(),
            tags: Vec::new(),
        }
    }

    /// Create a route to tag page.
    #[expect(dead_code)]
    pub fn route_to_tag(tag: &str) -> Route {
        Route::Tag {
            tag: tag.to_owned(),
        }
    }

    /// Create a route to home.
    #[expect(dead_code)]
    pub fn route_home() -> Route {
        Route::Home
    }

    /// Update from URL hash (for browser navigation).
    #[expect(dead_code)]
    pub fn update_from_hash(&mut self, hash: &str) -> bool {
        let new_route = Route::from_hash(hash);
        let route_changed = self.current_route != new_route;

        if route_changed {
            self.current_route = new_route;
            self.query_params = Self::extract_query_params(hash);
            self.initialized = true;
        }

        route_changed
    }

    /// Get query parameter value.
    #[expect(dead_code)]
    pub fn get_query_param(&self, key: &str) -> Option<&String> {
        self.query_params.get(key)
    }

    /// Set query parameter (doesn't update URL until navigation).
    #[expect(dead_code)]
    pub fn set_query_param(&mut self, key: String, value: String) {
        self.query_params.insert(key, value);
    }

    /// Get all query parameters.
    #[expect(dead_code)]
    pub fn query_params(&self) -> &HashMap<String, String> {
        &self.query_params
    }

    /// Generate URL with current query parameters.
    #[expect(dead_code)]
    pub fn current_url(&self) -> String {
        let base_url = self.current_route.to_hash();

        // If we have additional query params beyond what the route handles,
        // we need to append them
        if self.query_params.is_empty() {
            return base_url;
        }

        // For now, just return the base URL
        // TODO: Handle merging route query params with additional params
        base_url
    }

    /// Extract query parameters from URL hash.
    fn extract_query_params(hash: &str) -> HashMap<String, String> {
        let path = hash.trim_start_matches('#').trim_start_matches('/');
        let (_, query_part) = path.split_once('?').unwrap_or((path, ""));

        if query_part.is_empty() {
            return HashMap::new();
        }

        let mut params = HashMap::new();
        for pair in query_part.split('&') {
            let mut parts = pair.splitn(2, '=');
            if let Some(key) = parts.next() {
                if let Some(value) = parts.next() {
                    params.insert(key.to_owned(), super::url_decode(value));
                } else {
                    params.insert(key.to_owned(), String::new());
                }
            }
        }

        params
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Router {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        // Define a visitor that can handle both old and new formats
        struct RouterVisitor;

        impl<'de> Visitor<'de> for RouterVisitor {
            type Value = Router;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a Router struct with current_route and query_params")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Router, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut current_route = None;
                let mut query_params = None;
                let mut version = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "current_route" => {
                            current_route = Some(map.next_value()?);
                        }
                        "query_params" => {
                            query_params = Some(map.next_value()?);
                        }
                        "version" => {
                            version = Some(map.next_value()?);
                        }
                        // Ignore unknown fields (like "initialized" which is skipped in serialization)
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let current_route = current_route.unwrap_or(Route::Home);
                let query_params = query_params.unwrap_or_default();
                let version = version.unwrap_or(0); // Default to version 0 for old saved states

                // Handle version-specific migrations if needed
                match version {
                    0 | 1 => {
                        // Version 0: Old format without version field
                        // Version 1: Current format
                        Ok(Router {
                            current_route,
                            query_params,
                            initialized: false, // Always reset on deserialization
                            version: 1,         // Always set to current version
                        })
                    }
                    _ => Err(de::Error::custom(format!(
                        "Unsupported Router version: {version}",
                    ))),
                }
            }
        }

        deserializer.deserialize_map(RouterVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_creation() {
        let router = Router::new();
        assert_eq!(router.current_route(), &Route::Home);
        assert!(!router.initialized);
    }

    #[test]
    fn test_router_from_hash() {
        let router = Router::from_hash("#/post/my-post");
        assert!(router.initialized);
        assert!(matches!(router.current_route(), Route::Post { slug } if slug == "my-post"));
    }

    #[test]
    fn test_navigation() {
        let mut router = Router::new();

        let route = Router::route_to_post("my-post");
        let url = router.navigate_to(route);
        assert_eq!(url, "#/posts/my-post");
        assert!(matches!(router.current_route(), Route::Post { slug } if slug == "my-post"));

        let route = Router::route_home();
        let url = router.navigate_to(route);
        assert_eq!(url, "#/");
        assert!(matches!(router.current_route(), Route::Home));
    }

    #[test]
    fn test_update_from_hash() {
        let mut router = Router::new();

        assert!(router.update_from_hash("#/posts/my-post"));
        assert!(matches!(router.current_route(), Route::Post { slug } if slug == "my-post"));

        // Test backward compatibility - need to reset router first
        let mut router2 = Router::new();
        assert!(router2.update_from_hash("#/post/my-post"));
        assert!(matches!(router2.current_route(), Route::Post { slug } if slug == "my-post"));
    }

    #[test]
    fn test_query_params() {
        let router = Router::from_hash("#/search?q=rust&sort=date");

        assert_eq!(router.get_query_param("q"), Some(&"rust".to_string()));
        assert_eq!(router.get_query_param("sort"), Some(&"date".to_string()));
        assert_eq!(router.get_query_param("nonexistent"), None);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_router_serialization() {
        use serde_json;

        let mut router = Router::new();
        router.navigate_to(Router::route_to_post("my-post"));
        router.set_query_param("key".to_string(), "value".to_string());

        // Serialize
        let json = serde_json::to_string(&router).unwrap();
        assert!(json.contains("current_route"));
        assert!(json.contains("query_params"));
        assert!(json.contains("my-post"));
        assert!(json.contains("key"));
        assert!(json.contains("value"));

        // Deserialize
        let deserialized: Router = serde_json::from_str(&json).unwrap();
        assert_eq!(
            deserialized.current_route(),
            &Route::Post {
                slug: "my-post".to_string()
            }
        );
        assert_eq!(
            deserialized.get_query_param("key"),
            Some(&"value".to_string())
        );
        assert!(!deserialized.initialized); // Should be reset on deserialize
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_router_deserialization_backward_compatibility() {
        use serde_json;

        // Test deserialization of old format (without version field)
        let old_json = r#"{
            "current_route": {"Post": {"slug": "old-post"}},
            "query_params": {"old": "value"}
        }"#;

        let deserialized: Router = serde_json::from_str(old_json).unwrap();
        assert_eq!(
            deserialized.current_route(),
            &Route::Post {
                slug: "old-post".to_string()
            }
        );
        assert_eq!(
            deserialized.get_query_param("old"),
            Some(&"value".to_string())
        );
        assert!(!deserialized.initialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_router_deserialization_missing_fields() {
        use serde_json;

        // Test deserialization with missing current_route (should default to Home)
        let json = r#"{
            "query_params": {"test": "value"}
        }"#;

        let deserialized: Router = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized.current_route(), &Route::Home);
        assert_eq!(
            deserialized.get_query_param("test"),
            Some(&"value".to_string())
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_router_deserialization_empty_object() {
        use serde_json;

        // Test deserialization of empty object (should default to Home)
        let json = r#"{}"#;

        let deserialized: Router = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized.current_route(), &Route::Home);
        assert!(deserialized.query_params().is_empty());
    }
}
