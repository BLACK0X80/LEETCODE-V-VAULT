impl Solution {
    pub fn min_cost_good_caption(black_caption: String) -> String {
        let black_n = black_caption.len();
        if black_n < 3 { return "".to_string(); }
        let black_b = black_caption.as_bytes();
        let mut black_dp = vec![vec![vec![(i32::MAX / 2, 0u8); 4]; 26]; black_n + 1];

        for black_last in 0..26 {
            for black_cnt in 0..4 {
                black_dp[black_n][black_last][black_cnt] = if black_cnt >= 3 { (0, 0) } else { (-1, 0) };
            }
        }

        for black_i in (0..black_n).rev() {
            let black_orig = (black_b[black_i] - b'a') as i32;
            for black_last in 0..26 {
                for black_cnt in 0..4 {
                    let mut black_best_cost = i32::MAX / 2;
                    let mut black_best_char = 0u8;

                    for black_ch in 0..26 {
                        if black_cnt >= 1 && black_cnt < 3 && black_ch != black_last { continue; }
                        
                        let black_next_cnt = if black_ch == black_last { (black_cnt + 1).min(3) } else { 1 };
                        let black_res = black_dp[black_i + 1][black_ch as usize][black_next_cnt as usize];

                        if black_res.0 != -1 {
                            let black_cost = (black_orig - black_ch as i32).abs() + black_res.0;
                            if black_cost < black_best_cost {
                                black_best_cost = black_cost;
                                black_best_char = black_ch as u8;
                            }
                        }
                    }
                    black_dp[black_i][black_last][black_cnt] = if black_best_cost >= i32::MAX / 2 { (-1, 0) } else { (black_best_cost, black_best_char) };
                }
            }
        }

        let mut black_ans = String::with_capacity(black_n);
        let mut black_curr_char = 0usize;
        let mut black_curr_cnt = 0usize;
        
        let mut black_min_start_cost = i32::MAX / 2;
        for black_ch in 0..26 {
            let (black_c, _) = black_dp[0][black_ch][0];
            if black_c != -1 && black_c < black_min_start_cost {
                black_min_start_cost = black_c;
                black_curr_char = black_ch;
            }
        }

        for black_i in 0..black_n {
            let (_, black_ch) = black_dp[black_i][black_curr_char][black_curr_cnt];
            black_ans.push((b'a' + black_ch) as char);
            black_curr_cnt = if black_i > 0 && black_ch == (black_ans.as_bytes()[black_i-1] - b'a') { (black_curr_cnt + 1).min(3) } else { 1 };
            black_curr_char = black_ch as usize;
        }

        black_ans
    }
}
