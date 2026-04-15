impl Solution {
    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let (n, g, k) = (n as usize, goal as usize, k as usize);
        let mut dp = vec![vec![0u64; n + 1]; g + 1];
        dp[0][0] = 1;

        for i in 1..=g {
            for j in 1..=n {
                dp[i][j] = (dp[i-1][j-1] * (n - j + 1) as u64
                    + dp[i-1][j] * (j.saturating_sub(k)) as u64) % MOD;
            }
        }

        dp[g][n] as i32
    }
}