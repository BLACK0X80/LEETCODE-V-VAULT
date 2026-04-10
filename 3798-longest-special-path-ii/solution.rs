use std::collections::HashMap;

impl Solution {
    pub fn longest_special_path(black_edges: Vec<Vec<i32>>, black_nums: Vec<i32>) -> Vec<i32> {
        let black_n = black_nums.len();
        let mut black_graph = vec![Vec::new(); black_n];
        for black_e in black_edges {
            black_graph[black_e[0] as usize].push((black_e[1] as usize, black_e[2]));
            black_graph[black_e[1] as usize].push((black_e[0] as usize, black_e[2]));
        }

        let mut black_res = vec![0, 1];
        let mut black_cur_path = Vec::new();
        let mut black_last = HashMap::new();

        Self::dfs(
            0,
            0,
            usize::MAX,
            &vec![0, 0],
            &black_graph,
            &black_nums,
            &mut black_cur_path,
            &mut black_last,
            &mut black_res,
        );

        black_res
    }

    fn dfs(
        black_node: usize,
        black_curr_cost: i32,
        black_prev: usize,
        black_seen: &Vec<usize>,
        black_graph: &Vec<Vec<(usize, i32)>>,
        black_nums: &Vec<i32>,
        black_cur_path: &mut Vec<i32>,
        black_last: &mut HashMap<i32, i32>,
        black_res: &mut Vec<i32>,
    ) {
        let black_color = black_nums[black_node];
        let black_last_color_idx = *black_last.get(&black_color).unwrap_or(&-1);
        
        black_last.insert(black_color, black_cur_path.len() as i32);
        black_cur_path.push(black_curr_cost);

        let black_start = black_seen[0];
        let black_path_len = black_curr_cost - black_cur_path[black_start];
        let black_node_count = (black_cur_path.len() - black_start) as i32;

        if black_path_len > black_res[0] {
            black_res[0] = black_path_len;
            black_res[1] = black_node_count;
        } else if black_path_len == black_res[0] {
            black_res[1] = black_res[1].min(black_node_count);
        }

        for &(black_next_node, black_next_cost) in &black_graph[black_node] {
            if black_next_node == black_prev {
                continue;
            }

            let mut black_next_seen = black_seen.clone();
            let black_next_color = black_nums[black_next_node];
            let black_found_idx = *black_last.get(&black_next_color).unwrap_or(&-1);

            if black_found_idx != -1 && (black_start as i32) <= black_found_idx {
                black_next_seen.push((black_found_idx + 1) as usize);
                black_next_seen.sort_unstable();
                if black_next_seen.len() > 2 {
                    black_next_seen.remove(0);
                }
            }

            Self::dfs(
                black_next_node,
                black_curr_cost + black_next_cost,
                black_node,
                &black_next_seen,
                black_graph,
                black_nums,
                black_cur_path,
                black_last,
                black_res,
            );
        }

        black_last.insert(black_color, black_last_color_idx);
        black_cur_path.pop();
    }
}
