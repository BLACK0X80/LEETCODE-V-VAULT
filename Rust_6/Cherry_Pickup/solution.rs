impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dp = vec![vec![i32::MIN; n]; n];
        dp[0][0] = grid[0][0];

        for t in 1..2*n-1 {
            let mut ndp = vec![vec![i32::MIN; n]; n];
            for r1 in 0..n {
                let c1 = t as i32 - r1 as i32;
                if c1 < 0 || c1 >= n as i32 { continue; }
                let c1 = c1 as usize;
                if grid[r1][c1] == -1 { continue; }

                for r2 in r1..n {
                    let c2 = t as i32 - r2 as i32;
                    if c2 < 0 || c2 >= n as i32 { continue; }
                    let c2 = c2 as usize;
                    if grid[r2][c2] == -1 { continue; }

                    let mut best = i32::MIN;
                    for (pr1, pr2) in [(r1,r2),(r1+1,r2),(r1,r2+1),(r1+1,r2+1)] {
                        if pr1 > 0 && pr2 > 0 && dp[pr1-1][pr2-1] != i32::MIN {
                            best = best.max(dp[pr1-1][pr2-1]);
                        } else if pr1 == 0 && pr2 == 0 && dp[0][0] != i32::MIN && t == 1 {
                            best = best.max(dp[0][0]);
                        }
                    }

                    if best == i32::MIN { continue; }
                    let cherries = grid[r1][c1] + if r1 == r2 { 0 } else { grid[r2][c2] };
                    ndp[r1][r2] = ndp[r1][r2].max(best + cherries);
                }
            }
            dp = ndp;
        }

        dp[n-1][n-1].max(0)
    }
}