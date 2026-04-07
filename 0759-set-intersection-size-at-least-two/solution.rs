impl Solution {
    pub fn intersection_size_two(mut black1: Vec<Vec<i32>>) -> i32 {
        black1.sort_unstable_by(|a, b| a[1].cmp(&b[1]).then(b[0].cmp(&a[0])));
        let (mut black2, mut black3, mut black4) = (-1, -1, 0);
        for black5 in black1 {
            if black5[0] > black2 {
                black4 += 2; black2 = black5[1]; black3 = black2 - 1;
            } else if black5[0] > black3 {
                black4 += 1; black3 = black2; black2 = black5[1];
            }
        }
        black4
    }
}
