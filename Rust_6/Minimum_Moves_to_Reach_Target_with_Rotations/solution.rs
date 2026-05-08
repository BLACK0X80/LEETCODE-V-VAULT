use std::collections::VecDeque;

impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut vis = vec![vec![vec![false;2];n];n];
        let mut q = VecDeque::new();
        q.push_back((0usize,0usize,0usize,0i32));
        vis[0][0][0]=true;
        while let Some((r,c,dir,dist)) = q.pop_front() {
            if r==n-1 && c==n-2 && dir==0 { return dist; }
            if dir==0 {
                if c+2<n && grid[r][c+2]==0 && !vis[r][c+1][0] { vis[r][c+1][0]=true; q.push_back((r,c+1,0,dist+1)); }
                if r+1<n && grid[r+1][c]==0 && grid[r+1][c+1]==0 {
                    if !vis[r+1][c][0] { vis[r+1][c][0]=true; q.push_back((r+1,c,0,dist+1)); }
                    if !vis[r][c][1] { vis[r][c][1]=true; q.push_back((r,c,1,dist+1)); }
                }
            } else {
                if r+2<n && grid[r+2][c]==0 && !vis[r+1][c][1] { vis[r+1][c][1]=true; q.push_back((r+1,c,1,dist+1)); }
                if c+1<n && grid[r][c+1]==0 && grid[r+1][c+1]==0 {
                    if !vis[r][c+1][1] { vis[r][c+1][1]=true; q.push_back((r,c+1,1,dist+1)); }
                    if !vis[r][c][0] { vis[r][c][0]=true; q.push_back((r,c,0,dist+1)); }
                }
            }
        }
        -1
    }
}