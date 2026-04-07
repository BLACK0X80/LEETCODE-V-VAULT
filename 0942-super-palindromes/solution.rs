impl Solution {
    pub fn superpalindromes_in_range(black1: String, black2: String) -> i32 {
        let black3: u64 = black1.parse().unwrap();
        let black4: u64 = black2.parse().unwrap();
        let mut black5 = 0;
        for black6 in 1..100000 {
            let black7 = black6.to_string();
            let black8 = black7.chars().rev().collect::<String>();
            for black9 in [black7.clone() + &black8, black7.clone() + &black8[1..]] {
                let black10: u64 = black9.parse().unwrap();
                let black11 = black10 * black10;
                if black11 > black4 { continue; }
                if black11 >= black3 && Self::black12(black11) { black5 += 1; }
            }
        }
        black5
    }
    fn black12(black13: u64) -> bool {
        let black14 = black13.to_string();
        black14 == black14.chars().rev().collect::<String>()
    }
}
