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
    /// TTL in seconds
    ttl: u64,
}

impl ResolveCache {
    pub fn new(ttl: u64) -> Self {
        Self {
            spaces: DashMap::new(),
            types: DashMap::new(),
            ttl,
        }
    }

    // Space operations
    pub fn get_space(&self, name: &str) -> Option<String> {
        self.get_if_valid(&self.spaces, name)
    }

    pub fn insert_space(&self, name: String, id: String) {
        self.spaces.insert(name, CacheEntry::new(id, self.ttl));
    }

    // Type operations
    pub fn get_type(&self, space_id: &str, name: &str) -> Option<String> {
        self.get_if_valid(&self.types, &(space_id.to_string(), name.to_string()))
    }

    pub fn insert_type(&self, space_id: String, name: String, id: String) {
        self.types
            .insert((space_id, name), CacheEntry::new(id, self.ttl));
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
