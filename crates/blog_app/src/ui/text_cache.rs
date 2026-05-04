//! Text segment caching for markdown rendering performance.
//! Caches the output of `process_text_with_math()` to avoid re-parsing
//! text segments with math placeholders every frame.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use ordered_float::OrderedFloat;
use serde_yaml::with;

use super::markdown::ParagraphContent;

/// Cache key for text segments.
/// Uses a hash of the text content and the math resolution scale.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CacheKey {
    text_hash: u64,
    resolution_scale: OrderedFloat<f32>,
}

impl CacheKey {
    fn new(text: &str, resolution_scale: f32) -> Self {
        Self {
            text_hash: hash_text(text),
            resolution_scale: OrderedFloat(resolution_scale),
        }
    }
}

/// Cache for text segments with math placeholders.
/// Stores parsed `Vec<ParagraphContent>` to avoid re-parsing every frame.
#[derive(Default)]
pub struct TextSegmentCache {
    cache: HashMap<CacheKey, Vec<ParagraphContent>>,
    max_size: usize,
    hits: u64,
    misses: u64,
    inserts: u64,
}

impl TextSegmentCache {
    /// Create a new cache with the specified maximum size.
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
            inserts: 0,
        }
    }

    /// Get cached segments for the given text and resolution scale.
    /// Returns `None` if not in cache.
    pub fn get(&mut self, text: &str, resolution_scale: f32) -> Option<&Vec<ParagraphContent>> {
        let key = CacheKey::new(text, resolution_scale);

        if let Some(val) = self.cache.get(&key) {
            self.hits += 1;
            Some(val)
        } else {
            self.misses += 1;
            None
        }
    }

    /// Insert segments into the cache.
    /// If the cache exceeds max_size, it will be cleared.
    pub fn insert(&mut self, text: &str, resolution_scale: f32, segments: Vec<ParagraphContent>) {
        // Check if we need to clear the cache due to size
        if self.cache.len() >= self.max_size {
            self.clear();
        }

        let key = CacheKey::new(text, resolution_scale);
        self.cache.insert(key, segments);
        self.inserts += 1;
    }

    /// Clear the entire cache.
    pub fn clear(&mut self) {
        self.cache.clear();
        // Reset statistics on clear
        self.hits = 0;
        self.misses = 0;
        self.inserts = 0;
    }

    /// Get cache statistics.
    pub fn stats(&self) -> CacheStats {
        let total_requests = self.hits + self.misses;
        let hit_rate = if total_requests > 0 {
            (self.hits as f64 / total_requests as f64) * 100.0
        } else {
            0.0
        };

        CacheStats {
            entries: self.cache.len(),
            max_size: self.max_size,
            hits: self.hits,
            misses: self.misses,
            inserts: self.inserts,
            hit_rate,
        }
    }

    /// Get the number of entries in the cache.
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

/// Statistics about cache performance.
#[derive(Debug, Clone, Copy)]
pub struct CacheStats {
    /// Number of entries currently in cache
    pub entries: usize,
    /// Maximum allowed entries
    pub max_size: usize,
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Number of insertions
    pub inserts: u64,
    /// Cache hit rate as percentage
    pub hit_rate: f64,
}

impl CacheStats {
    /// Format statistics as a string for display.
    pub fn format(&self) -> String {
        format!(
            "Entries: {}/{}\nHits: {}\nMisses: {}\nHit rate: {:.1}%\nInserts: {}",
            self.entries, self.max_size, self.hits, self.misses, self.hit_rate, self.inserts
        )
    }
}

/// Hash a text string for use as a cache key.
fn hash_text(text: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic() {
        let mut cache = TextSegmentCache::new(100);

        // Test empty cache
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);

        // Test insert and get
        let segments = vec![ParagraphContent::Text("test".to_string())];
        cache.insert("test text", 1.0, segments.clone());

        assert!(!cache.is_empty());
        assert_eq!(cache.len(), 1);

        let cached = cache.get("test text", 1.0);
        assert!(cached.is_some());
        // Just check it exists - can't compare equality due to ImageSource not implementing PartialEq
        assert_eq!(cached.unwrap().len(), segments.len());

        // Test cache miss with different resolution
        let cached = cache.get("test text", 2.0);
        assert!(cached.is_none());

        // Test cache miss with different text
        let cached = cache.get("different text", 1.0);
        assert!(cached.is_none());
    }

    #[test]
    fn test_cache_stats() {
        let mut cache = TextSegmentCache::new(100);

        // Make some cache operations
        let segments = vec![ParagraphContent::Text("test".to_string())];

        // Miss
        assert!(cache.get("test", 1.0).is_none());

        // Insert
        cache.insert("test", 1.0, segments.clone());

        // Hit
        assert!(cache.get("test", 1.0).is_some());

        // Miss with different scale
        assert!(cache.get("test", 2.0).is_none());

        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 2);
        assert_eq!(stats.inserts, 1);
        assert_eq!(stats.entries, 1);
        assert_eq!(stats.max_size, 100);

        // Hit rate should be 33.3% (1 hit / 3 total)
        assert!(stats.hit_rate > 33.0 && stats.hit_rate < 34.0);
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = TextSegmentCache::new(100);

        // Add some entries
        let segments = vec![ParagraphContent::Text("test".to_string())];
        cache.insert("test1", 1.0, segments.clone());
        cache.insert("test2", 1.0, segments.clone());

        assert_eq!(cache.len(), 2);

        // Clear cache
        cache.clear();

        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);

        // Stats should be reset
        let stats = cache.stats();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.inserts, 0);
    }

    #[test]
    fn test_cache_size_limit() {
        let mut cache = TextSegmentCache::new(2); // Very small limit

        let segments = vec![ParagraphContent::Text("test".to_string())];

        // Fill cache to limit
        cache.insert("test1", 1.0, segments.clone());
        cache.insert("test2", 1.0, segments.clone());

        assert_eq!(cache.len(), 2);

        // Adding one more should clear the cache
        cache.insert("test3", 1.0, segments.clone());

        // Cache should be cleared and only have the new entry
        assert_eq!(cache.len(), 1);

        // Old entries should be gone
        assert!(cache.get("test1", 1.0).is_none());
        assert!(cache.get("test2", 1.0).is_none());
        assert!(cache.get("test3", 1.0).is_some());
    }
}
