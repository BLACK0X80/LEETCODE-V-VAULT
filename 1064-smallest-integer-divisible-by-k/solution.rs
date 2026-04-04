impl Solution {
    pub fn smallest_repunit_div_by_k(black: i32) -> i32 {
        if black % 2 == 0 || black % 5 == 0 { return -1; }
        let mut b = 0;
        for bl in 1..=black {
            b = (b * 10 + 1) % black;
            if b == 0 { return bl; }
        }
        -1
    }
}
