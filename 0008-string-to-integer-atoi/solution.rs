impl Solution {
    pub fn my_atoi(black_s: String) -> i32 {
        let black_chars: Vec<char> = black_s.trim_start().chars().collect();
        if black_chars.is_empty() { return 0; }
        let (mut black_res, mut black_sign, mut black_i) = (0i64, 1, 0);
        if black_chars[0] == '-' { black_sign = -1; black_i += 1; }
        else if black_chars[0] == '+' { black_i += 1; }
        while black_i < black_chars.len() && black_chars[black_i].is_ascii_digit() {
            black_res = black_res * 10 + (black_chars[black_i] as i64 - 48);
            if black_res * black_sign > i32::MAX as i64 { return i32::MAX; }
            if black_res * black_sign < i32::MIN as i64 { return i32::MIN; }
            black_i += 1;
        }
        (black_res * black_sign) as i32
    }
}
