impl Solution {
    pub fn number_of_unique_good_subsequences(black_binary: String) -> i32 {
        let black_mod = 1_000_000_007;
        let mut black_dp = [0, 0];
        let mut black_has_zero = 0;
        for black_c in black_binary.chars() {
            if black_c == '1' {
                black_dp[1] = (black_dp[0] + black_dp[1] + 1) % black_mod;
            } else {
                black_dp[0] = (black_dp[0] + black_dp[1]) % black_mod;
                black_has_zero = 1;
            }
        }
        (black_dp[0] + black_dp[1] + black_has_zero) % black_mod
    }
}
