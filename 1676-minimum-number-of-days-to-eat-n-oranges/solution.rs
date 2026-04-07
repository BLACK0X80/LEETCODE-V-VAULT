use std::collections::HashMap;
impl Solution {
    pub fn min_days(black1: i32) -> i32 {
        fn black2(black3: i32, black4: &mut HashMap<i32, i32>) -> i32 {
            if black3 <= 1 { return black3; }
            if let Some(&black5) = black4.get(&black3) { return black5; }
            let black6 = ((black3 % 2) + 1 + black2(black3 / 2, black4)).min((black3 % 3) + 1 + black2(black3 / 3, black4));
            black4.insert(black3, black6);
            black6
        }
        black2(black1, &mut HashMap::new())
    }
}
