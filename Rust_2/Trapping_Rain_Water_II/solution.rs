use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();
        if m < 3 || n < 3 { return 0; }
        let mut vis = vec![vec![false; n]; m];
        let mut pq = BinaryHeap::new();
        for i in 0..m {
            for j in 0..n {
                if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                    vis[i][j] = true;
                    pq.push(Reverse((height_map[i][j], i, j)));
                }
            }
        }
        let mut water = 0;
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        while let Some(Reverse((h, r, c))) = pq.pop() {
            for (dr, dc) in dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if !vis[nr][nc] {
                        vis[nr][nc] = true;
                        let nh = height_map[nr][nc];
                        if nh < h { water += h - nh; }
                        pq.push(Reverse((h.max(nh), nr, nc)));
                    }
                }
            }
        }
        water
    }
}