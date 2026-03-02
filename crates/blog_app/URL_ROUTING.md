# URL Routing Specification

## Overview
URL routing system for direct access to content and navigation between different app sections.

## URL Patterns

### Public URLs (Slug-based)
- `/post/{slug}` - Specific blog post
- `/notes/{slug}` - Specific note (if public)
- `/reviews/{slug}` - Specific review

### Private URLs (ID-based)
- `/private/notes/{id}` - Private note (requires auth in future)
- `/private/drafts/{id}` - Draft content

### Section URLs
- `/` - Home/default view (blog posts)
- `/notes` - Notes section
- `/reviews` - Reviews section  
- `/search?q=query&tags=tag1,tag2` - Search results
- `/tags/{tag}` - Posts with specific tag

### Query Parameters
- `?theme=light|dark` - Force theme
- `?search=query` - Pre-populate search
- `?post=index` - Select post by index
- `?panel=collapsed` - Control side panel state

## Implementation Details

### 1. Routing Logic
```rust
enum Route {
    Home,
    Post { slug: String },
    Notes,
    Note { slug_or_id: String, is_private: bool },
    Reviews,
    Review { slug: String },
    Search { query: String, tags: Vec<String> },
    Tag { tag: String },
    NotFound,
}

impl Route {
    fn from_url(url: &str) -> Self {
        // Parse URL and return appropriate route
        // Handle both hash-based (#/) and path-based routing
    }
    
    fn to_url(&self) -> String {
        // Convert route back to URL
    }
}
```

### 2. Browser History Integration
- Use `eframe::App::update` to check `ctx.input().raw.events`
- Listen for `Event::Key` with browser navigation keys
- Handle `Event::Navigation` for URL changes
- Push state to browser history when route changes

### 3. Hash-based Routing (SPA)
For single-page app behavior:
```
https://example.com/#/post/my-post
https://example.com/#/notes
https://example.com/#/search?q=rust
```

### 4. State Synchronization
```rust
struct BlogApp {
    current_route: Route,
    selected_post_index: usize,
    search_query: String,
    selected_tags: Vec<String>,
    side_panel_collapsed: bool,
    // ... other state
}

impl BlogApp {
    fn sync_state_to_url(&self) {
        // Update browser URL based on app state
        let url = self.current_route.to_url();
        // Add query parameters for other state
        // Push to browser history
    }
    
    fn sync_url_to_state(&mut self, url: &str) {
        // Parse URL and update app state
        self.current_route = Route::from_url(url);
        // Update other state from query parameters
    }
}
```

### 5. URL Generation
For creating shareable links:
```rust
fn generate_post_url(post: &BlogPost) -> String {
    format!("/post/{}", post.slug)
}

fn generate_search_url(query: &str, tags: &[String]) -> String {
    if tags.is_empty() {
        format!("/search?q={}", query)
    } else {
        format!("/search?q={}&tags={}", query, tags.join(","))
    }
}
```

### 6. Slug Management
- Generate slugs from titles: `my-great-post-2026`
- Handle special characters and Unicode
- Ensure uniqueness
- Support custom slugs in frontmatter

### 7. Error Handling
- 404 page for invalid URLs
- Redirect for moved/deleted content
- Fallback to home page for malformed URLs

### 8. Performance Considerations
- Lazy loading of content based on route
- Prefetching for likely next routes
- Caching of parsed routes

### 9. Security Considerations
- Validate URL parameters
- Sanitize slugs to prevent injection
- Private routes require authentication (future)

### 10. Testing
- Unit tests for URL parsing/generation
- Integration tests for navigation
- Browser history tests
- Edge cases: special characters, long URLs, etc.

## Example Workflows

### Direct Post Access
1. User visits `/post/getting-started-with-rust`
2. App parses URL, loads post with slug "getting-started-with-rust"
3. Post is displayed, side panel shows post selected
4. URL in browser updates with any query parameters

### Search Sharing
1. User performs search: "rust" + #performance tag
2. App generates URL: `/search?q=rust&tags=performance`
3. User shares URL
4. Recipient visits URL, sees same search results

### Browser Navigation
1. User clicks browser back button
2. App detects URL change
3. App updates state to match previous URL
4. UI updates accordingly

## Integration with Other Features

### With Tag System
- Clicking tag generates `/tags/{tag}` URL
- Tag search URLs include tag parameters
- Tag state preserved in URLs

### With Multiple Content Types
- Different URL prefixes for different types
- Consistent routing patterns
- Type-specific slug handling

### With Responsive Layout
- URL parameters can control layout (e.g., `?panel=collapsed`)
- Layout state can be encoded in URLs for sharing

### With Theme System
- `?theme=dark` parameter forces dark theme
- Theme preference can be included in URLs