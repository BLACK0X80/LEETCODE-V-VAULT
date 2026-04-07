impl Solution {
    pub fn strong_password_checker(black1: String) -> i32 {
        let black2 = black1.as_bytes();
        let black3 = black2.len();
        let (mut black4, mut black5, mut black6) = (1, 1, 1);
        for &b in black2 {
            if b.is_ascii_lowercase() { black4 = 0; }
            if b.is_ascii_uppercase() { black5 = 0; }
            if b.is_ascii_digit() { black6 = 0; }
        }
        let black7 = black4 + black5 + black6;
        let mut black8 = Vec::new();
        let mut i = 0;
        while i < black3 {
            let mut j = i;
            while j < black3 && black2[j] == black2[i] { j += 1; }
            if j - i >= 3 { black8.push((j - i) as i32); }
            i = j;
        }

        if black3 < 6 {
            return std::cmp::max(black7, 6 - black3 as i32);
        } else if black3 <= 20 {
            let mut black9 = 0;
            for r in black8 { black9 += r / 3; }
            return std::cmp::max(black7, black9);
        } else {
            let black10 = black3 as i32 - 20;
            let mut black11 = 0;
            let mut black12 = [0; 3];
            for r in black8 {
                black11 += r / 3;
                black12[(r % 3) as usize] += 1;
            }
            let mut black13 = black10;
            let black14 = std::cmp::min(black13, black12[0]);
            black11 -= black14;
            black13 -= black14;
            let black15 = std::cmp::min(black13, black12[1] * 2);
            black11 -= black15 / 2;
            black13 -= black15;
            black11 -= black13 / 3;
            return black10 + std::cmp::max(black7, black11);
        }
    }
}
