impl Solution {
    pub fn delete_string(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut lcp = vec![vec![0usize; n+1]; n+1];
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if s[i] == s[j] { lcp[i][j] = lcp[i+1][j+1] + 1; }
            }
        }
        let mut dp = vec![1usize; n];
        for i in (0..n-1).rev() {
            for l in 1..=(n-i)/2 {
                if lcp[i][i+l] >= l {
                    dp[i] = dp[i].max(1 + dp[i+l]);
                }
            }
        }
        dp[0] as i32
    }
}
