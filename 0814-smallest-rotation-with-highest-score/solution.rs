impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        let black_n = nums.len();
        let mut black_change = vec![0; black_n];
        
        for i in 0..black_n {
            
            let black_left = (i as i32 - nums[i] + 1 + black_n as i32) as usize % black_n;
            black_change[black_left] -= 1;
        }
        
        let (mut black_max_score, mut black_current_score, mut black_best_k) = (i32::MIN, 0, 0);
        for k in 0..black_n {
            black_current_score += black_change[k] + 1;
            if black_current_score > black_max_score {
                black_max_score = black_current_score;
                black_best_k = k as i32;
            }
        }
        
        black_best_k
    }
}
