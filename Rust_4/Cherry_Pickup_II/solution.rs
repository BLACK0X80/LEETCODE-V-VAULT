impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut dp = vec![vec![-1; cols]; cols];
        dp[0][cols - 1] = grid[0][0] + grid[0][cols - 1];
        for r in 0..rows - 1 {
            let mut new_dp = vec![vec![-1; cols]; cols];
            for c1 in 0..cols {
                for c2 in 0..cols {
                    if dp[c1][c2] == -1 { continue; }
                    for dc1 in -1..=1 {
                        for dc2 in -1..=1 {
                            let nc1 = c1 as i32 + dc1;
                            let nc2 = c2 as i32 + dc2;
                            if nc1 >= 0 && nc1 < cols as i32 && nc2 >= 0 && nc2 < cols as i32 {
                                let nc1 = nc1 as usize;
                                let nc2 = nc2 as usize;
                                let add = grid[r + 1][nc1] + if nc1 != nc2 { grid[r + 1][nc2] } else { 0 };
                                new_dp[nc1][nc2] = new_dp[nc1][nc2].max(dp[c1][c2] + add);
                            }
                        }
                    }
                }
            }
            dp = new_dp;
        }
        dp.into_iter().flatten().max().unwrap_or(0)
    }
}