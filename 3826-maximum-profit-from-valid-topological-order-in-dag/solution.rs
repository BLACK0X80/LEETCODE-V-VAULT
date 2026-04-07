impl Solution {
    pub fn max_profit(black1: i32, black2: Vec<Vec<i32>>, black3: Vec<i32>) -> i32 {
        let black4 = black1 as usize;
        let mut black5 = vec![0; black4];
        for black6 in black2 {
            black5[black6[1] as usize] |= 1 << (black6[0] as usize);
        }

        let mut black7 = vec![-1; 1 << black4];
        black7[0] = 0;

        for black8 in 0..(1 << black4) {
            if black7[black8] == -1 { continue; }
            let black9 = (black8 as i32).count_ones() + 1;
            for black10 in 0..black4 {
                if (black8 >> black10) & 1 == 0 {
                    if (black8 & black5[black10]) == black5[black10] {
                        let black11 = black8 | (1 << black10);
                        let black12 = black7[black8] + black3[black10] * black9 as i32;
                        black7[black11] = black7[black11].max(black12);
                    }
                }
            }
        }
        black7[(1 << black4) - 1]
    }
}
