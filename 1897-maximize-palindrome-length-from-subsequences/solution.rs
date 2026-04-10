impl Solution {
    pub fn longest_palindrome(black_word1: String, black_word2: String) -> i32 {
        let black_combined = format!("{}{}", black_word1, black_word2);
        let black_bytes = black_combined.as_bytes();
        let black_n = black_bytes.len();
        let black_n1 = black_word1.len();
        let mut black_dp = vec![vec![0; black_n]; black_n];
        let mut black_res = 0;

        for black_i in (0..black_n).rev() {
            black_dp[black_i][black_i] = 1;
            for black_j in black_i + 1..black_n {
                if black_bytes[black_i] == black_bytes[black_j] {
                    black_dp[black_i][black_j] = black_dp[black_i + 1][black_j - 1] + 2;
                    if black_i < black_n1 && black_j >= black_n1 {
                        black_res = black_res.max(black_dp[black_i][black_j]);
                    }
                } else {
                    black_dp[black_i][black_j] = black_dp[black_i + 1][black_j].max(black_dp[black_i][black_j - 1]);
                }
            }
        }
        black_res
    }
}
