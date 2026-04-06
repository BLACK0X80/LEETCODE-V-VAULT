use std::collections::VecDeque;

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let (black_m, black_n) = (grid.len(), grid[0].len());
        let mut black_dist = vec![vec![i32::MAX; black_n]; black_m];
        black_dist[0][0] = 0;
        let mut black_dq = VecDeque::new();
        black_dq.push_front((0usize, 0usize));

        while let Some((r, c)) = black_dq.pop_front() {
            for (dr, dc) in [(0,1),(0,usize::MAX),(1,0),(usize::MAX,0)] {
                let (nr, nc) = (r.wrapping_add(dr), c.wrapping_add(dc));
                if nr < black_m && nc < black_n {
                    let black_nd = black_dist[r][c] + grid[nr][nc];
                    if black_nd < black_dist[nr][nc] {
                        black_dist[nr][nc] = black_nd;
                        if grid[nr][nc] == 0 { black_dq.push_front((nr, nc)); }
                        else { black_dq.push_back((nr, nc)); }
                    }
                }
            }
        }
        black_dist[black_m-1][black_n-1]
    }
}
