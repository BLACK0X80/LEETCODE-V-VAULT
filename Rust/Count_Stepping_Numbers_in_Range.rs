impl Solution {
    pub fn count_stepping_numbers(black_low: String, black_high: String) -> i32 {
        let black_mod = 1_000_000_007;
        fn black_calc(black_s: &str, black_m: i32) -> i32 {
            let black_digits: Vec<i32> = black_s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
            let black_n = black_digits.len();
            let mut black_memo = std::collections::HashMap::new();
            fn black_dfs(black_idx: usize, black_prev: i32, black_is_limit: bool, black_is_leading: bool, black_digits: &[i32], black_memo: &mut std::collections::HashMap<(usize, i32, bool, bool), i32>, black_m: i32) -> i32 {
                if black_idx == black_digits.len() { return if black_is_leading { 0 } else { 1 }; }
                let black_state = (black_idx, black_prev, black_is_limit, black_is_leading);
                if let Some(&black_res) = black_memo.get(&black_state) { return black_res; }
                let mut black_ans = 0;
                let black_up = if black_is_limit { black_digits[black_idx] } else { 9 };
                for black_d in 0..=black_up {
                    if black_is_leading {
                        black_ans = (black_ans + black_dfs(black_idx + 1, black_d, black_is_limit && (black_d == black_up), black_is_leading && (black_d == 0), black_digits, black_memo, black_m)) % black_m;
                    } else if (black_d - black_prev).abs() == 1 {
                        black_ans = (black_ans + black_dfs(black_idx + 1, black_d, black_is_limit && (black_d == black_up), false, black_digits, black_memo, black_m)) % black_m;
                    }
                }
                black_memo.insert(black_state, black_ans);
                black_ans
            }
            black_dfs(0, -1, true, true, &black_digits, &mut black_memo, black_m)
        }
        let mut black_res = (black_calc(&black_high, black_mod) - black_calc(&black_low, black_mod) + black_mod) % black_mod;
        let black_low_bytes = black_low.as_bytes();
        let mut black_is_step = true;
        for black_i in 1..black_low_bytes.len() {
            if (black_low_bytes[black_i] as i32 - black_low_bytes[black_i-1] as i32).abs() != 1 { black_is_step = false; break; }
        }
        if black_is_step { black_res = (black_res + 1) % black_mod; }
        black_res
    }
}