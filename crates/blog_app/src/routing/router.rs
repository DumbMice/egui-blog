//! Router struct that encapsulates routing state and logic.

use super::Route;

/// Router that manages URL routing state and operations.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Router {
    /// Current route
    current_route: Route,
    /// Whether router has been initialized (set to true after first navigation)
    initialized: bool,
    /// Serialization version for backward compatibility
    #[cfg_attr(feature = "serde", serde(skip))]
    #[expect(dead_code)]
    version: u32,
}

impl Router {
    /// Create a new router with default state.
    pub fn new() -> Self {
        Self {
            current_route: Route::Home,
            initialized: false,
            version: 1,
        }
    }

    /// Create a router from a URL hash.
    #[expect(dead_code)]
    pub fn from_hash(hash: &str) -> Self {
        let current_route = Route::from_hash(hash);

        Self {
            current_route,
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
        self.current_route.to_hash()
    }

    /// Create a route to a post by slug.
    pub fn route_to_post(slug: &str) -> Route {
        Route::Post {
            slug: slug.to_owned(),
            fragment: None,
        }
    }

    /// Create a route to a note by slug.
    pub fn route_to_note(slug: &str) -> Route {
        Route::Note {
            slug: slug.to_owned(),
            fragment: None,
        }
    }

    /// Create a route to a review by slug.
    pub fn route_to_review(slug: &str) -> Route {
        Route::Review {
            slug: slug.to_owned(),
            fragment: None,
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

    /// Create a route to home page.
    #[expect(dead_code)]
    pub fn route_home() -> Route {
        Route::Home
    }

    /// Update from URL hash (for browser navigation).
    pub fn update_from_hash(&mut self, hash: &str) -> bool {
        let new_route = Route::from_hash(hash);
        let route_changed = self.current_route != new_route;

        if route_changed {
            self.current_route = new_route;
            self.initialized = true;
        }

        route_changed
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
                formatter.write_str("a Router struct with current_route")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Router, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut current_route = None;
                let mut version = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "current_route" => {
                            current_route = Some(map.next_value()?);
                        }
                        "version" => {
                            version = Some(map.next_value()?);
                        }
                        // Ignore unknown fields (like "query_params" from old format, "initialized" which is skipped in serialization)
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let current_route = current_route.unwrap_or(Route::Home);
                let version = version.unwrap_or(0); // Default to version 0 for old saved states

                // Handle version-specific migrations if needed
                match version {
                    0 | 1 => {
                        // Version 0: Old format without version field (may have query_params)
                        // Version 1: Current format
                        Ok(Router {
                            current_route,
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
        assert!(
            matches!(router.current_route(), Route::Post { slug, fragment: None } if slug == "my-post")
        );
    }

    #[test]
    fn test_navigation() {
        let mut router = Router::new();

        let route = Router::route_to_post("my-post");
        let url = router.navigate_to(route);
        assert_eq!(url, "#/posts/my-post");
        assert!(
            matches!(router.current_route(), Route::Post { slug, fragment: None } if slug == "my-post")
        );

        let route = Router::route_home();
        let url = router.navigate_to(route);
        assert_eq!(url, "#/");
        assert!(matches!(router.current_route(), Route::Home));
    }

    #[test]
    fn test_update_from_hash() {
        let mut router = Router::new();

        assert!(router.update_from_hash("#/posts/my-post"));
        assert!(
            matches!(router.current_route(), Route::Post { slug, fragment: None } if slug == "my-post")
        );

        // Test backward compatibility - need to reset router first
        let mut router2 = Router::new();
        assert!(router2.update_from_hash("#/post/my-post"));
        assert!(
            matches!(router2.current_route(), Route::Post { slug, fragment: None } if slug == "my-post")
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_router_serialization() {
        use serde_json;

        let mut router = Router::new();
        router.navigate_to(Router::route_to_post("my-post"));

        // Serialize
        let json = serde_json::to_string(&router).unwrap();
        assert!(json.contains("current_route"));
        assert!(json.contains("my-post"));

        // Deserialize
        let deserialized: Router = serde_json::from_str(&json).unwrap();
        assert_eq!(
            deserialized.current_route(),
            &Route::Post {
                slug: "my-post".to_string(),
                fragment: None,
            }
        );
        assert!(!deserialized.initialized); // Should be reset on deserialize
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_router_deserialization_backward_compatibility() {
        use serde_json;

        // Test deserialization of old format (without version field)
        let old_json = r#"{
            "current_route": {"Post": {"slug": "old-post"}}
        }"#;

        let deserialized: Router = serde_json::from_str(old_json).unwrap();
        assert_eq!(
            deserialized.current_route(),
            &Route::Post {
                slug: "old-post".to_string(),
                fragment: None,
            }
        );
        assert!(!deserialized.initialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_router_deserialization_missing_fields() {
        use serde_json;

        // Test deserialization with missing current_route (should default to Home)
        let json = r#"{}"#;

        let deserialized: Router = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized.current_route(), &Route::Home);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_router_deserialization_empty_object() {
        use serde_json;

        // Test deserialization of empty object (should default to Home)
        let json = r#"{}"#;

        let deserialized: Router = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized.current_route(), &Route::Home);
    }
}
