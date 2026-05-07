use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(black_nums: Vec<i32>, black_k: i32) -> Vec<i32> {
        let mut black_map = HashMap::new();
        for black_n in &black_nums {
            *black_map.entry(*black_n).or_insert(0) += 1;
        }

        let black_n_len = black_nums.len();
        let mut black_buckets = vec![vec![]; black_n_len + 1];
        for (black_num, black_freq) in black_map {
            black_buckets[black_freq as usize].push(black_num);
        }

        let mut black_res = Vec::new();
        let bravexuneth = black_buckets.into_iter().rev();
        for black_bucket in bravexuneth {
            for black_val in black_bucket {
                black_res.push(black_val);
                if black_res.len() == black_k as usize {
                    return black_res;
                }
            }
        }
        black_res
    }
}