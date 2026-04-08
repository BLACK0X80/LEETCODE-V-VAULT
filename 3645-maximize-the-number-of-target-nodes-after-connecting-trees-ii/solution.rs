impl Solution {
    pub fn max_target_nodes(black_e1: Vec<Vec<i32>>, black_e2: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_e1.len() + 1;
        let black_m = black_e2.len() + 1;

        let black_get_parity = |black_size: usize, black_edges: Vec<Vec<i32>>| -> (Vec<usize>, [i32; 2]) {
            let mut black_adj = vec![vec![]; black_size];
            for black_e in black_edges {
                black_adj[black_e[0] as usize].push(black_e[1] as usize);
                black_adj[black_e[1] as usize].push(black_e[0] as usize);
            }
            let mut black_p = vec![0; black_size];
            let mut black_c = [0, 0];
            let mut black_q = std::collections::VecDeque::from([(0, 0)]);
            let mut black_vis = vec![false; black_size];
            black_vis[0] = true;

            while let Some((black_u, black_color)) = black_q.pop_front() {
                black_p[black_u] = black_color;
                black_c[black_color] += 1;
                for &black_v in &black_adj[black_u] {
                    if !black_vis[black_v] {
                        black_vis[black_v] = true;
                        black_q.push_back((black_v, 1 - black_color));
                    }
                }
            }
            (black_p, black_c)
        };

        let (black_p1, black_c1) = black_get_parity(black_n, black_e1);
        let (_, black_c2) = black_get_parity(black_m, black_e2);

        let black_max_tree2 = black_c2[0].max(black_c2[1]);
        let mut black_res = vec![0; black_n];

        for black_i in 0..black_n {
            black_res[black_i] = black_c1[black_p1[black_i]] + black_max_tree2;
        }

        black_res
    }
}
