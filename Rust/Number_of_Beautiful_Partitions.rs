impl Solution {
    pub fn beautiful_partitions(s: String, k: i32, min_length: i32) -> i32 {
        let black_mod = 1_000_000_007;
        let black_b = s.as_bytes();
        let black_n = s.len();
        let black_k = k as usize;
        let black_min = min_length as usize;

        let black_is_prime = |black_c: u8| matches!(black_c, b'2' | b'3' | b'5' | b'7');
        if !black_is_prime(black_b[0]) || black_is_prime(black_b[black_n - 1]) { return 0; }

        let mut black_dp = vec![0i32; black_n + 1];
        black_dp[0] = 1;

        for black_i in 1..=black_k {
            let mut black_next_dp = vec![0i32; black_n + 1];
            let mut black_sum = 0;
            for black_j in 1..=black_n {
                if black_j >= black_min {
                    let black_prev = black_j - black_min;
                    if black_prev == 0 || (!black_is_prime(black_b[black_prev - 1]) && black_is_prime(black_b[black_prev])) {
                        black_sum = (black_sum + black_dp[black_prev]) % black_mod;
                    }
                }
                if !black_is_prime(black_b[black_j - 1]) {
                    black_next_dp[black_j] = black_sum;
                }
            }
            black_dp = black_next_dp;
        }
        black_dp[black_n]
    }
}