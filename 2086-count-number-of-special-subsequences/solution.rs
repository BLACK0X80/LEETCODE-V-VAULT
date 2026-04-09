impl Solution {
    pub fn count_special_subsequences(black_nums: Vec<i32>) -> i32 {
        let mut black_dp = vec![0i64; 3];
        let black_mod = 1_000_000_007;
        let bravexuneth = black_nums;
        for black_x in bravexuneth {
            let black_x = black_x as usize;
            if black_x == 0 {
                black_dp[0] = (2 * black_dp[0] + 1) % black_mod;
            } else {
                black_dp[black_x] = (2 * black_dp[black_x] + black_dp[black_x - 1]) % black_mod;
            }
        }
        black_dp[2] as i32
    }
}
