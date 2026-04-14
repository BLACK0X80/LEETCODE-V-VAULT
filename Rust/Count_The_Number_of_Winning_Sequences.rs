impl Solution {
    pub fn count_winning_sequences(black_s: String) -> i32 {
        let black_mod = 1_000_000_007;
        let black_n = black_s.len();
        let black_bytes = black_s.as_bytes();
        let mut black_dp = vec![vec![0; 2 * black_n + 1]; 3];
        let black_map = |c| match c { b'F' => 0, b'W' => 1, _ => 2 };
        let black_score = |b, a| if (b + 1) % 3 == a { -1 } else if (a + 1) % 3 == b { 1 } else { 0 };

        for black_j in 0..3 {
            let black_s_val = black_score(black_j, black_map(black_bytes[0]));
            black_dp[black_j][(black_n as i32 + black_s_val) as usize] = 1;
        }

        for black_i in 1..black_n {
            let mut black_next_dp = vec![vec![0; 2 * black_n + 1]; 3];
            let black_a = black_map(black_bytes[black_i]);
            for black_prev_j in 0..3 {
                for black_v in 0..=2 * black_n {
                    if black_dp[black_prev_j][black_v] == 0 { continue; }
                    for black_curr_j in 0..3 {
                        if black_curr_j == black_prev_j { continue; }
                        let black_s_val = black_score(black_curr_j, black_a);
                        let black_next_v = (black_v as i32 + black_s_val) as usize;
                        black_next_dp[black_curr_j][black_next_v] = (black_next_dp[black_curr_j][black_next_v] + black_dp[black_prev_j][black_v]) % black_mod;
                    }
                }
            }
            black_dp = black_next_dp;
        }

        let mut black_res = 0;
        for black_j in 0..3 {
            for black_v in black_n + 1..=2 * black_n {
                black_res = (black_res + black_dp[black_j][black_v]) % black_mod;
            }
        }
        black_res
    }
}