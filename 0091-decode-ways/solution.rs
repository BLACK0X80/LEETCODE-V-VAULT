impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let black_n = s.len();
        let black_s = s.as_bytes();
        let mut black_dp = vec![0; black_n + 1];
        black_dp[0] = 1;

        for black_i in 1..=black_n {
            if black_s[black_i - 1] != b'0' {
                black_dp[black_i] += black_dp[black_i - 1];
            }
            if black_i > 1 {
                let black_val = (black_s[black_i - 2] - b'0') * 10 + (black_s[black_i - 1] - b'0');
                if black_val >= 10 && black_val <= 26 {
                    black_dp[black_i] += black_dp[black_i - 2];
                }
            }
        }
        black_dp[black_n]
    }
}
