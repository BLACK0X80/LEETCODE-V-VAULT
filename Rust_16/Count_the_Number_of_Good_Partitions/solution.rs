use std::collections::HashMap;

impl Solution {
    pub fn number_of_good_partitions(black_nums: Vec<i32>) -> i32 {
        let black_n = black_nums.len();
        let mut black_last = HashMap::new();
        for i in 0..black_n { black_last.insert(black_nums[i], i); }

        let (mut black_res, mut black_j, mut black_parts) = (1i64, 0, 0);
        let mut i = 0;
        while i < black_n {
            if i > black_j { black_parts += 1; }
            black_j = black_j.max(black_last[&black_nums[i]]);
            i += 1;
        }

        for _ in 0..black_parts { black_res = (black_res * 2) % 1_000_000_007; }
        black_res as i32
    }
}