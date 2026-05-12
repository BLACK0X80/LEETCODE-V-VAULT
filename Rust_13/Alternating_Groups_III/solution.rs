use std::collections::BTreeSet;

struct BlackSegTree {
    black_n: usize,
    black_tree_val: Vec<i32>,
    black_tree_cnt: Vec<i32>,
}

impl BlackSegTree {
    fn new(black_n: usize) -> Self {
        Self {
            black_n,
            black_tree_val: vec![0; 4 * black_n],
            black_tree_cnt: vec![0; 4 * black_n],
        }
    }

    fn black_query_sum(&self, black_x: usize, black_y: usize, black_l: usize, black_r: usize, black_i: usize) -> i32 {
        if black_r < black_x || black_l > black_y { return 0; }
        if black_l >= black_x && black_r <= black_y { return self.black_tree_val[black_i]; }
        let black_m = (black_l + black_r) >> 1;
        self.black_query_sum(black_x, black_y, black_l, black_m, black_i * 2 + 1) +
        self.black_query_sum(black_x, black_y, black_m + 1, black_r, black_i * 2 + 2)
    }

    fn black_query_cnt(&self, black_x: usize, black_y: usize, black_l: usize, black_r: usize, black_i: usize) -> i32 {
        if black_r < black_x || black_l > black_y { return 0; }
        if black_l >= black_x && black_r <= black_y { return self.black_tree_cnt[black_i]; }
        let black_m = (black_l + black_r) >> 1;
        self.black_query_cnt(black_x, black_y, black_l, black_m, black_i * 2 + 1) +
        self.black_query_cnt(black_x, black_y, black_m + 1, black_r, black_i * 2 + 2)
    }

    fn black_update(&mut self, black_ind: usize, black_val: i32, black_l: usize, black_r: usize, black_i: usize) {
        if black_l == black_r {
            self.black_tree_cnt[black_i] += black_val;
            self.black_tree_val[black_i] = self.black_tree_cnt[black_i] * black_l as i32;
            return;
        }
        let black_m = (black_l + black_r) >> 1;
        if black_m >= black_ind { self.black_update(black_ind, black_val, black_l, black_m, black_i * 2 + 1); }
        else { self.black_update(black_ind, black_val, black_m + 1, black_r, black_i * 2 + 2); }
        self.black_tree_val[black_i] = self.black_tree_val[black_i * 2 + 1] + self.black_tree_val[black_i * 2 + 2];
        self.black_tree_cnt[black_i] = self.black_tree_cnt[black_i * 2 + 1] + self.black_tree_cnt[black_i * 2 + 2];
    }
}

impl Solution {
    pub fn number_of_alternating_groups(black_colors: Vec<i32>, black_queries: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_colors.len();
        let mut black_arr = vec![0; 2 * black_n];
        for black_j in 0..black_n { black_arr[black_j] = black_colors[black_j]; black_arr[black_n + black_j] = black_colors[black_j]; }
        let mut black_st = BlackSegTree::new(2 * black_n);
        let mut black_all: BTreeSet<(usize, usize)> = BTreeSet::new();
        let mut black_j = 0;
        while black_j < 2 * black_n - 1 {
            let black_l = black_j;
            let mut black_r = black_j + 1;
            while black_r < 2 * black_n - 1 && black_arr[black_r] != black_arr[black_r - 1] { black_r += 1; }
            Self::black_insert((black_l, black_r - 1), &mut black_all, &mut black_st, black_n);
            black_j = black_r;
        }

        let mut black_result = Vec::new();
        for black_q in black_queries {
            if black_q[0] == 1 {
                black_result.push(Self::black_count(black_q[1], Self::black_group_at(black_n, &black_all), &black_st, black_n));
            } else {
                let (black_idx, black_val) = (black_q[1] as usize, black_q[2]);
                Self::black_update_group(black_idx, black_val, &mut black_all, &mut black_arr, &mut black_st, black_n);
                Self::black_update_group(black_idx + black_n, black_val, &mut black_all, &mut black_arr, &mut black_st, black_n);
            }
        }
        black_result
    }

