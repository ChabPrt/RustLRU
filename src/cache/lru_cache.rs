use std::collections::{HashMap, VecDeque};

pub struct Cache<K, V> {
    capacity: usize,
    data: HashMap<K, V>,
    keys_order: VecDeque<K>,
}

impl<K, V> Cache<K, V>
where K: std::hash::Hash + Eq + Clone {

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

}
