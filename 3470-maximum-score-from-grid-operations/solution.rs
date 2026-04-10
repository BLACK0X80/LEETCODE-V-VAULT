impl Solution {
    pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
        let black_n = grid.len();
        let mut black_pref = vec![vec![0i64; black_n + 1]; black_n + 1];
        for black_j in 1..=black_n {
            for black_i in 1..=black_n {
                black_pref[black_i][black_j] = black_pref[black_i - 1][black_j] + grid[black_i - 1][black_j - 1] as i64;
            }
        }

        let black_size = black_n + 1;
        let mut black_dp = vec![-1i64; black_size * black_size];
        black_dp[0] = 0;

        for black_c in 1..=black_n {
            let mut black_next_dp = vec![-1i64; black_size * black_size];
            for black_pp in 0..=black_n {
                let black_pp_offset = black_pp * black_size;
                for black_p in 0..=black_n {
                    let black_prev_val = black_dp[black_pp_offset + black_p];
                    if black_prev_val == -1 { continue; }
                    
                    for black_curr in 0..=black_n {
                        let mut black_score = 0;
                        if black_c > 1 {
                            let black_limit = if black_pp > black_curr { black_pp } else { black_curr };
                            if black_limit > black_p {
                                black_score = black_pref[black_limit][black_c - 1] - black_pref[black_p][black_c - 1];
                            }
                        }
                        
                        let black_total = black_prev_val + black_score;
                        let black_next_idx = black_p * black_size + black_curr;
                        if black_total > black_next_dp[black_next_idx] {
                            black_next_dp[black_next_idx] = black_total;
                        }
                    }
                }
            }
            black_dp = black_next_dp;
        }

        let mut black_ans = 0i64;
        for black_pp in 0..=black_n {
            let black_pp_offset = black_pp * black_size;
            for black_p in 0..=black_n {
                let black_val = black_dp[black_pp_offset + black_p];
                if black_val == -1 { continue; }
                
                let black_limit = black_pp;
                let mut black_final_score = 0;
                if black_limit > black_p {
                    black_final_score = black_pref[black_limit][black_n] - black_pref[black_p][black_n];
                }
                
                if black_val + black_final_score > black_ans {
                    black_ans = black_val + black_final_score;
                }
            }
        }
        
        black_ans
    }
}
