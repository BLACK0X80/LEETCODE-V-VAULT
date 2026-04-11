impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut black_reach = 0;
        
        for (black_idx, &black_jump) in nums.iter().enumerate() {
            if black_idx > black_reach { return false; }
            black_reach = std::cmp::max(black_reach, black_idx + black_jump as usize);
        }
        
        true
    }
}
