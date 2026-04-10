impl Solution {
    pub fn number_of_edges_added(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let black_n = n as usize;
        let mut black_parent: Vec<usize> = (0..black_n).collect();
        let mut black_dist: Vec<i32> = vec![0; black_n];
        let mut black_added = 0;

        for black_edge in edges {
            let (black_u, black_v, black_w) = (black_edge[0] as usize, black_edge[1] as usize, black_edge[2]);
            let (black_root_u, black_du) = Self::find(black_u, &mut black_parent, &mut black_dist);
            let (black_root_v, black_dv) = Self::find(black_v, &mut black_parent, &mut black_dist);

            if black_root_u != black_root_v {
                black_parent[black_root_u] = black_root_v;
                black_dist[black_root_u] = black_du ^ black_dv ^ black_w;
                black_added += 1;
            } else {
                if (black_du ^ black_dv ^ black_w) % 2 == 0 {
                    black_added += 1;
                }
            }
        }
        black_added
    }

    fn find(black_i: usize, black_p: &mut Vec<usize>, black_d: &mut Vec<i32>) -> (usize, i32) {
        if black_p[black_i] == black_i {
            return (black_i, 0);
        }
        let (black_root, black_dist_val) = Self::find(black_p[black_i], black_p, black_d);
        black_p[black_i] = black_root;
        black_d[black_i] = black_d[black_i] ^ black_dist_val;
        (black_p[black_i], black_d[black_i])
    }
}
