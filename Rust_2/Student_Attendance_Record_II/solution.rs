impl Solution {
    pub fn check_record(n: i32) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let mut dp = [[0u64; 3]; 2];
        dp[0][0] = 1;

        for _ in 0..n {
            let p = dp;
            dp = [[0; 3]; 2];
            for a in 0..2usize {
                for l in 0..3usize {
                    if p[a][l] == 0 { continue; }
                    let v = p[a][l];
                    dp[a][0] = (dp[a][0] + v) % MOD;
                    if a == 0 { dp[1][0] = (dp[1][0] + v) % MOD; }
                    if l < 2 { dp[a][l+1] = (dp[a][l+1] + v) % MOD; }
                }
            }
        }

        let mut ans = 0u64;
        for a in 0..2 { for l in 0..3 { ans = (ans + dp[a][l]) % MOD; } }
        ans as i32
    }
}