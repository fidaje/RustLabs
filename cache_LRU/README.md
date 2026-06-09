# Traccia
## Cache LRU

Il tratto Cache descrive il contratto generale di una memoria cache:

```rust
pub trait Cache {

   fn put(&self, key: &str, value: Arc<Vec<u8>>, expires_in: Duration);

   fn get(&self, key: &str) -> Option<Arc<Vec<u8>>>;

}
```

Il metodo `put(...)` permette di inserire all'interno delle memoria un dato con possesso condiviso (`Arc<Vec<u8>>`) indicizzato da una chiave di tipo alfabetico, per un tempo non superiore alla durata indicata (parametro `expires_in`).

Il metodo `get(...)` cerca se la chiave indicata è presente e ancora valida e, nel caso, restituisce il valore corrispondente.

Si costruisca la struct LRUCache basata sulla politica LRU (least-recent) che implementi tale tratto avendo cura di garantire che l'implementazione sia *thread-safe*.

Tale struttura dati offrirà il metodo `new(size: usize) -> Self` per creare istanze in grado di accogliere fino a size coppie chiave/valore distinte. Eventuali tentativi di inserire una coppia ulteriore porterà a rimuovere dapprima le eventuali coppie scadute e, se non se ne trova nessuna, quella a cui da più tempo non si fa accesso, allo scopo di fare spazio alla nuova coppia.

La struttura dati progettata dovrà superare i seguenti test unitari:

```rust
#[test]

fn test_basic_put_get() {
    let cache = LRUCache::new(2);
    let data = Arc::new(vec![1, 2, 3]);
    cache.put("key1", Arc::clone(&data), Duration::from_secs(1));
    assert_eq!(cache.get("key1"), Some(data));
}

#[test]
fn test_expiration() {
    let cache = LRUCache::new(2);
    let data = Arc::new(vec![1, 2, 3]);
    cache.put("key1", Arc::clone(&data), Duration::from_millis(10));
    thread::sleep(Duration::from_millis(20));
    assert_eq!(cache.get("key1"), None);
}

#[test]
fn test_lru_eviction() {
    let cache = LRUCache::new(2);
    cache.put("key1", Arc::new(vec![1]), Duration::from_secs(10));
    cache.put("key2", Arc::new(vec![2]), Duration::from_secs(10));
    // Access key1 to make key2 the least recently used
    cache.get("key1");
    // This should evict key2
    cache.put("key3", Arc::new(vec![3]), Duration::from_secs(10));
    assert!(cache.get("key1").is_some());
    assert!(cache.get("key2").is_none());
    assert!(cache.get("key3").is_some());
}

#[test]
fn test_evict_expired_first() {
    let cache = LRUCache::new(2);
    cache.put("key1", Arc::new(vec![1]), Duration::from_millis(10));
    cache.put("key2", Arc::new(vec![2]), Duration::from_secs(10));
    thread::sleep(Duration::from_millis(20));
    // key1 is expired. Even if it was used last, it should be removed first.
    cache.put("key3", Arc::new(vec![3]), Duration::from_secs(10));
    assert!(cache.get("key1").is_none());
    assert!(cache.get("key2").is_some());
    assert!(cache.get("key3").is_some());
}

#[test]
fn test_thread_safety() {
    let cache = Arc::new(LRUCache::new(10));
    let mut handles = vec![];
    for i in 0..100 {
        let cache_clone = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            let key = format!("key{}", i % 20);
            cache_clone.put(&key, Arc::new(vec![i as u8]), Duration::from_secs(1));
            cache_clone.get(&key);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
```



### Possibile soluzione
```rust
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::collections::HashMap;

pub trait Cache {
    fn put(&self, key: &str, value: Arc<Vec<u8>>, expires_in: Duration);
    fn get(&self, key: &str) -> Option<Arc<Vec<u8>>>;
}

struct CacheEntry {
    value: Arc<Vec<u8>>,
    expiry: Instant,
    last_access: Instant,
}

pub struct LRUCache {
    size: usize,
    entries: Mutex<HashMap<String, CacheEntry>>,
}

impl LRUCache {

    pub fn new(size: usize) -> Self {
        LRUCache {
            size,
            entries: Mutex::new(HashMap::new()),
        }
    }
}

impl Cache for LRUCache {

    fn put(&self, key: &str, value: Arc<Vec<u8>>, expires_in: Duration) {
        let mut entries = self.entries.lock().unwrap();
        let now = Instant::now();
        let expiry = now + expires_in;
        if !entries.contains_key(key) && entries.len() >= self.size {
            // Rimuovi prima le scadenze
            entries.retain(|_, entry| entry.expiry > now);
            // Se è ancora piena, rimuovi LRU
            if entries.len() >= self.size {
                let lru_key = entries
                    .iter()
                    .min_by_key(|(_, entry)| entry.last_access)
                    .map(|(k, _)| k.*clone*());
                if let Some(k) = lru_key {
                    entries.remove(&k);
                }
            }
        }
        entries.insert(
            key.to_string(),
            CacheEntry {
                value,
                expiry,
                last_access: now,
            },
        );
    }

    fn get(&self, key: &str) -> Option<Arc<Vec<u8>>> {
        let mut entries = self.entries.lock().unwrap();
        let now = Instant::now();
        if let Some(entry) = entries.get_mut(key) {
            if entry.expiry > now {
                entry.last_access = now;
                return Some(Arc::clone(&entry.value));
            } else {
                entries.remove(key);    
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::thread;

    #[test]
    fn test_basic_put_get() {
        let cache = LRUCache::new(2);
        let data = Arc::new(vec![1, 2, 3]);
        cache.put("key1", Arc::clone(&data), Duration::from_secs(1));
        assert_eq!(cache.get("key1"), Some(data));
    }

    #[test]
    fn test_expiration() {
        let cache = LRUCache::new(2);
        let data = Arc::new(vec![1, 2, 3]);
        cache.put("key1", Arc::clone(&data), Duration::from_millis(10));
        thread::sleep(Duration::from_millis(20));
        assert_eq!(cache.get("key1"), None);
    }

    #[test]
    fn test_lru_eviction() {
        let cache = LRUCache::new(2);
        cache.put("key1", Arc::new(vec![1]), Duration::from_secs(10));
        cache.put("key2", Arc::new(vec![2]), Duration::from_secs(10));
        // Access key1 to make key2 the least recently used
        cache.get("key1");
        // This should evict key2
        cache.put("key3", Arc::new(vec![3]), Duration::from_secs(10));
        assert!(cache.get("key1").is_some());
        assert!(cache.get("key2").is_none());
        assert!(cache.get("key3").is_some());
    }

    #[test]
    fn test_evict_expired_first() {
        let cache = LRUCache::new(2);
        cache.put("key1", Arc::new(vec![1]), Duration::from_millis(10));
        cache.put("key2", Arc::new(vec![2]), Duration::from_secs(10));
        thread::sleep(Duration::from_millis(20));
        // key1 is expired. Even if it was used last, it should be removed first.
        cache.put("key3", Arc::new(vec![3]), Duration::from_secs(10));
        assert!(cache.get("key1").is_none());
        assert!(cache.get("key2").is_some());
        assert!(cache.get("key3").is_some());
    }

    #[test]
    fn test_thread_safety() {
        let cache = Arc::new(LRUCache::new(10));
        let mut handles = vec![];
        for i in 0..100 {
            let cache_clone = Arc::*clone*(&cache);
            let handle = thread::spawn(move || {
                let key = format!("key{}", i % 20);
                cache_clone.put(&key, Arc::new(vec![i as u8]), Duration::from_secs(1));
                cache_clone.get(&key);
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
```