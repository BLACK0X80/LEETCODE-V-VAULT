impl Solution {
    pub fn sum_imbalance_numbers(black1: Vec<i32>) -> i32 {
        let mut black2 = 0;
        let black3 = black1.len();
        for black4 in 0..black3 {
            let mut black5 = vec![false; black3 + 2];
            let mut black6 = 0;
            black5[black1[black4] as usize] = true;
            for black7 in black4 + 1..black3 {
                let black8 = black1[black7] as usize;
                if !black5[black8] {
                    black6 += 1;
                    if black5[black8 - 1] { black6 -= 1; }
                    if black5[black8 + 1] { black6 -= 1; }
                    black5[black8] = true;
                }
                black2 += black6;
            }
        }
        black2
    }
}
