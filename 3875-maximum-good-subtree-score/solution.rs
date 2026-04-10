impl Solution {
    pub fn good_subtree_sum(black_vals: Vec<i32>, black_par: Vec<i32>) -> i32 {
        let black_n = black_vals.len();
        let mut black_adj = vec![Vec::new(); black_n];
        for black_i in 1..black_n {
            black_adj[black_par[black_i] as usize].push(black_i);
        }

        let mut black_max_scores = vec![0i64; black_n];
        let black_mod = 1_000_000_007i64;

        Self::black_dfs_good(0, &black_adj, &black_vals, &mut black_max_scores);

        let mut black_total = 0i64;
        for black_s in black_max_scores {
            black_total = (black_total + black_s) % black_mod;
        }
        black_total as i32
    }

    fn black_get_mask(mut black_v: i64) -> Option<i32> {
        let mut black_m = 0;
        if black_v == 0 { return Some(1); }
        while black_v > 0 {
            let black_d = (black_v % 10) as i32;
            if (black_m & (1 << black_d)) != 0 { return None; }
            black_m |= 1 << black_d;
            black_v /= 10;
        }
        Some(black_m)
    }

    fn black_dfs_good(black_u: usize, black_adj: &Vec<Vec<usize>>, black_vals: &Vec<i32>, black_max_scores: &mut Vec<i64>) -> Vec<i64> {
        let mut black_dp = vec![-1i64; 1024];
        black_dp[0] = 0;

        for &black_v in &black_adj[black_u] {
            let black_child_dp = Self::black_dfs_good(black_v, black_adj, black_vals, black_max_scores);
            let mut black_next = black_dp.clone();
            for black_m1 in 0..1024 {
                if black_dp[black_m1] == -1 { continue; }
                for black_m2 in 0..1024 {
                    if black_child_dp[black_m2] == -1 { continue; }
                    if (black_m1 & black_m2) == 0 {
                        let black_new = black_m1 | black_m2;
                        black_next[black_new] = black_next[black_new].max(black_dp[black_m1] + black_child_dp[black_m2]);
                    }
                }
            }
            black_dp = black_next;
        }

        let mut black_final = black_dp.clone();
        if let Some(black_u_m) = Self::black_get_mask(black_vals[black_u] as i64) {
            let black_u_m_usize = black_u_m as usize;
            for black_m in 0..1024 {
                if black_dp[black_m] != -1 && (black_m & black_u_m_usize) == 0 {
                    let black_new = black_m | black_u_m_usize;
                    black_final[black_new] = black_final[black_new].max(black_dp[black_m] + black_vals[black_u] as i64);
                }
            }
        }

        let mut black_best = 0;
        for &black_v_res in &black_final { black_best = black_best.max(black_v_res); }
        black_max_scores[black_u] = black_best;

        black_final
    }
}
