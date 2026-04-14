use std::collections::HashMap;

struct LFUCache {
    cap: usize,
    min_freq: usize,
    time: usize,
    kv: HashMap<i32, (i32, usize, usize)>,
    fk: HashMap<usize, std::collections::BTreeMap<usize, i32>>,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache { cap: capacity as usize, min_freq: 0, time: 0, kv: HashMap::new(), fk: HashMap::new() }
    }

    fn touch(&mut self, key: i32) {
        let (val, freq, ts) = *self.kv.get(&key).unwrap();
        self.fk.get_mut(&freq).unwrap().remove(&ts);
        if self.fk[&freq].is_empty() {
            self.fk.remove(&freq);
            if self.min_freq == freq { self.min_freq += 1; }
        }
        self.time += 1;
        self.kv.insert(key, (val, freq + 1, self.time));
        self.fk.entry(freq + 1).or_default().insert(self.time, key);
    }

    fn get(&mut self, key: i32) -> i32 {
        if !self.kv.contains_key(&key) { return -1; }
        self.touch(key);
        self.kv[&key].0
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cap == 0 { return; }
        if self.kv.contains_key(&key) {
            self.touch(key);
            self.kv.get_mut(&key).unwrap().0 = value;
        } else {
            if self.kv.len() == self.cap {
                let (&ts, &evict) = self.fk.get(&self.min_freq).unwrap().iter().next().unwrap();
                self.fk.get_mut(&self.min_freq).unwrap().remove(&ts);
                if self.fk[&self.min_freq].is_empty() { self.fk.remove(&self.min_freq); }
                self.kv.remove(&evict);
            }
            self.time += 1;
            self.kv.insert(key, (value, 1, self.time));
            self.fk.entry(1).or_default().insert(self.time, key);
            self.min_freq = 1;
        }
    }
}