use std::collections::HashMap;

struct RandomizedSet {
    map: HashMap<i32, usize>,
    vals: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self { RandomizedSet { map: HashMap::new(), vals: vec![] } }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) { return false; }
        self.map.insert(val, self.vals.len());
        self.vals.push(val);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(&idx) = self.map.get(&val) {
            let last = *self.vals.last().unwrap();
            self.vals[idx] = last;
            *self.map.get_mut(&last).unwrap() = idx;
            self.vals.pop();
            self.map.remove(&val);
            true
        } else { false }
    }

    fn get_random(&self) -> i32 {
        self.vals[rand::random::<usize>() % self.vals.len()]
    }
}