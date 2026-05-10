impl Solution {
    pub fn max_active_sections_after_trade(black_s: String, black_queries: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_s.len();
        let black_b = black_s.as_bytes();
        let mut black_active_count = 0;
        for &black_byte in black_b {
            if black_byte == b'1' { black_active_count += 1; }
        }

        let mut black_segments = Vec::new();
        let mut black_start = 0;
        for black_i in 0..black_n {
            if black_i == black_n - 1 || black_b[black_i] != black_b[black_i + 1] {
                black_segments.push((black_start, black_i - black_start + 1));
                black_start = black_i + 1;
            }
        }

        let black_segment_count = black_segments.len();
        let black_max_power = 20;
        let black_inf = 1_000_000_000;
        let mut black_rmq = vec![vec![-black_inf; black_segment_count]; black_max_power];

        for black_i in 0..black_segment_count {
            if black_b[black_segments[black_i].0] == b'0' && black_i + 2 < black_segment_count {
                black_rmq[0][black_i] = (black_segments[black_i].1 + black_segments[black_i + 2].1) as i32;
            }
        }

        for black_power in 1..black_max_power {
            let black_range_len = 1 << black_power;
            let black_half = 1 << (black_power - 1);
            if black_range_len > black_segment_count { break; }
            for black_i in 0..=(black_segment_count - black_range_len) {
                black_rmq[black_power][black_i] = black_rmq[black_power - 1][black_i].max(black_rmq[black_power - 1][black_i + black_half]);
            }
        }

        let black_get_max_in_range = |black_l: usize, black_r: usize| -> i32 {
            if black_l > black_r { return -black_inf; }
            let black_range_size = black_r - black_l + 1;
            let black_p = (black_range_size as f64).log2().floor() as usize;
            black_rmq[black_p][black_l].max(black_rmq[black_p][black_r - (1 << black_p) + 1])
        };

        let mut black_result = Vec::with_capacity(black_queries.len());

        for black_query in black_queries {
            let black_left = black_query[0] as usize;
            let black_right = black_query[1] as usize;

            let black_left_idx = match black_segments.binary_search_by_key(&black_left, |&(black_s, _)| black_s) {
                Ok(black_found) => black_found,
                Err(black_e) => black_e - 1,
            };
            let black_right_idx = match black_segments.binary_search_by_key(&black_right, |&(black_s, _)| black_s) {
                Ok(black_found) => black_found,
                Err(black_e) => black_e - 1,
            };

            if black_right_idx - black_left_idx + 1 <= 2 {
                black_result.push(black_active_count);
                continue;
            }

            let black_get_size = |black_idx: usize| -> i32 {
                if black_idx == black_left_idx {
                    (black_segments[black_left_idx].1 - (black_left - black_segments[black_left_idx].0)) as i32
                } else if black_idx == black_right_idx {
                    (black_right - black_segments[black_right_idx].0 + 1) as i32
                } else {
                    black_segments[black_idx].1 as i32
                }
            };

            let black_calc = |black_idx: usize| -> i32 {
                if black_b[black_segments[black_idx].0] == b'1' { return -black_inf; }
                black_get_size(black_idx) + black_get_size(black_idx + 2)
            };

            let mut black_best = 0.max(black_get_max_in_range(black_left_idx + 1, black_right_idx.saturating_sub(3)));
            black_best = black_best.max(black_calc(black_left_idx));
            if black_right_idx >= 2 {
                black_best = black_best.max(black_calc(black_right_idx - 2));
            }

            black_result.push(black_active_count + black_best);
        }

        black_result
    }
}