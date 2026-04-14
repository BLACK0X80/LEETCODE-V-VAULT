impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (black_n1, black_n2) = (nums1.len(), nums2.len());
        let mut black_dp = vec![vec![-1_000_000_000; black_n2 + 1]; black_n1 + 1];
        for black_i in 1..=black_n1 {
            for black_j in 1..=black_n2 {
                let black_p = nums1[black_i - 1] * nums2[black_j - 1];
                black_dp[black_i][black_j] = *[black_p, black_p + black_dp[black_i - 1][black_j - 1].max(0), black_dp[black_i - 1][black_j], black_dp[black_i][black_j - 1]].iter().max().unwrap();
            }
        }
        black_dp[black_n1][black_n2]
    }
}