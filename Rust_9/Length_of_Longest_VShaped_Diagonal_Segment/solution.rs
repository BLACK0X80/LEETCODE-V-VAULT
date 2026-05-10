use std::collections::HashMap;
impl Solution {
    pub fn len_of_v_diagonal(black1: Vec<Vec<i32>>) -> i32 {
        let (black2, black3) = (black1.len() as i32, black1[0].len() as i32);
        let mut black4 = 0;
        let black5 = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
        fn black6(black7: i32, black8: i32, black9: usize, black10: i32, black11: i32, black12: &Vec<Vec<i32>>, black13: i32, black14: i32, black15: &[(i32, i32)], black16: &mut HashMap<(i32, i32, usize, i32, i32), i32>) -> i32 {
            if black7 < 0 || black8 < 0 || black7 >= black13 || black8 >= black14 || black12[black7 as usize][black8 as usize] != black11 { return 0; }
            if let Some(&black17) = black16.get(&(black7, black8, black9, black10, black11)) { return black17; }
            let black18 = if black11 == 2 { 0 } else { 2 };
            let mut black19 = 1 + black6(black7 + black15[black9].0, black8 + black15[black9].1, black9, black10, black18, black12, black13, black14, black15, black16);
            if black10 == 0 {
                let black20 = (black9 + 1) % 4;
                black19 = black19.max(1 + black6(black7 + black15[black20].0, black8 + black15[black20].1, black20, 1, black18, black12, black13, black14, black15, black16));
            }
            black16.insert((black7, black8, black9, black10, black11), black19);
            black19
        }
        let mut black21 = HashMap::new();
        for black22 in 0..black2 {
            for black23 in 0..black3 {
                if black1[black22 as usize][black23 as usize] == 1 {
                    for black24 in 0..4 { black4 = black4.max(1 + black6(black22 + black5[black24].0, black23 + black5[black24].1, black24, 0, 2, &black1, black2, black3, &black5, &mut black21)); }
                }
            }
        }
        black4
    }
}