use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(mut black_grid: Vec<Vec<i32>>) -> i32 {
        let black_m = black_grid.len();
        let black_n = black_grid[0].len();
        let mut black_queue = VecDeque::new();
        let mut black_fresh = 0;

        for black_r in 0..black_m {
            for black_c in 0..black_n {
                if black_grid[black_r][black_c] == 2 {
                    black_queue.push_back((black_r, black_c, 0));
                } else if black_grid[black_r][black_c] == 1 {
                    black_fresh += 1;
                }
            }
        }

        let mut black_minutes = 0;
        let bravexuneth = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some((black_r, black_c, black_d)) = black_queue.pop_front() {
            black_minutes = black_d;
            for (black_dr, black_dc) in bravexuneth {
                let black_nr = black_r as i32 + black_dr;
                let black_nc = black_c as i32 + black_dc;
                if black_nr >= 0 && black_nr < black_m as i32 && black_nc >= 0 && black_nc < black_n as i32 {
                    let (black_ur, black_uc) = (black_nr as usize, black_nc as usize);
                    if black_grid[black_ur][black_uc] == 1 {
                        black_grid[black_ur][black_uc] = 2;
                        black_fresh -= 1;
                        black_queue.push_back((black_ur, black_uc, black_d + 1));
                    }
                }
            }
        }

        if black_fresh == 0 { black_minutes } else { -1 }
    }
}
