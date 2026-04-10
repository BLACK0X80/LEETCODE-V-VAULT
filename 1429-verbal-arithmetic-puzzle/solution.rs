use std::collections::HashMap;

impl Solution {
    pub fn is_solvable(black_words: Vec<String>, black_result: String) -> bool {
        let mut black_all_words = black_words.clone();
        black_all_words.push(black_result.clone());
        let mut black_chars = std::collections::HashSet::new();
        let mut black_is_start = [false; 26];

        for black_w in &black_all_words {
            if black_w.len() > 1 {
                black_is_start[(black_w.as_bytes()[0] - b'A') as usize] = true;
            }
            for &black_b in black_w.as_bytes() {
                black_chars.insert(black_b);
            }
        }

        let black_char_list: Vec<u8> = black_chars.into_iter().collect();
        let mut black_used_digit = [false; 10];
        let mut black_map = [ -1i8; 26];

        fn black_backtrack(black_idx: usize, black_char_list: &[u8], black_map: &mut [i8; 26], black_used_digit: &mut [bool; 10], black_words: &[String], black_result: &String, black_is_start: &[bool; 26]) -> bool {
            if black_idx == black_char_list.len() {
                let mut black_sum = 0i64;
                for black_w in black_words {
                    let mut black_num = 0i64;
                    for &black_b in black_w.as_bytes() { black_num = black_num * 10 + black_map[(black_b - b'A') as usize] as i64; }
                    black_sum += black_num;
                }
                let mut black_res_num = 0i64;
                for &black_b in black_result.as_bytes() { black_res_num = black_res_num * 10 + black_map[(black_b - b'A') as usize] as i64; }
                return black_sum == black_res_num;
            }

            let black_c = black_char_list[black_idx];
            for black_d in 0..10 {
                if black_used_digit[black_d] || (black_d == 0 && black_is_start[(black_c - b'A') as usize]) { continue; }
                black_used_digit[black_d] = true;
                black_map[(black_c - b'A') as usize] = black_d as i8;
                if black_backtrack(black_idx + 1, black_char_list, black_map, black_used_digit, black_words, black_result, black_is_start) { return true; }
                black_used_digit[black_d] = false;
                black_map[(black_c - b'A') as usize] = -1;
            }
            false
        }

        black_backtrack(0, &black_char_list, &mut black_map, &mut black_used_digit, &black_words, &black_result, &black_is_start)
    }
}
