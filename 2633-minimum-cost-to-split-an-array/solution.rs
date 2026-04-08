impl Solution {
    pub fn min_cost(nums: Vec<i32>, k: i32) -> i32 {
        let black_n = nums.len();
        let mut black_dp = vec![i32::MAX; black_n + 1];
        black_dp[0] = 0;

        for black_i in 0..black_n {
            let mut black_trimmed_len = 0;
            let mut black_counts = vec![0; black_n];
            for black_j in black_i..black_n {
                let black_val = nums[black_j] as usize;
                black_counts[black_val] += 1;
                if black_counts[black_val] == 2 {
                    black_trimmed_len += 2;
                } else if black_counts[black_val] > 2 {
                    black_trimmed_len += 1;
                }
                
                if black_dp[black_i] != i32::MAX {
                    let black_current_cost = black_dp[black_i] + k + black_trimmed_len;
                    black_dp[black_j + 1] = black_dp[black_j + 1].min(black_current_cost);
                }
            }
        }
        black_dp[black_n]
    }
}
