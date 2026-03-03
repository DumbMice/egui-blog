//! URL routing for the blog application.

use std::collections::HashMap;

mod router;
pub use router::Router;

/// Represents different routes in the application.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    /// Home page - shows all posts
    Home,
    /// Specific blog post
    Post { slug: String },
    /// Search results page
    Search { query: String, tags: Vec<String> },
    /// Posts with a specific tag
    Tag { tag: String },
    /// Page not found
    NotFound,
}

impl Route {
    /// Parse a route from a URL hash and query string.
    /// Hash format: "#/post/my-post" or "#/search?q=rust"
    ///
    /// # Arguments
    /// * `hash` - The URL hash including the "#" (e.g., "#/post/my-post")
    pub fn from_hash(hash: &str) -> Self {
        // Remove leading # and optional leading /
        let path = hash.trim_start_matches('#').trim_start_matches('/');

        // Split path and query
        let (path_part, query_part) = path.split_once('?').unwrap_or((path, ""));
        let query_params = parse_query_params(query_part);

        match path_part {
            "" => Self::Home,
            path if path.starts_with("post/") => {
                let slug = path.trim_start_matches("post/").to_owned();
                if slug.is_empty() {
                    Self::NotFound
                } else {
                    Self::Post {
                        slug: url_decode(&slug),
                    }
                }
            }
            "search" => {
                let query = query_params
                    .get("q")
                    .map(|q| url_decode(q))
                    .unwrap_or_default();
                let tags = query_params
                    .get("tags")
                    .map(|t| t.split(',').map(url_decode).collect())
                    .unwrap_or_default();
                Self::Search { query, tags }
            }
            path if path.starts_with("tags/") => {
                let tag = path.trim_start_matches("tags/").to_owned();
                if tag.is_empty() {
                    Self::NotFound
                } else {
                    Self::Tag {
                        tag: url_decode(&tag),
                    }
                }
            }
            _ => Self::NotFound,
        }
    }

    /// Convert a route to URL hash.
    /// Returns hash format like "#/post/slug" or "#/search?q=query"
    pub fn to_hash(&self) -> String {
        match self {
            Self::Home => "#/".to_owned(),
            Self::Post { slug } => format!("#/post/{slug}"),
            Self::Search { query, tags } => {
                let mut query_parts = Vec::new();
                if !query.is_empty() {
                    query_parts.push(format!("q={}", url_encode(query)));
                }
                if !tags.is_empty() {
                    query_parts.push(format!("tags={}", url_encode(&tags.join(","))));
                }
                let query_str = if query_parts.is_empty() {
                    String::new()
                } else {
                    format!("?{}", query_parts.join("&"))
                };
                format!("#/search{query_str}")
            }
            Self::Tag { tag } => format!("#/tags/{}", url_encode(tag)),
            Self::NotFound => "#/404".to_owned(),
        }
    }

    /// Generate a full URL for this route (alias for `to_hash` for compatibility).
    pub fn to_url(&self) -> String {
        self.to_hash()
    }
}

/// Parse query parameters from a query string.
fn parse_query_params(query: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();

    if query.is_empty() {
        return params;
    }

    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        if let Some(key) = parts.next() {
            if let Some(value) = parts.next() {
                params.insert(key.to_owned(), url_decode(value));
            } else {
                params.insert(key.to_owned(), String::new());
            }
        }
    }

    params
}

/// Simple URL encoding (percent-encoding for query parameters).
fn url_encode(s: &str) -> String {
    let mut encoded = String::new();
    for c in s.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => {
                encoded.push(c);
            }
            ' ' => {
                encoded.push('+');
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", c as u32));
            }
        }
    }
    encoded
}

/// Simple URL decoding.
fn url_decode(s: &str) -> String {
    let mut decoded = String::new();
    let chars = s.chars().collect::<Vec<_>>();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '%' && i + 2 < chars.len() {
            if let Ok(hex) = u8::from_str_radix(&chars[i + 1..i + 3].iter().collect::<String>(), 16)
            {
                decoded.push(hex as char);
                i += 3;
                continue;
            }
        } else if chars[i] == '+' {
            decoded.push(' ');
            i += 1;
            continue;
        }

        decoded.push(chars[i]);
        i += 1;
    }

    decoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_parsing() {
        // Home route
        assert_eq!(Route::from_hash(""), Route::Home);
        assert_eq!(Route::from_hash("#"), Route::Home);
        assert_eq!(Route::from_hash("#/"), Route::Home);

        // Post route
        assert_eq!(
            Route::from_hash("#/post/my-post"),
            Route::Post {
                slug: "my-post".to_string()
            }
        );

        // Search route
        assert_eq!(
            Route::from_hash("#/search?q=rust"),
            Route::Search {
                query: "rust".to_string(),
                tags: Vec::new()
            }
        );
        assert_eq!(
            Route::from_hash("#/search?q=rust&tags=programming,web"),
            Route::Search {
                query: "rust".to_string(),
                tags: vec!["programming".to_string(), "web".to_string()]
            }
        );

        // Tag route
        assert_eq!(
            Route::from_hash("#/tags/rust"),
            Route::Tag {
                tag: "rust".to_string()
            }
        );

        // Not found
        assert_eq!(Route::from_hash("#/invalid"), Route::NotFound);
        assert_eq!(Route::from_hash("#/post/"), Route::NotFound);
        assert_eq!(Route::from_hash("#/tags/"), Route::NotFound);
    }

    #[test]
    fn test_route_to_url() {
        // Home
        assert_eq!(Route::Home.to_url(), "#/");

        // Post
        assert_eq!(
            Route::Post {
                slug: "my-post".to_string()
            }
            .to_url(),
            "#/post/my-post"
        );

        // Search
        assert_eq!(
            Route::Search {
                query: "rust".to_string(),
                tags: Vec::new()
            }
            .to_url(),
            "#/search?q=rust"
        );
        assert_eq!(
            Route::Search {
                query: "rust tutorial".to_string(),
                tags: vec!["programming".to_string(), "web".to_string()]
            }
            .to_url(),
            "#/search?q=rust+tutorial&tags=programming%2Cweb"
        );

        // Tag
        assert_eq!(
            Route::Tag {
                tag: "rust".to_string()
            }
            .to_url(),
            "#/tags/rust"
        );
    }

    #[test]
    fn test_url_encoding() {
        assert_eq!(url_encode("hello world"), "hello+world");
        assert_eq!(url_encode("hello&world"), "hello%26world");
        assert_eq!(url_decode("hello+world"), "hello world");
        assert_eq!(url_decode("hello%26world"), "hello&world");
    }
}
