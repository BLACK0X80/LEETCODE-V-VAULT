impl Solution {
    pub fn count_pyramids(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![0; n]; m];
        let mut ans = 0;
        for i in 0..m { for j in 0..n { dp[i][j] = grid[i][j]; } }
        for i in (0..m-1).rev() {
            for j in 1..n-1 {
                if dp[i][j] > 0 {
                    dp[i][j] = dp[i+1][j-1].min(dp[i+1][j]).min(dp[i+1][j+1]) + 1;
                    if dp[i][j] > 1 { ans += dp[i][j] - 1; }
                }
            }
        }
        for i in 0..m { for j in 0..n { dp[i][j] = grid[i][j]; } }
        for i in 1..m {
            for j in 1..n-1 {
                if dp[i][j] > 0 {
                    dp[i][j] = dp[i-1][j-1].min(dp[i-1][j]).min(dp[i-1][j+1]) + 1;
                    if dp[i][j] > 1 { ans += dp[i][j] - 1; }
                }
            }
        }
        ans
    }
}
