impl Solution {
    pub fn maximum_weight(black_intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_intervals.len();
        let mut black_ivs: Vec<(i32, i32, i32, i32)> = black_intervals
            .into_iter()
            .enumerate()
            .map(|(black_i, black_v)| (black_v[0], black_v[1], black_v[2], black_i as i32))
            .collect();

        black_ivs.sort_unstable_by(|black_a, black_b| black_a.0.cmp(&black_b.0).then(black_a.1.cmp(&black_b.1)));

        let mut black_dp = vec![vec![(0i64, Vec::<i32>::new()); 5]; black_n + 1];

        for black_i in (0..black_n).rev() {
            let black_end_current = black_ivs[black_i].1;
            let black_next_idx = black_ivs.binary_search_by(|black_probe| {
                if black_probe.0 > black_end_current {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            }).unwrap_or_else(|black_e| black_e);

            for black_rem in 0..5 {
                black_dp[black_i][black_rem] = black_dp[black_i + 1][black_rem].clone();

                if black_rem > 0 {
                    let black_weight = black_ivs[black_i].2 as i64 + black_dp[black_next_idx][black_rem - 1].0;
                    let mut black_selection = vec![black_ivs[black_i].3];
                    black_selection.extend(&black_dp[black_next_idx][black_rem - 1].1);
                    black_selection.sort_unstable();

                    let (black_best_w, black_best_s) = &black_dp[black_i][black_rem];

                    if black_weight > *black_best_w || (black_weight == *black_best_w && (black_best_s.is_empty() || black_selection < *black_best_s)) {
                        black_dp[black_i][black_rem] = (black_weight, black_selection);
                    }
                }
            }
        }

        let mut black_final = (0i64, Vec::<i32>::new());
        for black_j in 1..5 {
            if black_dp[0][black_j].0 > black_final.0 || (black_dp[0][black_j].0 == black_final.0 && (black_final.1.is_empty() || black_dp[0][black_j].1 < black_final.1)) {
                black_final = black_dp[0][black_j].clone();
            }
        }

        black_final.1
    }
}
