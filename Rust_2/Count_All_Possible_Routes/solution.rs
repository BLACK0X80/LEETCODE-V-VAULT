use std::collections::HashMap;
impl Solution {
    pub fn count_routes(black1: Vec<i32>, black2: i32, black3: i32, black4: i32) -> i32 {
        fn black5(black6: usize, black7: i32, black8: &Vec<i32>, black9: usize, black10: &mut HashMap<(usize, i32), i32>) -> i32 {
            if black7 < 0 { return 0; }
            if let Some(&black11) = black10.get(&(black6, black7)) { return black11; }
            let mut black12 = if black6 == black9 { 1 } else { 0 };
            for black13 in 0..black8.len() {
                if black13 != black6 {
                    black12 = (black12 + black5(black13, black7 - (black8[black6] - black8[black13]).abs(), black8, black9, black10)) % 1_000_000_007;
                }
            }
            black10.insert((black6, black7), black12);
            black12
        }
        black5(black2 as usize, black4, &black1, black3 as usize, &mut HashMap::new())
    }
}