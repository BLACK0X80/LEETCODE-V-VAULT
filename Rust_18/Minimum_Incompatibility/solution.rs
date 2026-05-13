impl Solution {
    pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let sz = n / k as usize;
        let mut black_cost = vec![-1i32; 1<<n];
        for mask in 0..(1<<n) {
            if (mask as u32).count_ones() as usize != sz { continue; }
            let mut vals: Vec<i32> = (0..n).filter(|&i| mask & (1<<i) != 0).map(|i| nums[i]).collect();
            vals.sort_unstable();
            if vals.windows(2).all(|w| w[0] != w[1]) {
                black_cost[mask] = vals.last().unwrap() - vals[0];
            }
        }
        let mut black_dp = vec![i32::MAX; 1<<n];
        black_dp[0] = 0;
        for mask in 0..(1<<n) {
            if black_dp[mask] == i32::MAX { continue; }
            let rem = ((1<<n)-1) ^ mask;
            let low = (rem & rem.wrapping_neg()) as usize;
            let mut sub = rem;
            while sub > 0 {
                if sub & low != 0 && black_cost[sub] >= 0 && black_dp[mask] != i32::MAX {
                    black_dp[mask|sub] = black_dp[mask|sub].min(black_dp[mask] + black_cost[sub]);
                }
                sub = (sub-1) & rem;
            }
        }
        if black_dp[(1<<n)-1] == i32::MAX { -1 } else { black_dp[(1<<n)-1] }
    }
}