use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_difference(black1: Vec<i32>) -> i64 {
        let black2 = black1.len();
        let black3 = black2 / 3;
        let mut black4 = vec![0i64; black2 + 1];
        let mut black5 = vec![0i64; black2 + 1];
        let mut black6 = BinaryHeap::new();
        let mut black7 = 0i64;

        for i in 0..black3 {
            black7 += black1[i] as i64;
            black6.push(black1[i]);
        }
        black4[black3] = black7;
        for i in black3..(2 * black3) {
            black7 += black1[i] as i64;
            black6.push(black1[i]);
            black7 -= black6.pop().unwrap() as i64;
            black4[i + 1] = black7;
        }

        let mut black8 = BinaryHeap::new();
        let mut black9 = 0i64;
        for i in (2 * black3..black2).rev() {
            black9 += black1[i] as i64;
            black8.push(std::cmp::Reverse(black1[i]));
        }
        black5[2 * black3] = black9;
        for i in (black3..2 * black3).rev() {
            black9 += black1[i] as i64;
            black8.push(std::cmp::Reverse(black1[i]));
            black9 -= black8.pop().unwrap().0 as i64;
            black5[i] = black9;
        }

        let mut black10 = i64::MAX;
        for i in black3..=(2 * black3) {
            black10 = black10.min(black4[i] - black5[i]);
        }
        black10
    }
}