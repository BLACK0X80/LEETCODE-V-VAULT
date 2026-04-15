impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let (black_m, black_n, black_t) = (m as usize, n as usize, target as usize);
        let mut black_dp = vec![vec![vec![1_000_000_000; black_t + 1]; black_n + 1]; black_m];
        if houses[0] == 0 {
            for black_c in 1..=black_n { black_dp[0][black_c][1] = cost[0][black_c - 1]; }
        } else { black_dp[0][houses[0] as usize][1] = 0; }
        for black_i in 1..black_m {
            for black_k in 1..=black_t {
                for black_c in 1..=black_n {
                    if houses[black_i] != 0 && houses[black_i] as usize != black_c { continue; }
                    let black_cost = if houses[black_i] == 0 { cost[black_i][black_c - 1] } else { 0 };
                    let mut black_prev = 1_000_000_000;
                    for black_p in 1..=black_n {
                        let black_pk = if black_p == black_c { black_k } else { black_k - 1 };
                        if black_pk > 0 { black_prev = black_prev.min(black_dp[black_i - 1][black_p][black_pk]); }
                    }
                    black_dp[black_i][black_c][black_k] = black_prev + black_cost;
                }
            }
        }
        let mut black_ans = 1_000_000_000;
        for black_c in 1..=black_n { black_ans = black_ans.min(black_dp[black_m - 1][black_c][black_t]); }
        if black_ans >= 1_000_000_000 { -1 } else { black_ans }
    }
}