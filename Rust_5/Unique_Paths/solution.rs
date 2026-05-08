impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (black_m, black_n) = (m as usize, n as usize);
        let mut black_dp = vec![1; black_n];
        
        for _ in 1..black_m {
            for j in 1..black_n {
                black_dp[j] += black_dp[j - 1];
            }
        }
        
        black_dp[black_n - 1]
    }
}