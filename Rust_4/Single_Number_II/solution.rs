impl Solution {
    pub fn single_number(black: Vec<i32>) -> i32 {
        let (mut b1, mut b2) = (0i32, 0i32);
        for &b in &black {
            b1 = (b1 ^ b) & !b2;
            b2 = (b2 ^ b) & !b1;
        }
        b1
    }
}