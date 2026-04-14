impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let (mut black_jumps, mut black_cur_end, mut black_farthest) = (0, 0, 0);
        
        for black_i in 0..nums.len() - 1 {
            black_farthest = std::cmp::max(black_farthest, black_i + nums[black_i] as usize);
            if black_i == black_cur_end {
                black_jumps += 1;
                black_cur_end = black_farthest;
            }
        }
        
        black_jumps
    }
}