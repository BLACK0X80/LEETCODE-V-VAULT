use std::collections::{HashMap, BTreeMap, HashSet};

struct AllOne {
    key_cnt: HashMap<String, i32>,
    cnt_keys: BTreeMap<i32, HashSet<String>>,
}

impl AllOne {
    fn new() -> Self { AllOne { key_cnt: HashMap::new(), cnt_keys: BTreeMap::new() } }

    fn inc(&mut self, key: String) {
        let cnt = self.key_cnt.get(&key).copied().unwrap_or(0);
        if cnt > 0 {
            self.cnt_keys.get_mut(&cnt).unwrap().remove(&key);
            if self.cnt_keys[&cnt].is_empty() { self.cnt_keys.remove(&cnt); }
        }
        self.key_cnt.insert(key.clone(), cnt + 1);
        self.cnt_keys.entry(cnt + 1).or_default().insert(key);
    }

    fn dec(&mut self, key: String) {
        let cnt = *self.key_cnt.get(&key).unwrap();
        self.cnt_keys.get_mut(&cnt).unwrap().remove(&key);
        if self.cnt_keys[&cnt].is_empty() { self.cnt_keys.remove(&cnt); }
        if cnt == 1 { self.key_cnt.remove(&key); }
        else {
            self.key_cnt.insert(key.clone(), cnt - 1);
            self.cnt_keys.entry(cnt - 1).or_default().insert(key);
        }
    }

    fn get_max_key(&self) -> String {
        self.cnt_keys.iter().next_back().and_then(|(_,s)| s.iter().next()).cloned().unwrap_or_default()
    }

    fn get_min_key(&self) -> String {
        self.cnt_keys.iter().next().and_then(|(_,s)| s.iter().next()).cloned().unwrap_or_default()
    }
}