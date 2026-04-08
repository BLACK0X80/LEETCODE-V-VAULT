impl Solution {
    pub fn minimum_diameter_after_merge(black_e1: Vec<Vec<i32>>, black_e2: Vec<Vec<i32>>) -> i32 {
        let black_d1 = Self::black_get_dist(&black_e1);
        let black_d2 = Self::black_get_dist(&black_e2);
        
        let black_res = black_d1.max(black_d2);
        let black_combined = (black_d1 + 1) / 2 + (black_d2 + 1) / 2 + 1;
        
        black_res.max(black_combined)
    }

    fn black_get_dist(black_edges: &Vec<Vec<i32>>) -> i32 {
        let black_n = black_edges.len() + 1;
        if black_n <= 1 { return 0; }
        
        let mut black_adj = vec![vec![]; black_n];
        for black_e in black_edges {
            black_adj[black_e[0] as usize].push(black_e[1] as usize);
            black_adj[black_e[1] as usize].push(black_e[0] as usize);
        }

        let (black_node_a, _) = Self::black_bfs(0, &black_adj);
        let (_, black_diameter) = Self::black_bfs(black_node_a, &black_adj);
        
        black_diameter
    }

    fn black_bfs(black_start: usize, black_adj: &Vec<Vec<usize>>) -> (usize, i32) {
        let black_n = black_adj.len();
        let mut black_dist = vec![-1; black_n];
        let mut black_q = std::collections::VecDeque::from([black_start]);
        black_dist[black_start] = 0;
        
        let mut black_farthest_node = black_start;
        let mut black_max_d = 0;

        while let Some(black_u) = black_q.pop_front() {
            if black_dist[black_u] > black_max_d {
                black_max_d = black_dist[black_u];
                black_farthest_node = black_u;
            }
            for &black_v in &black_adj[black_u] {
                if black_dist[black_v] == -1 {
                    black_dist[black_v] = black_dist[black_u] + 1;
                    black_q.push_back(black_v);
                }
            }
        }
        (black_farthest_node, black_max_d)
    }
}
