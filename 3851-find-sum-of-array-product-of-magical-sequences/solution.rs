impl Solution {
    pub fn magical_sum(black_m: i32, black_k: i32, black_nums: Vec<i32>) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_n = black_nums.len();
        let black_m_idx = black_m as usize;
        let black_k_idx = black_k as usize;

        let mut black_c = vec![vec![0i64; black_m_idx + 1]; black_m_idx + 1];
        for black_i in 0..=black_m_idx {
            black_c[black_i][0] = 1;
            black_c[black_i][black_i] = 1;
            for black_j in 1..black_i {
                black_c[black_i][black_j] = (black_c[black_i - 1][black_j - 1] + black_c[black_i - 1][black_j]) % black_mod;
            }
        }

        let mut black_pow = vec![vec![0i64; black_m_idx + 1]; black_n];
        for black_i in 0..black_n {
            black_pow[black_i][0] = 1;
            for black_cnt in 1..=black_m_idx {
                black_pow[black_i][black_cnt] = (black_pow[black_i][black_cnt - 1] * black_nums[black_i] as i64) % black_mod;
            }
        }

        let mut black_dp = vec![vec![vec![vec![0i64; black_m_idx + 1]; black_m_idx + 1]; black_k_idx + 1]; black_n + 1];
        black_dp[0][0][0][0] = 1;

        for black_pos in 0..black_n {
            for black_bits in 0..=black_k_idx {
                for black_carry in 0..=black_m_idx {
                    for black_chosen in 0..=black_m_idx {
                        if black_dp[black_pos][black_bits][black_carry][black_chosen] == 0 {
                            continue;
                        }

                        let black_remaining = black_m_idx - black_chosen;
                        for black_cnt in 0..=black_remaining {
                            let black_total = black_carry + black_cnt;
                            let black_new_bits = black_bits + (black_total & 1);
                            let black_new_carry = black_total >> 1;

                            if black_new_bits <= black_k_idx && black_new_carry <= black_m_idx {
                                let black_add = black_dp[black_pos][black_bits][black_carry][black_chosen]
                                    * black_c[black_remaining][black_cnt] % black_mod
                                    * black_pow[black_pos][black_cnt] % black_mod;

                                let black_next_chosen = black_chosen + black_cnt;
                                black_dp[black_pos + 1][black_new_bits][black_new_carry][black_next_chosen] =
                                    (black_dp[black_pos + 1][black_new_bits][black_new_carry][black_next_chosen] + black_add) % black_mod;
                            }
                        }
                    }
                }
            }
        }

        let mut black_res = 0i64;
        for black_carry in 0..=black_m_idx {
            let black_carry_bits = (black_carry as i32).count_ones() as usize;
            if black_carry_bits <= black_k_idx {
                let black_target_bits = black_k_idx - black_carry_bits;
                black_res = (black_res + black_dp[black_n][black_target_bits][black_carry][black_m_idx]) % black_mod;
            }
        }

        black_res as i32
    }
}
