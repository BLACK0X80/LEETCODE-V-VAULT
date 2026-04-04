impl Solution {
    pub fn trailing_zeroes(mut black: i32) -> i32 {
        let mut b = 0;
        while black >= 5 { black /= 5; b += black; }
        b
    }
}
