impl Solution {
    pub fn maximum_good(black1: Vec<Vec<i32>>) -> i32 {
        let black2 = black1.len();
        let mut black3 = 0;
        for black4 in 0..(1 << black2) {
            let mut black5 = true;
            for black6 in 0..black2 {
                if (black4 >> black6) & 1 == 1 {
                    for black7 in 0..black2 {
                        if black1[black6][black7] != 2 && black1[black6][black7] != ((black4 >> black7) & 1) {
                            black5 = false;
                            break;
                        }
                    }
                }
                if !black5 { break; }
            }
            if black5 { black3 = black3.max((black4 as i32).count_ones() as i32); }
        }
        black3
    }
}