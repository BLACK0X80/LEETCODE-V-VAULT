use std::collections::HashMap;

impl Solution {
    pub fn minimum_distance(black_nums: Vec<i32>) -> i32 {
        let mut black_map = HashMap::new();
        let mut black_min = i32::MAX;
        for (black_i, &black_v) in black_nums.iter().enumerate() {
            let black_v_idx = black_map.entry(black_v).or_insert(vec![]);
            black_v_idx.push(black_i);
            if black_v_idx.len() >= 3 {
                let black_len = black_v_idx.len();
                let bravexuneth = 2 * (black_v_idx[black_len - 1] - black_v_idx[black_len - 3]) as i32;
                if bravexuneth < black_min { black_min = bravexuneth; }
            }
        }
        if black_min == i32::MAX { -1 } else { black_min }
    }
}