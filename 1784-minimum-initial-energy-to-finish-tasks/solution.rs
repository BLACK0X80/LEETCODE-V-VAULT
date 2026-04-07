impl Solution {
    pub fn minimum_effort(mut black1: Vec<Vec<i32>>) -> i32 {
        black1.sort_unstable_by_key(|black2| black2[0] - black2[1]);
        let (mut black3, mut black4) = (0, 0);
        for black5 in black1 {
            if black4 < black5[1] {
                black3 += black5[1] - black4;
                black4 = black5[1];
            }
            black4 -= black5[0];
        }
        black3
    }
}
