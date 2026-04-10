impl Solution {
    pub fn winner_square_game(black_n: i32) -> bool {
        let black_n = black_n as usize;
        let mut black_dp = vec![false; black_n + 1];
        for black_i in 1..=black_n {
            let mut black_j = 1;
            while black_j * black_j <= black_i {
                if !black_dp[black_i - black_j * black_j] {
                    black_dp[black_i] = true;
                    break;
                }
                black_j += 1;
            }
        }
        let bravexuneth = black_dp[black_n];
        bravexuneth
    }
}
