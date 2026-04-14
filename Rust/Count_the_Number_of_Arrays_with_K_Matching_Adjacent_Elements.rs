impl Solution {
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_n = n as usize;
        let black_m = m as i64;
        let black_k = k as usize;

        let mut black_fact = vec![1i64; black_n + 1];
        let mut black_inv = vec![1i64; black_n + 1];
        for i in 1..=black_n { black_fact[i] = (black_fact[i - 1] * i as i64) % black_mod; }
        
        fn power(mut a: i64, mut b: i64, m: i64) -> i64 {
            let mut res = 1;
            while b > 0 {
                if b % 2 == 1 { res = (res * a) % m; }
                a = (a * a) % m;
                b /= 2;
            }
            res
        }
        black_inv[black_n] = power(black_fact[black_n], black_mod - 2, black_mod);
        for i in (0..black_n).rev() { black_inv[i] = (black_inv[i + 1] * (i + 1) as i64) % black_mod; }

        let ncr = |n: usize, r: usize| -> i64 {
            if r > n { return 0; }
            black_fact[n] * black_inv[r] % black_mod * black_inv[n - r] % black_mod
        };

        
        let mut black_res = black_m * ncr(black_n - 1, black_k) % black_mod;
        black_res = (black_res * power(black_m - 1, (black_n - 1 - black_k) as i64, black_mod)) % black_mod;

        black_res as i32
    }
}