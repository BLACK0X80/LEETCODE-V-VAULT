use std::collections::VecDeque;

impl Solution {
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());
        let start: u32 = mat.iter().enumerate().flat_map(|(i,r)| r.iter().enumerate().map(move |(j,&v)| v as u32 * (1<<(i*n+j)))).sum();
        if start == 0 { return 0; }
        let mut vis = std::collections::HashSet::new();
        let mut q = VecDeque::new();
        q.push_back((start, 0));
        vis.insert(start);
        while let Some((state, steps)) = q.pop_front() {
            for i in 0..m { for j in 0..n {
                let mut ns = state ^ (1<<(i*n+j));
                for (dr,dc) in [(0i32,1i32),(0,-1),(1,0),(-1,0)] {
                    let nr=i as i32+dr; let nc=j as i32+dc;
                    if nr>=0&&nc>=0&&(nr as usize)<m&&(nc as usize)<n { ns^=1<<(nr as usize*n+nc as usize); }
                }
                if ns==0 { return steps+1; }
                if vis.insert(ns) { q.push_back((ns,steps+1)); }
            }}
        }
        -1
    }
}