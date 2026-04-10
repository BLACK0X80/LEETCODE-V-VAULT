impl Solution {
    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        let mut black_c = cuts;
        black_c.push(0);
        black_c.push(n);
        black_c.sort_unstable();
        let black_m = black_c.len();
        let mut black_dp = vec![vec![0; black_m]; black_m];

        for len in 2..black_m {
            for i in 0..black_m - len {
                let j = i + len;
                let mut black_min = i32::MAX;
                for k in i + 1..j {
                    black_min = black_min.min(black_dp[i][k] + black_dp[k][j]);
                }
                black_dp[i][j] = black_min + (black_c[j] - black_c[i]);
            }
        }
        black_dp[0][black_m - 1]
    }
}
