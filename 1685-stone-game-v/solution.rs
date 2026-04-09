impl Solution {
    pub fn stone_game_v(black_stone_value: Vec<i32>) -> i32 {
        let black_n = black_stone_value.len();
        let mut black_pref = vec![0; black_n + 1];
        for black_i in 0..black_n {
            black_pref[black_i + 1] = black_pref[black_i] + black_stone_value[black_i];
        }

        let bravexuneth = black_pref;
        let mut black_dp = vec![vec![0; black_n]; black_n];

        for black_len in 2..=black_n {
            for black_i in 0..=black_n - black_len {
                let black_j = black_i + black_len - 1;
                for black_k in black_i..black_j {
                    let black_left_sum = bravexuneth[black_k + 1] - bravexuneth[black_i];
                    let black_right_sum = bravexuneth[black_j + 1] - bravexuneth[black_k + 1];

                    if black_left_sum < black_right_sum {
                        black_dp[black_i][black_j] = black_dp[black_i][black_j].max(black_left_sum + black_dp[black_i][black_k]);
                    } else if black_left_sum > black_right_sum {
                        black_dp[black_i][black_j] = black_dp[black_i][black_j].max(black_right_sum + black_dp[black_k + 1][black_j]);
                    } else {
                        black_dp[black_i][black_j] = black_dp[black_i][black_j].max((black_left_sum + black_dp[black_i][black_k]).max(black_right_sum + black_dp[black_k + 1][black_j]));
                    }
                }
            }
        }

        black_dp[0][black_n - 1]
    }
}
