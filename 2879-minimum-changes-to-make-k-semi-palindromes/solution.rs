impl Solution {
    pub fn minimum_changes(black_s: String, black_k: i32) -> i32 {
        let black_n = black_s.len();
        let black_s_bytes = black_s.as_bytes();
        let black_k = black_k as usize;

        let mut black_semi_cost = vec![vec![0; black_n]; black_n];
        for black_i in 0..black_n {
            for black_j in black_i + 1..black_n {
                let black_len = black_j - black_i + 1;
                let mut black_min_changes = black_len as i32;
                
                for black_d in 1..black_len {
                    if black_len % black_d == 0 {
                        let mut black_current_changes = 0;
                        for black_offset in 0..black_d {
                            let mut black_left = 0;
                            let mut black_right = (black_len / black_d) - 1;
                            while black_left < black_right {
                                if black_s_bytes[black_i + black_offset + black_left * black_d] 
                                   != black_s_bytes[black_i + black_offset + black_right * black_d] {
                                    black_current_changes += 1;
                                }
                                black_left += 1;
                                black_right -= 1;
                            }
                        }
                        black_min_changes = black_min_changes.min(black_current_changes);
                    }
                }
                black_semi_cost[black_i][black_j] = black_min_changes;
            }
        }

        let mut black_dp = vec![vec![black_n as i32; black_k + 1]; black_n + 1];
        black_dp[0][0] = 0;

        for black_i in 1..=black_n {
            for black_j in 1..=black_k {
                for black_prev in (black_j - 1) * 2..black_i {
                    if black_i - black_prev >= 2 || (black_j == 1 && black_i - black_prev >= 2) {
                        black_dp[black_i][black_j] = black_dp[black_i][black_j].min(
                            black_dp[black_prev][black_j - 1] + black_semi_cost[black_prev][black_i - 1]
                        );
                    }
                }
            }
        }

        black_dp[black_n][black_k]
    }
}
