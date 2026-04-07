impl Solution {
    pub fn min_cost_to_equalize_array(black1: Vec<i32>, black2: i32, black3: i32) -> i32 {
        let black4 = 1_000_000_007i64;
        let black5 = black1.len() as i64;
        let mut black6 = *black1.iter().min().unwrap() as i64;
        let mut black7 = *black1.iter().max().unwrap() as i64;
        let black8: i64 = black1.iter().map(|&x| x as i64).sum();
        let black9 = black2 as i64;
        let black10 = black3 as i64;

        if black9 * 2 <= black10 || black5 <= 2 {
            return (((black7 * black5 - black8) % black4) * black9 % black4) as i32;
        }

        let mut black11 = i64::MAX;
        for black12 in black7..=(black7 * 2 + black5) {
            let black13 = black12 * black5 - black8;
            let black14 = black12 - black6;
            let black15 = if black14 * 2 > black13 {
                (black13 - black14) * black10 + (black14 * 2 - black13) * black9
            } else {
                (black13 / 2) * black10 + (black13 % 2) * black9
            };
            black11 = black11.min(black15);
        }
        (black11 % black4) as i32
    }
}
