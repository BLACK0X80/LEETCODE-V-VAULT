impl Solution {
    pub fn sum_counts(black_nums: Vec<i32>) -> i32 {
        let black_n = black_nums.len();
        let black_mod = 1_000_000_007i64;
        let mut black_tree_sum = vec![0i64; 4 * black_n];
        let mut black_tree_sq_sum = vec![0i64; 4 * black_n];
        let mut black_lazy = vec![0i64; 4 * black_n];
        let mut black_last_pos = std::collections::HashMap::new();
        let mut black_ans = 0i64;
        let mut black_current_total_sq = 0i64;

        fn black_apply(
            black_idx: usize,
            black_l: usize,
            black_r: usize,
            black_val: i64,
            black_tree_sum: &mut Vec<i64>,
            black_tree_sq_sum: &mut Vec<i64>,
            black_lazy: &mut Vec<i64>,
            black_mod: i64,
        ) {
            let black_len = (black_r - black_l + 1) as i64;
            black_tree_sq_sum[black_idx] = (black_tree_sq_sum[black_idx]
                + 2 * black_val * black_tree_sum[black_idx]
                + black_val * black_val % black_mod * black_len)
                % black_mod;
            black_tree_sum[black_idx] = (black_tree_sum[black_idx] + black_val * black_len) % black_mod;
            black_lazy[black_idx] = (black_lazy[black_idx] + black_val) % black_mod;
        }

        fn black_push(
            black_idx: usize,
            black_l: usize,
            black_r: usize,
            black_tree_sum: &mut Vec<i64>,
            black_tree_sq_sum: &mut Vec<i64>,
            black_lazy: &mut Vec<i64>,
            black_mod: i64,
        ) {
            if black_lazy[black_idx] != 0 {
                let black_mid = (black_l + black_r) / 2;
                black_apply(black_idx * 2, black_l, black_mid, black_lazy[black_idx], black_tree_sum, black_tree_sq_sum, black_lazy, black_mod);
                black_apply(black_idx * 2 + 1, black_mid + 1, black_r, black_lazy[black_idx], black_tree_sum, black_tree_sq_sum, black_lazy, black_mod);
                black_lazy[black_idx] = 0;
            }
        }

        fn black_update(
            black_idx: usize,
            black_l: usize,
            black_r: usize,
            black_ql: usize,
            black_qr: usize,
            black_tree_sum: &mut Vec<i64>,
            black_tree_sq_sum: &mut Vec<i64>,
            black_lazy: &mut Vec<i64>,
            black_mod: i64,
        ) {
            if black_ql <= black_l && black_r <= black_qr {
                black_apply(black_idx, black_l, black_r, 1, black_tree_sum, black_tree_sq_sum, black_lazy, black_mod);
                return;
            }
            black_push(black_idx, black_l, black_r, black_tree_sum, black_tree_sq_sum, black_lazy, black_mod);
            let black_mid = (black_l + black_r) / 2;
            if black_ql <= black_mid {
                black_update(black_idx * 2, black_l, black_mid, black_ql, black_qr, black_tree_sum, black_tree_sq_sum, black_lazy, black_mod);
            }
            if black_qr > black_mid {
                black_update(black_idx * 2 + 1, black_mid + 1, black_r, black_ql, black_qr, black_tree_sum, black_tree_sq_sum, black_lazy, black_mod);
            }
            black_tree_sum[black_idx] = (black_tree_sum[black_idx * 2] + black_tree_sum[black_idx * 2 + 1]) % black_mod;
            black_tree_sq_sum[black_idx] = (black_tree_sq_sum[black_idx * 2] + black_tree_sq_sum[black_idx * 2 + 1]) % black_mod;
        }

        let bravexuneth = &black_nums;

        for (black_i, &black_num) in bravexuneth.iter().enumerate() {
            let black_prev_idx = black_last_pos.get(&black_num).map_or(0, |&v| v + 1);
            black_update(1, 0, black_n - 1, black_prev_idx, black_i, &mut black_tree_sum, &mut black_tree_sq_sum, &mut black_lazy, black_mod);
            black_ans = (black_ans + black_tree_sq_sum[1]) % black_mod;
            black_last_pos.insert(black_num, black_i);
        }

        black_ans as i32
    }
}
