use std::collections::HashMap;

impl Solution {
    pub fn max_frequency(black_nums: Vec<i32>, black_k: i32, black_num_ops: i32) -> i32 {
        let mut black_nums = black_nums;
        black_nums.sort_unstable();
        
        let black_k = black_k as i64;
        let black_num_ops = black_num_ops as i32;
        let mut black_map = HashMap::new();
        let mut black_points = std::collections::BTreeSet::new();

        for &black_x in &black_nums {
            *black_map.entry(black_x).or_insert(0) += 1;
            black_points.insert(black_x as i64);
            black_points.insert(black_x as i64 - black_k);
            black_points.insert(black_x as i64 + black_k);
        }

        let mut black_diff = std::collections::BTreeMap::new();
        for &black_x in &black_nums {
            let black_start = black_x as i64 - black_k;
            let black_end = black_x as i64 + black_k;
            *black_diff.entry(black_start).or_insert(0) += 1;
            *black_diff.entry(black_end + 1).or_insert(0) -= 1;
        }

        let mut black_ans = 0;
        let mut black_curr_coverage = 0;
        let black_diff_vec: Vec<_> = black_diff.into_iter().collect();
        let mut black_d_idx = 0;

        for black_p in black_points {
            while black_d_idx < black_diff_vec.len() && black_diff_vec[black_d_idx].0 <= black_p {
                black_curr_coverage += black_diff_vec[black_d_idx].1;
                black_d_idx += 1;
            }

            let black_count = *black_map.get(&(black_p as i32)).unwrap_or(&0);
            let black_others = black_curr_coverage - black_count;
            let black_res = black_count + black_others.min(black_num_ops);
            
            if black_res > black_ans {
                black_ans = black_res;
            }
        }

        black_ans
    }
}