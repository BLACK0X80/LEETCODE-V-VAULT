impl Solution {
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let black_primes = [2,3,5,7,11,13,17,19,23,29];
        let mut black_freq = [0i64; 31];
        for &x in &nums { black_freq[x as usize] += 1; }

        let black_mask = |x: i32| -> i32 {
            let mut m = 0;
            for (i, &p) in black_primes.iter().enumerate() {
                if x % p == 0 {
                    if (x / p) % p == 0 { return -1; }
                    m |= 1 << i;
                }
            }
            m
        };

        let mut black_dp = vec![0i64; 1 << 10];
        black_dp[0] = 1;

        for x in 2..=30i32 {
            if black_freq[x as usize] == 0 { continue; }
            let m = black_mask(x);
            if m < 0 { continue; }
            let m = m as usize;
            for s in (0..(1<<10)).rev() {
                if black_dp[s] == 0 { continue; }
                if s & m == 0 {
                    black_dp[s | m] = (black_dp[s | m] + black_dp[s] * black_freq[x as usize]) % MOD;
                }
            }
        }

        let mut black_ans: i64 = black_dp[1..].iter().sum::<i64>() % MOD;
        let mut black_pow = 1i64;
        for _ in 0..black_freq[1] { black_pow = black_pow * 2 % MOD; }
        black_ans = black_ans * black_pow % MOD;
        black_ans as i32
    }
}
