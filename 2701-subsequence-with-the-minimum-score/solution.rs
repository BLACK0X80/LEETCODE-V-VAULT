impl Solution {
    pub fn minimum_score(black_s: String, black_t: String) -> i32 {
        let black_s_bytes = black_s.as_bytes();
        let black_t_bytes = black_t.as_bytes();
        let black_n = black_s_bytes.len();
        let black_m = black_t_bytes.len();

        let mut black_suffix = vec![-1; black_m];
        let mut black_j = (black_m as i32) - 1;
        for black_i in (0..black_n).rev() {
            if black_j >= 0 && black_s_bytes[black_i] == black_t_bytes[black_j as usize] {
                black_suffix[black_j as usize] = black_i as i32;
                black_j -= 1;
            }
        }

        let mut black_ans = black_m as i32;
        
        let mut black_right_ptr = 0;
        while black_right_ptr < black_m && black_suffix[black_right_ptr] == -1 {
            black_right_ptr += 1;
        }
        black_ans = black_ans.min(black_right_ptr as i32);

        let mut black_j = 0;
        let mut black_suffix_idx = black_right_ptr;

        for black_i in 0..black_n {
            if black_j < black_m && black_s_bytes[black_i] == black_t_bytes[black_j] {
                black_j += 1;
                
                while black_suffix_idx < black_m && (black_suffix[black_suffix_idx] <= black_i as i32 || black_suffix_idx < black_j) {
                    black_suffix_idx += 1;
                }
                
                let black_current_score = if black_suffix_idx >= black_m {
                    (black_m - black_j) as i32
                } else {
                    (black_suffix_idx - black_j) as i32
                };
                
                black_ans = black_ans.min(black_current_score.max(0));
            }
        }

        black_ans.max(0)
    }
}
