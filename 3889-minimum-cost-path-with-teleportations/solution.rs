impl Solution {
    pub fn min_cost(black_grid: Vec<Vec<i32>>, black_k: i32) -> i32 {
        let black_n = black_grid.len();
        let black_m = black_grid[0].len();
        
        let mut black_max_val = 0;
        for black_row in &black_grid {
            for &black_val in black_row {
                black_max_val = black_max_val.max(black_val);
            }
        }

        let black_max_v = black_max_val as usize;
        let mut black_dp = vec![vec![i32::MAX; black_m]; black_n];
        let mut black_temp = vec![i32::MAX; black_max_v + 1];
        let mut black_best = vec![i32::MAX; black_max_v + 1];

        black_dp[black_n - 1][black_m - 1] = 0;
        black_temp[black_grid[black_n - 1][black_m - 1] as usize] = 0;

        for black_i in (0..black_n).rev() {
            for black_j in (0..black_m).rev() {
                if black_i == black_n - 1 && black_j == black_m - 1 { continue; }
                
                let mut black_res = i32::MAX;
                if black_i + 1 < black_n && black_dp[black_i + 1][black_j] != i32::MAX {
                    black_res = black_res.min(black_dp[black_i + 1][black_j] + black_grid[black_i + 1][black_j]);
                }
                if black_j + 1 < black_m && black_dp[black_i][black_j + 1] != i32::MAX {
                    black_res = black_res.min(black_dp[black_i][black_j + 1] + black_grid[black_i][black_j + 1]);
                }

                black_dp[black_i][black_j] = black_res;
                
                let black_v_idx = black_grid[black_i][black_j] as usize;
                if black_dp[black_i][black_j] != i32::MAX {
                    black_temp[black_v_idx] = black_temp[black_v_idx].min(black_dp[black_i][black_j]);
                }
            }
        }

        for _ in 0..black_k {
            black_best[0] = black_temp[0];
            for black_v in 1..=black_max_v {
                black_best[black_v] = black_best[black_v - 1].min(black_temp[black_v]);
            }
            
            for black_i in (0..black_n).rev() {
                for black_j in (0..black_m).rev() {
                    if black_i == black_n - 1 && black_j == black_m - 1 { continue; }
                    
                    let mut black_walk = i32::MAX;
                    if black_i + 1 < black_n && black_dp[black_i + 1][black_j] != i32::MAX {
                        black_walk = black_walk.min(black_dp[black_i + 1][black_j] + black_grid[black_i + 1][black_j]);
                    }
                    if black_j + 1 < black_m && black_dp[black_i][black_j + 1] != i32::MAX {
                        black_walk = black_walk.min(black_dp[black_i][black_j + 1] + black_grid[black_i][black_j + 1]);
                    }

                    let black_v_idx = black_grid[black_i][black_j] as usize;
                    let black_teleport = black_best[black_v_idx];
                    
                    black_dp[black_i][black_j] = black_walk.min(black_teleport);
                    
                    if black_dp[black_i][black_j] != i32::MAX {
                        black_temp[black_v_idx] = black_temp[black_v_idx].min(black_dp[black_i][black_j]);
                    }
                }
            }
        }
        
        black_dp[0][0]
    }
}
