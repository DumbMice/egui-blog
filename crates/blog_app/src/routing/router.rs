//! Router struct that encapsulates routing state and logic.

use std::collections::HashMap;

use super::Route;

/// Router that manages URL routing state and operations.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone)]
pub struct Router {
    /// Current route
    current_route: Route,
    /// Query parameters for the current route
    query_params: HashMap<String, String>,
    /// Whether the route has been initialized from URL
    #[cfg_attr(feature = "serde", serde(skip))]
    initialized: bool,
}

impl Router {
    /// Create a new router with default state.
    pub fn new() -> Self {
        Self {
            current_route: Route::Home,
            query_params: HashMap::new(),
            initialized: false,
        }
    }

    /// Create a router from a URL hash.
    pub fn from_hash(hash: &str) -> Self {
        let route = Route::from_hash(hash);
        let query_params = Self::extract_query_params(hash);

        Self {
            current_route: route,
            query_params,
            initialized: true,
        }
    }

    /// Get the current route.
    pub fn current_route(&self) -> &Route {
        &self.current_route
    }

    /// Check if the router has been initialized from URL.
    pub fn is_initialized(&self) -> bool {
        self.initialized
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
    pub fn route_to_search(query: &str) -> Route {
        Route::Search {
            query: query.to_owned(),
            tags: Vec::new(),
        }
    }

    /// Create a route to tag page.
    pub fn route_to_tag(tag: &str) -> Route {
        Route::Tag {
            tag: tag.to_owned(),
        }
    }

    /// Create a route to home.
    pub fn route_home() -> Route {
        Route::Home
    }

    /// Update from URL hash (for browser navigation).
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
    pub fn get_query_param(&self, key: &str) -> Option<&String> {
        self.query_params.get(key)
    }

    /// Set query parameter (doesn't update URL until navigation).
    pub fn set_query_param(&mut self, key: String, value: String) {
        self.query_params.insert(key, value);
    }

    /// Get all query parameters.
    pub fn query_params(&self) -> &HashMap<String, String> {
        &self.query_params
    }

    /// Generate URL with current query parameters.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_creation() {
        let router = Router::new();
        assert_eq!(router.current_route(), &Route::Home);
        assert!(!router.is_initialized());
    }

    #[test]
    fn test_router_from_hash() {
        let router = Router::from_hash("#/post/my-post");
        assert!(router.is_initialized());
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
}
