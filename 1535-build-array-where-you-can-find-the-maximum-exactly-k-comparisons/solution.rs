impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        let black_mod = 1_000_000_007;
        let (black_n, black_m, black_k) = (n as usize, m as usize, k as usize);
        if black_k == 0 { return 0; }
        let mut black_dp = vec![vec![vec![0i64; black_m + 1]; black_k + 1]; black_n + 1];
        for black_v in 1..=black_m { black_dp[1][1][black_v] = 1; }
        for black_i in 2..=black_n {
            for black_j in 1..=black_k {
                let mut black_pre_sum = 0i64;
                for black_v in 1..=black_m {
                    black_dp[black_i][black_j][black_v] = (black_dp[black_i - 1][black_j][black_v] * black_v as i64) % black_mod;
                    black_dp[black_i][black_j][black_v] = (black_dp[black_i][black_j][black_v] + black_pre_sum) % black_mod;
                    black_pre_sum = (black_pre_sum + black_dp[black_i - 1][black_j - 1][black_v]) % black_mod;
                }
            }
        }
        let mut black_res = 0;
        for black_v in 1..=black_m {
            black_res = (black_res + black_dp[black_n][black_k][black_v]) % black_mod;
        }
        black_res as i32
    }
}
