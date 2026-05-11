use std::collections::VecDeque;

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let n = colors.len();
        let b = colors.as_bytes();
        let mut g = vec![vec![]; n];
        let mut ind = vec![0usize; n];
        for e in &edges { g[e[0] as usize].push(e[1] as usize); ind[e[1] as usize]+=1; }
        let mut dp = vec![[0u32;26]; n];
        let mut q: VecDeque<usize> = (0..n).filter(|&i| ind[i]==0).collect();
        let mut processed = 0;
        let mut ans = 0u32;
        while let Some(u) = q.pop_front() {
            processed+=1;
            dp[u][(b[u]-b'a') as usize]+=1;
            ans=ans.max(dp[u].iter().copied().max().unwrap());
            for &v in &g[u] {
                for c in 0..26 { dp[v][c]=dp[v][c].max(dp[u][c]); }
                ind[v]-=1; if ind[v]==0 { q.push_back(v); }
            }
        }
        if processed<n { -1 } else { ans as i32 }
    }
}