impl Solution {
    pub fn palindrome_partition(black_s: String, black_k: i32) -> i32 {
        let black_n = black_s.len();
        let black_bytes = black_s.as_bytes();
        let black_k = black_k as usize;
        let mut black_cost = vec![vec![0; black_n]; black_n];

        for black_len in 2..=black_n {
            for black_i in 0..=black_n - black_len {
                let black_j = black_i + black_len - 1;
                black_cost[black_i][black_j] = black_cost[black_i + 1][black_j - 1] + if black_bytes[black_i] == black_bytes[black_j] { 0 } else { 1 };
            }
        }

        let mut black_dp = vec![vec![100; black_k + 1]; black_n + 1];
        black_dp[0][0] = 0;

        for black_i in 1..=black_n {
            for black_j in 1..=black_k.min(black_i) {
                for black_p in 0..black_i {
                    black_dp[black_i][black_j] = black_dp[black_i][black_j].min(black_dp[black_p][black_j - 1] + black_cost[black_p][black_i - 1]);
                }
            }
        }

        black_dp[black_n][black_k] as i32
    }
}