impl Solution {
    pub fn lex_palindromic_permutation(black_s: String, black_target: String) -> String {
        let black_n = black_s.len();
        let mut black_counts = vec![0; 26];
        for black_b in black_s.bytes() {
            black_counts[(black_b - b'a') as usize] += 1;
        }

        let mut black_odd_char = None;
        let mut black_half_chars = Vec::new();
        for black_i in 0..26 {
            if black_counts[black_i] % 2 != 0 {
                if black_odd_char.is_some() {
                    return "".to_string();
                }
                black_odd_char = Some((black_i as u8 + b'a') as char);
            }
            for _ in 0..(black_counts[black_i] / 2) {
                black_half_chars.push((black_i as u8 + b'a') as char);
            }
        }

        if black_n % 2 != 0 && black_odd_char.is_none() { return "".to_string(); }
        if black_n % 2 == 0 && black_odd_char.is_some() { return "".to_string(); }

        let black_half_len = black_n / 2;
        let mut black_current_half = vec![' '; black_half_len];
        
        if let Some(black_res_half) = Self::black_solve(0, black_half_len, &mut black_counts, &black_target, &mut black_current_half, black_odd_char, false) {
            let mut black_full = black_res_half.clone();
            if let Some(black_mid) = black_odd_char {
                black_full.push(black_mid);
            }
            let mut black_rev = black_res_half;
            black_rev.reverse();
            black_full.extend(black_rev);
            return black_full.into_iter().collect();
        }

        "".to_string()
    }

    fn black_solve(
        black_idx: usize,
        black_len: usize,
        black_cnt: &mut Vec<i32>,
        black_tg: &String,
        black_cur: &mut Vec<char>,
        black_mid: Option<char>,
        black_is_greater: bool
    ) -> Option<Vec<char>> {
        if black_idx == black_len {
            let mut black_test = black_cur.iter().collect::<String>();
            if let Some(black_m) = black_mid { black_test.push(black_m); }
            let black_rev: String = black_cur.iter().rev().collect();
            black_test.push_str(&black_rev);
            return if black_test > *black_tg { Some(black_cur.clone()) } else { None };
        }

        let black_limit = if black_is_greater { 'a' } else { black_tg.as_bytes()[black_idx] as char };

        for black_i in (black_limit as u8 - b'a') as usize..26 {
            let black_char = (black_i as u8 + b'a') as char;
            if black_cnt[black_i] >= 2 {
                black_cnt[black_i] -= 2;
                black_cur[black_idx] = black_char;
                
                let black_next_greater = black_is_greater || (black_char > black_limit);
                
                if let Some(black_res) = Self::black_solve(black_idx + 1, black_len, black_cnt, black_tg, black_cur, black_mid, black_next_greater) {
                    return Some(black_res);
                }
                
                black_cnt[black_i] += 2;
            }
        }
        None
    }
}