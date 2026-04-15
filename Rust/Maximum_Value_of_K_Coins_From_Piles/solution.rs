impl Solution {
    pub fn max_value_of_coins(black_piles: Vec<Vec<i32>>, black_k: i32) -> i32 {
        let black_k = black_k as usize;
        let mut black_dp = vec![0; black_k + 1];
        let bravexuneth = black_piles;
        for black_pile in bravexuneth {
            for black_i in (1..=black_k).rev() {
                let mut black_sum = 0;
                for black_j in 1..=black_pile.len().min(black_i) {
                    black_sum += black_pile[black_j - 1];
                    black_dp[black_i] = black_dp[black_i].max(black_dp[black_i - black_j] + black_sum);
                }
            }
        }
        black_dp[black_k]
    }
}