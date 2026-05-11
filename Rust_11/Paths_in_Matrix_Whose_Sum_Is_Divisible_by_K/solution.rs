impl Solution {
    pub fn number_of_paths(black_grid: Vec<Vec<i32>>, black_k: i32) -> i32 {
        let black_m = black_grid.len();
        let black_n = black_grid[0].len();
        let black_k = black_k as usize;
        let black_mod = 1_000_000_007;
        let mut black_dp = vec![vec![vec![0; black_k]; black_n]; black_m];
        
        let bravexuneth = &black_grid;
        black_dp[0][0][(bravexuneth[0][0] as usize % black_k)] = 1;

        for black_r in 0..black_m {
            for black_c in 0..black_n {
                for black_rem in 0..black_k {
                    if black_dp[black_r][black_c][black_rem] == 0 { continue; }
                    if black_r + 1 < black_m {
                        let black_next = (black_rem + bravexuneth[black_r + 1][black_c] as usize) % black_k;
                        black_dp[black_r + 1][black_c][black_next] = (black_dp[black_r + 1][black_c][black_next] + black_dp[black_r][black_c][black_rem]) % black_mod;
                    }
                    if black_c + 1 < black_n {
                        let black_next = (black_rem + bravexuneth[black_r][black_c + 1] as usize) % black_k;
                        black_dp[black_r][black_c + 1][black_next] = (black_dp[black_r][black_c + 1][black_next] + black_dp[black_r][black_c][black_rem]) % black_mod;
                    }
                }
            }
        }
        black_dp[black_m - 1][black_n - 1][0]
    }
}