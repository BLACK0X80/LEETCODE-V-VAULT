impl Solution {
    pub fn max_score(black_nums1: Vec<i32>, black_nums2: Vec<i32>, black_k: i32) -> i64 {
        let (black_n, black_m, black_k) = (black_nums1.len(), black_nums2.len(), black_k as usize);
        let black_inf = 1e16 as i64;
        let mut black_dp = vec![vec![vec![-black_inf; black_k + 1]; black_m + 1]; black_n + 1];

        for black_i in 0..=black_n {
            for black_j in 0..=black_m {
                black_dp[black_i][black_j][0] = 0;
            }
        }

        for black_p in 1..=black_k {
            for black_i in 1..=black_n {
                for black_j in 1..=black_m {
                    let mut black_res = black_dp[black_i - 1][black_j][black_p]
                        .max(black_dp[black_i][black_j - 1][black_p]);

                    let black_prev = black_dp[black_i - 1][black_j - 1][black_p - 1];
                    if black_prev != -black_inf {
                        let black_score = (black_nums1[black_i - 1] as i64 * black_nums2[black_j - 1] as i64) + black_prev;
                        black_res = black_res.max(black_score);
                    }
                    black_dp[black_i][black_j][black_p] = black_res;
                }
            }
        }

        black_dp[black_n][black_m][black_k]
    }
}
