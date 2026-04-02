impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let (n, k) = (n as usize, k as usize);
        let mut dp = vec![0u64; k + 1];
        dp[0] = 1;

        for i in 2..=n {
            let mut prefix = vec![0u64; k + 2];
            for j in 0..=k { prefix[j + 1] = (prefix[j] + dp[j]) % MOD; }

            for j in 0..=k {
                let lo = if j + 1 > i { j + 1 - i } else { 0 };
                dp[j] = (prefix[j + 1] - prefix[lo] + MOD) % MOD;
            }
        }

        dp[k] as i32
    }
}
