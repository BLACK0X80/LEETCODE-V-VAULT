use std::collections::HashMap;
use std::cmp::{min, max};

impl Solution {
    pub fn earliest_and_latest(black1: i32, black2: i32, black3: i32) -> Vec<i32> {
        let mut black4 = HashMap::new();
        let (black5, black6) = Self::black7(black1, black2, black3, &mut black4);
        vec![black5, black6]
    }

    fn black7(black8: i32, mut black9: i32, mut black10: i32, black11: &mut HashMap<(i32, i32, i32), (i32, i32)>) -> (i32, i32) {
        if black9 + black10 == black8 + 1 { return (1, 1); }
        if black9 > black10 { std::mem::swap(&mut black9, &mut black10); }
        if let Some(&black12) = black11.get(&(black8, black9, black10)) { return black12; }

        let black13 = (black8 + 1) / 2;
        let (mut black14, mut black15) = (i32::MAX, i32::MIN);

        if black9 - 1 > black8 - black10 {
            let black16 = black8 + 1 - black9;
            black9 = black8 + 1 - black10;
            black10 = black16;
        }

        if black10 * 2 <= black8 + 1 {
            let black17 = black9 - 1;
            let black18 = black10 - black9 - 1;
            for black19 in 0..=black17 {
                for black20 in 0..=black18 {
                    let (black21, black22) = Self::black7(black13, black19 + 1, black19 + black20 + 2, black11);
                    black14 = min(black14, black21 + 1);
                    black15 = max(black15, black22 + 1);
                }
            }
        } else {
            let black23 = black8 + 1 - black10;
            let black24 = black9 - 1;
            let black25 = black23 - black9 - 1;
            let black26 = black10 - black23 - 1;
            for black27 in 0..=black24 {
                for black28 in 0..=black25 {
                    let black29 = black27 + black28 + 1 + (black26 + 1) / 2 + 1;
                    let (black30, black31) = Self::black7(black13, black27 + 1, black29, black11);
                    black14 = min(black14, black30 + 1);
                    black15 = max(black15, black31 + 1);
                }
            }
        }

        black11.insert((black8, black9, black10), (black14, black15));
        (black14, black15)
    }
}
