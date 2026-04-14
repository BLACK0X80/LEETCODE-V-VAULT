use std::collections::BTreeMap;

struct CountIntervals {
    black_intervals: BTreeMap<i32, i32>,
    black_total_count: i32,
}

impl CountIntervals {
    fn new() -> Self {
        Self {
            black_intervals: BTreeMap::new(),
            black_total_count: 0,
        }
    }

    fn add(&mut self, mut black_left: i32, mut black_right: i32) {
        let mut black_it = self.black_intervals.range(..=black_right);
        while let Some((&black_l, &black_r)) = black_it.next_back() {
            if black_r < black_left {
                break;
            }
            black_left = black_left.min(black_l);
            black_right = black_right.max(black_r);
            self.black_total_count -= black_r - black_l + 1;
            let black_to_remove = black_l;
            self.black_intervals.remove(&black_to_remove);
            black_it = self.black_intervals.range(..=black_right);
        }
        self.black_total_count += black_right - black_left + 1;
        self.black_intervals.insert(black_left, black_right);
    }

    fn count(&self) -> i32 {
        self.black_total_count
    }
}