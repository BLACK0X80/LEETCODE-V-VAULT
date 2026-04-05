use std::collections::HashMap;

impl Solution {
    pub fn get_max_grid_happiness(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
        let black_m = m as usize;
        let black_n = n as usize;
        let mut black_memo = HashMap::new();
        let black_pow3: Vec<usize> = (0..=black_n).map(|black_i| 3usize.pow(black_i as u32)).collect();

        fn black_dfs(black_pos: usize, black_mask: usize, black_ic: usize, black_ec: usize, black_m: usize, black_n: usize, black_pow3: &[usize], black_memo: &mut HashMap<(usize, usize, usize, usize), i32>) -> i32 {
            if black_pos == black_m * black_n || (black_ic == 0 && black_ec == 0) { return 0; }
            if let Some(&black_v) = black_memo.get(&(black_pos, black_mask, black_ic, black_ec)) { return black_v; }

            let black_col = black_pos % black_n;
            let black_up = (black_mask / black_pow3[black_n - 1]) % 3;
            let black_left = if black_col > 0 { black_mask % 3 } else { 0 };
            let black_base = (black_mask * 3) % black_pow3[black_n];

            let mut black_res = black_dfs(black_pos + 1, black_base, black_ic, black_ec, black_m, black_n, black_pow3, black_memo);

            if black_ic > 0 {
                let mut black_score = 120;
                if black_up == 1 { black_score -= 60; } else if black_up == 2 { black_score -= 10; }
                if black_left == 1 { black_score -= 60; } else if black_left == 2 { black_score -= 10; }
                black_res = black_res.max(black_score + black_dfs(black_pos + 1, black_base + 1, black_ic - 1, black_ec, black_m, black_n, black_pow3, black_memo));
            }

            if black_ec > 0 {
                let mut black_score = 40;
                if black_up == 1 { black_score -= 10; } else if black_up == 2 { black_score += 40; }
                if black_left == 1 { black_score -= 10; } else if black_left == 2 { black_score += 40; }
                black_res = black_res.max(black_score + black_dfs(black_pos + 1, black_base + 2, black_ic, black_ec - 1, black_m, black_n, black_pow3, black_memo));
            }

            black_memo.insert((black_pos, black_mask, black_ic, black_ec), black_res);
            black_res
        }

        black_dfs(0, 0, introverts_count as usize, extroverts_count as usize, black_m, black_n, &black_pow3, &mut black_memo)
    }
}
