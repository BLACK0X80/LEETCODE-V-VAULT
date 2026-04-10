impl Solution {
    pub fn min_stable(black_nums: Vec<i32>, black_max_c: i32) -> i32 {
        let black_n = black_nums.len();
        let black_log_n = (black_n as f64).log2() as usize + 1;
        let mut black_st = vec![vec![0; black_n]; black_log_n];

        for black_j in 0..black_n {
            black_st[0][black_j] = black_nums[black_j];
        }

        for black_i in 1..black_log_n {
            for black_j in 0..=(black_n - (1 << black_i)) {
                black_st[black_i][black_j] = Self::black_gcd(
                    black_st[black_i - 1][black_j],
                    black_st[black_i - 1][black_j + (1 << (black_i - 1))],
                );
            }
        }

        let black_query = |black_l: usize, black_r: usize, black_st: &Vec<Vec<i32>>| -> i32 {
            let black_dist = black_r - black_l + 1;
            let black_i = (black_dist as f64).log2() as usize;
            Self::black_gcd(black_st[black_i][black_l], black_st[black_i][black_r - (1 << black_i) + 1])
        };

        let mut black_left_stable = vec![0; black_n];
        let mut black_j_ptr = 0;
        for black_i in 0..black_n {
            while black_j_ptr < black_i && black_query(black_j_ptr, black_i, &black_st) == 1 {
                black_j_ptr += 1;
            }
            black_left_stable[black_i] = black_j_ptr;
        }

        let mut black_l_bs = 0;
        let mut black_r_bs = black_n as i32;
        while black_l_bs < black_r_bs {
            let black_mid = (black_l_bs + black_r_bs) / 2;
            let mut black_cnt = 0;
            let mut black_curr_j = 0;
            let mut black_curr_g = 0;
            let mut black_idx = 0;

            while black_idx < black_n {
                black_curr_g = if black_curr_g == 0 { black_nums[black_idx] } else { Self::black_gcd(black_curr_g, black_nums[black_idx]) };
                
                if black_curr_g == 1 {
                    black_curr_j = black_curr_j.max(black_left_stable[black_idx]);
                    black_curr_g = black_query(black_curr_j, black_idx, &black_st);
                }

                if (black_idx - black_curr_j) as i32 >= black_mid {
                    if black_curr_g > 1 {
                        black_cnt += 1;
                    }
                    black_curr_g = 0;
                    black_curr_j = black_idx + 1;
                }
                black_idx += 1;
            }

            if black_cnt <= black_max_c {
                black_r_bs = black_mid;
            } else {
                black_l_bs = black_mid + 1;
            }
        }

        black_l_bs
    }

    fn black_gcd(mut black_a: i32, mut black_b: i32) -> i32 {
        while black_b != 0 {
            black_a %= black_b;
            std::mem::swap(&mut black_a, &mut black_b);
        }
        black_a
    }
}
