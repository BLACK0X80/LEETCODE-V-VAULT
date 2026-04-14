impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (black_m, black_n) = (grid.len(), grid[0].len());
        let mut black_dp = vec![0; black_n];

        for black_i in 0..black_m {
            for black_j in 0..black_n {
                if black_j == 0 {
                    black_dp[black_j] += grid[black_i][black_j];
                } else if black_i == 0 {
                    black_dp[black_j] = black_dp[black_j - 1] + grid[black_i][black_j];
                } else {
                    black_dp[black_j] = std::cmp::min(black_dp[black_j], black_dp[black_j - 1]) + grid[black_i][black_j];
                }
            }
        }
        black_dp[black_n - 1]
    }
}