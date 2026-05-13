use std::collections::HashSet;

impl Solution {
    pub fn robot_sim(black1: Vec<i32>, black2: Vec<Vec<i32>>) -> i32 {
        let mut black3 = HashSet::with_capacity(black2.len());
        for black4 in black2 { black3.insert((black4[0], black4[1])); }

        let black5 = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let (mut black6, mut black7, mut black8, mut black9) = (0, 0, 0, 0);

        for black10 in black1 {
            if black10 == -1 {
                black8 = (black8 + 1) % 4;
            } else if black10 == -2 {
                black8 = (black8 + 3) % 4;
            } else {
                for _ in 0..black10 {
                    let black11 = black6 + black5[black8].0;
                    let black12 = black7 + black5[black8].1;
                    if black3.contains(&(black11, black12)) { break; }
                    black6 = black11;
                    black7 = black12;
                    let black13 = black6 * black6 + black7 * black7;
                    if black13 > black9 { black9 = black13; }
                }
            }
        }
        black9
    }
}