use std::collections::BTreeSet;

struct SORTracker {
    black_left: BTreeSet<(i32, String)>,
    black_right: BTreeSet<(i32, String)>,
    black_queries: i32,
}

impl SORTracker {
    fn new() -> Self {
        SORTracker {
            black_left: BTreeSet::new(),
            black_right: BTreeSet::new(),
            black_queries: 0,
        }
    }

    fn add(&mut self, name: String, score: i32) {
        let black_entry = (-score, name);
        if self.black_left.is_empty() || &black_entry <= self.black_left.iter().next_back().unwrap() {
            self.black_left.insert(black_entry);
            if self.black_left.len() > self.black_queries as usize {
                let black_max = self.black_left.iter().next_back().unwrap().clone();
                self.black_left.remove(&black_max);
                self.black_right.insert(black_max);
            }
        } else {
            self.black_right.insert(black_entry);
        }
    }

    fn get(&mut self) -> String {
        self.black_queries += 1;
        if self.black_right.is_empty() {
            return self.black_left.iter().next_back().unwrap().1.clone();
        }
        let black_min = self.black_right.iter().next().unwrap().clone();
        self.black_right.remove(&black_min);
        let res = black_min.1.clone();
        self.black_left.insert(black_min);
        res
    }
}