impl Solution {
    pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
        let black_t = target as usize;
        let mut black_dp = vec![-10000; black_t + 1];
        black_dp[0] = 0;
        for black_i in 1..=black_t {
            for black_d in 0..9 {
                let black_c = cost[black_d] as usize;
                if black_i >= black_c { black_dp[black_i] = black_dp[black_i].max(black_dp[black_i - black_c] + 1); }
            }
        }
        if black_dp[black_t] < 0 { return "0".to_string(); }
        let (mut black_ans, mut black_curr) = (String::new(), black_t);
        while black_curr > 0 {
            for black_d in (0..9).rev() {
                let black_c = cost[black_d] as usize;
                if black_curr >= black_c && black_dp[black_curr] == black_dp[black_curr - black_c] + 1 {
                    black_ans.push((b'1' + black_d as u8) as char);
                    black_curr -= black_c;
                    break;
                }
            }
        }
        black_ans
    }
}