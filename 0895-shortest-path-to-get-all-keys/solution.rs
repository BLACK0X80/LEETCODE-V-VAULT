use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let g: Vec<&[u8]> = grid.iter().map(|s| s.as_bytes()).collect();
        let (m,n) = (g.len(), g[0].len());
        let mut start = (0,0);
        let mut num_keys = 0;
        for i in 0..m { for j in 0..n {
            if g[i][j]==b'@' { start=(i,j); }
            if g[i][j].is_ascii_lowercase() { num_keys+=1; }
        }}
        let all = (1<<num_keys)-1;
        let mut vis = vec![vec![vec![false;1<<num_keys];n];m];
        let mut q = VecDeque::new();
        q.push_back((start.0,start.1,0,0));
        vis[start.0][start.1][0]=true;

        while let Some((r,c,keys,dist)) = q.pop_front() {
            for (dr,dc) in [(0i32,1i32),(0,-1),(1,0),(-1,0)] {
                let nr=r as i32+dr; let nc=c as i32+dc;
                if nr<0||nc<0||nr>=m as i32||nc>=n as i32{continue;}
                let (nr,nc)=(nr as usize,nc as usize);
                let cell=g[nr][nc];
                if cell==b'#' {continue;}
                if cell.is_ascii_uppercase() && keys&(1<<(cell-b'A'))==0 {continue;}
                let mut nk=keys;
                if cell.is_ascii_lowercase() { nk|=1<<(cell-b'a'); }
                if nk==all { return dist+1; }
                if !vis[nr][nc][nk] { vis[nr][nc][nk]=true; q.push_back((nr,nc,nk,dist+1)); }
            }
        }
        -1
    }
}
