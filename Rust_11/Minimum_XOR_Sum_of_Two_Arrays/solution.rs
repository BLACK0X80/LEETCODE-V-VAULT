impl Solution {
    pub fn minimum_xor_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut black_dp = vec![i32::MAX; 1 << n];
        black_dp[0] = 0;
        for i in 0..n {
            for mask in 0..(1<<n) {
                if black_dp[mask] == i32::MAX { continue; }
                let black_cnt = (mask as i32).count_ones() as usize;
                if black_cnt != i { continue; }
                for j in 0..n {
                    if mask & (1<<j) == 0 {
                        let nm = mask | (1<<j);
                        black_dp[nm] = black_dp[nm].min(black_dp[mask] + (nums1[i] ^ nums2[j]));
                    }
                }
            }
        }
        black_dp[(1<<n)-1]
    }
}