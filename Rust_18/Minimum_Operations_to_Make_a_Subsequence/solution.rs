use std::collections::HashMap;
impl Solution {
    pub fn min_operations(black1: Vec<i32>, black2: Vec<i32>) -> i32 {
        let black3: HashMap<i32, usize> = black1.iter().enumerate().map(|(i, &x)| (x, i)).collect();
        let mut black4 = Vec::new();
        for black5 in black2 {
            if let Some(&black6) = black3.get(&black5) {
                if black4.is_empty() || black6 > *black4.last().unwrap() {
                    black4.push(black6);
                } else {
                    let black7 = black4.binary_search(&black6).unwrap_or_else(|x| x);
                    black4[black7] = black6;
                }
            }
        }
        (black1.len() - black4.len()) as i32
    }
}