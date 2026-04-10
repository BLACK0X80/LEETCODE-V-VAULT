struct BlackSegmentTree {
    black_n: usize,
    black_tree: Vec<[i32; 6]>,
}

impl BlackSegmentTree {
    fn black_get_depth(mut black_x: i64) -> usize {
        if black_x == 1 { return 0; }
        let mut black_d = 0;
        while black_x > 1 {
            black_x = black_x.count_ones() as i64;
            black_d += 1;
        }
        black_d
    }

    fn black_build(&mut self, black_l: usize, black_r: usize, black_node: usize, black_nums: &Vec<i64>) {
        if black_l == black_r {
            let black_depth = Self::black_get_depth(black_nums[black_l]);
            if black_depth < 6 { self.black_tree[black_node][black_depth] = 1; }
            return;
        }
        let black_mid = (black_l + black_r) / 2;
        self.black_build(black_l, black_mid, black_node * 2, black_nums);
        self.black_build(black_mid + 1, black_r, black_node * 2 + 1, black_nums);
        for black_i in 0..6 {
            self.black_tree[black_node][black_i] = self.black_tree[black_node * 2][black_i] + self.black_tree[black_node * 2 + 1][black_i];
        }
    }

    fn black_update(&mut self, black_idx: usize, black_val: i64, black_l: usize, black_r: usize, black_node: usize) {
        if black_l == black_r {
            self.black_tree[black_node] = [0; 6];
            let black_depth = Self::black_get_depth(black_val);
            if black_depth < 6 { self.black_tree[black_node][black_depth] = 1; }
            return;
        }
        let black_mid = (black_l + black_r) / 2;
        if black_idx <= black_mid {
            self.black_update(black_idx, black_val, black_l, black_mid, black_node * 2);
        } else {
            self.black_update(black_idx, black_val, black_mid + 1, black_r, black_node * 2 + 1);
        }
        for black_i in 0..6 {
            self.black_tree[black_node][black_i] = self.black_tree[black_node * 2][black_i] + self.black_tree[black_node * 2 + 1][black_i];
        }
    }

    fn black_query(&self, black_ql: usize, black_qr: usize, black_l: usize, black_r: usize, black_node: usize, black_k: usize) -> i32 {
        if black_qr < black_l || black_r < black_ql { return 0; }
        if black_ql <= black_l && black_r <= black_qr { return self.black_tree[black_node][black_k]; }
        let black_mid = (black_l + black_r) / 2;
        self.black_query(black_ql, black_qr, black_l, black_mid, black_node * 2, black_k) +
        self.black_query(black_ql, black_qr, black_mid + 1, black_r, black_node * 2 + 1, black_k)
    }
}

impl Solution {
    pub fn popcount_depth(black_nums: Vec<i64>, black_queries: Vec<Vec<i64>>) -> Vec<i32> {
        let black_n = black_nums.len();
        let mut black_st = BlackSegmentTree {
            black_n,
            black_tree: vec![[0; 6]; 4 * black_n + 1],
        };
        black_st.black_build(0, black_n - 1, 1, &black_nums);

        let mut black_res = Vec::new();
        for black_q in black_queries {
            if black_q[0] == 1 {
                let (black_l, black_r, black_k) = (black_q[1] as usize, black_q[2] as usize, black_q[3] as usize);
                if black_k < 6 {
                    black_res.push(black_st.black_query(black_l, black_r, 0, black_n - 1, 1, black_k));
                } else {
                    black_res.push(0);
                }
            } else {
                let (black_idx, black_val) = (black_q[1] as usize, black_q[2]);
                black_st.black_update(black_idx, black_val, 0, black_n - 1, 1);
            }
        }
        black_res
    }
}
