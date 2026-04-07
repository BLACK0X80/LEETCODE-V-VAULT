impl Solution {
    pub fn consecutive_numbers_sum(black1: i32) -> i32 {
        let mut black2 = 0;
        let mut black3 = 1;
        while black3 * (black3 - 1) / 2 < black1 {
            if (black1 - black3 * (black3 - 1) / 2) % black3 == 0 { black2 += 1; }
            black3 += 1;
        }
        black2
    }
}
