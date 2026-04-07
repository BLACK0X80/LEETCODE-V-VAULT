impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let (black_n, black_d) = (job_difficulty.len(), d as usize);
        if black_n < black_d { return -1; }
        let mut black_dp = vec![vec![10000; black_n + 1]; black_d + 1];
        black_dp[0][0] = 0;
        for black_i in 1..=black_d {
            for black_j in black_i..=black_n {
                let mut black_mx = 0;
                for black_k in (black_i - 1..black_j).rev() {
                    black_mx = black_mx.max(job_difficulty[black_k]);
                    black_dp[black_i][black_j] = black_dp[black_i][black_j].min(black_dp[black_i - 1][black_k] + black_mx);
                }
            }
        }
        black_dp[black_d][black_n]
    }
}
