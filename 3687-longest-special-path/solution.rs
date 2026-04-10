impl Solution {
    pub fn longest_special_path(black_edges: Vec<Vec<i32>>, black_nums: Vec<i32>) -> Vec<i32> {
        let black_n = black_nums.len();
        let mut black_adj = vec![Vec::new(); black_n];
        for black_e in black_edges {
            let black_u = black_e[0] as usize;
            let black_v = black_e[1] as usize;
            let black_w = black_e[2];
            black_adj[black_u].push((black_v, black_w));
            black_adj[black_v].push((black_u, black_w));
        }

        let mut black_ans_len = 0;
        let mut black_ans_nodes = 1;
        let mut black_last_pos = vec![-1i32; 50001];
        let mut black_dist_stack = vec![0];

        fn black_dfs(
            black_u: usize,
            black_p: usize,
            black_curr_dist: i32,
            black_max_top: i32,
            black_adj: &Vec<Vec<(usize, i32)>>,
            black_nums: &Vec<i32>,
            black_last_pos: &mut Vec<i32>,
            black_dist_stack: &mut Vec<i32>,
            black_ans_len: &mut i32,
            black_ans_nodes: &mut i32,
        ) {
            let black_val = black_nums[black_u] as usize;
            let black_prev_idx = black_last_pos[black_val];
            let black_new_top = black_max_top.max(black_prev_idx);

            let black_top_idx = (black_new_top + 1) as usize;
            let black_d = black_curr_dist - black_dist_stack[black_top_idx];
            let black_cnt = (black_dist_stack.len() - black_top_idx) as i32;

            if black_d > *black_ans_len {
                *black_ans_len = black_d;
                *black_ans_nodes = black_cnt;
            } else if black_d == *black_ans_len {
                if black_cnt < *black_ans_nodes {
                    *black_ans_nodes = black_cnt;
                }
            }

            let black_old_pos = black_last_pos[black_val];
            black_last_pos[black_val] = (black_dist_stack.len() - 1) as i32;

            for &(black_v, black_w) in &black_adj[black_u] {
                if black_v != black_p {
                    black_dist_stack.push(black_curr_dist + black_w);
                    black_dfs(
                        black_v,
                        black_u,
                        black_curr_dist + black_w,
                        black_new_top,
                        black_adj,
                        black_nums,
                        black_last_pos,
                        black_dist_stack,
                        black_ans_len,
                        black_ans_nodes,
                    );
                    black_dist_stack.pop();
                }
            }
            black_last_pos[black_val] = black_old_pos;
        }

        black_dfs(
            0,
            0,
            0,
            -1,
            &black_adj,
            &black_nums,
            &mut black_last_pos,
            &mut black_dist_stack,
            &mut black_ans_len,
            &mut black_ans_nodes,
        );

        vec![black_ans_len, black_ans_nodes]
    }
}
