use std::collections::HashSet;
impl Solution {
    pub fn count_good_integers(black1: i32, black2: i32) -> i64 {
        let mut black3 = HashSet::new();
        let black4 = (black1 + 1) / 2;
        let black5 = 10i64.pow(black4 as u32 - 1);
        let black6 = 10i64.pow(black4 as u32);

        for black7 in black5..black6 {
            let mut black8 = black7.to_string();
            let mut black9 = black8.chars().rev().collect::<String>();
            if black1 % 2 == 1 { black9.remove(0); }
            black8.push_str(&black9);
            let black10: i64 = black8.parse().unwrap();
            if black10 % black2 as i64 == 0 {
                let mut black11: Vec<char> = black8.chars().collect();
                black11.sort();
                black3.insert(black11);
            }
        }

        let mut black12 = 0;
        let black13 = Self::black_fact(black1 as i64);
        for black14 in black3 {
            let mut black15 = vec![0; 10];
            for black16 in black14 { black15[(black16 as u8 - b'0') as usize] += 1; }
            
            let mut black17 = black13;
            for black18 in 0..10 { black17 /= Self::black_fact(black15[black18]); }
            
            if black15[0] > 0 {
                let mut black19 = Self::black_fact(black1 as i64 - 1);
                black15[0] -= 1;
                for black20 in 0..10 { black19 /= Self::black_fact(black15[black20]); }
                black17 -= black19;
            }
            black12 += black17;
        }
        black12
    }

    fn black_fact(black21: i64) -> i64 {
        (1..=black21).product()
    }
}