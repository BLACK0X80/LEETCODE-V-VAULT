impl Solution {
    pub fn min_distance(mut houses: Vec<i32>, k: i32) -> i32 {
        houses.sort();
        let n = houses.len();
        let k = k as usize;
        let mut cost = vec![vec![0; n]; n];
        for i in 0..n {
            for j in i..n {
                let mid = (i + j) / 2;
                for l in i..=j { cost[i][j] += (houses[l] - houses[mid]).abs(); }
            }
        }
        let mut dp = vec![vec![i32::MAX; n]; k];
        for j in 0..n { dp[0][j] = cost[0][j]; }
        for i in 1..k {
            for j in i..n {
                for m in i-1..j {
                    if dp[i-1][m] != i32::MAX {
                        dp[i][j] = dp[i][j].min(dp[i-1][m] + cost[m+1][j]);
                    }
                }
            }
        }
        dp[k-1][n-1]
    }
}