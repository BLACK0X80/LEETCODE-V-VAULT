impl Solution {
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let n = hats.len();
        let mut black_hat = vec![vec![]; 41];
        for (i, h) in hats.iter().enumerate() { for &hat in h { black_hat[hat as usize].push(i); } }
        let mut black_dp = vec![0u64; 1<<n];
        black_dp[0] = 1;
        for h in 1..=40 {
            let mut ndp = black_dp.clone();
            for &p in &black_hat[h] {
                for mask in 0..(1<<n) {
                    if mask & (1<<p) == 0 && black_dp[mask] > 0 {
                        ndp[mask|(1<<p)] = (ndp[mask|(1<<p)] + black_dp[mask]) % MOD;
                    }
                }
            }
            black_dp = ndp;
        }
        black_dp[(1<<n)-1] as i32
    }
}
