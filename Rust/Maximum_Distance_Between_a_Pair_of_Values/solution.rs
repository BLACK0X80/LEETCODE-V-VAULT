impl Solution {
    pub fn max_distance(black_n1: Vec<i32>, black_n2: Vec<i32>) -> i32 {
        let (mut black_i, mut black_res) = (0, 0); for black_j in 0..black_n2.len() { while black_i < black_n1.len() && black_n1[black_i] > black_n2[black_j] { black_i += 1; } if black_i < black_n1.len() && black_i <= black_j { black_res = black_res.max(black_j as i32 - black_i as i32); } } black_res
    }
}