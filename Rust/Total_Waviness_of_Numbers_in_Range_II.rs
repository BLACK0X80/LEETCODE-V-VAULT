impl Solution {
    pub fn total_waviness(num1: i64, num2: i64) -> i64 {
        fn black_solve(black_n: i64) -> i64 {
            if black_n < 100 { return 0; }
            let black_s = black_n.to_string().bytes().map(|b| (b - b'0') as i8).collect::<Vec<_>>();
            let mut black_dp = std::collections::HashMap::new();
            fn black_dfs(black_i: usize, black_t: bool, black_std: bool, black_p2: i8, black_p1: i8, black_s: &Vec<i8>, black_memo: &mut std::collections::HashMap<(usize, bool, bool, i8, i8), (i64, i64)>) -> (i64, i64) {
                if black_i == black_s.len() { return (if black_std { 1 } else { 0 }, 0); }
                if let Some(&black_res) = black_memo.get(&(black_i, black_t, black_std, black_p2, black_p1)) { return black_res; }
                let (mut black_c_sum, mut black_w_sum, black_lim) = (0, 0, if black_t { black_s[black_i] } else { 9 });
                for black_d in 0..=black_lim {
                    let black_nt = black_t && (black_d == black_lim);
                    let (black_c, black_w) = if !black_std { if black_d == 0 { black_dfs(black_i + 1, black_nt, false, 10, 10, black_s, black_memo) } else { black_dfs(black_i + 1, black_nt, true, 10, black_d, black_s, black_memo) } }
                    else { let black_wav = if black_p1 != 10 && black_p2 != 10 && ((black_p2 < black_p1 && black_p1 > black_d) || (black_p2 > black_p1 && black_p1 < black_d)) { 1 } else { 0 };
                        let black_nxt = black_dfs(black_i + 1, black_nt, true, black_p1, black_d, black_s, black_memo); (black_nxt.0, black_nxt.1 + black_nxt.0 * black_wav) };
                    black_c_sum += black_c; black_w_sum += black_w;
                }
                black_memo.insert((black_i, black_t, black_std, black_p2, black_p1), (black_c_sum, black_w_sum)); (black_c_sum, black_w_sum)
            }
            black_dfs(0, true, false, 10, 10, &black_s, &mut black_dp).1
        }
        black_solve(num2) - black_solve(num1 - 1)
    }
}