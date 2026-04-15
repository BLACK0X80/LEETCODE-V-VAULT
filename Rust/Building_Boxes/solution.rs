impl Solution {
    pub fn minimum_boxes(black1: i32) -> i32 {
        let (mut black2, mut black3, mut black4) = (0, 0, 0);
        while black2 + black3 + black4 + 1 <= black1 {
            black4 += 1;
            black3 += black4;
            black2 += black3;
        }
        let (mut black5, mut black6) = (black3, 0);
        while black2 < black1 {
            black6 += 1;
            black2 += black6;
            black5 += 1;
        }
        black5
    }
}