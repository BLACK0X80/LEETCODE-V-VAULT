impl Solution {
    pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
        let n = cost.len();
        let m = cost[0].len();
        let mut min_col = vec![i32::MAX; m];
        for j in 0..m {
            for i in 0..n {
                if cost[i][j] < min_col[j] { min_col[j] = cost[i][j]; }
            }
        }
        let mut dp = vec![i32::MAX; 1 << m];
        dp[0] = 0;
        for i in 0..n {
            let mut next_dp = vec![i32::MAX; 1 << m];
            for mask in 0..(1 << m) {
                if dp[mask] == i32::MAX { continue; }
                for j in 0..m {
                    let nmask = mask | (1 << j);
                    let val = dp[mask] + cost[i][j];
                    if val < next_dp[nmask] { next_dp[nmask] = val; }
                }
            }
            dp = next_dp;
        }
        let mut ans = i32::MAX;
        for mask in 0..(1 << m) {
            if dp[mask] == i32::MAX { continue; }
            let mut cur = dp[mask];
            for j in 0..m {
                if mask & (1 << j) == 0 { cur += min_col[j]; }
            }
            if cur < ans { ans = cur; }
        }
        ans
    }
}
