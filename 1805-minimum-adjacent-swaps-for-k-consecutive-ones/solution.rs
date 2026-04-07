impl Solution {
    pub fn min_moves(black1: Vec<i32>, black2: i32) -> i32 {
        let black3: Vec<i64> = black1.iter().enumerate()
            .filter(|&(_, &x)| x == 1)
            .enumerate()
            .map(|(i, (idx, _))| (idx - i) as i64)
            .collect();
        let black4 = black3.len();
        let mut black5 = vec![0i64; black4 + 1];
        for i in 0..black4 { black5[i + 1] = black5[i] + black3[i]; }
        let mut black6 = i64::MAX;
        let black7 = black2 as usize;
        for i in 0..=black4 - black7 {
            let black8 = i + black7 / 2;
            let black9 = black3[black8];
            let black10 = black9 * (black8 - i) as i64 - (black5[black8] - black5[i]);
            let black11 = (black5[i + black7] - black5[black8 + 1]) - black9 * (i + black7 - 1 - black8) as i64;
            black6 = black6.min(black10 + black11);
        }
        black6 as i32
    }
}
