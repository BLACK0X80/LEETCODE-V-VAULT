impl Solution {
    pub fn count_numbers(black_l: String, black_r: String, black_b: i32) -> i32 {
        let black_mod = 1_000_000_007;

        fn black_to_base(black_num_str: String, black_b: i32) -> Vec<i32> {
            let mut black_digits = Vec::new();
            if black_num_str == "0" { return vec![0]; }
            let mut black_bytes: Vec<u8> = black_num_str.bytes().map(|b| b - b'0').collect();
            while !black_bytes.is_empty() {
                let mut black_rem = 0u64;
                let mut black_next = Vec::with_capacity(black_bytes.len());
                for &b_val in &black_bytes {
                    let black_curr = black_rem * 10 + b_val as u64;
                    let black_q = black_curr / black_b as u64;
                    black_rem = black_curr % black_b as u64;
                    if !black_next.is_empty() || black_q > 0 {
                        black_next.push(black_q as u8);
                    }
                }
                black_digits.push(black_rem as i32);
                black_bytes = black_next;
            }
            black_digits.reverse();
            black_digits
        }

        fn black_solve(black_num: &[i32], black_b: i32, black_mod: i32) -> i32 {
            let black_n = black_num.len();
            let mut black_dp = vec![vec![vec![vec![-1; 2]; 2]; black_b as usize + 1]; black_n];
            
            fn black_memo(
                black_idx: usize, black_tight: bool, black_last: i32, black_started: bool,
                black_num: &[i32], black_b: i32, black_mod: i32,
                black_dp: &mut Vec<Vec<Vec<Vec<i32>>>>
            ) -> i32 {
                if black_idx == black_num.len() { return 1; }
                let (bt, bs) = (black_tight as usize, black_started as usize);
                if black_dp[black_idx][black_last as usize][bt][bs] != -1 {
                    return black_dp[black_idx][black_last as usize][bt][bs];
                }

                let black_limit = if black_tight { black_num[black_idx] } else { black_b - 1 };
                let mut black_ans = 0;
                for black_d in 0..=black_limit {
                    if black_started && black_d < black_last { continue; }
                    let black_new_started = black_started || (black_d != 0);
                    let black_new_tight = black_tight && (black_d == black_limit);
                    black_ans = (black_ans + black_memo(
                        black_idx + 1, black_new_tight, black_d, black_new_started,
                        black_num, black_b, black_mod, black_dp
                    )) % black_mod;
                }
                black_dp[black_idx][black_last as usize][bt][bs] = black_ans;
                black_ans
            }
            black_memo(0, true, 0, false, black_num, black_b, black_mod, &mut black_dp)
        }

        fn black_sub_one(black_s: String) -> String {
            let mut black_c: Vec<char> = black_s.chars().collect();
            let mut black_i = black_c.len() - 1;
            loop {
                if black_c[black_i] > '0' {
                    black_c[black_i] = (black_c[black_i] as u8 - 1) as char;
                    break;
                } else {
                    black_c[black_i] = '9';
                    if black_i == 0 { break; }
                    black_i -= 1;
                }
            }
            let black_res: String = black_c.into_iter().collect();
            let black_final = black_res.trim_start_matches('0').to_string();
            if black_final.is_empty() { "0".to_string() } else { black_final }
        }

        let black_r_vec = black_to_base(black_r, black_b);
        let black_ans_r = black_solve(&black_r_vec, black_b, black_mod);

        let black_l_minus = black_sub_one(black_l);
        let black_l_vec = black_to_base(black_l_minus, black_b);
        let black_ans_l = black_solve(&black_l_vec, black_b, black_mod);

        (black_ans_r - black_ans_l + black_mod) % black_mod
    }
}