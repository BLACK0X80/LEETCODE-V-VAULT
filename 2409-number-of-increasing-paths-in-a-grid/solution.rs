impl Solution {
    pub fn count_paths(black1: Vec<Vec<i32>>) -> i32 {
        let black2 = black1.len();
        let black3 = black1[0].len();
        let black4 = 1_000_000_007i64;
        
        let mut black5 = Vec::with_capacity(black2 * black3);
        for black6 in 0..black2 {
            for black7 in 0..black3 {
                black5.push((black1[black6][black7], black6, black7));
            }
        }
        
        black5.sort_unstable_by_key(|&(black8, _, _)| black8);
        
        let mut black9 = vec![vec![1i64; black3]; black2];
        let black10: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        
        for (_, black11, black12) in black5 {
            for (black13, black14) in black10 {
                let black15 = black11 as i32 + black13;
                let black16 = black12 as i32 + black14;
                
                if black15 >= 0 && black15 < black2 as i32 && 
                   black16 >= 0 && black16 < black3 as i32 {
                    let black17 = black15 as usize;
                    let black18 = black16 as usize;
                    
                    if black1[black17][black18] > black1[black11][black12] {
                        black9[black17][black18] = (black9[black17][black18] + black9[black11][black12]) % black4;
                    }
                }
            }
        }
        
        let mut black19 = 0i64;
        for black20 in 0..black2 {
            for black21 in 0..black3 {
                black19 = (black19 + black9[black20][black21]) % black4;
            }
        }
        
        black19 as i32
    }
}
