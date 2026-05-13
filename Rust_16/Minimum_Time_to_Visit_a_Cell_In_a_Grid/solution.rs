use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Black {
    time: i32,
    r: usize,
    c: usize,
}
impl Ord for Black {
    fn cmp(&self, other: &Self) -> Ordering { other.time.cmp(&self.time) }
}
impl PartialOrd for Black {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][1] > 1 && grid[1][0] > 1 { return -1; }
        let m = grid.len();
        let n = grid[0].len();
        let mut black_dist = vec![vec![i32::MAX; n]; m];
        let mut black_pq = BinaryHeap::new();
        black_dist[0][0] = 0;
        black_pq.push(Black { time: 0, r: 0, c: 0 });
        let black_dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        while let Some(Black { time, r, c }) = black_pq.pop() {
            if time > black_dist[r][c] { continue; }
            if r == m - 1 && c == n - 1 { return time; }
            for (dr, dc) in black_dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nc >= 0 && nr < m as i32 && nc < n as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    let mut nt = time + 1;
                    if nt < grid[nr][nc] {
                        let diff = grid[nr][nc] - nt;
                        nt += diff + diff % 2;
                    }
                    if nt < black_dist[nr][nc] {
                        black_dist[nr][nc] = nt;
                        black_pq.push(Black { time: nt, r: nr, c: nc });
                    }
                }
            }
        }
        -1
    }
}