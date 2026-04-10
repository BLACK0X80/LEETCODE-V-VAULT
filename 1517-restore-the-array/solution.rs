impl Solution {
    pub fn number_of_arrays(black_s: String, black_k: i32) -> i32 {
        let black_mod = 1_000_000_007;
        let black_bytes = black_s.as_bytes();
        let black_n = black_bytes.len();
        let black_k64 = black_k as i64;
        let mut black_dp = vec![0; black_n + 1];
        black_dp[black_n] = 1;

        for black_i in (0..black_n).rev() {
            if black_bytes[black_i] == b'0' { continue; }
            let mut black_num = 0i64;
            for black_j in black_i..black_n {
                black_num = black_num * 10 + (black_bytes[black_j] - b'0') as i64;
                if black_num > black_k64 { break; }
                black_dp[black_i] = (black_dp[black_i] + black_dp[black_j + 1]) % black_mod;
            }
        }
        black_dp[0]
    }
}
