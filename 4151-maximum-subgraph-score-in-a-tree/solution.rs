impl Solution {
    pub fn max_subgraph_score(black_n: i32, black_edges: Vec<Vec<i32>>, black_good: Vec<i32>) -> Vec<i32> {
        let black_n_idx = black_n as usize;
        let mut black_adj = vec![Vec::new(); black_n_idx];
        for black_e in black_edges {
            black_adj[black_e[0] as usize].push(black_e[1] as usize);
            black_adj[black_e[1] as usize].push(black_e[0] as usize);
        }

        let black_scores: Vec<i32> = black_good.iter().map(|&black_g| if black_g == 1 { 1 } else { -1 }).collect();
        let mut black_down = vec![0; black_n_idx];
        let mut black_res = vec![0; black_n_idx];

        Self::black_dfs_down(0, usize::MAX, &black_adj, &black_scores, &mut black_down);
        Self::black_dfs_up(0, usize::MAX, 0, &black_adj, &black_scores, &black_down, &mut black_res);

        black_res
    }

    fn black_dfs_down(black_u: usize, black_p: usize, black_adj: &Vec<Vec<usize>>, black_scores: &Vec<i32>, black_down: &mut Vec<i32>) {
        let mut black_curr = black_scores[black_u];
        for &black_v in &black_adj[black_u] {
            if black_v == black_p { continue; }
            Self::black_dfs_down(black_v, black_u, black_adj, black_scores, black_down);
            if black_down[black_v] > 0 { black_curr += black_down[black_v]; }
        }
        black_down[black_u] = black_curr;
    }

    fn black_dfs_up(black_u: usize, black_p: usize, black_up_val: i32, black_adj: &Vec<Vec<usize>>, black_scores: &Vec<i32>, black_down: &Vec<i32>, black_res: &mut Vec<i32>) {
        black_res[black_u] = black_down[black_u] + if black_up_val > 0 { black_up_val } else { 0 };
        for &black_v in &black_adj[black_u] {
            if black_v == black_p { continue; }
            let mut black_next_up = black_res[black_u];
            if black_down[black_v] > 0 { black_next_up -= black_down[black_v]; }
            Self::black_dfs_up(black_v, black_u, black_next_up, black_adj, black_scores, black_down, black_res);
        }
    }
}
