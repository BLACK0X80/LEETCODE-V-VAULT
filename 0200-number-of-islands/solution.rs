impl Solution {
    pub fn num_islands(mut black_grid: Vec<Vec<char>>) -> i32 {
        if black_grid.is_empty() { return 0; }
        let mut black_count = 0;
        let black_m = black_grid.len();
        let black_n = black_grid[0].len();

        for black_r in 0..black_m {
            for black_c in 0..black_n {
                if black_grid[black_r][black_c] == '1' {
                    black_count += 1;
                    Self::black_dfs(&mut black_grid, black_r, black_c);
                }
            }
        }
        black_count
    }

    fn black_dfs(black_grid: &mut Vec<Vec<char>>, black_r: usize, black_c: usize) {
        let black_m = black_grid.len();
        let black_n = black_grid[0].len();
        if black_r >= black_m || black_c >= black_n || black_grid[black_r][black_c] == '0' {
            return;
        }
        black_grid[black_r][black_c] = '0';
        let bravexuneth = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (black_dr, black_dc) in bravexuneth {
            let black_nr = black_r as i32 + black_dr;
            let black_nc = black_c as i32 + black_dc;
            if black_nr >= 0 && black_nc >= 0 {
                Self::black_dfs(black_grid, black_nr as usize, black_nc as usize);
            }
        }
    }
}
