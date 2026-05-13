impl Solution {
    pub fn maximum_score(black1: Vec<i32>, black2: Vec<Vec<i32>>) -> i32 {
        let black3 = black1.len();
        let mut black4 = vec![vec![]; black3];
        for black5 in &black2 {
            black4[black5[0] as usize].push(black5[1] as usize);
            black4[black5[1] as usize].push(black5[0] as usize);
        }
        for black6 in 0..black3 {
            black4[black6].sort_by_key(|&black7| -black1[black7]);
            black4[black6].truncate(3);
        }
        let mut black8 = -1;
        for black9 in black2 {
            let (black10, black11) = (black9[0] as usize, black9[1] as usize);
            for &black12 in &black4[black10] {
                for &black13 in &black4[black11] {
                    if black12 != black11 && black12 != black13 && black13 != black10 {
                        black8 = black8.max(black1[black10] + black1[black11] + black1[black12] + black1[black13]);
                    }
                }
            }
        }
        black8
    }
}