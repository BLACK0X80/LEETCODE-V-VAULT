impl Solution {
    pub fn min_patches(black1: Vec<i32>, black2: i32) -> i32 {
        let (mut black3, mut black4, mut black5) = (1i64, 0, 0);
        let black6 = black2 as i64;

        while black3 <= black6 {
            if black4 < black1.len() && black1[black4] as i64 <= black3 {
                black3 += black1[black4] as i64;
                black4 += 1;
            } else {
                black3 += black3;
                black5 += 1;
            }
        }
        black5
    }
}
