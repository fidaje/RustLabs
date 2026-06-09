pub mod cache {
    use std::{
        collections::BTreeMap,
        sync::{Arc, Mutex},
        time::{Duration, Instant},
    };

    pub trait Cache {
        fn put(&self, key: &str, value: Arc<Vec<u8>>, expires_in: Duration);
        fn get(&self, key: &str) -> Option<Arc<Vec<u8>>>;
    }

    pub struct LRUCache {
        size: usize,
        // Chiave, (Valore, expirese_in, inserted_at, used_at)
        map: Mutex<BTreeMap<String, (Arc<Vec<u8>>, Duration, Instant, Instant)>>,
    }

    impl LRUCache {
        pub fn new(size: usize) -> Self {
            LRUCache {
                size,
                map: Mutex::new(BTreeMap::new()),
            }
        }
    }

    impl Cache for LRUCache {
        fn put(&self, key: &str, value: Arc<Vec<u8>>, expires_in: Duration) {
            let lock = self.map.lock().unwrap();
            let mut keys_to_remove = vec![];

            let btree = lock.clone();
            let mut min = (Instant::now(), String::default());

            drop(lock);

            if btree.len() >= self.size {
                print!("qui il problema");
                //btree.insert(key.to_string(), (value, expires_in, Instant::now()));
                for key in btree.keys() {
                    let value = btree.get(key).unwrap();

                    if Instant::now() > value.2 + value.1 {
                        // è scaduto
                        keys_to_remove.push(key);
                    }
                }
                if keys_to_remove.is_empty() {
                    // se keys è vuoto dobbiamo cercare la LRU
                    print!("vacante");

                    for key in btree.keys() {
                        let value = btree.get(key).unwrap();

                        if value.3 < min.0 {
                            min.0 = value.3;
                            min.1 = key.to_string();
                        }
                    }

                    keys_to_remove.push(&min.1);
                }
            }

            let mut lock = self.map.lock().unwrap();

            for k in keys_to_remove {
                lock.remove(k);
            }

            lock.insert(
                key.to_string(),
                (value, expires_in, Instant::now(), Instant::now()),
            );
            println!("inserito");

            //let btree: BTreeMap<String, String> = BTreeMap::new();
        }
        fn get(&self, key: &str) -> Option<Arc<Vec<u8>>> {
            let mut lock = self.map.lock().unwrap();

            match lock.get_mut(key) {
                Some(value) if Instant::now() < value.2 + value.1 => {
                    value.3 = Instant::now();
                    // print!("arrivo");
                    //self.put(key, Arc::clone(&value.0), value.1);

                    return Some(Arc::clone(&value.0));
                }
                _ => return None,
            }
        }
    }
}

use cache::{Cache, LRUCache};
use std::thread;
use std::{sync::Arc, time::Duration};

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
