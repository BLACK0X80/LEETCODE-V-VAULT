use std::collections::{HashMap, HashSet};

struct RandomizedCollection {
    map: HashMap<i32, HashSet<usize>>,
    vals: Vec<i32>,
}

impl RandomizedCollection {
    fn new() -> Self { RandomizedCollection { map: HashMap::new(), vals: vec![] } }

    fn insert(&mut self, val: i32) -> bool {
        let idx = self.vals.len();
        self.vals.push(val);
        let e = self.map.entry(val).or_default();
        let fresh = e.is_empty();
        e.insert(idx);
        fresh
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.map.contains_key(&val) || self.map[&val].is_empty() { return false; }
        let idx = *self.map[&val].iter().next().unwrap();
        self.map.get_mut(&val).unwrap().remove(&idx);
        let last_idx = self.vals.len() - 1;
        if idx != last_idx {
            let last_val = self.vals[last_idx];
            self.vals[idx] = last_val;
            self.map.get_mut(&last_val).unwrap().remove(&last_idx);
            self.map.get_mut(&last_val).unwrap().insert(idx);
        }
        self.vals.pop();
        true
    }

    fn get_random(&self) -> i32 {
        self.vals[rand::random::<usize>() % self.vals.len()]
    }
}
