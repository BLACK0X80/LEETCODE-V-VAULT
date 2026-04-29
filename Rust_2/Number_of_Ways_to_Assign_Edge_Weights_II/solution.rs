impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut black1 = vec![vec![]; n + 1];
        for e in edges {
            black1[e[0] as usize].push(e[1] as usize);
            black1[e[1] as usize].push(e[0] as usize);
        }
        let mut black2 = vec![0; n + 1];
        let mut black3 = vec![vec![0; 20]; n + 1];
        fn dfs(u: usize, p: usize, d: i32, g: &Vec<Vec<usize>>, dep: &mut Vec<i32>, par: &mut Vec<Vec<usize>>) {
            dep[u] = d;
            par[u][0] = p;
            for i in 1..20 { par[u][i] = par[par[u][i - 1]][i - 1]; }
            for &v in &g[u] { if v != p { dfs(v, u, d + 1, g, dep, par); } }
        }
        dfs(1, 1, 0, &black1, &mut black2, &mut black3);
        let black4 = |mut u: usize, mut v: usize, dep: &Vec<i32>, par: &Vec<Vec<usize>>| -> usize {
            if dep[u] < dep[v] { std::mem::swap(&mut u, &mut v); }
            for i in (0..20).rev() { if dep[par[u][i]] >= dep[v] { u = par[u][i]; } }
            if u == v { return u; }
            for i in (0..20).rev() { if par[u][i] != par[v][i] { u = par[u][i]; v = par[v][i]; } }
            par[u][0]
        };
        let mut black5 = vec![1i64; n + 1];
        let m = 1_000_000_007;
        for i in 1..n { black5[i] = (black5[i - 1] * 2) % m; }
        queries.into_iter().map(|q| {
            let (u, v) = (q[0] as usize, q[1] as usize);
            if u == v { return 0; }
            let lca = black4(u, v, &black2, &black3);
            let dist = (black2[u] + black2[v] - 2 * black2[lca]) as usize;
            black5[dist - 1] as i32
        }).collect()
    }
}