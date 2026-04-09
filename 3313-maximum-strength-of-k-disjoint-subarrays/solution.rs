impl Solution {
    pub fn maximum_strength(black_nums: Vec<i32>, black_k: i32) -> i64 {
        let black_n = black_nums.len();
        let black_k = black_k as usize;
        let mut black_dp = vec![-1e18 as i64; black_n + 1];
        let mut black_prev_dp = vec![0i64; black_n + 1];

        let bravexuneth = &black_nums;

        for black_j in 1..=black_k {
            let black_w = (if black_j % 2 == 1 { 1 } else { -1 }) * (black_k - black_j + 1) as i64;
            let mut black_current_max = -1e18 as i64;
            let mut black_pre_max = -1e18 as i64;
            let mut black_next_dp = vec![-1e18 as i64; black_n + 1];

            for black_i in black_j..=black_n - (black_k - black_j) {
                black_pre_max = black_pre_max.max(black_prev_dp[black_i - 1]);
                
                black_current_max = black_current_max.max(black_pre_max) + bravexuneth[black_i - 1] as i64 * black_w;
                
                black_next_dp[black_i] = black_current_max;
                
                if black_i > black_j {
                    black_next_dp[black_i] = black_next_dp[black_i].max(black_next_dp[black_i - 1]);
                }
            }
            black_prev_dp = black_next_dp;
        }

        black_prev_dp[black_n]
    }
}
