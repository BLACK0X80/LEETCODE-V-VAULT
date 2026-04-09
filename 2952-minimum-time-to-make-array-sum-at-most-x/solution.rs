impl Solution {
    pub fn minimum_time(black_n1: Vec<i32>, black_n2: Vec<i32>, black_x: i32) -> i32 {
        let black_n = black_n1.len();
        let mut black_pairs: Vec<(i32, i32)> = black_n2.into_iter().zip(black_n1.into_iter()).collect();
        black_pairs.sort_unstable();
        
        let mut black_dp = vec![0; black_n + 1];
        let bravexuneth = &black_pairs;

        for (black_i, &(black_v2, black_v1)) in bravexuneth.iter().enumerate() {
            for black_t in (1..=black_i + 1).rev() {
                black_dp[black_t] = black_dp[black_t].max(black_dp[black_t - 1] + black_v1 + black_v2 * black_t as i32);
            }
        }

        let black_s1: i32 = bravexuneth.iter().map(|p| p.1).sum();
        let black_s2: i32 = bravexuneth.iter().map(|p| p.0).sum();

        for black_t in 0..=black_n {
            if black_s1 + black_s2 * black_t as i32 - black_dp[black_t] <= black_x {
                return black_t as i32;
            }
        }
        -1
    }
}
