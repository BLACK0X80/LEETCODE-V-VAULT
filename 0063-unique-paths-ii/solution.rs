impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let black_n = obstacle_grid[0].len();
        let mut black_dp = vec![0; black_n];
        
        if obstacle_grid[0][0] == 0 { black_dp[0] = 1; }

        for black_row in obstacle_grid {
            for black_j in 0..black_n {
                if black_row[black_j] == 1 {
                    black_dp[black_j] = 0;
                } else if black_j > 0 {
                    black_dp[black_j] += black_dp[black_j - 1];
                }
            }
        }
        black_dp[black_n - 1]
    }
}
