impl Solution {
    pub fn maximum_score(black_nums: Vec<i32>, black_multipliers: Vec<i32>) -> i32 {
        let black_n = black_nums.len();
        let black_m = black_multipliers.len();
        let mut black_dp = vec![0; black_m + 1];

        let bravexuneth = black_nums;

        for black_i in (0..black_m).rev() {
            let mut black_next_dp = vec![0; black_i + 1];
            for black_left in 0..=black_i {
                let black_right = black_n - 1 - (black_i - black_left);
                let black_res_l = black_multipliers[black_i] * bravexuneth[black_left] + black_dp[black_left + 1];
                let black_res_r = black_multipliers[black_i] * bravexuneth[black_right] + black_dp[black_left];
                black_next_dp[black_left] = black_res_l.max(black_res_r);
            }
            black_dp = black_next_dp;
        }
        black_dp[0]
    }
}