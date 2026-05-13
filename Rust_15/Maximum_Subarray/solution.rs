impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut black_max, mut black_current) = (nums[0], nums[0]);
        
        for &black_val in nums.iter().skip(1) {
            black_current = std::cmp::max(black_val, black_current + black_val);
            black_max = std::cmp::max(black_max, black_current);
        }
        
        black_max
    }
}