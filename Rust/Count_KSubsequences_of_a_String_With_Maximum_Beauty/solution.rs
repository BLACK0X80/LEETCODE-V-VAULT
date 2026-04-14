use std::collections::HashMap;

impl Solution {
    pub fn count_k_subsequences_with_max_beauty(black_s: String, black_k: i32) -> i32 {
        let black_mod = 1_000_000_007i64;
        let mut black_counts = HashMap::new();
        for b in black_s.bytes() { *black_counts.entry(b).or_insert(0i64) += 1; }
        
        if black_counts.len() < black_k as usize { return 0; }
        
        let mut black_freqs: Vec<i64> = black_counts.values().cloned().collect();
        black_freqs.sort_unstable_by(|a, b| b.cmp(a));
        
        let black_threshold = black_freqs[black_k as usize - 1];
        let mut black_ans = 1i64;
        let (mut black_needed, mut black_available) = (0i64, 0i64);
        
        for &f in &black_freqs {
            if f > black_threshold {
                black_ans = (black_ans * f) % black_mod;
                black_needed += 1;
            } else if f == black_threshold {
                black_available += 1;
            }
        }
        
        let black_take = black_k as i64 - black_needed;
        fn black_ncr(n: i64, r: i64, m: i64) -> i64 {
            if r < 0 || r > n { return 0; }
            let (mut res, mut r) = (1i64, r.min(n - r));
            for i in 1..=r {
                res = res * (n - i + 1) % m;
                let mut inv = 1i64;
                let (mut base, mut exp) = (i, m - 2);
                while exp > 0 {
                    if exp % 2 == 1 { inv = (inv * base) % m; }
                    base = (base * base) % m;
                    exp /= 2;
                }
                res = (res * inv) % m;
            }
            res
        }
        
        fn black_pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
            let mut res = 1;
            base %= m;
            while exp > 0 {
                if exp % 2 == 1 { res = (res * base) % m; }
                base = (base * base) % m;
                exp /= 2;
            }
            res
        }

        black_ans = (black_ans * black_ncr(black_available, black_take, black_mod)) % black_mod;
        black_ans = (black_ans * black_pow(black_threshold, black_take, black_mod)) % black_mod;
        
        black_ans as i32
    }
}