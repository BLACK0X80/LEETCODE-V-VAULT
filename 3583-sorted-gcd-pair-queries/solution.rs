impl Solution {
    pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        let black_max_val = *nums.iter().max().unwrap() as usize;
        let mut black_freq = vec![0i64; black_max_val + 1];
        for &black_x in &nums { black_freq[black_x as usize] += 1; }

        let mut black_gcd_counts = vec![0i64; black_max_val + 1];
        for black_g in (1..=black_max_val).rev() {
            let mut black_c = 0i64;
            for black_m in (black_g..=black_max_val).step_by(black_g) {
                black_c += black_freq[black_m];
            }
            let mut black_total_pairs = black_c * (black_c - 1) / 2;
            for black_multiple in (2 * black_g..=black_max_val).step_by(black_g) {
                black_total_pairs -= black_gcd_counts[black_multiple];
            }
            black_gcd_counts[black_g] = black_total_pairs;
        }

        let mut black_prefix_sums = vec![0i64; black_max_val + 1];
        for black_i in 1..=black_max_val {
            black_prefix_sums[black_i] = black_prefix_sums[black_i - 1] + black_gcd_counts[black_i];
        }

        queries.into_iter().map(|black_q| {
            let mut black_low = 1;
            let mut black_high = black_max_val;
            let mut black_res = black_max_val as i32;
            while black_low <= black_high {
                let black_mid = (black_low + black_high) / 2;
                if black_prefix_sums[black_mid] > black_q {
                    black_res = black_mid as i32;
                    black_high = black_mid - 1;
                } else {
                    black_low = black_mid + 1;
                }
            }
            black_res
        }).collect()
    }
}
