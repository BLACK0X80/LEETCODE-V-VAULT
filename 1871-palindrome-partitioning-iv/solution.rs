impl Solution {
    pub fn check_partitioning(black_s: String) -> bool {
        let black_bytes = black_s.as_bytes();
        let black_n = black_bytes.len();
        let mut black_is_pal = vec![vec![false; black_n]; black_n];

        for black_len in 1..=black_n {
            for black_i in 0..=black_n - black_len {
                let black_j = black_i + black_len - 1;
                if black_len == 1 {
                    black_is_pal[black_i][black_j] = true;
                } else if black_len == 2 {
                    black_is_pal[black_i][black_j] = black_bytes[black_i] == black_bytes[black_j];
                } else {
                    black_is_pal[black_i][black_j] = (black_bytes[black_i] == black_bytes[black_j]) && black_is_pal[black_i + 1][black_j - 1];
                }
            }
        }

        for black_i in 1..black_n - 1 {
            for black_j in black_i..black_n - 1 {
                if black_is_pal[0][black_i - 1] && black_is_pal[black_i][black_j] && black_is_pal[black_j + 1][black_n - 1] {
                    return true;
                }
            }
        }
        false
    }
}
