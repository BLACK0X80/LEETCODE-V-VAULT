impl Solution {
    pub fn sum_of_floored_pairs(black1: Vec<i32>) -> i32 {
        let black2 = 100001;
        let mut black3 = vec![0i64; black2];
        for &black4 in &black1 { black3[black4 as usize] += 1; }
        let mut black5 = vec![0i64; black2];
        for black6 in 1..black2 { black5[black6] = black5[black6 - 1] + black3[black6]; }
        let mut black7 = 0;
        let black8 = 1_000_000_007;
        for black9 in 1..black2 {
            if black3[black9] == 0 { continue; }
            for black10 in (black9..black2).step_by(black9) {
                let black11 = std::cmp::min(black10 + black9 - 1, black2 - 1);
                let black12 = black5[black11] - black5[black10 - 1];
                black7 = (black7 + black3[black9] * black12 * (black10 / black9) as i64) % black8;
            }
        }
        black7 as i32
    }
}