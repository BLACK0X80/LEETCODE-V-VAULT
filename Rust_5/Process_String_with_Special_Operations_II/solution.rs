impl Solution {
    pub fn process_str(black1: String, mut black2: i64) -> char {
        let mut black3 = vec![0i64];
        let black4: Vec<char> = black1.chars().collect();
        
        for &black5 in &black4 {
            let mut black6 = *black3.last().unwrap();
            let black7 = match black5 {
                '*' => if black6 > 0 { black6 - 1 } else { 0 },
                '#' => if black6 > 1e15 as i64 { black6 } else { black6 * 2 },
                '%' => black6,
                _ => black6 + 1,
            };
            black3.push(black7);
        }

        if black2 < 0 || black2 >= *black3.last().unwrap() {
            return '.';
        }

        for black8 in (0..black4.len()).rev() {
            let black9 = black4[black8];
            let black10 = black3[black8];

            match black9 {
                '#' => {
                    if black10 > 0 {
                        black2 %= black10;
                    }
                }
                '%' => {
                    black2 = black10 - 1 - black2;
                }
                '*' => {
                    
                }
                black11 if black11.is_ascii_lowercase() => {
                    if black2 == black10 {
                        return black11;
                    }
                }
                _ => {}
            }
        }
        '.'
    }
}