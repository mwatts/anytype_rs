pub mod resolver;

pub use resolver::Resolver;

use dashmap::DashMap;
use std::time::Instant;

/// Cache entry with TTL
#[derive(Debug, Clone)]
pub struct CacheEntry<T> {
    pub value: T,
    pub expires_at: Instant,
}

impl<T> CacheEntry<T> {
    pub fn new(value: T, ttl_seconds: u64) -> Self {
        Self {
            value,
            expires_at: Instant::now() + std::time::Duration::from_secs(ttl_seconds),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.expires_at > Instant::now()
    }
}

/// Thread-safe, in-memory cache for name-to-ID mappings
pub struct ResolveCache {
    /// Cache for space names -> IDs
    spaces: DashMap<String, CacheEntry<String>>,
    /// Cache for (space_id, type_name) -> type_id
    types: DashMap<(String, String), CacheEntry<String>>,
    /// Cache for (space_id, object_name) -> object_id
    objects: DashMap<(String, String), CacheEntry<String>>,
    /// Cache for (space_id, list_name) -> list_id
    lists: DashMap<(String, String), CacheEntry<String>>,
    /// Cache for (type_id, property_name) -> property_id
    properties: DashMap<(String, String), CacheEntry<String>>,
    /// Cache for (property_id, tag_name) -> tag_id
    tags: DashMap<(String, String), CacheEntry<String>>,
    /// TTL in seconds
    ttl: u64,
}

impl ResolveCache {
    pub fn new(ttl: u64) -> Self {
        Self {
            spaces: DashMap::new(),
            types: DashMap::new(),
            objects: DashMap::new(),
            lists: DashMap::new(),
            properties: DashMap::new(),
            tags: DashMap::new(),
            ttl,
        }
    }

    // Space operations
    pub fn get_space(&self, name: &str) -> Option<String> {
        self.get_if_valid(&self.spaces, name)
    }

    pub fn insert_space(&self, name: String, id: String) {
        self.spaces
            .insert(name, CacheEntry::new(id, self.ttl));
    }

    pub fn invalidate_space(&self, space_id: &str) {
        // Remove space
        self.spaces.retain(|_, entry| entry.value != space_id);
        // Cascade: remove all types, objects, lists in this space
        self.types
            .retain(|k, _| k.0 != space_id);
        self.objects
            .retain(|k, _| k.0 != space_id);
        self.lists
            .retain(|k, _| k.0 != space_id);
    }

    // Type operations
    pub fn get_type(&self, space_id: &str, name: &str) -> Option<String> {
        self.get_if_valid(&self.types, &(space_id.to_string(), name.to_string()))
    }

    pub fn insert_type(&self, space_id: String, name: String, id: String) {
        self.types
            .insert((space_id, name), CacheEntry::new(id, self.ttl));
    }

    pub fn invalidate_type(&self, space_id: &str, type_id: &str) {
        // Collect property IDs to invalidate tags
        let property_ids: Vec<String> = self
            .properties
            .iter()
            .filter(|entry| entry.key().0 == type_id)
            .map(|entry| entry.value().value.clone())
            .collect();

        // Remove type
        self.types
            .retain(|k, entry| !(k.0 == space_id && entry.value == type_id));
        // Cascade: remove all properties for this type
        self.properties
            .retain(|k, _| k.0 != type_id);
        // Cascade: remove all tags for those properties
        for prop_id in property_ids {
            self.tags.retain(|k, _| k.0 != prop_id);
        }
    }

    // Object operations
    pub fn get_object(&self, space_id: &str, name: &str) -> Option<String> {
        self.get_if_valid(&self.objects, &(space_id.to_string(), name.to_string()))
    }

    pub fn insert_object(&self, space_id: String, name: String, id: String) {
        self.objects
            .insert((space_id, name), CacheEntry::new(id, self.ttl));
    }

    pub fn invalidate_object(&self, space_id: &str, name: &str) {
        self.objects
            .remove(&(space_id.to_string(), name.to_string()));
    }

    // List operations
    pub fn get_list(&self, space_id: &str, name: &str) -> Option<String> {
        self.get_if_valid(&self.lists, &(space_id.to_string(), name.to_string()))
    }

