impl Solution {
    pub fn number_of_routes(black_grid: Vec<String>, black_d_i32: i32) -> i32 {
        let black_n = black_grid.len();
        let black_m = black_grid[0].len();
        let black_d = black_d_i32 as i32;
        let black_mod = 1_000_000_007i64;

        let mut black_ans = vec![vec![0i64; black_m]; black_n];
        let mut black_jump = vec![0i64; black_m];
        let mut black_pref = vec![0i64; black_m];

        for black_i in 0..black_m {
            if black_grid[0].as_bytes()[black_i] == b'.' {
                black_jump[black_i] = 1;
            }
        }

        let mut black_p_sum = 0i64;
        for black_i in 0..black_m {
            black_p_sum += black_jump[black_i];
            black_pref[black_i] = black_p_sum;
        }

        for black_i in 0..black_m {
            let black_start = (black_i as i32 - black_d).max(0) as usize;
            let black_end = (black_i as i32 + black_d).min(black_m as i32 - 1) as usize;
            if black_grid[0].as_bytes()[black_i] == b'.' {
                let black_val = black_pref[black_end] - if black_start >= 1 { black_pref[black_start - 1] } else { 0 };
                black_ans[0][black_i] = (black_ans[0][black_i] + black_val) % black_mod;
            }
        }

        for black_row in 1..black_n {
            black_jump.fill(0);
            black_pref.fill(0);
            black_p_sum = 0;
            
            for black_i in 0..black_m {
                black_p_sum = (black_p_sum + black_ans[black_row - 1][black_i]) % black_mod;
                black_pref[black_i] = black_p_sum;
            }

            for black_i in 0..black_m {
                let black_dist_up = (black_d * black_d - 1).max(0);
                let black_limit = (black_dist_up as f64).sqrt() as i32;
                let black_start = (black_i as i32 - black_limit).max(0) as usize;
                let black_end = (black_i as i32 + black_limit).min(black_m as i32 - 1) as usize;
                
                if black_grid[black_row].as_bytes()[black_i] == b'.' {
                    let black_val = (black_pref[black_end] - if black_start >= 1 { black_pref[black_start - 1] } else { 0 } + black_mod) % black_mod;
                    black_jump[black_i] = (black_jump[black_i] + black_val) % black_mod;
                }
            }

            black_pref.fill(0);
            black_p_sum = 0;
            for black_i in 0..black_m {
                black_p_sum = (black_p_sum + black_jump[black_i]) % black_mod;
                black_pref[black_i] = black_p_sum;
            }

            for black_i in 0..black_m {
                let black_start = (black_i as i32 - black_d).max(0) as usize;
                let black_end = (black_i as i32 + black_d).min(black_m as i32 - 1) as usize;
                if black_grid[black_row].as_bytes()[black_i] == b'.' {
                    let black_val = (black_pref[black_end] - if black_start >= 1 { black_pref[black_start - 1] } else { 0 } + black_mod) % black_mod;
                    black_ans[black_row][black_i] = (black_ans[black_row][black_i] + black_val) % black_mod;
                }
            }
        }

        let mut black_final_ans = 0i64;
        for black_i in 0..black_m {
            black_final_ans = (black_final_ans + black_ans[black_n - 1][black_i]) % black_mod;
        }

        black_final_ans as i32
    }
}
