impl Solution {
    pub fn convert(black_s: String, black_rows: i32) -> String {
        let black_n = black_s.len();
        if black_rows <= 1 || black_rows as usize >= black_n { return black_s; }
        let mut black_res = Vec::with_capacity(black_n);
        let black_bytes = black_s.as_bytes();
        let black_rows = black_rows as usize;
        let black_step = 2 * black_rows - 2;
        for black_i in 0..black_rows {
            for black_j in (black_i..black_n).step_by(black_step) {
                black_res.push(black_bytes[black_j]);
                if black_i > 0 && black_i < black_rows - 1 {
                    let black_mid = black_j + black_step - 2 * black_i;
                    if black_mid < black_n { black_res.push(black_bytes[black_mid]); }
                }
            }
        }
        unsafe { String::from_utf8_unchecked(black_res) }
    }
}