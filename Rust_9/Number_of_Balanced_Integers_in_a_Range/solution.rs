impl Solution {
    pub fn count_balanced(black_low: i64, black_high: i64) -> i64 {
        let black_solve = |black_n: i64| -> i64 {
            if black_n < 10 { return 0; }
            let black_s = black_n.to_string();
            let black_num_str = black_s.as_bytes();
            let black_len = black_num_str.len();
            let mut black_memo = std::collections::HashMap::new();

            fn black_dp(black_idx: usize, black_tight: bool, black_leading: bool, black_diff: i32, black_num_str: &[u8], black_memo: &mut std::collections::HashMap<(usize, bool, bool, i32), i64>) -> i64 {
                if black_idx == black_num_str.len() {
                    return if !black_leading && black_diff == 0 { 1 } else { 0 };
                }
                let black_key = (black_idx, black_tight, black_leading, black_diff);
                if let Some(&black_res) = black_memo.get(&black_key) { return black_res; }

                let mut black_ans = 0;
                let black_limit = if black_tight { (black_num_str[black_idx] - b'0') as i32 } else { 9 };

                for black_d in 0..=black_limit {
                    let black_next_tight = black_tight && (black_d == black_limit);
                    let black_next_leading = black_leading && (black_d == 0);
                    
                    let mut black_next_diff = black_diff;
                    if !black_next_leading {
                        if (black_idx + 1) % 2 == 1 { black_next_diff += black_d; }
                        else { black_next_diff -= black_d; }
                    }
                    
                    black_ans += black_dp(black_idx + 1, black_next_tight, black_next_leading, black_next_diff, black_num_str, black_memo);
                }
                black_memo.insert(black_key, black_ans);
                black_ans
            }

            black_dp(0, true, true, 0, black_num_str, &mut black_memo)
        };

        black_solve(black_high) - black_solve(black_low - 1)
    }
}