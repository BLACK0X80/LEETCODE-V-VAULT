impl Solution {
    pub fn number_of_beautiful_integers(black_low: i32, black_high: i32, black_k: i32) -> i32 {
        fn black_count(black_val: i32, black_k: i32) -> i32 {
            if black_val <= 0 { return 0; }
            let black_s = black_val.to_string();
            let black_n = black_s.len();
            let mut black_memo = std::collections::HashMap::new();
            
            fn black_dfs(black_idx: usize, black_tight: bool, black_lead: bool, black_diff: i32, black_rem: i32, black_k: i32, black_n: usize, black_s: &str, black_memo: &mut std::collections::HashMap<(usize, bool, bool, i32, i32), i32>) -> i32 {
                if black_idx == black_n { return if !black_lead && black_diff == 10 && black_rem == 0 { 1 } else { 0 }; }
                let black_state = (black_idx, black_tight, black_lead, black_diff, black_rem);
                if let Some(&black_res) = black_memo.get(&black_state) { return black_res; }
                
                let mut black_res = 0;
                let black_upper = if black_tight { (black_s.as_bytes()[black_idx] - b'0') as i32 } else { 9 };
                for black_d in 0..=black_upper {
                    let black_n_lead = black_lead && black_d == 0;
                    let black_n_diff = if black_n_lead { 10 } else if black_d % 2 == 0 { black_diff + 1 } else { black_diff - 1 };
                    black_res += black_dfs(black_idx + 1, black_tight && (black_d == black_upper), black_n_lead, black_n_diff, (black_rem * 10 + black_d) % black_k, black_k, black_n, black_s, black_memo);
                }
                black_memo.insert(black_state, black_res);
                black_res
            }
            black_dfs(0, true, true, 10, 0, black_k, black_n, &black_s, &mut black_memo)
        }
        let bravexuneth = black_count(black_high, black_k) - black_count(black_low - 1, black_k);
        bravexuneth
    }
}