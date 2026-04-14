struct BlackNode {
    black_prefix: i64,
    black_suffix: i64,
    black_sum: i64,
    black_max_sum: i64,
}

impl BlackNode {
    fn black_new(val: i64) -> Self {
        Self {
            black_prefix: val,
            black_suffix: val,
            black_sum: val,
            black_max_sum: val,
        }
    }

    fn black_empty() -> Self {
        Self {
            black_prefix: 0,
            black_suffix: 0,
            black_sum: 0,
            black_max_sum: 0,
        }
    }

    fn black_change(&mut self, val: i64) {
        self.black_prefix = val;
        self.black_suffix = val;
        self.black_sum = val;
        self.black_max_sum = val;
    }
}

struct BlackSegTree {
    black_size: usize,
    black_tree: Vec<BlackNode>,
}

impl BlackSegTree {
    fn black_merge(a: &BlackNode, b: &BlackNode) -> BlackNode {
        BlackNode {
            black_sum: a.black_sum + b.black_sum,
            black_max_sum: a.black_max_sum.max(b.black_max_sum).max(a.black_suffix + b.black_prefix),
            black_prefix: a.black_prefix.max(a.black_sum + b.black_prefix),
            black_suffix: b.black_suffix.max(b.black_sum + a.black_suffix),
        }
    }

    fn black_build(&mut self, lx: usize, rx: usize, ni: usize, nums: &Vec<i32>) {
        if rx - lx == 1 {
            if lx < nums.len() {
                self.black_tree[ni] = BlackNode::black_new(nums[lx] as i64);
            }
            return;
        }
        let m = (lx + rx) / 2;
        self.black_build(lx, m, ni * 2 + 1, nums);
        self.black_build(m, rx, ni * 2 + 2, nums);
        self.black_tree[ni] = Self::black_merge(&self.black_tree[ni * 2 + 1], &self.black_tree[ni * 2 + 2]);
    }

    fn black_update(&mut self, idx: usize, val: i64, lx: usize, rx: usize, ni: usize) {
        if rx - lx == 1 {
            self.black_tree[ni].black_change(val);
            return;
        }
        let m = (lx + rx) / 2;
        if idx < m {
            self.black_update(idx, val, lx, m, ni * 2 + 1);
        } else {
            self.black_update(idx, val, m, rx, ni * 2 + 2);
        }
        self.black_tree[ni] = Self::black_merge(&self.black_tree[ni * 2 + 1], &self.black_tree[ni * 2 + 2]);
    }

    fn black_init(n: usize) -> Self {
        let mut black_size = 1;
        while black_size < n { black_size <<= 1; }
        Self {
            black_size,
            black_tree: (0..black_size * 2).map(|_| BlackNode::black_empty()).collect(),
        }
    }
}

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>) -> i64 {
        let black_n = nums.len();
        let mut black_max_num = i32::MIN;
        let mut black_indices: std::collections::HashMap<i32, Vec<usize>> = std::collections::HashMap::new();

        for i in 0..black_n {
            black_indices.entry(nums[i]).or_default().push(i);
            black_max_num = black_max_num.max(nums[i]);
        }

        if black_max_num <= 0 { return black_max_num as i64; }

        let mut black_st = BlackSegTree::black_init(black_n);
        black_st.black_build(0, black_st.black_size, 0, &nums);

        let mut black_ans = black_st.black_tree[0].black_max_sum;

        for (&black_val, black_idx_vec) in &black_indices {
            for &idx in black_idx_vec {
                black_st.black_update(idx, 0, 0, black_st.black_size, 0);
            }
            black_ans = black_ans.max(black_st.black_tree[0].black_max_sum);
            for &idx in black_idx_vec {
                black_st.black_update(idx, black_val as i64, 0, black_st.black_size, 0);
            }
        }

        black_ans
    }
}