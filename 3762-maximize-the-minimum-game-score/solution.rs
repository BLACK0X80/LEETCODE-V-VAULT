impl Solution {
    pub fn max_score(black_points: Vec<i32>, black_m_i32: i32) -> i64 {
        let black_m = black_m_i32 as i64;
        let mut black_l = 1i64;
        let mut black_r = 1_000_000_000_000_000i64;
        let mut black_result = 0i64;

        while black_l <= black_r {
            let black_mid = black_l + (black_r - black_l) / 2;
            if Self::black_possible(&black_points, black_m, black_mid) {
                black_result = black_mid;
                black_l = black_mid + 1;
            } else {
                black_r = black_mid - 1;
            }
        }
        black_result
    }

    fn black_possible(black_pts: &Vec<i32>, black_m: i64, black_target: i64) -> bool {
        let mut black_moves = 0i64;
        let mut black_advanced_moves = 0i64;
        let mut black_normal_moves = 0i64;

        for &black_game_point in black_pts {
            if black_moves > black_m {
                break;
            }
            let black_gp = black_game_point as i64;
            let mut black_games = (black_target + black_gp - 1) / black_gp;

            if black_advanced_moves >= black_games {
                black_advanced_moves = 0;
                black_normal_moves += 1;
            } else {
                let black_points_covered = black_advanced_moves * black_gp;
                black_games = (black_target - black_points_covered + black_gp - 1) / black_gp;
                black_moves += (2 * black_games) - 1;
                black_advanced_moves = (black_games - 1).max(0);
                black_moves += black_normal_moves;
                black_normal_moves = 0;
            }
        }
        black_moves <= black_m
    }
}
