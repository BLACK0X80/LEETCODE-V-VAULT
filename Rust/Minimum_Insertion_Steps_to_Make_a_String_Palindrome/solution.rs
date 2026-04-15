impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let black_bytes = s.as_bytes();
        let black_n = black_bytes.len();
        let mut black_dp = vec![vec![0; black_n]; black_n];
        for black_len in 2..=black_n {
            for black_i in 0..=black_n - black_len {
                let black_j = black_i + black_len - 1;
                if black_bytes[black_i] == black_bytes[black_j] {
                    black_dp[black_i][black_j] = if black_len == 2 { 0 } else { black_dp[black_i + 1][black_j - 1] };
                } else {
                    black_dp[black_i][black_j] = 1 + black_dp[black_i + 1][black_j].min(black_dp[black_i][black_j - 1]);
                }
            }
        }
        black_dp[0][black_n - 1]
    }
}