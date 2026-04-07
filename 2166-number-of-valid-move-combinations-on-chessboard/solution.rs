impl Solution {
    pub fn count_combinations(black1: Vec<String>, black2: Vec<Vec<i32>>) -> i32 {
        let black3 = black1.len();
        let mut black4 = vec![];
        for black5 in 0..black3 {
            let mut black6 = vec![(0, 0, 0)];
            let black7 = if black1[black5] == "rook" { vec![(0,1),(0,-1),(1,0),(-1,0)] }
                        else if black1[black5] == "bishop" { vec![(1,1),(1,-1),(-1,1),(-1,-1)] }
                        else { vec![(0,1),(0,-1),(1,0),(-1,0),(1,1),(1,-1),(-1,1),(-1,-1)] };
            for (black8, black9) in black7 {
                for black10 in 1..8 {
                    let (black11, black12) = (black2[black5][0] + black8 * black10, black2[black5][1] + black9 * black10);
                    if black11 >= 1 && black11 <= 8 && black12 >= 1 && black12 <= 8 {
                        black6.push((black8, black9, black10));
                    } else { break; }
                }
            }
            black4.push(black6);
        }

        let mut black13 = vec![(0, 0, 0); black3];
        fn black14(black15: usize, black16: &Vec<Vec<(i32, i32, i32)>>, black17: &mut Vec<(i32, i32, i32)>, black18: &Vec<Vec<i32>>) -> i32 {
            if black15 == black16.len() {
                for black19 in 1..8 {
                    let mut black20 = 0;
                    let mut black21 = std::collections::HashSet::new();
                    for black22 in 0..black16.len() {
                        let (black23, black24, black25) = black17[black22];
                        let black26 = black19.min(black25);
                        let (black27, black28) = (black18[black22][0] + black23 * black26, black18[black22][1] + black24 * black26);
                        if black21.contains(&(black27, black28)) { return 0; }
                        black21.insert((black27, black28));
                        if black26 < black25 { black20 += 1; }
                    }
                    if black20 == 0 { break; }
                }
                return 1;
            }
            let mut black29 = 0;
            for &black30 in &black16[black15] {
                black17[black15] = black30;
                black29 += black14(black15 + 1, black16, black17, black18);
            }
            black29
        }
        black14(0, &black4, &mut black13, &black2)
    }
}
