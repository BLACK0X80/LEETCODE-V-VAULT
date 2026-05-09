impl Solution {
    pub fn max_sum_trionic(black_nums: Vec<i32>) -> i64 {
        let black_n = black_nums.len();
        let mut black_max_ending_at = vec![0i64; black_n];
        
        for black_i in 0..black_n {
            black_max_ending_at[black_i] = black_nums[black_i] as i64;
            if black_i > 0 && black_nums[black_i - 1] < black_nums[black_i] {
                if black_max_ending_at[black_i - 1] > 0 {
                    black_max_ending_at[black_i] += black_max_ending_at[black_i - 1];
                }
            }
        }

        let mut black_max_starting_at = vec![0i64; black_n];
        for black_i in (0..black_n).rev() {
            black_max_starting_at[black_i] = black_nums[black_i] as i64;
            if black_i < black_n - 1 && black_nums[black_i] < black_nums[black_i + 1] {
                if black_max_starting_at[black_i + 1] > 0 {
                    black_max_starting_at[black_i] += black_max_starting_at[black_i + 1];
                }
            }
        }

        let black_pqs = Self::black_decompose(&black_nums);
        let mut black_ans = i64::MIN;

        for (black_p, black_q, black_sum) in black_pqs {
            if black_p > 0 && black_q < black_n - 1 && black_p < black_q {
                if black_nums[black_p - 1] < black_nums[black_p] && black_nums[black_q] < black_nums[black_q + 1] {
                    black_ans = black_ans.max(black_max_ending_at[black_p - 1] + black_sum + black_max_starting_at[black_q + 1]);
                }
            }
        }

        black_ans
    }

    fn black_decompose(black_nums: &Vec<i32>) -> Vec<(usize, usize, i64)> {
        let black_n = black_nums.len();
        let mut black_subarrays = Vec::new();
        if black_n == 0 { return black_subarrays; }

        let mut black_l = 0;
        let mut black_sum = black_nums[0] as i64;

        for black_i in 1..black_n {
            if black_nums[black_i - 1] <= black_nums[black_i] {
                black_subarrays.push((black_l, black_i - 1, black_sum));
                black_l = black_i;
                black_sum = 0;
            }
            black_sum += black_nums[black_i] as i64;
        }
        black_subarrays.push((black_l, black_n - 1, black_sum));
        black_subarrays
    }
}