impl Solution {
    pub fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
        let n = values.len();
        let mut adj = vec![vec![]; n];
        for e in &edges {
            adj[e[0] as usize].push((e[1] as usize, e[2]));
            adj[e[1] as usize].push((e[0] as usize, e[2]));
        }
        let mut black = 0;
        let mut vis = vec![false; n];
        vis[0] = true;
        Self::dfs(0, 0, values[0], max_time, &adj, &values, &mut vis, &mut black);
        black
    }
    fn dfs(u: usize, time: i32, score: i32, limit: i32, adj: &[Vec<(usize, i32)>], vals: &[i32], vis: &mut Vec<bool>, ans: &mut i32) {
        if u == 0 && score > *ans { *ans = score; }
        for &(v, t) in &adj[u] {
            if time + t <= limit {
                let first = !vis[v];
                if first { vis[v] = true; }
                Self::dfs(v, time + t, if first { score + vals[v] } else { score }, limit, adj, vals, vis, ans);
                if first { vis[v] = false; }
            }
        }
    }
}