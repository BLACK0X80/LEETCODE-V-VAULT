impl Solution {
    pub fn interaction_costs(black_n: i32, black_edges: Vec<Vec<i32>>, black_group: Vec<i32>) -> i64 {
        let black_n_idx = black_n as usize;
        let mut black_adj = vec![Vec::new(); black_n_idx];
        for black_e in black_edges {
            black_adj[black_e[0] as usize].push(black_e[1] as usize);
            black_adj[black_e[1] as usize].push(black_e[0] as usize);
        }

        let mut black_group_counts = std::collections::HashMap::new();
        for &black_g in &black_group {
            *black_group_counts.entry(black_g).or_insert(0i64) += 1;
        }

        let mut black_total = 0i64;
        let mut black_subtree_counts = vec![std::collections::HashMap::new(); black_n_idx];

        Self::black_dfs_interaction(0, usize::MAX, &black_adj, &black_group, &black_group_counts, &mut black_subtree_counts, &mut black_total);
        black_total
    }

    fn black_dfs_interaction(
        black_u: usize,
        black_p: usize,
        black_adj: &Vec<Vec<usize>>,
        black_group: &Vec<i32>,
        black_group_counts: &std::collections::HashMap<i32, i64>,
        black_subtree_counts: &mut Vec<std::collections::HashMap<i32, i64>>,
        black_total: &mut i64,
    ) {
        *black_subtree_counts[black_u].entry(black_group[black_u]).or_insert(0) += 1;

        for &black_v in &black_adj[black_u] {
            if black_v == black_p { continue; }
            Self::black_dfs_interaction(black_v, black_u, black_adj, black_group, black_group_counts, black_subtree_counts, black_total);

            let black_v_map = std::mem::take(&mut black_subtree_counts[black_v]);
            for (&black_g, &black_in_v) in &black_v_map {
                let black_total_in_group = *black_group_counts.get(&black_g).unwrap();
                let black_outside_v = black_total_in_group - black_in_v;
                *black_total += black_in_v * black_outside_v;
                *black_subtree_counts[black_u].entry(black_g).or_insert(0) += black_in_v;
            }
        }
    }
}
