use std::collections::BinaryHeap;

impl Solution {
    pub fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut black_max_sum = 0i64;
        let mut black_abs_nums = Vec::new();
        
        for &x in &nums {
            if x > 0 { black_max_sum += x as i64; }
            black_abs_nums.push(x.abs() as i64);
        }
        
        black_abs_nums.sort_unstable();
        
        if k == 1 { return black_max_sum; }
        
        let mut black_pq = BinaryHeap::new();
        black_pq.push((-(black_abs_nums[0]), 0usize));
        
        let mut black_current_diff = 0;
        for _ in 0..k-1 {
            let (diff_neg, idx) = black_pq.pop().unwrap();
            black_current_diff = -diff_neg;
            
            if idx + 1 < black_abs_nums.len() {
                black_pq.push((-(black_current_diff + black_abs_nums[idx + 1]), idx + 1));
                black_pq.push((-(black_current_diff - black_abs_nums[idx] + black_abs_nums[idx + 1]), idx + 1));
            }
        }
        
        black_max_sum - black_current_diff
    }
}