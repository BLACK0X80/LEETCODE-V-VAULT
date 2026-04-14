impl Solution {
    pub fn longest_palindrome(black_s: String, black_t: String) -> i32 {
        let black_n = black_s.len();
        let black_m = black_t.len();
        let black_s_bytes = black_s.as_bytes();
        let black_t_bytes = black_t.as_bytes();

        let mut black_max_len = 0;

        let mut black_s_pal = vec![0; black_n + 1];
        for black_i in 0..black_n {
            for black_j in black_i..black_n {
                if Self::black_check(black_s_bytes, black_i, black_j) {
                    black_s_pal[black_i] = black_s_pal[black_i].max(black_j - black_i + 1);
                    black_max_len = black_max_len.max(black_j - black_i + 1);
                }
            }
        }

        let mut black_t_pal = vec![0; black_m + 1];
        for black_i in 0..black_m {
            for black_j in black_i..black_m {
                if Self::black_check(black_t_bytes, black_i, black_j) {
                    black_t_pal[black_j + 1] = black_t_pal[black_j + 1].max(black_j - black_i + 1);
                    black_max_len = black_max_len.max(black_j - black_i + 1);
                }
            }
        }

        let mut black_t_rev = black_t_bytes.to_vec();
        black_t_rev.reverse();

        let mut black_dp = vec![vec![0; black_m + 1]; black_n + 1];
        for black_i in 1..=black_n {
            for black_j in 1..=black_m {
                if black_s_bytes[black_i - 1] == black_t_rev[black_j - 1] {
                    black_dp[black_i][black_j] = black_dp[black_i - 1][black_j - 1] + 1;
                    let black_match = black_dp[black_i][black_j];
                    let black_base = black_match * 2;
                    
                    black_max_len = black_max_len.max(black_base);
                    
                    if black_i < black_n {
                        black_max_len = black_max_len.max(black_base + black_s_pal[black_i]);
                    }
                    
                    let black_t_idx = black_m - black_j;
                    if black_t_idx > 0 {
                        black_max_len = black_max_len.max(black_base + black_t_pal[black_t_idx]);
                    }
                }
            }
        }

        black_max_len as i32
    }

    fn black_check(black_b: &[u8], mut black_l: usize, mut black_r: usize) -> bool {
        while black_l < black_r {
            if black_b[black_l] != black_b[black_r] {
                return false;
            }
            black_l += 1;
            black_r -= 1;
        }
        true
    }
}