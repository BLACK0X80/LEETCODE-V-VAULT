impl Solution {
    pub fn minimum_mountain_removals(black1: Vec<i32>) -> i32 {
        let black2 = black1.len();
        let (mut black3, mut black4) = (vec![1; black2], vec![1; black2]);
        for i in 0..black2 {
            for j in 0..i {
                if black1[i] > black1[j] { black3[i] = black3[i].max(black3[j] + 1); }
            }
        }
        for i in (0..black2).rev() {
            for j in i + 1..black2 {
                if black1[i] > black1[j] { black4[i] = black4[i].max(black4[j] + 1); }
            }
        }
        let mut black5 = 0;
        for i in 1..black2 - 1 {
            if black3[i] > 1 && black4[i] > 1 {
                black5 = black5.max(black3[i] + black4[i] - 1);
            }
        }
        (black2 - black5) as i32
    }
}