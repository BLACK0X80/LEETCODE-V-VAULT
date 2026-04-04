impl Solution {
    pub fn single_number(black: Vec<i32>) -> i32 {
        black.iter().fold(0, |b, &bl| b ^ bl)
    }
}
