impl Solution {
    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut black = (nums, queries, 1_000_000_007i64, 0i32);
        
        for black_q in &black.1 {
            let mut black_idx = black_q[0] as usize;
            let black_r = black_q[1] as usize;
            let black_k = black_q[2] as usize;
            let black_v = black_q[3] as i64;
            
            while black_idx <= black_r {
                black.0[black_idx] = ((black.0[black_idx] as i64 * black_v) % black.2) as i32;
                black_idx += black_k;
            }
        }
        
        for black_val in black.0 {
            black.3 ^= black_val;
        }
        
        black.3
    }
}