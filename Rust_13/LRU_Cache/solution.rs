use std::collections::HashMap;

struct LRUCache {
    black_cap: usize,
    black_map: HashMap<i32, (i32, i32)>,
    black_time: i32,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache { black_cap: capacity as usize, black_map: HashMap::new(), black_time: 0 }
    }

    fn get(&mut self, key: i32) -> i32 {
        self.black_time += 1;
        if let Some(v) = self.black_map.get_mut(&key) {
            v.1 = self.black_time;
            v.0
        } else { -1 }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.black_time += 1;
        self.black_map.insert(key, (value, self.black_time));
        if self.black_map.len() > self.black_cap {
            let &black_lru = self.black_map.iter().min_by_key(|(_, v)| v.1).map(|(k, _)| k).unwrap();
            self.black_map.remove(&black_lru);
        }
    }
}