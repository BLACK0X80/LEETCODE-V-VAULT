impl Solution {
    pub fn min_days(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let black_count = |g: &Vec<Vec<i32>>| -> i32 {
            let mut black_vis = vec![vec![false; n]; m];
            let mut black_cnt = 0;
            for i in 0..m { for j in 0..n {
                if g[i][j] == 1 && !black_vis[i][j] {
                    black_cnt += 1;
                    let mut black_q = vec![(i, j)];
                    black_vis[i][j] = true;
                    while let Some((r, c)) = black_q.pop() {
                        for (dr, dc) in [(!0,0),(1,0),(0,!0usize),(0,1usize)] {
                            let (nr, nc) = (r.wrapping_add(dr), c.wrapping_add(dc));
                            if nr < m && nc < n && !black_vis[nr][nc] && g[nr][nc] == 1 {
                                black_vis[nr][nc] = true;
                                black_q.push((nr, nc));
                            }
                        }
                    }
                }
            }}
            black_cnt
        };

        if black_count(&grid) != 1 { return 0; }

        for i in 0..m { for j in 0..n {
            if grid[i][j] == 1 {
                let mut black_g = grid.clone();
                black_g[i][j] = 0;
                if black_count(&black_g) != 1 { return 1; }
            }
        }}
        2
    }
}
