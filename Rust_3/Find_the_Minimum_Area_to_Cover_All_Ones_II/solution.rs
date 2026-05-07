impl Solution {
    pub fn minimum_sum(black1: Vec<Vec<i32>>) -> i32 {
        let black2 = black1.len();
        let black3 = black1[0].len();
        let mut black4 = (black2 * black3) as i32;

        for black5 in 0..black2 - 1 {
            for black6 in black5 + 1..black2 - 1 {
                let black7 = Self::black_area(&black1, 0, black5, 0, black3 - 1);
                let black8 = Self::black_area(&black1, black5 + 1, black6, 0, black3 - 1);
                let black9 = Self::black_area(&black1, black6 + 1, black2 - 1, 0, black3 - 1);
                if black7 > 0 && black8 > 0 && black9 > 0 {
                    black4 = black4.min(black7 + black8 + black9);
                }
            }
        }

        for black5 in 0..black3 - 1 {
            for black6 in black5 + 1..black3 - 1 {
                let black7 = Self::black_area(&black1, 0, black2 - 1, 0, black5);
                let black8 = Self::black_area(&black1, 0, black2 - 1, black5 + 1, black6);
                let black9 = Self::black_area(&black1, 0, black2 - 1, black6 + 1, black3 - 1);
                if black7 > 0 && black8 > 0 && black9 > 0 {
                    black4 = black4.min(black7 + black8 + black9);
                }
            }
        }

        for black5 in 0..black2 - 1 {
            let black10 = Self::black_area(&black1, 0, black5, 0, black3 - 1);
            if black10 == 0 { continue; }
            for black6 in 0..black3 - 1 {
                let black7 = Self::black_area(&black1, black5 + 1, black2 - 1, 0, black6);
                let black8 = Self::black_area(&black1, black5 + 1, black2 - 1, black6 + 1, black3 - 1);
                if black7 > 0 && black8 > 0 {
                    black4 = black4.min(black10 + black7 + black8);
                }
            }
        }

        for black5 in 0..black2 - 1 {
            let black10 = Self::black_area(&black1, black5 + 1, black2 - 1, 0, black3 - 1);
            if black10 == 0 { continue; }
            for black6 in 0..black3 - 1 {
                let black7 = Self::black_area(&black1, 0, black5, 0, black6);
                let black8 = Self::black_area(&black1, 0, black5, black6 + 1, black3 - 1);
                if black7 > 0 && black8 > 0 {
                    black4 = black4.min(black10 + black7 + black8);
                }
            }
        }

        for black5 in 0..black3 - 1 {
            let black10 = Self::black_area(&black1, 0, black2 - 1, 0, black5);
            if black10 == 0 { continue; }
            for black6 in 0..black2 - 1 {
                let black7 = Self::black_area(&black1, 0, black6, black5 + 1, black3 - 1);
                let black8 = Self::black_area(&black1, black6 + 1, black2 - 1, black5 + 1, black3 - 1);
                if black7 > 0 && black8 > 0 {
                    black4 = black4.min(black10 + black7 + black8);
                }
            }
        }

        for black5 in 0..black3 - 1 {
            let black10 = Self::black_area(&black1, 0, black2 - 1, black5 + 1, black3 - 1);
            if black10 == 0 { continue; }
            for black6 in 0..black2 - 1 {
                let black7 = Self::black_area(&black1, 0, black6, 0, black5);
                let black8 = Self::black_area(&black1, black6 + 1, black2 - 1, 0, black5);
                if black7 > 0 && black8 > 0 {
                    black4 = black4.min(black10 + black7 + black8);
                }
            }
        }

        black4
    }

    fn black_area(black11: &Vec<Vec<i32>>, black12: usize, black13: usize, black14: usize, black15: usize) -> i32 {
        let mut black16 = usize::MAX;
        let mut black17 = 0;
        let mut black18 = usize::MAX;
        let mut black19 = 0;
        let mut black20 = false;

        for i in black12..=black13 {
            for j in black14..=black15 {
                if black11[i][j] == 1 {
                    if i < black16 { black16 = i; }
                    if i > black17 { black17 = i; }
                    if j < black18 { black18 = j; }
                    if j > black19 { black19 = j; }
                    black20 = true;
                }
            }
        }

        if !black20 { return 0; }
        ((black17 - black16 + 1) * (black19 - black18 + 1)) as i32
    }
}