    pub fn insert_list(&self, space_id: String, name: String, id: String) {
        self.lists
            .insert((space_id, name), CacheEntry::new(id, self.ttl));
    }

    pub fn invalidate_list(&self, space_id: &str, name: &str) {
        self.lists
            .remove(&(space_id.to_string(), name.to_string()));
    }

    // Property operations
    pub fn get_property(&self, type_id: &str, name: &str) -> Option<String> {
        self.get_if_valid(&self.properties, &(type_id.to_string(), name.to_string()))
    }

    pub fn insert_property(&self, type_id: String, name: String, id: String) {
        self.properties
            .insert((type_id, name), CacheEntry::new(id, self.ttl));
    }

    pub fn invalidate_property(&self, type_id: &str, property_id: &str) {
        // Remove property
        self.properties
            .retain(|k, entry| !(k.0 == type_id && entry.value == property_id));
        // Cascade: remove all tags for this property
        self.tags
            .retain(|k, _| k.0 != property_id);
    }

    // Tag operations
    pub fn get_tag(&self, property_id: &str, name: &str) -> Option<String> {
        self.get_if_valid(&self.tags, &(property_id.to_string(), name.to_string()))
    }

    pub fn insert_tag(&self, property_id: String, name: String, id: String) {
        self.tags
            .insert((property_id, name), CacheEntry::new(id, self.ttl));
    }

    pub fn invalidate_tag(&self, property_id: &str, name: &str) {
        self.tags
            .remove(&(property_id.to_string(), name.to_string()));
    }

    // Clear all caches
    pub fn clear_all(&self) {
        self.spaces.clear();
        self.types.clear();
        self.objects.clear();
        self.lists.clear();
        self.properties.clear();
        self.tags.clear();
    }

    // Helper to get value if valid (TTL check)
    fn get_if_valid<K, Q, V>(&self, map: &DashMap<K, CacheEntry<V>>, key: &Q) -> Option<V>
    where
        K: Eq + std::hash::Hash + std::borrow::Borrow<Q>,
        Q: Eq + std::hash::Hash + ?Sized,
        V: Clone,
    {
        map.get(key).and_then(|entry| {
            if entry.is_valid() {
                Some(entry.value.clone())
            } else {
                drop(entry);
                map.remove(key);
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_cache_entry_expiration() {
        let entry = CacheEntry::new("value".to_string(), 1);
        assert!(entry.is_valid());

        thread::sleep(Duration::from_secs(2));
        assert!(!entry.is_valid());
    }

    #[test]
    fn test_space_cache() {
        let cache = ResolveCache::new(300);
        cache.insert_space("Work".to_string(), "sp_123".to_string());

        assert_eq!(cache.get_space("Work"), Some("sp_123".to_string()));
        assert_eq!(cache.get_space("NonExistent"), None);
    }

    #[test]
    fn test_cascade_invalidation() {
        let cache = ResolveCache::new(300);

        // Insert space, type, and property
        cache.insert_space("Work".to_string(), "sp_123".to_string());
        cache.insert_type("sp_123".to_string(), "Task".to_string(), "ot_456".to_string());
        cache.insert_property(
            "ot_456".to_string(),
            "Status".to_string(),
            "prop_789".to_string(),
        );
        cache.insert_tag(
            "prop_789".to_string(),
            "Done".to_string(),
            "tag_999".to_string(),
        );

        // Verify all are cached
        assert!(cache.get_space("Work").is_some());
        assert!(cache.get_type("sp_123", "Task").is_some());
        assert!(cache.get_property("ot_456", "Status").is_some());
        assert!(cache.get_tag("prop_789", "Done").is_some());

        // Invalidate space (should cascade)
        cache.invalidate_space("sp_123");

        // Space should be gone, and types should be cascaded
        assert!(cache.get_space("Work").is_none());
        assert!(cache.get_type("sp_123", "Task").is_none());

        // But property cache uses type_id as key, so it won't be affected by space invalidation
        // (This is correct behavior - property cache is keyed by type_id)
        assert!(cache.get_property("ot_456", "Status").is_some());

        // Now invalidate the type to cascade to properties
        cache.invalidate_type("sp_123", "ot_456");
        assert!(cache.get_property("ot_456", "Status").is_none());
        assert!(cache.get_tag("prop_789", "Done").is_none());
    }
}
