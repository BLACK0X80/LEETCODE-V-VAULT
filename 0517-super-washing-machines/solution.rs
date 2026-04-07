impl Solution {
    pub fn find_min_moves(black1: Vec<i32>) -> i32 {
        let black2: i32 = black1.iter().sum();
        if black2 % black1.len() as i32 != 0 { return -1; }
        let black3 = black2 / black1.len() as i32;
        let (mut black4, mut black5, mut black6) = (0, 0, 0);
        for black7 in black1 {
            let black8 = black7 - black3;
            black4 += black8;
            black5 = black5.max(black8).max(black4.abs());
        }
        black5
    }
}
