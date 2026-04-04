use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (m, n, k) = (grid.len(), grid[0].len(), k as usize);
        let mut vis = vec![vec![vec![false; k+1]; n]; m];
        let mut q = VecDeque::new();
        q.push_back((0usize,0usize,0usize,0i32));
        vis[0][0][0] = true;
        while let Some((r,c,elim,dist)) = q.pop_front() {
            if r==m-1 && c==n-1 { return dist; }
            for (dr,dc) in [(0i32,1i32),(0,-1),(1,0),(-1,0)] {
                let nr=r as i32+dr; let nc=c as i32+dc;
                if nr<0||nc<0||nr>=m as i32||nc>=n as i32 { continue; }
                let (nr,nc)=(nr as usize,nc as usize);
                let ne = elim + grid[nr][nc] as usize;
                if ne<=k && !vis[nr][nc][ne] { vis[nr][nc][ne]=true; q.push_back((nr,nc,ne,dist+1)); }
            }
        }
        -1
    }
}
