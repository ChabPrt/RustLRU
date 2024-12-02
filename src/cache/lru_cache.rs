use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::fmt;

pub struct Cache<K, V> {
    capacity: usize,
    data: HashMap<K, V>,
    keys_order: VecDeque<K>,
}

// Définition du trait CacheOperations
pub trait CacheOperations<K, V> {
    fn put(&mut self, key: K, value: V);
    fn get(&mut self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn clear(&mut self);
}

impl<K, V> Cache<K, V>
where K: Hash + Eq + Clone {

    // Constructor
    pub fn new(capacity: usize) -> Cache<K, V> {
        Cache {
            capacity,
            data: HashMap::new(),
            keys_order: VecDeque::new(),
        }
    }

    // Put
    pub fn put(&mut self, key: K, value: V) {
        if self.data.contains_key(&key) {
            self.keys_order.retain(|k| k != &key);
        } else if self.data.len() == self.capacity {
            if let Some(oldest) = self.keys_order.pop_front() {
                self.data.remove(&oldest);
            }
        }

        self.data.insert(key.clone(), value);
        self.keys_order.push_back(key);
    }

    // Get
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.data.contains_key(key) {
            self.keys_order.retain(|k| k != key);
            self.keys_order.push_front(key.clone());
            return self.data.get(key);
        }
        None
    }

    // Remove
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if self.data.contains_key(key) {
            self.keys_order.retain(|k| k != key);
            self.data.remove(key)
        } else {
            None
        }
    }

    // Clear
    pub fn clear(&mut self) {
        self.data.clear();
        self.keys_order.clear();
    }

    // ---- UTILS ----
    // Get Size
    pub fn len(&self) -> usize {
        self.data.len()
    }

    // IsEmpty ?
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// Implémentation du trait CacheOperations pour Cache
impl<K, V> CacheOperations<K, V> for Cache<K, V>
where K: Hash + Eq + Clone {
    fn put(&mut self, key: K, value: V) {
        self.put(key, value);
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }

    fn clear(&mut self) {
        self.clear();
    }
}

// Display => for each K
impl<K: fmt::Display + Hash + Eq + Clone, V> fmt::Display for Cache<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cache [Capacity: {}, Keys: [", self.capacity)?;
        for key in &self.keys_order {
            write!(f, "{}, ", key)?;
        }
        write!(f, "]]")
    }
}
