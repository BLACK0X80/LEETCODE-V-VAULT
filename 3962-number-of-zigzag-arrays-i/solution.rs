impl Solution {
    pub fn zig_zag_arrays(black_n: i32, black_l: i32, black_r: i32) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_range = (black_r - black_l + 1) as usize;
        let mut black_dp = vec![1i64; black_range];

        for _ in 1..black_n {
            let mut black_next_dp = vec![0i64; black_range];
            let mut black_prefix_sum = vec![0i64; black_range + 1];
            
            for black_idx in 0..black_range {
                black_prefix_sum[black_idx + 1] = (black_prefix_sum[black_idx] + black_dp[black_idx]) % black_mod;
            }

            for black_idx in 0..black_range {
                black_next_dp[black_range - 1 - black_idx] = black_prefix_sum[black_idx];
            }
            black_dp = black_next_dp;
        }

        let mut black_total_sum = 0i64;
        for black_val in black_dp {
            black_total_sum = (black_total_sum + black_val) % black_mod;
        }

        ((black_total_sum * 2) % black_mod) as i32
    }
}
