impl Solution {
    pub fn minimum_distance(black_word: String) -> i32 {
        let black_bytes = black_word.as_bytes();
        let black_n = black_bytes.len();
        let mut black_dp = vec![vec![vec![1000000; 27]; 27]; black_n + 1];

        black_dp[0][26][26] = 0;

        for black_i in 0..black_n {
            let black_target = (black_bytes[black_i] - b'A') as usize;
            for black_f1 in 0..27 {
                for black_f2 in 0..27 {
                    let black_current_dist = black_dp[black_i][black_f1][black_f2];
                    if black_current_dist == 1000000 { continue; }

                    let black_cost1 = if black_f1 == 26 { 0 } else {
                        (black_f1 as i32 / 6 - black_target as i32 / 6).abs() + (black_f1 as i32 % 6 - black_target as i32 % 6).abs()
                    };
                    black_dp[black_i + 1][black_target][black_f2] = black_dp[black_i + 1][black_target][black_f2].min(black_current_dist + black_cost1);

                    let black_cost2 = if black_f2 == 26 { 0 } else {
                        (black_f2 as i32 / 6 - black_target as i32 / 6).abs() + (black_f2 as i32 % 6 - black_target as i32 % 6).abs()
                    };
                    black_dp[black_i + 1][black_f1][black_target] = black_dp[black_i + 1][black_f1][black_target].min(black_current_dist + black_cost2);
                }
            }
        }

        let mut black_ans = 1000000;
        for black_i in 0..27 {
            for black_j in 0..27 {
                black_ans = black_ans.min(black_dp[black_n][black_i][black_j]);
            }
        }
        black_ans
    }
}