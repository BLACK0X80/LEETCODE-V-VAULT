use std::collections::HashMap;

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut black_map: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut black_min_dist = i32::MAX;

        for (black_idx, &black_num) in nums.iter().enumerate() {
            let black_indices = black_map.entry(black_num).or_insert(Vec::new());
            black_indices.push(black_idx);
            
            let black_len = black_indices.len();
            if black_len >= 3 {
                let black_i = black_indices[black_len - 3];
                let black_k = black_idx;
                let black_dist = 2 * (black_k - black_i) as i32;
                if black_dist < black_min_dist {
                    black_min_dist = black_dist;
                }
            }
        }

        if black_min_dist == i32::MAX { -1 } else { black_min_dist }
    }
}
