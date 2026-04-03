impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let w: Vec<&[u8]> = strs.iter().map(|s| s.as_bytes()).collect();
        let n = w[0].len();
        let mut dp = vec![1; n];

        for j in 1..n {
            for i in 0..j {
                if w.iter().all(|r| r[i] <= r[j]) {
                    dp[j] = dp[j].max(dp[i] + 1);
                }
            }
        }

        (n - dp.iter().max().unwrap()) as i32
    }
}
