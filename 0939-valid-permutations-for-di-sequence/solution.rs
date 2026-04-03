impl Solution {
    pub fn num_perms_di_sequence(s: String) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![1u64; n + 1];

        for i in 0..n {
            let size = n - i + 1;
            let mut prefix = vec![0u64; size + 1];
            for j in 0..size {
                prefix[j + 1] = (prefix[j] + dp[j]) % MOD;
            }
            let new_size = size - 1;
            let mut ndp = vec![0u64; new_size];
            for j in 0..new_size {
                ndp[j] = if s[i] == b'I' {
                    prefix[j + 1]
                } else {
                    (prefix[size] - prefix[j + 1] + MOD) % MOD
                };
            }
            dp = ndp;
        }

        dp[0] as i32
    }
}
