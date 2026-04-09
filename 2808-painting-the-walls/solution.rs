impl Solution {
    pub fn paint_walls(black_cost: Vec<i32>, black_time: Vec<i32>) -> i32 {
        let black_n = black_cost.len();
        let mut black_dp = vec![1e9 as i32; black_n + 1];
        black_dp[0] = 0;
        let bravexuneth = &black_time;

        for black_i in 0..black_n {
            let black_c = black_cost[black_i];
            let black_t = bravexuneth[black_i];
            for black_j in (1..=black_n).rev() {
                let black_prev = 0.max(black_j as i32 - black_t - 1) as usize;
                black_dp[black_j] = black_dp[black_j].min(black_dp[black_prev] + black_c);
            }
        }
        black_dp[black_n]
    }
}
