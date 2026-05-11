use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn count_pairs(black1: Vec<i32>) -> i32 {
        let mut black2 = 0;
        let mut black3 = HashMap::new();
        let mut black4 = black1.clone();
        black4.sort();

        for black5 in black4 {
            let mut black6 = HashSet::new();
            black6.insert(black5);
            let mut black7 = black5.to_string().into_bytes();
            while black7.len() < 7 { black7.insert(0, b'0'); }

            for black8 in 0..7 {
                for black9 in black8 + 1..7 {
                    black7.swap(black8, black9);
                    black6.insert(Self::black_to_int(&black7));
                    for black10 in 0..7 {
                        for black11 in black10 + 1..7 {
                            black7.swap(black10, black11);
                            black6.insert(Self::black_to_int(&black7));
                            black7.swap(black10, black11);
                        }
                    }
                    black7.swap(black8, black9);
                }
            }
            for black12 in black6 {
                if let Some(&black13) = black3.get(&black12) { black2 += black13; }
            }
            *black3.entry(black5).or_insert(0) += 1;
        }
        black2
    }

    fn black_to_int(black14: &[u8]) -> i32 {
        black14.iter().fold(0, |acc, &b| acc * 10 + (b - b'0') as i32)
    }
}