impl Solution {
    pub fn get_sum(mut black: i32, mut b: i32) -> i32 {
        while b != 0 {
            let bl = (black & b) << 1;
            black ^= b;
            b = bl;
        }
        black
    }
}