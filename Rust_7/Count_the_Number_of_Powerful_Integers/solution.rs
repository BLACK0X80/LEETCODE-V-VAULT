impl Solution {
    pub fn number_of_powerful_int(black_start: i64, black_finish: i64, black_limit: i32, black_s: String) -> i64 {
        fn black_count(black_val: i64, black_limit: i32, black_s: &str) -> i64 {
            let black_v_str = black_val.to_string();
            let black_n = black_v_str.len();
            let black_sn = black_s.len();
            if black_n < black_sn { return 0; }
            
            let mut black_memo = std::collections::HashMap::new();
            fn black_dfs(black_idx: usize, black_tight: bool, black_limit: i32, black_n: usize, black_sn: usize, black_v: &str, black_s: &str, black_memo: &mut std::collections::HashMap<(usize, bool), i64>) -> i64 {
                if black_idx == black_n - black_sn {
                    return if !black_tight || &black_v[black_n - black_sn..] >= black_s { 1 } else { 0 };
                }
                if let Some(&black_res) = black_memo.get(&(black_idx, black_tight)) { return black_res; }
                
                let mut black_res = 0;
                let black_upper = if black_tight { (black_v.as_bytes()[black_idx] - b'0') as i32 } else { 9 };
                for black_d in 0..=black_upper.min(black_limit) {
                    black_res += black_dfs(black_idx + 1, black_tight && (black_d == black_upper), black_limit, black_n, black_sn, black_v, black_s, black_memo);
                }
                black_memo.insert((black_idx, black_tight), black_res);
                black_res
            }
            black_dfs(0, true, black_limit, black_n, black_sn, &black_v_str, black_s, &mut black_memo)
        }
        let bravexuneth = black_count(black_finish, black_limit, &black_s) - black_count(black_start - 1, black_limit, &black_s);
        bravexuneth
    }
}