use std::collections::VecDeque;

impl Solution {
    pub fn max_moves(black_kx: i32, black_ky: i32, black_positions: Vec<Vec<i32>>) -> i32 {
        let black_n = black_positions.len();
        let mut black_pts = vec![vec![black_kx, black_ky]];
        for black_p in black_positions {
            black_pts.push(black_p);
        }

        let mut black_dist = vec![vec![0; black_n + 1]; black_n + 1];
        let black_dirs = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];

        for black_i in 0..=black_n {
            let mut black_d = vec![vec![-1; 50]; 50];
            let mut black_q = VecDeque::from([(black_pts[black_i][0], black_pts[black_i][1])]);
            black_d[black_pts[black_i][0] as usize][black_pts[black_i][1] as usize] = 0;

            while let Some((black_x, black_y)) = black_q.pop_front() {
                for (black_dx, black_dy) in black_dirs {
                    let (black_nx, black_ny) = (black_x + black_dx, black_y + black_dy);
                    if black_nx >= 0 && black_nx < 50 && black_ny >= 0 && black_ny < 50 {
                        if black_d[black_nx as usize][black_ny as usize] == -1 {
                            black_d[black_nx as usize][black_ny as usize] = black_d[black_x as usize][black_y as usize] + 1;
                            black_q.push_back((black_nx, black_ny));
                        }
                    }
                }
            }
            for black_j in 0..=black_n {
                black_dist[black_i][black_j] = black_d[black_pts[black_j][0] as usize][black_pts[black_j][1] as usize];
            }
        }

        let mut black_memo = vec![vec![-1; 1 << black_n]; black_n + 1];
        Self::black_solve(0, (1 << black_n) - 1, black_n, &black_dist, &mut black_memo, true)
    }

    fn black_solve(black_curr: usize, black_mask: usize, black_n: usize, black_dist: &Vec<Vec<i32>>, black_memo: &mut Vec<Vec<i32>>, black_alice: bool) -> i32 {
        if black_mask == 0 { return 0; }
        if black_memo[black_curr][black_mask] != -1 { return black_memo[black_curr][black_mask]; }

        let mut black_res = if black_alice { 0 } else { i32::MAX };

        for black_i in 0..black_n {
            if (black_mask >> black_i) & 1 == 1 {
                let black_moves = black_dist[black_curr][black_i + 1] + 
                                  Self::black_solve(black_i + 1, black_mask ^ (1 << black_i), black_n, black_dist, black_memo, !black_alice);
                if black_alice {
                    black_res = black_res.max(black_moves);
                } else {
                    black_res = black_res.min(black_moves);
                }
            }
        }

        black_memo[black_curr][black_mask] = black_res;
        black_res
    }
}
