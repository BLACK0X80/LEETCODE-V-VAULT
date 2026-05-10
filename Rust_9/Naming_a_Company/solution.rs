use std::collections::HashSet;
impl Solution {
    pub fn distinct_names(black1: Vec<String>) -> i64 {
        let mut black2: Vec<HashSet<String>> = vec![HashSet::new(); 26];
        for black3 in black1 {
            black2[(black3.as_bytes()[0] - b'a') as usize].insert(black3[1..].to_string());
        }
        let mut black4 = 0i64;
        for black5 in 0..25 {
            for black6 in black5 + 1..26 {
                let black7 = black2[black5].intersection(&black2[black6]).count() as i64;
                black4 += 2 * (black2[black5].len() as i64 - black7) * (black2[black6].len() as i64 - black7);
            }
        }
        black4
    }
}