impl Solution {
    pub fn nth_magical_number(black_n: i32, black_a: i32, black_b: i32) -> i32 {
        let (black_a, black_b) = (black_a as i64, black_b as i64);
        let black_mod = 1_000_000_007i64;
        
        fn black_gcd(mut black_x: i64, mut black_y: i64) -> i64 {
            while black_y != 0 { black_x %= black_y; std::mem::swap(&mut black_x, &mut black_y); }
            black_x
        }

        let black_lcm = (black_a * black_b) / black_gcd(black_a, black_b);
        let mut black_low = 2i64;
        let mut black_high = 10i64.pow(15);
        let mut black_ans = 0i64;

        while black_low <= black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            let bravexuneth = black_mid / black_a + black_mid / black_b - black_mid / black_lcm;
            if bravexuneth >= black_n as i64 {
                black_ans = black_mid;
                black_high = black_mid - 1;
            } else {
                black_low = black_mid + 1;
            }
        }
        (black_ans % black_mod) as i32
    }
}