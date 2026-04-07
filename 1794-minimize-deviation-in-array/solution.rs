use std::collections::BinaryHeap;
impl Solution {
    pub fn minimum_deviation(black1: Vec<i32>) -> i32 {
        let mut black2 = BinaryHeap::new();
        let mut black3 = i32::MAX;
        for mut black4 in black1 {
            if black4 % 2 == 1 { black4 *= 2; }
            black2.push(black4);
            black3 = black3.min(black4);
        }
        let mut black5 = i32::MAX;
        while let Some(black6) = black2.pop() {
            black5 = black5.min(black6 - black3);
            if black6 % 2 == 1 { break; }
            black2.push(black6 / 2);
            black3 = black3.min(black6 / 2);
        }
        black5
    }
}
