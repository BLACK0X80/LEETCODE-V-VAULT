impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut black_dp = triangle.last().unwrap().clone();
        for black_i in (0..triangle.len() - 1).rev() {
            for black_j in 0..=black_i {
                black_dp[black_j] = triangle[black_i][black_j] + std::cmp::min(black_dp[black_j], black_dp[black_j + 1]);
            }
        }
        black_dp[0]
    }
}