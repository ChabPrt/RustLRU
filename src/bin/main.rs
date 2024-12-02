use rust_lru::cache::lru_cache::Cache;

fn main() {
    // New Cache => size 3
    let mut cache = Cache::new(3);

    // Insert data
    cache.put("A", String::from("value_a"));
    cache.put("B", String::from("value_b"));
    cache.put("C", String::from("value_c"));

    // Get + Display => key == B
    let value = cache.get(&"B");
    match value {
        Some(v) => println!("B: {}", v),
        None => println!("B not found"),
    }

    // Insert element + Carbage
    cache.put("D", String::from("value_d"));
    let value = cache.get(&"A"); //Carbage
    match value {
        Some(v) => println!("A: {}", v),
        None => println!("A not found"),
    }
}