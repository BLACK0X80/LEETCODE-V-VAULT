impl Solution {
    pub fn find_products_of_elements(black_queries: Vec<Vec<i64>>) -> Vec<i32> {
        let black_count_bits = |mut black_n: i64| { let (mut black_c, mut black_temp) = (0, black_n); for black_i in 0..50 { let black_full = (black_n + 1) / (1 << (black_i + 1)); black_c += black_full * (1 << black_i) + 0.max((black_n + 1) % (1 << (black_i + 1)) - (1 << black_i)); } black_c };
        let black_sum_powers = |mut black_n: i64| { let mut black_s = 0; for black_i in 0..50 { let black_full = (black_n + 1) / (1 << (black_i + 1)); black_s += (black_full * (1 << black_i) + 0.max((black_n + 1) % (1 << (black_i + 1)) - (1 << black_i))) * black_i as i64; } black_s };
        let black_solve = |black_idx: i64| {
            let (mut black_l, mut black_r, mut black_num) = (0, 1e15 as i64, 0);
            while black_l <= black_r { let black_m = (black_l + black_r) / 2; if black_count_bits(black_m) <= black_idx { black_num = black_m; black_l = black_m + 1; } else { black_r = black_m - 1; } }
            let (mut black_res, mut black_rem) = (black_sum_powers(black_num), black_idx - black_count_bits(black_num));
            for black_i in 0..50 { if black_rem > 0 && ((black_num + 1) >> black_i) & 1 == 1 { black_res += black_i as i64; black_rem -= 1; } }
            black_res
        };
        black_queries.iter().map(|black_q| {
            let black_p = black_solve(black_q[1] + 1) - black_solve(black_q[0]);
            let (mut black_base, mut black_exp, mut black_res, black_mod) = (2i64, black_p, 1i64, black_q[2]);
            while black_exp > 0 { if black_exp % 2 == 1 { black_res = (black_res * black_base) % black_mod; } black_base = (black_base * black_base) % black_mod; black_exp /= 2; }
            (black_res % black_mod) as i32
        }).collect()
    }
}