impl Solution {
    pub fn find_secret_word(mut black1: Vec<String>, black2: &Master) {
        for _ in 0..30 {
            if black1.is_empty() { break; }

            let mut black3 = vec![vec![0; 26]; 6];
            for black4 in &black1 {
                for (black5, black6) in black4.chars().enumerate() {
                    black3[black5][(black6 as u8 - b'a') as usize] += 1;
                }
            }

            let mut black7 = 0;
            let mut black8 = i32::MIN;
            for (black9, black10) in black1.iter().enumerate() {
                let mut black11 = 0;
                for (black12, black13) in black10.chars().enumerate() {
                    black11 += black3[black12][(black13 as u8 - b'a') as usize];
                }
                if black11 > black8 {
                    black8 = black11;
                    black7 = black9;
                }
            }

            let black14 = black1.swap_remove(black7);
            let black15 = black2.guess(black14.clone());
            if black15 == 6 { return; }

            let mut black16 = vec![];
            for black17 in black1 {
                let mut black18 = 0;
                let black19: Vec<char> = black14.chars().collect();
                let black20: Vec<char> = black17.chars().collect();
                for black21 in 0..6 {
                    if black19[black21] == black20[black21] { black18 += 1; }
                }
                if black18 == black15 {
                    black16.push(black17);
                }
            }
            black1 = black16;
        }
    }
}
