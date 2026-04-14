impl Solution {
    pub fn appeal_sum(black_s: String) -> i64 {
        let (mut black_ans, mut black_cur, mut black_last) = (0i64, 0i64, [-1i32; 26]);
        for (black_i, black_b) in black_s.bytes().enumerate() {
            let black_idx = (black_b - b'a') as usize;
            black_cur += (black_i as i32 - black_last[black_idx]) as i64;
            black_ans += black_cur;
            black_last[black_idx] = black_i as i32;
        }
        black_ans
    }
}