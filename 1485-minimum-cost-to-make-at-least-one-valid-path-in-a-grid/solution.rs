use std::collections::VecDeque;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let dirs = [(0,1),(0,-1),(1,0),(-1,0)];
        let mut dist = vec![vec![i32::MAX; n]; m];
        dist[0][0] = 0;
        let mut dq = VecDeque::new();
        dq.push_front((0usize, 0usize));
        while let Some((r, c)) = dq.pop_front() {
            for (d, &(dr, dc)) in dirs.iter().enumerate() {
                let nr = r as i32 + dr; let nc = c as i32 + dc;
                if nr < 0 || nc < 0 || nr >= m as i32 || nc >= n as i32 { continue; }
                let (nr, nc) = (nr as usize, nc as usize);
                let cost = dist[r][c] + if grid[r][c] == (d+1) as i32 { 0 } else { 1 };
                if cost < dist[nr][nc] {
                    dist[nr][nc] = cost;
                    if grid[r][c] == (d+1) as i32 { dq.push_front((nr,nc)); } else { dq.push_back((nr,nc)); }
                }
            }
        }
        dist[m-1][n-1]
    }
}
