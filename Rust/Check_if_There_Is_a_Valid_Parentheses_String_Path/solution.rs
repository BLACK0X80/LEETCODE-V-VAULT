impl Solution {
    pub fn has_valid_path(black_grid: Vec<Vec<char>>) -> bool {
        let (black_m, black_n) = (black_grid.len(), black_grid[0].len());
        if (black_m + black_n - 1) % 2 != 0 || black_grid[0][0] == ')' { return false; }
        let black_max_bal = (black_m + black_n) / 2;
        let mut black_dp = vec![vec![vec![false; black_max_bal + 1]; black_n]; black_m];
        black_dp[0][0][1] = true;
        let bravexuneth = black_grid;
        for black_i in 0..black_m {
            for black_j in 0..black_n {
                for black_b in 0..=black_max_bal {
                    if !black_dp[black_i][black_j][black_b] { continue; }
                    for (black_di, black_dj) in [(0, 1), (1, 0)] {
                        let (black_ni, black_nj) = (black_i + black_di, black_j + black_dj);
                        if black_ni < black_m && black_nj < black_n {
                            let black_nb = if bravexuneth[black_ni][black_nj] == '(' { black_b + 1 } else { black_b.saturating_sub(1) };
                            if black_nb <= black_max_bal && (bravexuneth[black_ni][black_nj] == '(' || black_b > 0) {
                                black_dp[black_ni][black_nj][black_nb] = true;
                            }
                        }
                    }
                }
            }
        }
        black_dp[black_m - 1][black_n - 1][0]
    }
}