impl Solution {
    pub fn maximize_sum_of_weights(black_edges: Vec<Vec<i32>>, black_k: i32) -> i64 {
        let black_n = black_edges.len() + 1;
        let mut black_adj = vec![vec![]; black_n];
        for e in black_edges {
            black_adj[e[0] as usize].push((e[1] as usize, e[2] as i64));
            black_adj[e[1] as usize].push((e[0] as usize, e[2] as i64));
        }

        fn black_dfs(u: usize, p: usize, k: i32, adj: &Vec<Vec<(usize, i64)>>) -> (i64, i64) {
            let mut black_base_sum = 0;
            let mut black_diffs = vec![];

            for &(v, w) in &adj[u] {
                if v == p { continue; }
                let (black_v_k, black_v_k1) = black_dfs(v, u, k, adj);
                black_base_sum += black_v_k;
                let black_gain = w + black_v_k1 - black_v_k;
                if black_gain > 0 { black_diffs.push(black_gain); }
            }

            black_diffs.sort_unstable_by(|a, b| b.cmp(a));
            let black_d_sum: i64 = black_diffs.iter().take(k as usize).sum();
            let black_d_sum_minus: i64 = black_diffs.iter().take(k as usize - 1).sum();

            (black_base_sum + black_d_sum, black_base_sum + black_d_sum_minus)
        }

        black_dfs(0, 0, black_k, &black_adj).0
    }
}
