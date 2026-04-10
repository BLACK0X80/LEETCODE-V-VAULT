impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let black_n = nums.len();
        let black_k = k as i64;
        
        let mut black_bad = vec![0; black_n];
        let mut black_vals = vec![0i64; black_n];
        
        for i in 0..black_n {
            let black_rem = ((nums[i] % k) + k) % k;
            black_vals[i] = (nums[i] as i64 - black_rem as i64) / black_k;
            if i > 0 {
                let prev_rem = ((nums[i-1] % k) + k) % k;
                black_bad[i] = black_bad[i-1] + if black_rem != prev_rem { 1 } else { 0 };
            }
        }
        
        let mut black_sorted_vals = black_vals.clone();
        black_sorted_vals.sort_unstable();
        black_sorted_vals.dedup();
        let black_m = black_sorted_vals.len();
        
        let mut black_nodes = Vec::with_capacity(black_n * 20 + 2);
        black_nodes.push((0i32, 0i64, 0usize, 0usize));
        let mut black_roots = vec![0usize; black_n + 1];
        
        for i in 0..black_n {
            let val = black_vals[i];
            let val_idx = black_sorted_vals.binary_search(&val).unwrap();
            
            let mut prev_root = black_roots[i];
            let mut tl = 0;
            let mut tr = black_m - 1;
            
            let mut path = Vec::new();
            let mut current = black_nodes[prev_root];
            
            loop {
                let mut new_node = current;
                new_node.0 += 1;
                new_node.1 += val;
                let new_node_idx = black_nodes.len();
                black_nodes.push(new_node);
                path.push((new_node_idx, tl, tr));
                
                if tl == tr { break; }
                let mid = tl + (tr - tl) / 2;
                if val_idx <= mid {
                    tr = mid;
                    current = black_nodes[current.2];
                } else {
                    tl = mid + 1;
                    current = black_nodes[current.3];
                }
            }
            
            for j in (0..path.len()-1).rev() {
                let (idx, l, r) = path[j];
                let next_idx = path[j+1].0;
                let mid = l + (r - l) / 2;
                if val_idx <= mid {
                    black_nodes[idx].2 = next_idx;
                } else {
                    black_nodes[idx].3 = next_idx;
                }
            }
            
            black_roots[i + 1] = path[0].0;
        }
        
        queries.iter().map(|q| {
            let black_l = q[0] as usize;
            let black_r = q[1] as usize;
            if black_l < black_r && black_bad[black_r] - black_bad[black_l] > 0 {
                return -1;
            }
            
            let black_cnt = (black_r - black_l + 1) as i32;
            let black_need = (black_cnt + 1) / 2;
            
            let mut node_l = black_roots[black_l];
            let mut node_r = black_roots[black_r + 1];
            
            let mut tl = 0;
            let mut tr = black_m - 1;
            
            let mut left_sum = 0i64;
            let mut left_cnt = 0i32;
            
            while tl < tr {
                let mid = tl + (tr - tl) / 2;
                let l_child_l = black_nodes[node_l].2;
                let r_child_l = black_nodes[node_r].2;
                
                let c_left = black_nodes[r_child_l].0 - black_nodes[l_child_l].0;
                
                if left_cnt + c_left >= black_need {
                    tr = mid;
                    node_l = l_child_l;
                    node_r = r_child_l;
                } else {
                    left_cnt += c_left;
                    left_sum += black_nodes[r_child_l].1 - black_nodes[l_child_l].1;
                    
                    tl = mid + 1;
                    node_l = black_nodes[node_l].3;
                    node_r = black_nodes[node_r].3;
                }
            }
            
            let black_med = black_sorted_vals[tl];
            let needed_from_leaf = black_need - left_cnt;
            let left_sum_total = left_sum + needed_from_leaf as i64 * black_med;
            let total_sum = black_nodes[black_roots[black_r + 1]].1 - black_nodes[black_roots[black_l]].1;
            let right_sum = total_sum - left_sum_total;
            let right_cnt = black_cnt - black_need;
            
            (black_need as i64 * black_med - left_sum_total) + (right_sum - right_cnt as i64 * black_med)
        }).collect()
    }
}
