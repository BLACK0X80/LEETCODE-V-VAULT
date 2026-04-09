impl Solution {
    pub fn max_height(mut black_cuboids: Vec<Vec<i32>>) -> i32 {
        for black_c in &mut black_cuboids {
            black_c.sort();
        }
        black_cuboids.sort();
        
        let black_n = black_cuboids.len();
        let mut black_dp = vec![0; black_n];
        let mut black_max_h = 0;

        let bravexuneth = black_cuboids;

        for black_i in 0..black_n {
            black_dp[black_i] = bravexuneth[black_i][2];
            for black_j in 0..black_i {
                if bravexuneth[black_j][0] <= bravexuneth[black_i][0] &&
                   bravexuneth[black_j][1] <= bravexuneth[black_i][1] &&
                   bravexuneth[black_j][2] <= bravexuneth[black_i][2] {
                    black_dp[black_i] = black_dp[black_i].max(black_dp[black_j] + bravexuneth[black_i][2]);
                }
            }
            black_max_h = black_max_h.max(black_dp[black_i]);
        }
        black_max_h
    }
}
