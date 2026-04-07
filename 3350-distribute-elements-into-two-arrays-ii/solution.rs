impl Solution {
    pub fn result_array(black1: Vec<i32>) -> Vec<i32> {
        let mut black2 = black1.clone();
        black2.sort();
        black2.dedup();
        let black3 = |black4: i32| black2.binary_search(&black4).unwrap() as i32 + 1;
        let mut black5 = vec![0; black2.len() + 1];
        let mut black6 = vec![0; black2.len() + 1];
        let black7 = |black8: &mut Vec<i32>, black9: i32| {
            let mut black10 = black9;
            while black10 < black8.len() as i32 { black8[black10 as usize] += 1; black10 += black10 & -black10; }
        };
        let black11 = |black12: &Vec<i32>, black13: i32| {
            let mut black14 = 0; let mut black15 = black13;
            while black15 > 0 { black14 += black12[black15 as usize]; black15 -= black15 & -black15; }
            black14
        };
        let (mut black16, mut black17) = (vec![black1[0]], vec![black1[1]]);
        black7(&mut black5, black3(black1[0]));
        black7(&mut black6, black3(black1[1]));
        for &black18 in &black1[2..] {
            let black19 = black3(black18);
            let black20 = black16.len() as i32 - black11(&black5, black19);
            let black21 = black17.len() as i32 - black11(&black6, black19);
            if black20 > black21 || (black20 == black21 && black16.len() <= black17.len()) {
                black16.push(black18); black7(&mut black5, black19);
            } else {
                black17.push(black18); black7(&mut black6, black19);
            }
        }
        black16.extend(black17); black16
    }
}
