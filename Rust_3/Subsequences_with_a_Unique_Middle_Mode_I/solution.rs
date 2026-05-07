use std::collections::HashMap;

impl Solution {
    pub fn subsequences_with_middle_mode(black_a: Vec<i32>) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_n = black_a.len();
        let mut black_ans = 0i64;
        let mut black_pre = HashMap::new();
        let mut black_suf = HashMap::new();

        for &black_v in &black_a {
            *black_suf.entry(black_v).or_insert(0i64) += 1;
        }

        fn black_comb2(black_n: i64) -> i64 {
            if black_n < 2 { 0 } else { black_n * (black_n - 1) / 2 }
        }

        fn black_comb(black_n: i64, black_r: i64) -> i64 {
            if black_r == 2 { black_comb2(black_n) }
            else if black_r == 0 { 1 }
            else { 0 }
        }

        for (black_i, &black_v) in black_a.iter().enumerate() {
            let black_i_idx = black_i as i64;
            let black_n_idx = black_n as i64;
            
            if let Some(black_count) = black_suf.get_mut(&black_v) {
                *black_count -= 1;
            }

            let black_left = black_i_idx;
            let black_right = black_n_idx - 1 - black_i_idx;
            let black_pre_v = *black_pre.get(&black_v).unwrap_or(&0i64);
            let black_suf_v = *black_suf.get(&black_v).unwrap_or(&0i64);

            black_ans = (black_ans + black_comb(black_left, 2) * black_comb(black_right, 2)) % black_mod;
            black_ans = (black_ans - black_comb(black_left - black_pre_v, 2) * black_comb(black_right - black_suf_v, 2)) % black_mod;

            let mut black_keys: std::collections::HashSet<i32> = black_pre.keys().cloned().collect();
            black_keys.extend(black_suf.keys().cloned());

            for &black_x in &black_keys {
                if black_x == black_v {
                    continue;
                }

                let black_pre_x = *black_pre.get(&black_x).unwrap_or(&0i64);
                let black_suf_x = *black_suf.get(&black_x).unwrap_or(&0i64);

                let black_term1 = black_comb(black_pre_x, 2) * black_suf_v * (black_right - black_suf_v - black_suf_x);
                let black_term2 = black_comb(black_suf_x, 2) * black_pre_v * (black_left - black_pre_v - black_pre_x);
                let black_term3 = black_pre_v * black_pre_x * black_suf_x * (black_right - black_suf_v - black_suf_x);
                let black_term4 = black_suf_v * black_suf_x * black_pre_x * (black_left - black_pre_v - black_pre_x);
                
                let black_term5 = black_comb(black_pre_x, 2) * black_suf_v * black_suf_x;
                let black_term6 = black_comb(black_suf_x, 2) * black_pre_v * black_pre_x;

                black_ans = (black_ans - black_term1 - black_term2 - black_term3 - black_term4 - black_term5 - black_term6) % black_mod;
            }

            *black_pre.entry(black_v).or_insert(0i64) += 1;
        }

        ((black_ans % black_mod) + black_mod) as i32 % black_mod as i32
    }
}