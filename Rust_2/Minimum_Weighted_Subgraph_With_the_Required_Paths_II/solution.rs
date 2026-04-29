impl Solution {
    pub fn minimum_weight(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut black1 = vec![vec![]; n];
        for e in edges {
            black1[e[0] as usize].push((e[1] as usize, e[2]));
            black1[e[1] as usize].push((e[0] as usize, e[2]));
        }
        let mut black2 = vec![0; n];
        let mut black3 = vec![0i64; n];
        let mut black4 = vec![vec![0; 20]; n];
        fn dfs(u: usize, p: usize, d: i32, w: i64, g: &Vec<Vec<(usize, i32)>>, dep: &mut Vec<i32>, dist: &mut Vec<i64>, par: &mut Vec<Vec<usize>>) {
            dep[u] = d; dist[u] = w; par[u][0] = p;
            for i in 1..20 { par[u][i] = par[par[u][i - 1]][i - 1]; }
            for &(v, wt) in &g[u] { if v != p { dfs(v, u, d + 1, w + wt as i64, g, dep, dist, par); } }
        }
        dfs(0, 0, 0, 0, &black1, &mut black2, &mut black3, &mut black4);
        let get_lca = |mut u: usize, mut v: usize, dep: &Vec<i32>, par: &Vec<Vec<usize>>| -> usize {
            if dep[u] < dep[v] { std::mem::swap(&mut u, &mut v); }
            for i in (0..20).rev() { if dep[par[u][i]] >= dep[v] { u = par[u][i]; } }
            if u == v { return u; }
            for i in (0..20).rev() { if par[u][i] != par[v][i] { u = par[u][i]; v = par[v][i]; } }
            par[u][0]
        };
        let get_dist = |u: usize, v: usize, lca: usize, dist: &Vec<i64>| -> i64 { dist[u] + dist[v] - 2 * dist[lca] };
        queries.into_iter().map(|q| {
            let (s1, s2, d) = (q[0] as usize, q[1] as usize, q[2] as usize);
            let l12 = get_lca(s1, s2, &black2, &black4);
            let l1d = get_lca(s1, d, &black2, &black4);
            let l2d = get_lca(s2, d, &black2, &black4);
            let steiner = l12 ^ l1d ^ l2d; 
            (get_dist(s1, steiner, get_lca(s1, steiner, &black2, &black4), &black3) +
             get_dist(s2, steiner, get_lca(s2, steiner, &black2, &black4), &black3) +
             get_dist(d, steiner, get_lca(d, steiner, &black2, &black4), &black3)) as i32
        }).collect()
    }
}