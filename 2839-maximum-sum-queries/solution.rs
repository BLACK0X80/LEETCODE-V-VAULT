impl Solution {
    pub fn maximum_sum_queries(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut black_pairs: Vec<(i32, i32)> = nums1.into_iter().zip(nums2.into_iter()).collect();
        black_pairs.sort_by(|a, b| b.0.cmp(&a.0));
        
        let mut black_y_coords: Vec<i32> = black_pairs.iter().map(|p| p.1).collect();
        for q in &queries { black_y_coords.push(q[1]); }
        black_y_coords.sort_unstable();
        black_y_coords.dedup();

        let mut black_q_indices: Vec<usize> = (0..queries.len()).collect();
        black_q_indices.sort_by(|&a, &b| queries[b][0].cmp(&queries[a][0]));

        let mut black_bit = vec![-1; black_y_coords.len() + 1];
        let mut black_ans = vec![-1; queries.len()];
        let mut black_p_idx = 0;

        for &black_qi in &black_q_indices {
            let black_qx = queries[black_qi][0];
            let black_qy = queries[black_qi][1];
            while black_p_idx < black_pairs.len() && black_pairs[black_p_idx].0 >= black_qx {
                let (black_x, black_y) = black_pairs[black_p_idx];
                let mut black_idx = black_y_coords.len() - black_y_coords.binary_search(&black_y).unwrap();
                while black_idx < black_bit.len() {
                    black_bit[black_idx] = black_bit[black_idx].max(black_x + black_y);
                    black_idx += (black_idx as i32 & -(black_idx as i32)) as usize;
                }
                black_p_idx += 1;
            }
            let mut black_idx = black_y_coords.len() - black_y_coords.binary_search(&black_qy).unwrap();
            let mut black_max_val = -1;
            while black_idx > 0 {
                black_max_val = black_max_val.max(black_bit[black_idx]);
                black_idx -= (black_idx as i32 & -(black_idx as i32)) as usize;
            }
            black_ans[black_qi] = black_max_val;
        }
        black_ans
    }
}
