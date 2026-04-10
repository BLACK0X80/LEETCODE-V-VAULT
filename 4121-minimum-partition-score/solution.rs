impl Solution {
    pub fn min_partition_score(black_nums: Vec<i32>, black_k: i32) -> i64 {
        let black_n = black_nums.len();
        let black_k = black_k as usize;
        let mut black_prefix_sum = vec![0i64; black_n + 1];
        for black_i in 0..black_n {
            black_prefix_sum[black_i + 1] = black_prefix_sum[black_i] + black_nums[black_i] as i64;
        }

        let black_calc_value = |black_s: i64| -> i64 {
            (black_s * (black_s + 1)) / 2
        };

        let mut black_dp = vec![i64::MAX / 2; black_n + 1];
        black_dp[0] = 0;

        for black_p in 1..=black_k {
            let mut black_next_dp = vec![i64::MAX / 2; black_n + 1];
            for black_i in black_p..=black_n {
                for black_j in (black_p - 1)..black_i {
                    let black_current_sum = black_prefix_sum[black_i] - black_prefix_sum[black_j];
                    let black_score = black_dp[black_j] + black_calc_value(black_current_sum);
                    if black_score < black_next_dp[black_i] {
                        black_next_dp[black_i] = black_score;
                    }
                }
            }
            black_dp = black_next_dp;
        }

        black_dp[black_n]
    }
}
