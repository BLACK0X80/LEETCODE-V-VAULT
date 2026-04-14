impl Solution {
    pub fn minimum_difference(black_nums: Vec<i32>) -> i32 {
        let black_2n = black_nums.len();
        let black_n = black_2n / 2;
        let black_sum_total: i64 = black_nums.iter().map(|&x| x as i64).sum();
        
        let black_left_half = &black_nums[0..black_n];
        let black_right_half = &black_nums[black_n..black_2n];

        let mut black_left_sums = vec![vec![]; black_n + 1];
        let mut black_right_sums = vec![vec![]; black_n + 1];

        for black_i in 0..(1 << black_n) {
            let mut black_cur_sum: i64 = 0;
            let mut black_count = 0;
            for black_bit in 0..black_n {
                if (black_i >> black_bit) & 1 == 1 {
                    black_cur_sum += black_left_half[black_bit] as i64;
                    black_count += 1;
                }
            }
            black_left_sums[black_count].push(black_cur_sum);
        }

        for black_i in 0..(1 << black_n) {
            let mut black_cur_sum: i64 = 0;
            let mut black_count = 0;
            for black_bit in 0..black_n {
                if (black_i >> black_bit) & 1 == 1 {
                    black_cur_sum += black_right_half[black_bit] as i64;
                    black_count += 1;
                }
            }
            black_right_sums[black_count].push(black_cur_sum);
        }

        for black_i in 0..=black_n {
            black_right_sums[black_i].sort_unstable();
        }

        let mut black_min_diff = i64::MAX;

        for black_k in 0..=black_n {
            let black_needed_in_right = black_n - black_k;
            let black_target = black_sum_total / 2;

            for &black_s1 in &black_left_sums[black_k] {
                let black_s_target = black_target - black_s1;
                
                let black_vec = &black_right_sums[black_needed_in_right];
                let black_pos = match black_vec.binary_search(&black_s_target) {
                    Ok(black_p) => black_p,
                    Err(black_p) => black_p,
                };

                if black_pos < black_vec.len() {
                    let black_s2 = black_vec[black_pos];
                    let black_s_full = black_s1 + black_s2;
                    black_min_diff = black_min_diff.min((black_sum_total - 2 * black_s_full).abs());
                }
                if black_pos > 0 {
                    let black_s2 = black_vec[black_pos - 1];
                    let black_s_full = black_s1 + black_s2;
                    black_min_diff = black_min_diff.min((black_sum_total - 2 * black_s_full).abs());
                }
            }
        }

        black_min_diff as i32
    }
}