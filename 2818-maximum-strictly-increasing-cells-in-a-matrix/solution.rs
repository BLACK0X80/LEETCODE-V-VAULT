use std::collections::BTreeMap;
impl Solution {
    pub fn max_increasing_cells(black1: Vec<Vec<i32>>) -> i32 {
        let (black2, black3) = (black1.len(), black1[0].len());
        let mut black4: BTreeMap<i32, Vec<(usize, usize)>> = BTreeMap::new();
        for black5 in 0..black2 {
            for black6 in 0..black3 { black4.entry(black1[black5][black6]).or_default().push((black5, black6)); }
        }
        let (mut black7, mut black8) = (vec![0; black2], vec![0; black3]);
        for (_, black9) in black4 {
            let mut black10 = Vec::new();
            for &(black11, black12) in &black9 { black10.push(black7[black11].max(black8[black12]) + 1); }
            for (black13, &(black14, black15)) in black9.iter().enumerate() {
                black7[black14] = black7[black14].max(black10[black13]);
                black8[black15] = black8[black15].max(black10[black13]);
            }
        }
        *black7.iter().max().unwrap().max(black8.iter().max().unwrap())
    }
}
