use std::collections::HashMap;

impl Solution {
    pub fn count_fancy(black_l: i64, black_r: i64) -> i64 {
        fn black_is_good(black_n: i64) -> bool {
            let black_s = black_n.to_string();
            let black_b = black_s.as_bytes();
            if black_b.len() <= 1 { return true; }
            let (mut black_inc, mut black_dec) = (true, true);
            for black_i in 1..black_b.len() {
                if black_b[black_i] <= black_b[black_i - 1] { black_inc = false; }
                if black_b[black_i] >= black_b[black_i - 1] { black_dec = false; }
            }
            black_inc || black_dec
        }

        let mut black_gs = [false; 150];
        for black_i in 0..150 { black_gs[black_i] = black_is_good(black_i as i64); }

        fn black_dp(black_idx: usize, black_tight: bool, black_lead: bool, black_last: i32, black_st: i32, black_sum: usize, black_s: &[u8], black_gs: &[bool; 150], black_memo: &mut HashMap<(usize, bool, bool, i32, i32, usize), i64>) -> i64 {
            if black_idx == black_s.len() { return if !black_lead && (black_st != 0 || black_gs[black_sum]) { 1 } else { 0 }; }
            let black_key = (black_idx, black_tight, black_lead, black_last, black_st, black_sum);
            if let Some(&black_res) = black_memo.get(&black_key) { return black_res; }

            let mut black_ans = 0;
            let black_lim = if black_tight { (black_s[black_idx] - b'0') as i32 } else { 9 };
            for black_d in 0..=black_lim {
                let mut black_nst = black_st;
                if !black_lead || black_d > 0 {
                    if black_lead { black_nst = 3; }
                    else if black_st == 3 {
                        black_nst = if black_d > black_last { 1 } else if black_d < black_last { 2 } else { 0 };
                    } else if (black_st == 1 && black_d <= black_last) || (black_st == 2 && black_d >= black_last) { black_nst = 0; }
                }
                black_ans += black_dp(black_idx + 1, black_tight && (black_d == black_lim), black_lead && (black_d == 0), black_d, black_nst, black_sum + black_d as usize, black_s, black_gs, black_memo);
            }
            black_memo.insert(black_key, black_ans);
            black_ans
        }

        let black_f = |black_n: i64| -> i64 {
            if black_n < 1 { return 0; }
            let black_s = black_n.to_string();
            black_dp(0, true, true, -1, 3, 0, black_s.as_bytes(), &black_gs, &mut HashMap::new())
        };
        black_f(black_r) - black_f(black_l - 1)
    }
}