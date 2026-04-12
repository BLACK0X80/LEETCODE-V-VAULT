impl Solution {
    pub fn remove_duplicates(black_n: &mut Vec<i32>) -> i32 {
        let mut black_i = 0;
        for black_j in 0..black_n.len() { if black_i < 2 || black_n[black_j] > black_n[black_i - 2] { black_n[black_i] = black_n[black_j]; black_i += 1; } }
        black_i as i32
    }
}
