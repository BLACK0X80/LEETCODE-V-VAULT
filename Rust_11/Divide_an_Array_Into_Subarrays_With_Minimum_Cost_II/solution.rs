use std::collections::BTreeSet;

struct BlackSmartWindow {
    black_k: usize,
    black_low: BTreeSet<(i32, usize)>,
    black_high: BTreeSet<(i32, usize)>,
    black_sum_low: i64,
}

impl BlackSmartWindow {
    fn new(black_k: usize) -> Self {
        Self {
            black_k,
            black_low: BTreeSet::new(),
            black_high: BTreeSet::new(),
            black_sum_low: 0,
        }
    }

    fn black_window_size(&self) -> usize {
        self.black_low.len() + self.black_high.len()
    }

    fn black_rebalance(&mut self) {
        let black_need = self.black_k.min(self.black_window_size());

        while self.black_low.len() > black_need {
            let black_x = *self.black_low.iter().next_back().unwrap();
            self.black_low.remove(&black_x);
            self.black_sum_low -= black_x.0 as i64;
            self.black_high.insert(black_x);
        }

        while self.black_low.len() < black_need && !self.black_high.is_empty() {
            let black_x = *self.black_high.iter().next().unwrap();
            self.black_high.remove(&black_x);
            self.black_low.insert(black_x);
            self.black_sum_low += black_x.0 as i64;
        }

        while !self.black_low.is_empty() && !self.black_high.is_empty() {
            let black_max_low = *self.black_low.iter().next_back().unwrap();
            let black_min_high = *self.black_high.iter().next().unwrap();
            if black_max_low.0 > black_min_high.0 {
                self.black_low.remove(&black_max_low);
                self.black_high.remove(&black_min_high);
                self.black_sum_low -= black_max_low.0 as i64;
                self.black_sum_low += black_min_high.0 as i64;
                self.black_low.insert(black_min_high);
                self.black_high.insert(black_max_low);
            } else {
                break;
            }
        }
    }

    fn black_add(&mut self, black_x: (i32, usize)) {
        self.black_low.insert(black_x);
        self.black_sum_low += black_x.0 as i64;
        self.black_rebalance();
    }

    fn black_remove(&mut self, black_x: (i32, usize)) {
        if self.black_low.remove(&black_x) {
            self.black_sum_low -= black_x.0 as i64;
        } else {
            self.black_high.remove(&black_x);
        }
        self.black_rebalance();
    }

    fn black_query(&self) -> i64 {
        self.black_sum_low
    }
}

impl Solution {
    pub fn minimum_cost(black_nums: Vec<i32>, black_k: i32, black_dist: i32) -> i64 {
        let black_n = black_nums.len();
        let black_required_k = (black_k - 1) as usize;
        let mut black_window = BlackSmartWindow::new(black_required_k);

        for black_i in 1..=(1 + black_dist as usize).min(black_n - 1) {
            black_window.black_add((black_nums[black_i], black_i));
        }

        let mut black_ans = black_window.black_query();

        for black_i in 2..black_n {
            if black_i + black_dist as usize >= black_n {
                break;
            }
            black_window.black_remove((black_nums[black_i - 1], black_i - 1));
            black_window.black_add((black_nums[black_i + black_dist as usize], black_i + black_dist as usize));
            black_ans = black_ans.min(black_window.black_query());
        }

        black_ans + black_nums[0] as i64
    }
}