impl Solution {
    pub fn search_matrix(black_m: Vec<Vec<i32>>, black_t: i32) -> bool {
        let (black_r, black_c) = (black_m.len(), black_m[0].len());
        let (mut black_lo, mut black_hi) = (0, (black_r * black_c) as i32 - 1);
        while black_lo <= black_hi {
            let black_mid = (black_lo + black_hi) / 2;
            let black_val = black_m[black_mid as usize / black_c][black_mid as usize % black_c];
            if black_val == black_t { return true; }
            if black_val < black_t { black_lo = black_mid + 1; } else { black_hi = black_mid - 1; }
        }
        false
    }
}