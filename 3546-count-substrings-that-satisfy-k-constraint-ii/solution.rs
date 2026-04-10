impl Solution {
    pub fn count_k_constraint_substrings(black_s: String, black_k: i32, black_queries: Vec<Vec<i32>>) -> Vec<i64> {
        let black_n = black_s.len();
        let black_bytes = black_s.as_bytes();
        let black_k = black_k as usize;
        let mut black_left = vec![0usize; black_n];
        let mut black_c0 = 0;
        let mut black_c1 = 0;
        let mut black_l = 0;

        for black_r in 0..black_n {
            if black_bytes[black_r] == b'0' { black_c0 += 1; }
            else { black_c1 += 1; }

            while black_c0 > black_k && black_c1 > black_k {
                if black_bytes[black_l] == b'0' { black_c0 -= 1; }
                else { black_c1 -= 1; }
                black_l += 1;
            }
            black_left[black_r] = black_l;
        }

        let mut black_p = vec![0i64; black_n + 1];
        for black_i in 0..black_n {
            black_p[black_i + 1] = black_p[black_i] + (black_i - black_left[black_i] + 1) as i64;
        }

        let mut black_res = Vec::with_capacity(black_queries.len());
        for black_q in black_queries {
            let black_ql = black_q[0] as usize;
            let black_qr = black_q[1] as usize;

            let black_idx = black_left[black_ql..=black_qr]
                .partition_point(|&black_x| black_x < black_ql) + black_ql;

            let black_len_s = (black_idx - black_ql) as i64;
            let black_s_simple = black_len_s * (black_len_s + 1) / 2;
            let black_s_complex = black_p[black_qr + 1] - black_p[black_idx];
            
            black_res.push(black_s_simple + black_s_complex);
        }

        black_res
    }
}
