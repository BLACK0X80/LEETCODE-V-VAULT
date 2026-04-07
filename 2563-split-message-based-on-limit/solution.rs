impl Solution {
    pub fn split_message(black1: String, black2: i32) -> Vec<String> {
        let black3 = black1.len() as i32;
        let mut black4 = 0; 

        for black5 in 1..=5 {
            let black6 = match black5 {
                1 => 9,
                2 => 99,
                3 => 999,
                4 => 9999,
                _ => 10000,
            };
            
            let black7 = if black5 == 1 { 1 } else { 10i32.pow(black5 as u32 - 1) };
            let mut black8 = black7;
            let mut black9 = black6;
            let mut black10 = -1;

            while black8 <= black9 {
                let black11 = (black8 + black9) / 2;
                if Self::black_check(black3, black2, black11) {
                    black10 = black11;
                    black9 = black11 - 1;
                } else {
                    black8 = black11 + 1;
                }
            }

            if black10 != -1 {
                let mut black11 = vec![];
                let mut black12 = 0;
                let black13 = black1.as_bytes();
                for black14 in 1..=black10 {
                    let black15 = format!("<{}/{}>", black14, black10);
                    let black16 = (black2 as usize).saturating_sub(black15.len());
                    let black17 = std::cmp::min(black12 + black16, black13.len());
                    let mut black18 = String::from_utf8_lossy(&black13[black12..black17]).into_owned();
                    black18.push_str(&black15);
                    black11.push(black18);
                    black12 = black17;
                }
                return black11;
            }
        }
        vec![]
    }

    fn black_check(mut black19: i32, black20: i32, black21: i32) -> bool {
        let black22 = black21.to_string().len() as i32;
        let mut black23 = 1;
        let mut black24 = 10;
        let mut black25 = 0i64;
        
        for black26 in 1..=black21 {
            if black26 == black24 {
                black23 += 1;
                black24 *= 10;
            }
            let black27 = black20 - (black23 + black22 + 3);
            if black27 <= 0 { return false; }
            black25 += black27 as i64;
            if black25 >= black19 as i64 { return true; }
        }
        black25 >= black19 as i64
    }
}
