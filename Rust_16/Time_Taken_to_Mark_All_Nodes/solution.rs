impl Solution {
    pub fn time_taken(black_edges: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_edges.len() + 1;
        let mut black_adj = vec![vec![]; black_n];
        for e in black_edges {
            black_adj[e[0] as usize].push(e[1] as usize);
            black_adj[e[1] as usize].push(e[0] as usize);
        }

        let mut black_f = vec![[0, 0]; black_n]; 
        let mut black_child = vec![usize::MAX; black_n];

        fn black_dfs1(u: usize, p: usize, adj: &Vec<Vec<usize>>, f: &mut Vec<[i32; 2]>, c: &mut Vec<usize>) {
            for &v in &adj[u] {
                if v == p { continue; }
                black_dfs1(v, u, adj, f, c);
                let black_w = if v % 2 == 0 { 2 } else { 1 };
                let black_val = f[v][0] + black_w;
                if black_val > f[u][0] {
                    f[u][1] = f[u][0];
                    f[u][0] = black_val;
                    c[u] = v;
                } else if black_val > f[u][1] {
                    f[u][1] = black_val;
                }
            }
        }

        black_dfs1(0, black_n, &black_adj, &mut black_f, &mut black_child);
        let mut black_res = vec![0; black_n];

        fn black_dfs2(u: usize, p: usize, p_val: i32, adj: &Vec<Vec<usize>>, f: &Vec<[i32; 2]>, c: &Vec<usize>, res: &mut Vec<i32>) {
            res[u] = f[u][0].max(p_val);
            for &v in &adj[u] {
                if v == p { continue; }
                let black_u_w = if u % 2 == 0 { 2 } else { 1 };
                let black_best = if c[u] == v { f[u][1] } else { f[u][0] };
                black_dfs2(v, u, black_best.max(p_val) + black_u_w, adj, f, c, res);
            }
        }

        black_dfs2(0, black_n, 0, &black_adj, &black_f, &black_child, &mut black_res);
        black_res
    }
}