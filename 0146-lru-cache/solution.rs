use std::collections::HashMap;

struct LRUCache {
    cap: usize,
    map: HashMap<i32, i32>,
    order: std::collections::VecDeque<i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache { cap: capacity as usize, map: HashMap::new(), order: std::collections::VecDeque::new() }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&val) = self.map.get(&key) {
            self.order.retain(|&k| k != key);
            self.order.push_back(key);
            val
        } else { -1 }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            self.order.retain(|&k| k != key);
        } else if self.map.len() == self.cap {
            let lru = self.order.pop_front().unwrap();
            self.map.remove(&lru);
        }
        self.map.insert(key, value);
        self.order.push_back(key);
    }
}
