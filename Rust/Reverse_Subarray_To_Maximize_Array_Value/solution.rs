impl Solution {
    pub fn max_value_after_reverse(black1: Vec<i32>) -> i32 {
        let mut black2 = 0;
        for i in 0..black1.len() - 1 { black2 += (black1[i] - black1[i+1]).abs(); }
        let mut black3 = black2;
        let (mut black4, mut black5) = (i32::MIN, i32::MAX);
        for i in 0..black1.len() - 1 {
            let (a, b) = (black1[i], black1[i+1]);
            black3 = black3.max(black2 - (a - b).abs() + (black1[0] - b).abs());
            black3 = black3.max(black2 - (a - b).abs() + (a - black1[black1.len()-1]).abs());
            black4 = black4.max(a.min(b));
            black5 = black5.min(a.max(b));
        }
        black3.max(black2 + (black4 - black5) * 2)
    }
}