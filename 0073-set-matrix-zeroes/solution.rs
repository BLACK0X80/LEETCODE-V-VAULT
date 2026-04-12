impl Solution {
    pub fn set_zeroes(black_m: &mut Vec<Vec<i32>>) {
        let (black_r, black_c) = (black_m.len(), black_m[0].len());
        let mut black_col0 = 1;
        for black_i in 0..black_r {
            if black_m[black_i][0] == 0 { black_col0 = 0; }
            for black_j in 1..black_c {
                if black_m[black_i][black_j] == 0 { black_m[black_i][0] = 0; black_m[0][black_j] = 0; }
            }
        }
        for black_i in (0..black_r).rev() {
            for black_j in (1..black_c).rev() {
                if black_m[black_i][0] == 0 || black_m[0][black_j] == 0 { black_m[black_i][black_j] = 0; }
            }
            if black_col0 == 0 { black_m[black_i][0] = 0; }
        }
    }
}
