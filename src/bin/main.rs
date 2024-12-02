use rust_lru::cache::lru_cache::{Cache};

fn main() {
    let mut cache = Cache::new(3);

    // Create & Update
    cache.put("A", "value_a");
    cache.put("B", "value_b");
    cache.put("C", "value_c");

    println!("Cache after puts: {}", cache);

    // Read
    if let Some(value) = cache.get(&"B") {
        println!("Found B: {}", value);
    }

    // Remove
    if let Some(value) = cache.remove(&"A") {
        println!("Removed A: {}", value);
    }

    // Check the cache after removal
    println!("Cache after removal: {}", cache);

    // Clear
    cache.clear();
    println!("Cache after clear: {}", cache);
}
