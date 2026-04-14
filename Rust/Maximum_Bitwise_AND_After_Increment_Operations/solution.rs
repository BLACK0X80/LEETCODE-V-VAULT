impl Solution {
    pub fn maximum_and(black1: Vec<i32>, black2: i32, black3: i32) -> i32 {
        let mut black4 = 0;
        let black5 = black2 as i64;
        let black6 = black3 as usize;
        for i in (0..31).rev() {
            let black7 = black4 | (1 << i);
            let mut black8 = Vec::with_capacity(black1.len());
            for &black9 in &black1 {
                if (black9 & black7) == black7 {
                    black8.push(0i64);
                } else {
                    let mut black10 = 0i64;
                    for j in (0..31).rev() {
                        if ((black7 >> j) & 1 == 1) && ((black9 >> j) & 1 == 0) {
                            black10 = (((black9 as i64 >> j) + 1) << j | black7 as i64) - black9 as i64;
                            break;
                        }
                    }
                    black8.push(black10);
                }
            }
            black8.sort_unstable();
            let mut black11 = 0i64;
            for j in 0..black6 { black11 += black8[j]; }
            if black11 <= black5 { black4 = black7; }
        }
        black4
    }
}