impl Solution {
    pub fn minimum_increments(black_nums: Vec<i32>, black_target: Vec<i32>) -> i32 {
        let black_m = black_target.len();
        let mut black_lcm_map = vec![0i64; 1 << black_m];

        fn black_gcd(mut a: i64, mut b: i64) -> i64 {
            while b != 0 {
                a %= b;
                std::mem::swap(&mut a, &mut b);
            }
            a
        }

        fn black_lcm(a: i64, b: i64) -> i64 {
            if a == 0 || b == 0 { return 0; }
            let g = black_gcd(a, b);
            let black_res = a.saturating_mul(b / g);
            if black_res > 200_000 { 200_001 } else { black_res }
        }

        for black_mask in 1..(1 << black_m) {
            let mut black_res_lcm = 0;
            for black_i in 0..black_m {
                if (black_mask >> black_i) & 1 == 1 {
                    if black_res_lcm == 0 {
                        black_res_lcm = black_target[black_i] as i64;
                    } else {
                        black_res_lcm = black_lcm(black_res_lcm, black_target[black_i] as i64);
                    }
                }
            }
            black_lcm_map[black_mask] = black_res_lcm;
        }

        let mut black_dp = vec![i32::MAX / 2; 1 << black_m];
        black_dp[0] = 0;

        for &black_num in &black_nums {
            let mut black_next_dp = black_dp.clone();
            for black_mask in 1..(1 << black_m) {
                let black_l = black_lcm_map[black_mask];
                let black_rem = (black_num as i64) % black_l;
                let black_cost = if black_rem == 0 { 0 } else { (black_l - black_rem) as i32 };
                
                for black_prev_mask in 0..(1 << black_m) {
                    let black_new_mask = black_prev_mask | black_mask;
                    let black_total_cost = black_dp[black_prev_mask] + black_cost;
                    if black_total_cost < black_next_dp[black_new_mask] {
                        black_next_dp[black_new_mask] = black_total_cost;
                    }
                }
            }
            black_dp = black_next_dp;
        }

        black_dp[(1 << black_m) - 1]
    }
}
