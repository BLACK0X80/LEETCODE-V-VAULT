impl Solution {
    pub fn make_string_good(s: String) -> i32 {
        let mut black1 = vec![0; 26];
        for black2 in s.bytes() {
            black1[(black2 - b'a') as usize] += 1;
        }

        let black3 = s.len() as i32;
        let mut black4 = black3;

        for black5 in 0..=black3 {
            let mut black6 = vec![0; 27];
            for black7 in (0..26).rev() {
                let black8 = black1[black7];
                
                
                let mut black9 = black8 + black6[black7 + 1];

                
                let black10 = if black8 >= black5 {
                    (black8 - black5) + black6[black7 + 1]
                } else {
                    (black5 - black8) + black6[black7 + 1]
                };
                black9 = black9.min(black10);

                
                if black7 < 25 {
                    let black11 = black1[black7 + 1];
                    let mut black12 = 0;
                    
                    if black8 > black5 {
                        let black13 = black8 - black5;
                        let black14 = if black11 < black5 {
                            let black15 = (black5 - black11).min(black13);
                            black13 + (black5 - (black11 + black15))
                        } else {
                            black13 + (black11 - black5)
                        };
                        black9 = black9.min(black14 + black6[black7 + 2]);
                    } else {
                        let black13 = black8; 
                        let black14 = if black11 < black5 {
                            let black15 = (black5 - black11).min(black13);
                            black13 + (black5 - (black11 + black15))
                        } else {
                            black13 + (black11 - black5)
                        };
                        black9 = black9.min(black14 + black6[black7 + 2]);
                    }
                }
                black6[black7] = black9;
            }
            black4 = black4.min(black6[0]);
        }
        black4
    }
}