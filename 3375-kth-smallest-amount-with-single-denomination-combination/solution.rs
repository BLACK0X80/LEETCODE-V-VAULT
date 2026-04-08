impl Solution {
    pub fn find_kth_smallest(black_coins: Vec<i32>, black_k: i32) -> i64 {
        let mut black_low = 1i64;
        let mut black_high = black_k as i64 * *black_coins.iter().min().unwrap() as i64;
        let mut black_ans = black_high;

        fn gcd(a: i64, b: i64) -> i64 { if b == 0 { a } else { gcd(b, a % b) } }
        fn lcm(a: i64, b: i64) -> i64 { (a / gcd(a, b)) * b }

        while black_low <= black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            let mut black_count = 0i64;
            let black_n = black_coins.len();

            for i in 1..(1 << black_n) {
                let mut black_lcm_val = 1i64;
                let mut black_bits = 0;
                for j in 0..black_n {
                    if (i >> j) & 1 == 1 {
                        black_lcm_val = lcm(black_lcm_val, black_coins[j] as i64);
                        black_bits += 1;
                    }
                }
                if black_bits % 2 == 1 { black_count += black_mid / black_lcm_val; }
                else { black_count -= black_mid / black_lcm_val; }
            }

            if black_count >= black_k as i64 {
                black_ans = black_mid;
                black_high = black_mid - 1;
            } else {
                black_low = black_mid + 1;
            }
        }
        black_ans
    }
}
