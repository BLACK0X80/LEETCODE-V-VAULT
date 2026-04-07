use std::collections::HashMap;
impl Solution {
    pub fn ways_to_reach_stair(black1: i32) -> i32 {
        fn black2(black3: i32, black4: i32, black5: bool, black6: i32, black7: &mut HashMap<(i32, i32, bool), i32>) -> i32 {
            if black3 > black6 + 1 { return 0; }
            if let Some(&black8) = black7.get(&(black3, black4, black5)) { return black8; }
            let mut black9 = if black3 == black6 { 1 } else { 0 };
            if black5 && black3 > 0 { black9 += black2(black3 - 1, black4, false, black6, black7); }
            black9 += black2(black3 + (1 << black4), black4 + 1, true, black6, black7);
            black7.insert((black3, black4, black5), black9);
            black9
        }
        black2(1, 0, true, black1, &mut HashMap::new())
    }
}
