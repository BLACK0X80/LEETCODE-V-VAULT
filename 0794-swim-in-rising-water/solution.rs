use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut vis = vec![vec![false; n]; n];
        let mut pq = BinaryHeap::new();
        pq.push(Reverse((grid[0][0], 0, 0)));
        vis[0][0] = true;
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        while let Some(Reverse((t, r, c))) = pq.pop() {
            if r == n - 1 && c == n - 1 { return t; }
            for (dr, dc) in dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if !vis[nr][nc] {
                        vis[nr][nc] = true;
                        pq.push(Reverse((t.max(grid[nr][nc]), nr, nc)));
                    }
                }
            }
        }
        0
    }
}
