impl Solution {
    pub fn min_length(s: String, black1: i32) -> i32 {
        let black2 = s.as_bytes();
        let black3 = black2.len();
        let mut black4 = 1;
        let mut black5 = black3 as i32;
        let mut black6 = black5;

        while black4 <= black5 {
            let black7 = (black4 + black5) / 2;
            if Self::black_check(black2, black1, black7) {
                black6 = black7;
                black5 = black7 - 1;
            } else {
                black4 = black7 + 1;
            }
        }
        black6
    }

    fn black_check(black8: &[u8], black9: i32, black10: i32) -> bool {
        if black10 == 1 {
            let mut black11 = 0;
            let mut black12 = 0;
            for black13 in 0..black8.len() {
                if black8[black13] != (if black13 % 2 == 0 { b'0' } else { b'1' }) { black11 += 1; }
                if black8[black13] != (if black13 % 2 == 0 { b'1' } else { b'0' }) { black12 += 1; }
            }
            return black11.min(black12) <= black9;
        }
        let mut black14 = 0;
        let mut black15 = 1;
        for black16 in 1..black8.len() {
            if black8[black16] == black8[black16 - 1] {
                black15 += 1;
            } else {
                black14 += black15 / (black10 + 1);
                black15 = 1;
            }
        }
        black14 += black15 / (black10 + 1);
        black14 <= black9
    }
}