    fn black_insert(black_val: (usize, usize), black_all: &mut BTreeSet<(usize, usize)>, black_st: &mut BlackSegTree, black_n: usize) {
        black_all.insert(black_val);
        if black_val.0 < black_n { black_st.black_update(black_val.1 - black_val.0 + 1, 1, 0, black_st.black_n - 1, 0); }
    }

    fn black_remove(black_val: (usize, usize), black_all: &mut BTreeSet<(usize, usize)>, black_st: &mut BlackSegTree, black_n: usize) {
        black_all.remove(&black_val);
        if black_val.0 < black_n { black_st.black_update(black_val.1 - black_val.0 + 1, -1, 0, black_st.black_n - 1, 0); }
    }

    fn black_group_at(black_ind: usize, black_all: &BTreeSet<(usize, usize)>) -> (usize, usize) {
        let black_res = black_all.range(..(black_ind + 1, 0)).next_back().or_else(|| black_all.range((black_ind, 0)..).next()).unwrap();
        *black_res
    }

    fn black_count(black_q: i32, black_last: (usize, usize), black_st: &BlackSegTree, black_n: usize) -> i32 {
        let black_sum = black_st.black_query_sum(black_q as usize, black_st.black_n - 1, 0, black_st.black_n - 1, 0);
        let black_cnt = black_st.black_query_cnt(black_q as usize, black_st.black_n - 1, 0, black_st.black_n - 1, 0);
        let mut black_ans = black_sum - (black_q - 1) * black_cnt;
        let (black_l, black_r) = black_last;
        if black_l >= black_n || (black_r - black_l + 1) < black_q as usize { return black_ans; }
        if black_r >= black_n {
            let black_can = black_n - black_l;
            let black_has = (black_r - black_l + 1) - (black_q as usize - 1);
            if black_can < black_has { black_ans -= (black_has - black_can) as i32; }
        }
        black_ans
    }

    fn black_update_group(black_ind: usize, black_val: i32, black_all: &mut BTreeSet<(usize, usize)>, black_arr: &mut Vec<i32>, black_st: &mut BlackSegTree, black_n: usize) {
        if black_ind == 2 * black_n - 1 || black_arr[black_ind] == black_val { return; }
        black_arr[black_ind] = black_val;
        let black_with_ind = Self::black_group_at(black_ind, black_all);
        Self::black_remove(black_with_ind, black_all, black_st, black_n);
        let (mut black_l, mut black_r) = black_with_ind;
        if black_l < black_ind && black_r > black_ind {
            Self::black_insert((black_l, black_ind - 1), black_all, black_st, black_n);
            Self::black_insert((black_ind, black_ind), black_all, black_st, black_n);
            Self::black_insert((black_ind + 1, black_r), black_all, black_st, black_n);
            return;
        }
        if black_l == black_ind && black_r != black_ind { Self::black_insert((black_l + 1, black_r), black_all, black_st, black_n); }
        if black_r == black_ind && black_l != black_ind { Self::black_insert((black_l, black_r - 1), black_all, black_st, black_n); }
        black_l = black_ind; black_r = black_ind;
        let mut black_to_remove = Vec::new();
        while black_l > 0 && black_arr[black_l - 1] != black_arr[black_l] {
            let black_prev = Self::black_group_at(black_l - 1, black_all);
            black_to_remove.push(black_prev);
            black_l = black_prev.0;
        }
        while black_r < 2 * black_n - 2 && black_arr[black_r + 1] != black_arr[black_r] {
            let black_next = Self::black_group_at(black_r + 1, black_all);
            black_to_remove.push(black_next);
            black_r = black_next.1;
        }
        for black_i in black_to_remove { Self::black_remove(black_i, black_all, black_st, black_n); }
        Self::black_insert((black_l, black_r), black_all, black_st, black_n);
    }
}