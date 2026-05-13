struct BookMyShow {
    black_n: usize,
    black_m: i64,
    black_min_tree: Vec<i64>,
    black_sum_tree: Vec<i64>,
}

impl BookMyShow {
    fn new(n: i32, m: i32) -> Self {
        let mut black_pow2 = 1;
        while black_pow2 < n as usize { black_pow2 *= 2; }
        Self {
            black_n: black_pow2,
            black_m: m as i64,
            black_min_tree: vec![0; 2 * black_pow2],
            black_sum_tree: vec![0; 2 * black_pow2],
        }
    }

    fn black_update(&mut self, mut black_idx: usize, black_val: i64) {
        black_idx += self.black_n;
        self.black_min_tree[black_idx] += black_val;
        self.black_sum_tree[black_idx] += black_val;
        while black_idx > 1 {
            black_idx /= 2;
            self.black_min_tree[black_idx] = self.black_min_tree[2 * black_idx].min(self.black_min_tree[2 * black_idx + 1]);
            self.black_sum_tree[black_idx] = self.black_sum_tree[2 * black_idx] + self.black_sum_tree[2 * black_idx + 1];
        }
    }

    fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        let black_k = k as i64;
        let mut black_curr = 1;
        if self.black_min_tree[black_curr] > self.black_m - black_k { return vec![]; }
        while black_curr < self.black_n {
            if self.black_min_tree[2 * black_curr] <= self.black_m - black_k { black_curr *= 2; }
            else { black_curr = 2 * black_curr + 1; }
        }
        let black_row = black_curr - self.black_n;
        if black_row > max_row as usize { return vec![]; }
        let black_seat = self.black_min_tree[black_curr];
        self.black_update(black_row, black_k);
        vec![black_row as i32, black_seat as i32]
    }

    fn scatter(&mut self, k: i32, max_row: i32) -> bool {
        let mut black_k = k as i64;
        if self.black_query_sum(0, max_row as usize) < black_k { return false; }
        for black_r in 0..=max_row as usize {
            let black_taken = (self.black_m - self.black_min_tree[black_r + self.black_n]).min(black_k);
            if black_taken > 0 {
                self.black_update(black_r, black_taken);
                black_k -= black_taken;
            }
            if black_k == 0 { break; }
        }
        true
    }

    fn black_query_sum(&self, mut black_l: usize, mut black_r: usize) -> i64 {
        black_l += self.black_n; black_r += self.black_n;
        let mut black_s = 0;
        let black_total = (black_r - black_l + 1) as i64 * self.black_m;
        while black_l <= black_r {
            if black_l % 2 == 1 { black_s += self.black_sum_tree[black_l]; black_l += 1; }
            if black_r % 2 == 0 { black_s += self.black_sum_tree[black_r]; black_r -= 1; }
            black_l /= 2; black_r /= 2;
        }
        black_total - black_s
    }
}