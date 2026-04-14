impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let black_k = k as usize;
        let mut black_dp = vec![std::collections::HashMap::new(); black_k + 1];
        let mut black_max = vec![0; black_k + 1];
        for x in nums {
            for j in (0..=black_k).rev() {
                let black_cur = black_dp[j].get(&x).cloned().unwrap_or(0) + 1;
                let black_prev = if j > 0 { black_max[j - 1] + 1 } else { 0 };
                let black_res = black_cur.max(black_prev);
                black_dp[j].insert(x, black_res);
                black_max[j] = black_max[j].max(black_res);
            }
        }
        black_max[black_k]
    }
}