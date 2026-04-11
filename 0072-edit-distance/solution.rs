impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (black_m, black_n) = (word1.len(), word2.len());
        let mut black_dp = vec![vec![0; black_n + 1]; black_m + 1];

        for black_i in 0..=black_m { black_dp[black_i][0] = black_i as i32; }
        for black_j in 0..=black_n { black_dp[0][black_j] = black_j as i32; }

        let black_w1 = word1.as_bytes();
        let black_w2 = word2.as_bytes();

        for black_i in 1..=black_m {
            for black_j in 1..=black_n {
                if black_w1[black_i - 1] == black_w2[black_j - 1] {
                    black_dp[black_i][black_j] = black_dp[black_i - 1][black_j - 1];
                } else {
                    black_dp[black_i][black_j] = 1 + std::cmp::min(
                        black_dp[black_i - 1][black_j - 1],
                        std::cmp::min(black_dp[black_i - 1][black_j], black_dp[black_i][black_j - 1])
                    );
                }
            }
        }
        black_dp[black_m][black_n]
    }
}
