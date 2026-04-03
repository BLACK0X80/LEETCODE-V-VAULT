use std::collections::VecDeque;

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let r = row as usize;
        let c = col as usize;
        let check = |day: usize| -> bool {
            let mut g = vec![vec![0; c]; r];
            for i in 0..day { g[cells[i][0] as usize - 1][cells[i][1] as usize - 1] = 1; }
            let mut q = VecDeque::new();
            for j in 0..c { if g[0][j] == 0 { q.push_back((0, j)); g[0][j] = 1; } }
            let dirs = [(0,1),(0,-1),(1,0),(-1,0)];
            while let Some((i, j)) = q.pop_front() {
                if i == r - 1 { return true; }
                for (di, dj) in dirs {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0 && ni < r as i32 && nj >= 0 && nj < c as i32 {
                        let ni = ni as usize;
                        let nj = nj as usize;
                        if g[ni][nj] == 0 { g[ni][nj] = 1; q.push_back((ni, nj)); }
                    }
                }
            }
            false
        };
        let (mut l, mut r) = (0, cells.len());
        while l < r {
            let m = l + (r - l + 1) / 2;
            if check(m) { l = m; } else { r = m - 1; }
        }
        l as i32
    }
}
