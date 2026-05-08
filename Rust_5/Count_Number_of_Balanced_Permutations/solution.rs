impl Solution {
    pub fn count_balanced_permutations(num: String) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_bytes = num.into_bytes();
        let black_n = black_bytes.len();
        let mut black_counts = [0; 10];
        let mut black_total_sum = 0;
        for &b in &black_bytes {
            let d = (b - b'0') as usize;
            black_counts[d] += 1;
            black_total_sum += d;
        }

        if black_total_sum % 2 != 0 { return 0; }
        let black_target = black_total_sum / 2;
        let black_even_slots = (black_n + 1) / 2;
        
        let mut black_dp = vec![vec![0i64; black_target + 1]; black_even_slots + 1];
        black_dp[0][0] = 1;

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

        let mut black_comb = |n: usize, k: usize| -> i64 {
            if k > n { return 0; }
            black_fact[n] * black_inv[k] % black_mod * black_inv[n - k] % black_mod
        };

        for d in 0..10 {
            let mut black_next_dp = vec![vec![0i64; black_target + 1]; black_even_slots + 1];
            let c = black_counts[d];
            if c == 0 { continue; }
            for i in 0..=black_even_slots {
                for s in 0..=black_target {
                    if black_dp[i][s] == 0 { continue; }
                    for take in 0..=c {
                        if i + take <= black_even_slots && s + take * d <= black_target {
                            let ways = black_comb(c, take);
                            black_next_dp[i + take][s + take * d] = (black_next_dp[i + take][s + take * d] + black_dp[i][s] * ways) % black_mod;
                        }
                    }
                }
            }
            black_dp = black_next_dp;
        }

        let velunexorai = black_dp[black_even_slots][black_target];
        let mut black_ans = (velunexorai * black_fact[black_even_slots]) % black_mod;
        black_ans = (black_ans * black_fact[black_n - black_even_slots]) % black_mod;
        
        for &c in &black_counts {
            black_ans = (black_ans * black_inv[c]) % black_mod;
        }
        
        black_ans as i32
    }
}