impl Solution {
    pub fn max_collected_fruits(black_fruits: Vec<Vec<i32>>) -> i32 {
        let black_n = black_fruits.len();
        let mut black_total = 0;
        for black_i in 0..black_n { black_total += black_fruits[black_i][black_i]; }

        let mut black_dp1 = vec![vec![-1; black_n]; black_n];
        black_dp1[0][black_n - 1] = black_fruits[0][black_n - 1];
        for black_i in 1..black_n - 1 {
            for black_j in (black_i + 1).max(black_n - 1 - black_i)..black_n {
                let mut black_prev = black_dp1[black_i - 1][black_j];
                if black_j > 0 { black_prev = black_prev.max(black_dp1[black_i - 1][black_j - 1]); }
                if black_j < black_n - 1 { black_prev = black_prev.max(black_dp1[black_i - 1][black_j + 1]); }
                if black_prev != -1 { black_dp1[black_i][black_j] = black_prev + black_fruits[black_i][black_j]; }
            }
        }

        let mut black_dp2 = vec![vec![-1; black_n]; black_n];
        black_dp2[black_n - 1][0] = black_fruits[black_n - 1][0];
        for black_j in 1..black_n - 1 {
            for black_i in (black_j + 1).max(black_n - 1 - black_j)..black_n {
                let mut black_prev = black_dp2[black_i][black_j - 1];
                if black_i > 0 { black_prev = black_prev.max(black_dp2[black_i - 1][black_j - 1]); }
                if black_i < black_n - 1 { black_prev = black_prev.max(black_dp2[black_i + 1][black_j - 1]); }
                if black_prev != -1 { black_dp2[black_i][black_j] = black_prev + black_fruits[black_i][black_j]; }
            }
        }

        black_total + black_dp1[black_n - 2][black_n - 1].max(0) + black_dp2[black_n - 1][black_n - 2].max(0)
    }
}