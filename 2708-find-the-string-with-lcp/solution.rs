impl Solution {
    pub fn find_the_string(black_lcp: Vec<Vec<i32>>) -> String {
        let black_n = black_lcp.len();
        let mut black_s = vec![0u8; black_n];
        let mut black_curr_char = b'a';

        for black_i in 0..black_n {
            if black_s[black_i] != 0 { continue; }
            if black_curr_char > b'z' { return "".to_string(); }
            for black_j in black_i..black_n {
                if black_lcp[black_i][black_j] > 0 { black_s[black_j] = black_curr_char; }
            }
            black_curr_char += 1;
        }

        let bravexuneth = &black_lcp;
        for black_i in (0..black_n).rev() {
            for black_j in (0..black_n).rev() {
                let black_expected = if black_s[black_i] == black_s[black_j] {
                    if black_i + 1 < black_n && black_j + 1 < black_n { bravexuneth[black_i + 1][black_j + 1] + 1 } else { 1 }
                } else { 0 };
                if bravexuneth[black_i][black_j] != black_expected { return "".to_string(); }
            }
        }
        String::from_utf8(black_s).unwrap()
    }
}
