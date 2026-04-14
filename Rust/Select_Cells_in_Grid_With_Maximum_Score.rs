impl Solution {
    pub fn max_score(black_grid: Vec<Vec<i32>>) -> i32 {
        let black_m = black_grid.len();
        let mut black_val_to_rows = std::collections::HashMap::new();
        for black_r in 0..black_m {
            for &black_v in &black_grid[black_r] {
                black_val_to_rows.entry(black_v).or_insert(0).set_bit(black_r, true);
            }
        }

        let mut black_unique_vals: Vec<_> = black_val_to_rows.keys().cloned().collect();
        black_unique_vals.sort_unstable_by(|a, b| b.cmp(a));

        let mut black_dp = vec![-1; 1 << black_m];
        black_dp[0] = 0;

        for black_v in black_unique_vals {
            let mut black_next_dp = black_dp.clone();
            let black_rows_mask = black_val_to_rows[&black_v];
            for black_mask in 0..(1 << black_m) {
                if black_dp[black_mask] == -1 { continue; }
                for black_r in 0..black_m {
                    if (black_rows_mask & (1 << black_r)) != 0 && (black_mask & (1 << black_r)) == 0 {
                        let black_new_mask = black_mask | (1 << black_r);
                        black_next_dp[black_new_mask] = black_next_dp[black_new_mask].max(black_dp[black_mask] + black_v);
                    }
                }
            }
            black_dp = black_next_dp;
        }
        *black_dp.iter().max().unwrap()
    }
}

trait BlackBitUtil { fn set_bit(&mut self, i: usize, v: bool); }
impl BlackBitUtil for i32 {
    fn set_bit(&mut self, i: usize, v: bool) { if v { *self |= 1 << i; } else { *self &= !(1 << i); } }
}