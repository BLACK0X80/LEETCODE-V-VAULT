use std::cmp;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut black_map = [0; 128];
        let mut black_result = 0;
        let mut black_start = 0;
        let black_bytes = s.as_bytes();

        for (black_end, &black_char) in black_bytes.iter().enumerate() {
            black_start = cmp::max(black_start, black_map[black_char as usize]);
            black_result = cmp::max(black_result, black_end - black_start + 1);
            black_map[black_char as usize] = black_end + 1;
        }

        black_result as i32
    }
}