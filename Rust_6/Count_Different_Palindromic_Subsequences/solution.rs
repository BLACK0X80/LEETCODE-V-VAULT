impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0i64; n]; n];
        for i in 0..n { dp[i][i] = 1; }

        for len in 2..=n {
            for l in 0..=n-len {
                let r = l + len - 1;
                for c in b'a'..=b'd' {
                    let lo = (l..=r).find(|&i| s[i] == c);
                    let hi = (l..=r).rev().find(|&i| s[i] == c);
                    match (lo, hi) {
                        (None, _) => {}
                        (Some(i), Some(j)) if i == j => dp[l][r] += 1,
                        (Some(i), Some(j)) if i + 1 > j - 1 => dp[l][r] += 2,
                        (Some(i), Some(j)) => dp[l][r] += dp[i+1][j-1] + 2,
                        _ => {}
                    }
                }
                dp[l][r] %= MOD;
            }
        }

        dp[0][n-1] as i32
    }
}