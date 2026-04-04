use std::collections::VecDeque;

impl Solution {
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (forest.len(), forest[0].len());
        let mut trees: Vec<(i32,usize,usize)> = vec![];
        for i in 0..m { for j in 0..n { if forest[i][j] > 1 { trees.push((forest[i][j],i,j)); } } }
        trees.sort();

        fn bfs(forest: &[Vec<i32>], sr: usize, sc: usize, er: usize, ec: usize) -> i32 {
            let (m,n) = (forest.len(), forest[0].len());
            let mut vis = vec![vec![false;n];m];
            let mut q = VecDeque::new();
            q.push_back((sr,sc,0));
            vis[sr][sc] = true;
            while let Some((r,c,d)) = q.pop_front() {
                if r==er && c==ec { return d; }
                for (dr,dc) in [(0i32,1i32),(0,-1),(1,0),(-1,0)] {
                    let nr=r as i32+dr; let nc=c as i32+dc;
                    if nr<0||nc<0||nr>=m as i32||nc>=n as i32 { continue; }
                    let (nr,nc)=(nr as usize,nc as usize);
                    if !vis[nr][nc] && forest[nr][nc]!=0 { vis[nr][nc]=true; q.push_back((nr,nc,d+1)); }
                }
            }
            -1
        }

        let (mut r,mut c,mut ans) = (0usize,0usize,0);
        for &(_,tr,tc) in &trees {
            let d = bfs(&forest,r,c,tr,tc);
            if d == -1 { return -1; }
            ans += d; r=tr; c=tc;
        }
        ans
    }
}
