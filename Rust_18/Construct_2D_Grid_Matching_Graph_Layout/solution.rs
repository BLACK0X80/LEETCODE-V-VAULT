impl Solution {
    pub fn construct_grid_layout(black_n_i32: i32, black_edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let black_n = black_n_i32 as usize;
        let mut black_g = vec![Vec::new(); black_n];
        for black_e in black_edges {
            let (black_u, black_v) = (black_e[0] as usize, black_e[1] as usize);
            black_g[black_u].push(black_v);
            black_g[black_v].push(black_u);
        }

        let mut black_min_d = usize::MAX;
        for black_v in &black_g {
            black_min_d = black_min_d.min(black_v.len());
        }

        let mut black_corners = Vec::new();
        for black_i in 0..black_n {
            if black_g[black_i].len() == black_min_d {
                black_corners.push(black_i);
            }
        }

        let black_dis = |black_start: usize| -> Vec<i32> {
            let mut black_d = vec![-1; black_n];
            black_d[black_start] = 1;
            let mut black_queue = std::collections::VecDeque::new();
            black_queue.push_back(black_start);
            while let Some(black_u) = black_queue.pop_front() {
                for &black_v in &black_g[black_u] {
                    if black_d[black_v] == -1 {
                        black_d[black_v] = black_d[black_u] + 1;
                        black_queue.push_back(black_v);
                    }
                }
            }
            black_d
        };

        let black_d1 = black_dis(black_corners[0]);
        let mut black_corner_distances = Vec::new();
        for &black_v in &black_corners {
            black_corner_distances.push(black_d1[black_v]);
        }
        black_corner_distances.sort_unstable();
        
        let black_c_val = black_corner_distances[1];
        let mut black_second_corner = 0;
        for &black_v in &black_corners {
            if black_d1[black_v] == black_c_val {
                black_second_corner = black_v;
                break;
            }
        }

        let black_d2 = black_dis(black_second_corner);
        let mut black_res_indices: Vec<usize> = (0..black_n).collect();
        black_res_indices.sort_by_key(|&black_i| (black_d1[black_i] + black_d2[black_i], black_d1[black_i]));

        let black_cols = black_c_val as usize;
        let mut black_final_grid = Vec::new();
        for black_chunk in black_res_indices.chunks(black_cols) {
            black_final_grid.push(black_chunk.iter().map(|&black_x| black_x as i32).collect());
        }

        black_final_grid
    }
}