//! DNS response caching

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Cache entry with expiration
struct CacheEntry {
    value: Option<String>,
    expires_at: Instant,
}

/// DNS cache for reverse lookups
#[derive(Clone)]
pub struct DnsCache {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    ttl: Duration,
}

impl DnsCache {
    /// Create a new DNS cache with specified TTL
    pub fn new(ttl: Duration) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl,
        }
    }

    /// Get a cached value if it exists and hasn't expired
    pub async fn get(&self, key: &str) -> Option<Option<String>> {
        let cache = self.cache.read().await;

        if let Some(entry) = cache.get(key) {
            if Instant::now() < entry.expires_at {
                return Some(entry.value.clone());
            }
        }

        None
    }

    /// Insert a value into the cache
    pub async fn insert(&self, key: String, value: Option<String>) {
        let mut cache = self.cache.write().await;

        cache.insert(
            key,
            CacheEntry {
                value,
                expires_at: Instant::now() + self.ttl,
            },
        );
    }

    /// Clean up expired entries
    pub async fn cleanup(&self) {
        let mut cache = self.cache.write().await;
        let now = Instant::now();

        cache.retain(|_, entry| now < entry.expires_at);
    }

    /// Get cache size
    pub async fn size(&self) -> usize {
        self.cache.read().await.len()
    }
}
