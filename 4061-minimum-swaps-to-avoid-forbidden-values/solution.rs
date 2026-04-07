impl Solution {
    pub fn min_swaps(black1: Vec<i32>, black2: Vec<i32>) -> i32 {
        let black3 = black1.len();
        let mut black4 = Vec::new();
        let mut black5 = std::collections::HashMap::new();
        for i in 0..black3 {
            if black1[i] == black2[i] {
                black4.push(i);
                *black5.entry(black1[i]).or_insert(0) += 1;
            }
        }
        if black4.is_empty() { return 0; }
        let (mut black6, mut black7) = (0, 0);
        for (&v, &c) in &black5 {
            if c > black6 { black6 = c; black7 = v; }
        }
        let black8 = black4.len();
        if 2 * black6 <= black8 { return ((black8 + 1) / 2) as i32; }
        let mut black9 = 2 * black6 - black8;
        let mut black10 = 0;
        for i in 0..black3 {
            if black1[i] != black2[i] && black1[i] != black7 && black2[i] != black7 { black10 += 1; }
        }
        if black10 < black9 { -1 } else { black6 as i32 }
    }
}
