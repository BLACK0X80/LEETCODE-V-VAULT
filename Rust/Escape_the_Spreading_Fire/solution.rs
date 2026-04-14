use std::collections::VecDeque;

impl Solution {
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut fire = vec![vec![i32::MAX; n]; m];
        let mut q = VecDeque::new();
        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == 1 {
                    fire[r][c] = 0;
                    q.push_back((r, c));
                }
            }
        }
        let dirs = [(0,1),(0,-1),(1,0),(-1,0)];
        while let Some((r, c)) = q.pop_front() {
            for (dr, dc) in dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if grid[nr][nc] != 2 && fire[nr][nc] == i32::MAX {
                        fire[nr][nc] = fire[r][c] + 1;
                        q.push_back((nr, nc));
                    }
                }
            }
        }
        let check = |w: i32| -> bool {
            let mut vis = vec![vec![false; n]; m];
            let mut q = VecDeque::new();
            if fire[0][0] > w {
                q.push_back((0, 0, w));
                vis[0][0] = true;
            }
            while let Some((r, c, t)) = q.pop_front() {
                for (dr, dc) in dirs {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                        let nr = nr as usize;
                        let nc = nc as usize;
                        if !vis[nr][nc] && grid[nr][nc] != 2 {
                            let nt = t + 1;
                            if (nr == m - 1 && nc == n - 1 && nt <= fire[nr][nc]) || nt < fire[nr][nc] {
                                if nr == m - 1 && nc == n - 1 { return true; }
                                vis[nr][nc] = true;
                                q.push_back((nr, nc, nt));
                            }
                        }
                    }
                }
            }
            false
        };
        if !check(0) { return -1; }
        if check(1_000_000_000) { return 1_000_000_000; }
        let (mut l, mut r) = (0, 1_000_000_000);
        while l < r {
            let mid = l + (r - l + 1) / 2;
            if check(mid) { l = mid; } else { r = mid - 1; }
        }
        l
    }
}