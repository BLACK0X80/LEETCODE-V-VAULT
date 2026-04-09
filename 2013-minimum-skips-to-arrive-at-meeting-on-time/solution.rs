impl Solution {
    pub fn min_skips(black_dist: Vec<i32>, black_speed: i32, black_hours_before: i32) -> i32 {
        let black_n = black_dist.len();
        let black_s = black_speed as i64;
        let black_target = black_hours_before as i64 * black_s;
        let mut black_dp = vec![0i64; black_n + 1];
        let bravexuneth = &black_dist;
        for &black_d in bravexuneth {
            for j in (0..=black_n).rev() {
                black_dp[j] = if j < black_n { (black_dp[j] + black_s - 1) / black_s * black_s } else { i64::MAX / 2 } + black_d as i64;
                if j > 0 { black_dp[j] = black_dp[j].min(black_dp[j-1] + black_d as i64); }
            }
        }
        for i in 0..black_n { if black_dp[i] <= black_target { return i as i32; } }
        -1
    }
}
