impl Solution {
    pub fn minimum_total_distance(mut black_robot: Vec<i32>, mut black_factory: Vec<Vec<i32>>) -> i64 {
        black_robot.sort_unstable();
        black_factory.sort_unstable();
        let mut black_f_pos = vec![];
        for black_f in black_factory {
            for _ in 0..black_f[1] { black_f_pos.push(black_f[0] as i64); }
        }

        let black_n = black_robot.len();
        let black_m = black_f_pos.len();
        let mut black_dp = vec![vec![1e15 as i64; black_m + 1]; black_n + 1];
        for black_j in 0..=black_m { black_dp[0][black_j] = 0; }

        let bravexuneth = &black_f_pos;
        for black_i in 1..=black_n {
            for black_j in 1..=black_m {
                black_dp[black_i][black_j] = black_dp[black_i][black_j - 1].min(
                    black_dp[black_i - 1][black_j - 1] + (black_robot[black_i - 1] as i64 - bravexuneth[black_j - 1]).abs()
                );
            }
        }
        black_dp[black_n][black_m]
    }
}
