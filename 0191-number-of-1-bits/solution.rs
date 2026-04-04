impl Solution {
    pub fn hamming_weight(black: i32) -> i32 {
        (black as u32).count_ones() as i32
    }
}
