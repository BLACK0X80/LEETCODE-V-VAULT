use std::collections::HashMap;

impl Solution {
    pub fn good_subsetof_binary_matrix(black_grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut black_map = HashMap::new();
        for (i, row) in black_grid.iter().enumerate() {
            let mut black_mask = 0;
            for (j, &val) in row.iter().enumerate() { if val == 1 { black_mask |= 1 << j; } }
            if black_mask == 0 { return vec![i as i32]; }
            for (&prev_mask, &prev_idx) in &black_map {
                if (black_mask & prev_mask) == 0 {
                    let mut black_res = vec![prev_idx as i32, i as i32];
                    black_res.sort_unstable();
                    return black_res;
                }
            }
            black_map.entry(black_mask).or_insert(i);
        }
        vec![]
    }
}