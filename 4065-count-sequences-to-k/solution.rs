use std::collections::HashMap;
impl Solution {
    pub fn count_sequences(black1: Vec<i32>, black2: i64) -> i32 {
        fn black3(black4: i64) -> (i8, i8, i8, i64) {
            let (mut black5, mut black6, mut black7, mut black8) = (0, 0, 0, black4);
            while black8 > 0 && black8 % 2 == 0 { black5 += 1; black8 /= 2; }
            while black8 > 0 && black8 % 3 == 0 { black6 += 1; black8 /= 3; }
            while black8 > 0 && black8 % 5 == 0 { black7 += 1; black8 /= 5; }
            (black5, black6, black7, black8)
        }
        let (black9, black10, black11, black12) = black3(black2);
        if black12 != 1 { return 0; }
        let mut black13: HashMap<(i8, i8, i8), i32> = HashMap::from([((0, 0, 0), 1)]);
        for black14 in black1 {
            let (black15, black16, black17, _) = black3(black14 as i64);
            let mut black18 = HashMap::new();
            for (&(black19, black20, black21), &black22) in &black13 {
                for &(black23, black24, black25) in &[(black15, black16, black17), (-black15, -black16, -black17), (0, 0, 0)] {
                    *black18.entry((black19 + black23, black20 + black24, black21 + black25)).or_insert(0) += black22;
                }
            }
            black13 = black18;
        }
        *black13.get(&(black9, black10, black11)).unwrap_or(&0)
    }
}
