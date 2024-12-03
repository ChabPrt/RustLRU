#[cfg(test)]
mod tests {
    use rust_lru::cache::cache::Cache;

    // Test => Basic operation
    #[test]
    fn test_lru_cache_basic_operations() {
        // Size 2
        let mut cache = Cache::new(2);

        // Add items
        cache.put("key_a".to_string(), "value_a".to_string());
        cache.put("key_b".to_string(), "value_b".to_string());

        // Check items
        assert_eq!(cache.get(&"key_a".to_string()), Some(&"value_a".to_string()));
        assert_eq!(cache.get(&"key_b".to_string()), Some(&"value_b".to_string()));

        //Display cache
        //println!("Cache test_lru_cache_basic_operations : {}", cache);
    }

    // Test => Reoder
    #[test]
    fn test_lru_cache_eviction_order() {
        // Size 2
        let mut cache = Cache::new(2);

        // Add items
        cache.put("key_a".to_string(), "value_a".to_string());
        cache.put("key_b".to_string(), "value_b".to_string());

        // Get key_b => 1st place
        cache.get(&"key_b".to_string());

        //Display items
        //println!("Cache test_lru_cache_eviction_order [B - A] : {}", cache);

        // Replace key_b => key_c
        cache.put("key_c".to_string(), "value_c".to_string());

        // Verification
        assert_eq!(cache.get(&"key_a".to_string()), Some(&"value_a".to_string()));
        assert_eq!(cache.get(&"key_b".to_string()), None); // "key_b" ?
        assert_eq!(cache.get(&"key_c".to_string()), Some(&"value_c".to_string()));

        //Display cache
        //println!("Cache test_lru_cache_eviction_order [C - A]  : {}", cache);
    }

    //Test => 0 capacity
    #[test]
    fn test_lru_cache_zero_capacity() {
        //Size 0
        let mut cache = Cache::new(0);

        //Add item
        cache.put("key_a".to_string(), "value_a".to_string());

        // Verification
        assert_eq!(cache.get(&"key_a".to_string()), Some(&"value_a".to_string()));

        //Display cache
        //println!("Cache test_lru_cache_zero_capacity : {}", cache);
    }

    // Test => Overwrite item
    #[test]
    fn test_lru_cache_overwrite_value() {
        //Size 2
        let mut cache = Cache::new(2);

        // Add items
        cache.put("key_a".to_string(), "value_a".to_string());

        // New value => key_a
        cache.put("key_a".to_string(), "value_newa".to_string());

        // Verification
        assert_eq!(cache.get(&"key_a".to_string()), Some(&"value_newa".to_string()));

        //Display cache
        //println!("Cache test_lru_cache_overwrite_value : {}", cache);
    }

    //Test => Full capacity
    #[test]
    fn test_lru_cache_eviction_with_full_capacity() {
        //Size 4
        let mut cache = Cache::new(4);

        // Add items
        cache.put("key_a".to_string(), "value_a".to_string());
        cache.put("key_b".to_string(), "value_b".to_string());
        cache.put("key_c".to_string(), "value_c".to_string());
        cache.put("key_d".to_string(), "value_d".to_string());

        // Get "key_a" => more recent
        cache.get(&"key_a".to_string());

        // Add new item
        cache.put("key_e".to_string(), "value_e".to_string());

        // Verification
        assert_eq!(cache.get(&"key_a".to_string()), None);
        assert_eq!(cache.get(&"key_b".to_string()), Some(&"value_b".to_string()));
        assert_eq!(cache.get(&"key_c".to_string()), Some(&"value_c".to_string()));
        assert_eq!(cache.get(&"key_d".to_string()), Some(&"value_d".to_string()));
        assert_eq!(cache.get(&"key_e".to_string()), Some(&"value_e".to_string()));

        //Display cache
        //println!("Cache test_lru_cache_eviction_with_full_capacity : {}", cache);
    }
}
