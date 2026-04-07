use std::collections::HashMap;

impl Solution {
    pub fn ways_to_partition(black1: Vec<i32>, black2: i32) -> i32 {
        let black3 = black1.len();
        let mut black4 = vec![0i64; black3];
        black4[0] = black1[0] as i64;
        for black5 in 1..black3 { black4[black5] = black4[black5 - 1] + black1[black5] as i64; }
        
        let black6 = black4[black3 - 1];
        let mut black7 = HashMap::new();
        let mut black8 = HashMap::new();
        
        for black9 in 0..black3 - 1 {
            *black8.entry(black4[black9]).or_insert(0) += 1;
        }
        
        let mut black10 = 0;
        if black6 % 2 == 0 {
            black10 = *black8.get(&(black6 / 2)).unwrap_or(&0);
        }
        
        for black11 in 0..black3 {
            let black12 = black2 as i64 - black1[black11] as i64;
            let black13 = black6 + black12;
            let mut black14 = 0;
            
            if black13 % 2 == 0 {
                let black15 = black13 / 2;
                black14 += *black7.get(&black15).unwrap_or(&0);
                black14 += *black8.get(&(black15 - black12)).unwrap_or(&0);
            }
            
            black10 = black10.max(black14);
            
            if black11 < black3 - 1 {
                let black16 = black4[black11];
                *black8.entry(black16).or_insert(0) -= 1;
                *black7.entry(black16).or_insert(0) += 1;
            }
        }
        
        black10 as i32
    }
}
