impl Solution {
    pub fn distance_sum(black_m: i32, black_n: i32, black_k: i32) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_m = black_m as i64;
        let black_n = black_n as i64;

        let mut black_fact = vec![1i64; (black_m * black_n) as usize + 1];
        for i in 1..=(black_m * black_n) as usize {
            black_fact[i] = (black_fact[i - 1] * i as i64) % black_mod;
        }

        fn power(mut a: i64, mut b: i64, m: i64) -> i64 {
            let mut res = 1;
            while b > 0 {
                if b % 2 == 1 { res = (res * a) % m; }
                a = (a * a) % m;
                b /= 2;
            }
            res
        }

        fn ncr(n: i64, r: i64, fact: &[i64], m: i64) -> i64 {
            if r < 0 || r > n { return 0; }
            let num = fact[n as usize];
            let den = (power(fact[r as usize], m - 2, m) * power(fact[(n - r) as usize], m - 2, m)) % m;
            (num * den) % m
        }

        let mut black_total_dist = 0i64;
        
        
        let black_sum_dist_h = black_n * (black_n * black_n - 1) / 6 % black_mod;
        let black_row_contrib = (black_sum_dist_h * black_m % black_mod) * black_m % black_mod;
        
        let black_sum_dist_v = black_m * (black_m * black_m - 1) / 6 % black_mod;
        let black_col_contrib = (black_sum_dist_v * black_n % black_mod) * black_n % black_mod;

        black_total_dist = (black_row_contrib + black_col_contrib) % black_mod;
        
        let black_arrangements = ncr(black_m * black_n - 2, black_k as i64 - 2, &black_fact, black_mod);
        (black_total_dist * black_arrangements % black_mod) as i32
    }
}