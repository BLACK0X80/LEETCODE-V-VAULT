impl Solution {
    pub fn max_len(n: i32, edges: Vec<Vec<i32>>, label: String) -> i32 {
        let n = n as usize;
        let lb = label.as_bytes();
        let mut adj = vec![vec![]; n];
        for e in &edges {
            adj[e[0] as usize].push(e[1] as usize);
            adj[e[1] as usize].push(e[0] as usize);
        }
        let mut dp = [[[false; 15]; 15]; 16384];
        let mut res = 1;
        fn dfs(i: usize, j: usize, adj: &[Vec<usize>], lb: &[u8], mask: u32, vis: &mut [bool], dp: &mut [[[bool; 15]; 15]; 16384], res: &mut i32) {
            let nm = mask | (1 << i) | (1 << j);
            if dp[nm as usize][i][j] { return; }
            dp[nm as usize][i][j] = true;
            vis[i] = true; vis[j] = true;
            *res = (*res).max(nm.count_ones() as i32);
            for &x in &adj[i] {
                if !vis[x] {
                    for &y in &adj[j] {
                        if !vis[y] && x != y && lb[x] == lb[y] {
                            dfs(x, y, adj, lb, nm, vis, dp, res);
                        }
                    }
                }
            }
            vis[i] = false; vis[j] = false;
        }
        for i in 0..n {
            let mut vis = vec![false; n];
            dfs(i, i, &adj, lb, 0, &mut vis, &mut dp, &mut res);
            for &j in &adj[i] {
                if i < j && lb[i] == lb[j] {
                    let mut vis = vec![false; n];
                    dfs(i, j, &adj, lb, 0, &mut vis, &mut dp, &mut res);
                }
            }
        }
        res
    }
}
