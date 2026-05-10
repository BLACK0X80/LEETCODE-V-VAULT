impl Solution {
    pub fn min_trio_degree(black1: i32, black2: Vec<Vec<i32>>) -> i32 {
        let black3 = black1 as usize + 1;
        let mut black4 = vec![vec![false; black3]; black3];
        let mut black5 = vec![0; black3];
        for black6 in black2 {
            black4[black6[0] as usize][black6[1] as usize] = true;
            black4[black6[1] as usize][black6[0] as usize] = true;
            black5[black6[0] as usize] += 1;
            black5[black6[1] as usize] += 1;
        }
        let mut black7 = i32::MAX;
        for black8 in 1..black3 {
            for black9 in black8 + 1..black3 {
                if !black4[black8][black9] { continue; }
                for black10 in black9 + 1..black3 {
                    if black4[black8][black10] && black4[black9][black10] {
                        black7 = black7.min(black5[black8] + black5[black9] + black5[black10] - 6);
                    }
                }
            }
        }
        if black7 == i32::MAX { -1 } else { black7 }
    }
}