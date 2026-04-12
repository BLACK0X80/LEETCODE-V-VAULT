impl Solution {
    pub fn num_trees(black_n: i32) -> i32 {
        let mut black_res: i64 = 1;
        for black_i in 0..black_n as i64 {
            black_res = black_res * 2 * (2 * black_i + 1) / (black_i + 2);
        }
        black_res as i32
    }
}
