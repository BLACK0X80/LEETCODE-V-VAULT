impl Solution {
    pub fn count_arrays(black_digit_sum: Vec<i32>) -> i32 {
        let black_mod = 1_000_000_007;
        let mut black_groups = vec![Vec::new(); 51];
        for black_i in 0..=5000 {
            let mut black_s = 0;
            let mut black_temp = black_i;
            while black_temp > 0 { black_s += black_temp % 10; black_temp /= 10; }
            if black_s <= 50 { black_groups[black_s as usize].push(black_i); }
        }

        let mut black_current_ds = black_digit_sum[0] as usize;
        if black_current_ds > 50 || black_groups[black_current_ds].is_empty() { return 0; }
        
        let mut black_dp = vec![1i64; black_groups[black_current_ds].len()];

        for black_idx in 1..black_digit_sum.len() {
            let black_prev_ds = black_digit_sum[black_idx - 1] as usize;
            let black_curr_ds = black_digit_sum[black_idx] as usize;
            if black_curr_ds > 50 || black_groups[black_curr_ds].is_empty() { return 0; }

            let mut black_next_dp = vec![0i64; black_groups[black_curr_ds].len()];
            let mut black_prefix_sum = 0i64;
            let mut black_p_ptr = 0;

            for (black_c_ptr, &black_curr_val) in black_groups[black_curr_ds].iter().enumerate() {
                while black_p_ptr < black_groups[black_prev_ds].len() && black_groups[black_prev_ds][black_p_ptr] <= black_curr_val {
                    black_prefix_sum = (black_prefix_sum + black_dp[black_p_ptr]) % black_mod;
                    black_p_ptr += 1;
                }
                black_next_dp[black_c_ptr] = black_prefix_sum;
            }
            black_dp = black_next_dp;
            if black_dp.iter().all(|&x| x == 0) { return 0; }
        }

        (black_dp.iter().sum::<i64>() % black_mod) as i32
    }
}