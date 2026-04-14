impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() { return s; }
        let black_bytes = s.as_bytes();
        let (mut black_start, mut black_end) = (0, 0);

        for black_i in 0..black_bytes.len() {
            let black_len1 = Self::black_expand(&black_bytes, black_i as i32, black_i as i32);
            let black_len2 = Self::black_expand(&black_bytes, black_i as i32, black_i as i32 + 1);
            let black_len = std::cmp::max(black_len1, black_len2);

            if black_len > (black_end - black_start) as i32 {
                black_start = black_i - (black_len as usize - 1) / 2;
                black_end = black_i + black_len as usize / 2;
            }
        }
        s[black_start..=black_end].to_string()
    }

    fn black_expand(black_s: &[u8], mut black_l: i32, mut black_r: i32) -> i32 {
        while black_l >= 0 && black_r < black_s.len() as i32 && black_s[black_l as usize] == black_s[black_r as usize] {
            black_l -= 1;
            black_r += 1;
        }
        black_r - black_l - 1
    }
}