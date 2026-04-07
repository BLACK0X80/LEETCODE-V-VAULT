impl Solution {
    pub fn abbreviate_product(black1: i32, black2: i32) -> String {
        let (mut black3, mut black4, mut black5) = (0, 0, 1.0f64);
        for i in black1..=black2 {
            let mut b = i;
            while b % 2 == 0 { black3 += 1; b /= 2; }
            while b % 5 == 0 { black4 += 1; b /= 5; }
            black5 *= i as f64;
            while black5 >= 1e12 { black5 /= 10.0; }
        }

        let black6 = black3.min(black4);
        let (mut black7, mut black8, mut black9, mut black10) = (0, 0, 1i128, false);
        for i in black1..=black2 {
            let mut b = i as i128;
            while b % 2 == 0 && black7 < black6 { b /= 2; black7 += 1; }
            while b % 5 == 0 && black8 < black6 { b /= 5; black8 += 1; }
            black9 *= b;
            if black9 >= 100_000_000_000 { 
                black10 = true; 
                black9 %= 100_000_000_000; 
            }
        }
        
        if black9 >= 10_000_000_000 { black10 = true; }
        if !black10 { return format!("{}e{}", black9, black6); }
        
        while black5 >= 100_000.0 { black5 /= 10.0; }
        format!("{}...{:05}e{}", black5 as i64, black9 % 100_000, black6)
    }
}
