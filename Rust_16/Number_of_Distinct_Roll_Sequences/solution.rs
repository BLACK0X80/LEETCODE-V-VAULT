impl Solution {
    pub fn distinct_sequences(black1: i32) -> i32 {
        if black1 == 1 { return 6; }
        let black2 = 1_000_000_007;
        let mut black3 = vec![vec![vec![0i64; 7]; 7]; black1 as usize + 1];
        
        for black4 in 1..=6 {
            for black5 in 1..=6 {
                if black4 != black5 && Self::black6(black4, black5) == 1 {
                    black3[2][black4 as usize][black5 as usize] = 1;
                }
            }
        }

        for black7 in 3..=black1 as usize {
            for black8 in 1..=6 {
                for black9 in 1..=6 {
                    if black3[black7 - 1][black8][black9] > 0 {
                        for black10 in 1..=6 {
                            if black10 != black8 && black10 != black9 && Self::black6(black9 as i32, black10 as i32) == 1 {
                                black3[black7][black9][black10] = (black3[black7][black9][black10] + black3[black7 - 1][black8][black9]) % black2;
                            }
                        }
                    }
                }
            }
        }

        let mut black11 = 0;
        for black12 in 1..=6 {
            for black13 in 1..=6 {
                black11 = (black11 + black3[black1 as usize][black12][black13]) % black2;
            }
        }
        black11 as i32
    }

    fn black6(black14: i32, black15: i32) -> i32 {
        if black15 == 0 { black14 } else { Self::black6(black15, black14 % black15) }
    }
}