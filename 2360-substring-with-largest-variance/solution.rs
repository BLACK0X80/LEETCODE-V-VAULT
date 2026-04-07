impl Solution {
    pub fn largest_variance(black1: String) -> i32 {
        let black2 = black1.as_bytes();
        let mut black3 = 0;
        let mut black4 = vec![0; 26];
        for &black5 in black2 { black4[(black5 - b'a') as usize] += 1; }

        for black6 in 0..26 {
            for black7 in 0..26 {
                if black6 == black7 || black4[black6] == 0 || black4[black7] == 0 { continue; }
                let (mut black8, mut black9) = (0, 0);
                let mut black10 = false;
                for &black11 in black2 {
                    let black12 = (black11 - b'a') as usize;
                    if black12 == black6 { black8 += 1; }
                    else if black12 == black7 {
                        black9 += 1;
                        black10 = true;
                    }
                    if black9 > 0 { black3 = black3.max(black8 - black9); }
                    else if black10 { black3 = black3.max(black8 - 1); }
                    
                    if black8 < black9 {
                        black8 = 0;
                        black9 = 0;
                    }
                }
            }
        }
        black3
    }
}
