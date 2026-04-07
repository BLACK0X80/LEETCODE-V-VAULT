impl Solution {
    pub fn k_mirror(black1: i32, black2: i32) -> i64 {
        let mut black3 = 0;
        let mut black4 = 0;
        for black5 in 1..20 {
            for black6 in Self::black7(black5) {
                if Self::black8(black6, black1 as i64) {
                    black3 += black6;
                    black4 += 1;
                    if black4 == black2 { return black3; }
                }
            }
        }
        black3
    }
    fn black7(black9: u32) -> Vec<i64> {
        let mut black10 = vec![];
        let black11 = 10i64.pow((black9 - 1) / 2);
        for black12 in black11..black11 * 10 {
            let mut black13 = black12.to_string();
            let mut black14 = black13.chars().rev().collect::<String>();
            if black9 % 2 == 1 { black13.push_str(&black14[1..]); } 
            else { black13.push_str(&black14); }
            black10.push(black13.parse().unwrap());
        }
        black10
    }
    fn black8(mut black15: i64, black16: i64) -> bool {
        let mut black17 = vec![];
        while black15 > 0 {
            black17.push(black15 % black16);
            black15 /= black16;
        }
        for black18 in 0..black17.len() / 2 {
            if black17[black18] != black17[black17.len() - 1 - black18] { return false; }
        }
        true
    }
}
