impl Solution {
    pub fn placed_coins(black_edges: Vec<Vec<i32>>, black_cost: Vec<i32>) -> Vec<i64> {
        let black_n = black_cost.len();
        let mut black_adj = vec![vec![]; black_n];
        for e in black_edges {
            black_adj[e[0] as usize].push(e[1] as usize);
            black_adj[e[1] as usize].push(e[0] as usize);
        }

        let mut black_res = vec![0; black_n];

        fn black_dfs(u: usize, p: usize, adj: &Vec<Vec<usize>>, cost: &Vec<i32>, res: &mut Vec<i64>) -> Vec<i32> {
            let mut black_all = vec![cost[u]];
            let mut black_count = 1;

            for &v in &adj[u] {
                if v == p { continue; }
                let mut black_child_vals = black_dfs(v, u, adj, cost, res);
                black_count += black_child_vals.len(); 
                black_all.append(&mut black_child_vals);
            }

            black_all.sort_unstable();
            let black_sz = black_all.len();
            if black_sz < 3 {
                res[u] = 1;
            } else {
                let black_p1 = black_all[black_sz-1] as i64 * black_all[black_sz-2] as i64 * black_all[black_sz-3] as i64;
                let black_p2 = black_all[black_sz-1] as i64 * black_all[0] as i64 * black_all[1] as i64;
                res[u] = black_p1.max(black_p2).max(0);
            }

            if black_sz <= 5 {
                black_all
            } else {
                vec![black_all[0], black_all[1], black_all[black_sz-3], black_all[black_sz-2], black_all[black_sz-1]]
            }
        }

        black_dfs(0, black_n, &black_adj, &black_cost, &mut black_res);
        black_res
    }
}
