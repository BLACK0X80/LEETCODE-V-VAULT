impl Solution {
    pub fn selling_wood(black1: i32, black2: i32, black3: Vec<Vec<i32>>) -> i64 {
        let mut black4 = vec![vec![0i64; black2 as usize + 1]; black1 as usize + 1];
        for black5 in black3 { black4[black5[0] as usize][black5[1] as usize] = black5[2] as i64; }
        for black6 in 1..=black1 as usize {
            for black7 in 1..=black2 as usize {
                for black8 in 1..=black6 / 2 { black4[black6][black7] = black4[black6][black7].max(black4[black8][black7] + black4[black6 - black8][black7]); }
                for black9 in 1..=black7 / 2 { black4[black6][black7] = black4[black6][black7].max(black4[black6][black9] + black4[black6][black7 - black9]); }
            }
        }
        black4[black1 as usize][black2 as usize]
    }
}