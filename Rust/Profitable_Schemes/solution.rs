impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let (n, mp) = (n as usize, min_profit as usize);
        let mut dp = vec![vec![0u64; mp + 1]; n + 1];
        dp[0][0] = 1;

        for i in 0..group.len() {
            let (g, p) = (group[i] as usize, profit[i] as usize);
            for members in (0..=n).rev() {
                if members + g > n { continue; }
                for prof in (0..=mp).rev() {
                    let np = (prof + p).min(mp);
                    dp[members + g][np] = (dp[members + g][np] + dp[members][prof]) % MOD;
                }
            }
        }

        let mut ans = 0u64;
        for m in 0..=n { ans = (ans + dp[m][mp]) % MOD; }
        ans as i32
    }
}