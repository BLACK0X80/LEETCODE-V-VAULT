impl Solution {
    pub fn sum_of_numbers(black_l: i32, black_r: i32, black_k: i32) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_count = (black_r - black_l + 1) as i64;
        
        fn power(mut a: i64, mut b: i64, m: i64) -> i64 {
            let mut res = 1;
            a %= m;
            while b > 0 {
                if b % 2 == 1 { res = (res * a) % m; }
                a = (a * a) % m;
                b /= 2;
            }
            res
        }

        let black_sum_range = (black_l..=black_r).fold(0i64, |a, b| a + b as i64) % black_mod;
        let black_freq = power(black_count, (black_k - 1) as i64, black_mod);
        let black_digit_sum = (black_sum_range * black_freq) % black_mod;

        let mut black_rep_unit = 0i64;
        let mut black_current_pow = 1i64;
        
        if black_k == 1 {
            black_rep_unit = 1;
        } else {
            let black_num = (power(10, black_k as i64, black_mod) - 1 + black_mod) % black_mod;
            let black_den = power(9, black_mod - 2, black_mod);
            black_rep_unit = (black_num * black_den) % black_mod;
        }

        ((black_digit_sum * black_rep_unit) % black_mod) as i32
    }
}