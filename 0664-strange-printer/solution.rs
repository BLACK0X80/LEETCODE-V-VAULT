impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0i32; n]; n];

        for l in (0..n).rev() {
            dp[l][l] = 1;
            for r in l+1..n {
                dp[l][r] = dp[l][r-1] + 1;
                for m in l..r {
                    if s[m] == s[r] {
                        let left  = if m + 1 <= r - 1 { dp[m+1][r-1] } else { 0 };
                        dp[l][r] = dp[l][r].min(dp[l][m] + left);
                    }
                }
            }
        }

        dp[0][n-1]
    }
}
