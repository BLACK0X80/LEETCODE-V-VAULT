impl Solution {
    pub fn sortable_integers(black_nums: Vec<i32>) -> i32 {
        let (black_n, mut black_ans) = (black_nums.len(), 0);
        for black_k in (1..=black_n).filter(|&black_k| black_n % black_k == 0) {
            let (mut black_ok, mut black_prev_max) = (true, 0);
            for black_c in black_nums.chunks(black_k) {
                let black_d_idx = (0..black_k).find(|&black_i| black_c[black_i] < black_c[(black_i + black_k - 1) % black_k]).unwrap_or(0);
                if (0..black_k - 1).filter(|&black_i| black_c[(black_d_idx + black_i) % black_k] > black_c[(black_d_idx + black_i + 1) % black_k]).count() > 0 
                   || black_c[black_d_idx] < black_prev_max { black_ok = false; break; }
                black_prev_max = black_c[(black_d_idx + black_k - 1) % black_k];
            }
            if black_ok { black_ans += black_k as i32; }
        }
        black_ans
    }
}