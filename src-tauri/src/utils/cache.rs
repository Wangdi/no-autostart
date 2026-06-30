use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Cache entry with expiration
#[derive(Debug, Clone)]
pub struct CacheEntry<T> {
    pub data: T,
    pub timestamp: Instant,
    pub ttl: Duration,
}

impl<T> CacheEntry<T> {
    pub fn new(data: T, ttl: Duration) -> Self {
        Self {
            data,
            timestamp: Instant::now(),
            ttl,
        }
    }

    pub fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.timestamp) > self.ttl
    }

    pub fn age(&self) -> Duration {
        Instant::now().duration_since(self.timestamp)
    }
}

/// Simple cache with TTL support
#[derive(Debug)]
pub struct Cache<K, V> {
    entries: HashMap<K, CacheEntry<V>>,
    default_ttl: Duration,
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> Cache<K, V> {
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            entries: HashMap::new(),
            default_ttl,
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries.get(key).and_then(|entry| {
            if entry.is_expired() {
                None
            } else {
                Some(&entry.data)
            }
        })
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.entries.insert(key.clone(), CacheEntry::new(value, self.default_ttl));
    }

    pub fn insert_with_ttl(&mut self, key: K, value: V, ttl: Duration) {
        self.entries.insert(key.clone(), CacheEntry::new(value, ttl));
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.entries.remove(key).map(|entry| entry.data)
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn cleanup_expired(&mut self) {
        self.entries.retain(|_, entry| !entry.is_expired());
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> Default for Cache<K, V> {
    fn default() -> Self {
        Self::new(Duration::from_secs(60))
    }
}

/// Process cache for storing process information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub timestamp: u64,
}

/// Rate limiter for preventing too frequent operations
#[derive(Debug)]
pub struct RateLimiter {
    last_call: Option<Instant>,
    min_interval: Duration,
}

impl RateLimiter {
    pub fn new(min_interval: Duration) -> Self {
        Self {
            last_call: None,
            min_interval,
        }
    }

    pub fn try_call(&mut self) -> bool {
        let now = Instant::now();
        if let Some(last) = self.last_call {
            if now.duration_since(last) < self.min_interval {
                return false;
            }
        }
        self.last_call = Some(now);
        true
    }

    pub fn time_until_next_call(&self) -> Duration {
        if let Some(last) = self.last_call {
            let elapsed = Instant::now().duration_since(last);
            if elapsed < self.min_interval {
                return self.min_interval - elapsed;
            }
        }
        Duration::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_cache_entry_new() {
        let entry = CacheEntry::new("test", Duration::from_secs(10));
        assert!(!entry.is_expired());
    }

    #[test]
    fn test_cache_entry_expiration() {
        let entry = CacheEntry::new("test", Duration::from_millis(10));
        thread::sleep(Duration::from_millis(20));
        assert!(entry.is_expired());
    }

    #[test]
    fn test_cache_insert_and_get() {
        let mut cache: Cache<String, i32> = Cache::new(Duration::from_secs(10));
        cache.insert("key1".to_string(), 42);

        assert_eq!(cache.get(&"key1".to_string()), Some(&42));
        assert_eq!(cache.get(&"key2".to_string()), None);
    }

    #[test]
    fn test_cache_expiration() {
        let mut cache: Cache<String, i32> = Cache::new(Duration::from_millis(10));
        cache.insert("key1".to_string(), 42);

        thread::sleep(Duration::from_millis(20));

        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_remove() {
        let mut cache: Cache<String, i32> = Cache::new(Duration::from_secs(10));
        cache.insert("key1".to_string(), 42);

        assert_eq!(cache.remove(&"key1".to_string()), Some(42));
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_clear() {
        let mut cache: Cache<String, i32> = Cache::new(Duration::from_secs(10));
        cache.insert("key1".to_string(), 42);
        cache.insert("key2".to_string(), 43);

        cache.clear();

        assert!(cache.is_empty());
    }

    #[test]
    fn test_cache_cleanup_expired() {
        let mut cache: Cache<String, i32> = Cache::new(Duration::from_millis(10));
        cache.insert("key1".to_string(), 42);

        thread::sleep(Duration::from_millis(20));

        cache.insert("key2".to_string(), 43);

        cache.cleanup_expired();

        assert_eq!(cache.len(), 1);
        assert_eq!(cache.get(&"key2".to_string()), Some(&43));
    }

    #[test]
    fn test_rate_limiter_allows_first_call() {
        let mut limiter = RateLimiter::new(Duration::from_millis(100));
        assert!(limiter.try_call());
    }

    #[test]
    fn test_rate_limiter_blocks_rapid_calls() {
        let mut limiter = RateLimiter::new(Duration::from_millis(100));
        limiter.try_call();
        assert!(!limiter.try_call());
    }

    #[test]
    fn test_rate_limiter_allows_after_interval() {
        let mut limiter = RateLimiter::new(Duration::from_millis(10));
        limiter.try_call();

        thread::sleep(Duration::from_millis(20));

        assert!(limiter.try_call());
    }

    #[test]
    fn test_rate_limiter_time_until_next_call() {
        let mut limiter = RateLimiter::new(Duration::from_millis(100));
        limiter.try_call();

        let time_left = limiter.time_until_next_call();
        assert!(time_left > Duration::ZERO);
        assert!(time_left <= Duration::from_millis(100));
    }
}
