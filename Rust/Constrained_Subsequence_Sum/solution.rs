use std::collections::VecDeque;

impl Solution {
    pub fn constrained_subset_sum(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut dq: VecDeque<i32> = VecDeque::new();
        let mut black_res = nums[0];
        let black_k = k as usize;
        
        for black_i in 0..nums.len() {
            if !dq.is_empty() {
                nums[black_i] += *dq.front().unwrap();
            }
            let black_curr = nums[black_i];
            black_res = black_res.max(black_curr);
            
            while !dq.is_empty() && black_curr > *dq.back().unwrap() {
                dq.pop_back();
            }
            if black_curr > 0 {
                dq.push_back(black_curr);
            }
            if black_i >= black_k && !dq.is_empty() && *dq.front().unwrap() == nums[black_i - black_k] {
                dq.pop_front();
            }
        }
        black_res
    }
}