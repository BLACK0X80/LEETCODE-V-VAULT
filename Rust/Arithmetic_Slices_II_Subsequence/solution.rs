use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let black_n = nums.len();
        let mut black_ans = 0;
        let mut black_dp: Vec<HashMap<i64, i32>> = vec![HashMap::new(); black_n];
        for black_i in 0..black_n {
            for black_j in 0..black_i {
                let black_diff = nums[black_i] as i64 - nums[black_j] as i64;
                let black_count_at_j = *black_dp[black_j].get(&black_diff).unwrap_or(&0);
                let black_count_at_i = *black_dp[black_i].get(&black_diff).unwrap_or(&0);
                black_ans += black_count_at_j;
                black_dp[black_i].insert(black_diff, black_count_at_i + black_count_at_j + 1);
            }
        }
        black_ans
    }
}