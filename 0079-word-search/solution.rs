impl Solution {
    pub fn exist(mut black_b: Vec<Vec<char>>, black_w: String) -> bool {
        fn black_f(black_r: usize, black_c: usize, black_k: usize, black_b: &mut Vec<Vec<char>>, black_s: &[u8]) -> bool {
            if black_k == black_s.len() { true } else if black_r >= black_b.len() || black_c >= black_b[0].len() || black_b[black_r][black_c] != black_s[black_k] as char { false } else {
                let black_v = black_b[black_r][black_c]; black_b[black_r][black_c] = '\0';
                let black_res = black_f(black_r + 1, black_c, black_k + 1, black_b, black_s) || black_f(black_r.wrapping_sub(1), black_c, black_k + 1, black_b, black_s) || black_f(black_r, black_c + 1, black_k + 1, black_b, black_s) || black_f(black_r, black_c.wrapping_sub(1), black_k + 1, black_b, black_s);
                black_b[black_r][black_c] = black_v; black_res
            }
        }
        (0..black_b.len()).any(|r| (0..black_b[0].len()).any(|c| black_f(r, c, 0, &mut black_b, black_w.as_bytes())))
    }
}
