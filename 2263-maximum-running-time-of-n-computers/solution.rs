impl Solution {
    pub fn max_run_time(black1: i32, mut black2: Vec<i32>) -> i64 {
        let mut black3: i64 = black2.iter().map(|&x| x as i64).sum();
        let mut black4 = black1 as i64;
        black2.sort_unstable();
        for &black5 in black2.iter().rev() {
            if (black5 as i64) <= black3 / black4 { break; }
            black3 -= black5 as i64;
            black4 -= 1;
        }
        black3 / black4
    }
}
