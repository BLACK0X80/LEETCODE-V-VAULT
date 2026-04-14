use std::collections::HashSet;

impl Solution {
    pub fn longest_valid_substring(black_word: String, black_forbidden: Vec<String>) -> i32 {
        let black_set: HashSet<String> = black_forbidden.into_iter().collect();
        let black_bytes = black_word.as_bytes();
        let black_n = black_bytes.len();
        let (mut black_ans, mut black_right) = (0, black_n);

        for black_left in (0..black_n).rev() {
            for black_len in 1..=10 {
                let black_end = (black_left + black_len).min(black_right);
                if black_left + black_len > black_right { break; }
                let black_sub = std::str::from_utf8(&black_bytes[black_left..black_end]).unwrap();
                if black_set.contains(black_sub) {
                    black_right = black_left + black_len - 1;
                    break;
                }
            }
            black_ans = black_ans.max((black_right - black_left) as i32);
        }
        black_ans
    }
}