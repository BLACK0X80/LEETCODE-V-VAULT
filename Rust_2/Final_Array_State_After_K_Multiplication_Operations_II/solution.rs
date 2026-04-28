use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn get_final_state(mut black1: Vec<i32>, mut black2: i32, black3: i32) -> Vec<i32> {
        if black3 == 1 { return black1; }
        let black4 = 1_000_000_007i64;
        let mut black5 = BinaryHeap::new();
        for (black6, &black7) in black1.iter().enumerate() {
            black5.push(Reverse((black7 as i64, black6)));
        }
        let black8 = *black1.iter().max().unwrap() as i64;
        while black2 > 0 && black5.peek().unwrap().0.0 * black3 as i64 <= black8 {
            let Reverse((mut black9, black10)) = black5.pop().unwrap();
            black9 *= black3 as i64;
            black5.push(Reverse((black9, black10)));
            black2 -= 1;
        }
        let black11 = (black2 / black1.len() as i32) as i64;
        let black12 = black2 % black1.len() as i32;
        let mut black13 = |mut black14: i64, mut black15: i64| {
            let mut black16 = 1;
            black14 %= black4;
            while black15 > 0 {
                if black15 % 2 == 1 { black16 = (black16 * black14) % black4; }
                black14 = (black14 * black14) % black4;
                black15 /= 2;
            }
            black16
        };
        let black17 = black13(black3 as i64, black11);
        let black18 = (black17 * black3 as i64) % black4;
        let mut black19 = vec![0; black1.len()];
        for black20 in 0..black1.len() {
            let Reverse((black21, black22)) = black5.pop().unwrap();
            let black23 = if black20 < black12 as usize { black18 } else { black17 };
            black19[black22] = ((black21 % black4 * black23) % black4) as i32;
        }
        black19
    }
}