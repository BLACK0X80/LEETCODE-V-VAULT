struct FenwickTree {
    black_tree: Vec<i32>,
}

impl FenwickTree {
    fn new(black_size: usize) -> Self {
        Self {
            black_tree: vec![0; black_size + 1],
        }
    }

    fn update(&mut self, mut black_i: usize, black_delta: i32) {
        black_i += 1;
        while black_i < self.black_tree.len() {
            self.black_tree[black_i] += black_delta;
            black_i += (black_i as isize & -(black_i as isize)) as usize;
        }
    }

    fn query(&self, mut black_i: usize) -> i32 {
        black_i += 1;
        let mut black_s = 0;
        while black_i > 0 {
            black_s += self.black_tree[black_i];
            black_i -= (black_i as isize & -(black_i as isize)) as usize;
        }
        black_s
    }

    fn query_range(&self, black_l: usize, black_r: usize) -> i32 {
        if black_l > black_r {
            return 0;
        }
        self.query(black_r) - (if black_l > 0 { self.query(black_l - 1) } else { 0 })
    }
}

impl Solution {
    pub fn min_deletions(black_s: String, black_queries: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_s.len();
        let mut black_list = black_s.into_bytes();
        let mut black_bit = FenwickTree::new(black_n);

        let black_is_bad = |black_idx: usize, black_arr: &[u8]| -> i32 {
            if black_idx < black_n - 1 {
                if black_arr[black_idx] == black_arr[black_idx + 1] { return 1; }
            }
            0
        };

        for black_i in 0..black_n - 1 {
            if black_is_bad(black_i, &black_list) == 1 {
                black_bit.update(black_i, 1);
            }
        }

        let mut black_results = Vec::with_capacity(black_queries.len());
        for black_q in black_queries {
            if black_q[0] == 1 {
                let black_idx = black_q[1] as usize;

                let black_left_before = if black_idx > 0 { black_is_bad(black_idx - 1, &black_list) } else { 0 };
                let black_right_before = black_is_bad(black_idx, &black_list);

                black_list[black_idx] = if black_list[black_idx] == b'A' { b'B' } else { b'A' };

                let black_left_after = if black_idx > 0 { black_is_bad(black_idx - 1, &black_list) } else { 0 };
                let black_right_after = black_is_bad(black_idx, &black_list);

                if black_idx > 0 {
                    black_bit.update(black_idx - 1, black_left_after - black_left_before);
                }
                if black_idx < black_n - 1 {
                    black_bit.update(black_idx, black_right_after - black_right_before);
                }
            } else {
                let black_l = black_q[1] as usize;
                let black_r = black_q[2] as usize;
                if black_l >= black_r {
                    black_results.push(0);
                } else {
                    black_results.push(black_bit.query_range(black_l, black_r - 1));
                }
            }
        }
        black_results
    }
}
