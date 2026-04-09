impl Solution {
    pub fn stone_game_iii(black_stone_value: Vec<i32>) -> String {
        let black_n = black_stone_value.len();
        let mut black_dp = vec![i32::MIN; black_n + 1];
        black_dp[black_n] = 0;

        for black_i in (0..black_n).rev() {
            let mut black_take = 0;
            for black_k in 0..3 {
                if black_i + black_k < black_n {
                    black_take += black_stone_value[black_i + black_k];
                    let black_res = black_take - black_dp[black_i + black_k + 1];
                    if black_res > black_dp[black_i] {
                        black_dp[black_i] = black_res;
                    }
                }
            }
        }

        if black_dp[0] > 0 {
            "Alice".to_string()
        } else if black_dp[0] < 0 {
            "Bob".to_string()
        } else {
            "Tie".to_string()
        }
    }
}
