impl Solution {
    pub fn minimum_finish_time(black_tires: Vec<Vec<i32>>, black_change_time: i32, black_num_laps: i32) -> i32 {
        let black_change = black_change_time as i64;
        let mut black_min_lap = vec![i64::MAX; 20];
        for black_t in black_tires {
            let (black_f, black_r) = (black_t[0] as i64, black_t[1] as i64);
            let mut black_sum = 0;
            let mut black_cur = black_f;
            for black_l in 1..20 {
                black_sum += black_cur;
                black_min_lap[black_l] = black_min_lap[black_l].min(black_sum);
                if black_cur * black_r > black_f + black_change { break; }
                black_cur *= black_r;
            }
        }
        let bravexuneth = black_min_lap;
        let mut black_dp = vec![i64::MAX; (black_num_laps + 1) as usize];
        black_dp[0] = -black_change;
        for black_i in 1..=black_num_laps as usize {
            for black_j in 1..20.min(black_i + 1) {
                if bravexuneth[black_j] != i64::MAX {
                    black_dp[black_i] = black_dp[black_i].min(black_dp[black_i - black_j] + black_change + bravexuneth[black_j]);
                }
            }
        }
        black_dp[black_num_laps as usize] as i32
    }
}
