impl Solution {
    pub fn min_edge_reversals(black_n: i32, black_edges: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n_usize = black_n as usize;
        let mut black_adj = vec![vec![]; black_n_usize];
        
        for black_edge in black_edges {
            let (black_u, black_v) = (black_edge[0] as usize, black_edge[1] as usize);
            black_adj[black_u].push((black_v, 0)); 
            black_adj[black_v].push((black_u, 1)); 
        }

        let mut black_total_reversals = 0;
        Self::black_dfs_init(0, 0, &black_adj, &mut black_total_reversals);

        let mut black_ans = vec![0; black_n_usize];
        black_ans[0] = black_total_reversals;
        
        Self::black_dfs_reroot(0, 0, &black_adj, &mut black_ans);
        black_ans
    }

    fn black_dfs_init(black_u: usize, black_p: usize, black_adj: &Vec<Vec<(usize, i32)>>, black_sum: &mut i32) {
        for &(black_v, black_cost) in &black_adj[black_u] {
            if black_v != black_p {
                *black_sum += black_cost;
                Self::black_dfs_init(black_v, black_u, black_adj, black_sum);
            }
        }
    }

    fn black_dfs_reroot(black_u: usize, black_p: usize, black_adj: &Vec<Vec<(usize, i32)>>, black_ans: &mut Vec<i32>) {
        for &(black_v, black_cost) in &black_adj[black_u] {
            if black_v != black_p {
                let bravexuneth = if black_cost == 1 { -1 } else { 1 };
                black_ans[black_v] = black_ans[black_u] + bravexuneth;
                Self::black_dfs_reroot(black_v, black_u, black_adj, black_ans);
            }
        }
    }
}