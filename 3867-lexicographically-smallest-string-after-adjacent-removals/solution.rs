impl Solution {
    pub fn lexicographically_smallest_string(black_s: String) -> String {
        let black_n = black_s.len();
        let black_b = black_s.as_bytes();
        let mut black_dp = vec![vec![String::new(); black_n + 1]; black_n + 1];

        fn black_is_consec(a: u8, b: u8) -> bool {
            let black_diff = (a as i16 - b as i16).abs();
            black_diff == 1 || black_diff == 25
        }

        for black_len in 1..=black_n {
            for black_i in 0..=black_n - black_len {
                let black_j = black_i + black_len;
                
                let mut black_res = format!("{}{}", black_b[black_i] as char, black_dp[black_i + 1][black_j]);

                for black_k in black_i + 1..black_j {
                    if black_is_consec(black_b[black_i], black_b[black_k]) && black_dp[black_i + 1][black_k].is_empty() {
                        let black_candidate = black_dp[black_k + 1][black_j].clone();
                        if black_candidate < black_res {
                            black_res = black_candidate;
                        }
                    }
                }
                black_dp[black_i][black_j] = black_res;
            }
        }

        black_dp[0][black_n].clone()
    }
}
