impl Solution {
    pub fn minimum_time(black_s: String) -> i32 {
        let black_n = black_s.len();
        let black_bytes = black_s.as_bytes();
        let mut black_left = vec![0; black_n + 1];
        for black_i in 0..black_n {
            if black_bytes[black_i] == b'1' {
                black_left[black_i + 1] = (black_left[black_i] + 2).min(black_i as i32 + 1);
            } else {
                black_left[black_i + 1] = black_left[black_i];
            }
        }
        let mut black_res = black_left[black_n];
        let mut black_right = 0;
        for black_i in (0..black_n).rev() {
            if black_bytes[black_i] == b'1' {
                black_right = (black_right + 2).min(black_n as i32 - black_i as i32);
            }
            black_res = black_res.min(black_left[black_i] + black_right);
        }
        black_res
    }
}