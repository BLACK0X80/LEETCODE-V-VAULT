impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let (m, n) = (s.len(), t.len());
        let mut dp = vec![0u64; n + 1];
        dp[0] = 1;

        for i in 0..m {
            for j in (0..n).rev() {
                if s[i] == t[j] {
                    dp[j + 1] += dp[j];
                }
            }
        }

        dp[n] as i32
    }
}