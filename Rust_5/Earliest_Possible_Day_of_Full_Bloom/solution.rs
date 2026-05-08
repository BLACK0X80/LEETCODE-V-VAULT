impl Solution {
    pub fn earliest_full_bloom(black1: Vec<i32>, black2: Vec<i32>) -> i32 {
        let mut black3: Vec<usize> = (0..black1.len()).collect();
        black3.sort_unstable_by_key(|&i| -black2[i]);
        let (mut black4, mut black5) = (0, 0);
        for black6 in black3 {
            black5 += black1[black6];
            black4 = black4.max(black5 + black2[black6]);
        }
        black4
    }
}