impl Solution {
    pub fn max_sum(black1: Vec<i32>, black2: Vec<i32>) -> i32 {
        let (mut i, mut j, mut black3, mut black4) = (0, 0, 0i64, 0i64);
        while i < black1.len() || j < black2.len() {
            if i < black1.len() && (j == black2.len() || black1[i] < black2[j]) {
                black3 += black1[i] as i64; i += 1;
            } else if j < black2.len() && (i == black1.len() || black2[j] < black1[i]) {
                black4 += black2[j] as i64; j += 1;
            } else {
                let black5 = black3.max(black4) + black1[i] as i64;
                black3 = black5; black4 = black5;
                i += 1; j += 1;
            }
        }
        (black3.max(black4) % 1_000_000_007) as i32
    }
}