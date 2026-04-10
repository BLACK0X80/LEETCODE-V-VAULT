impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i64 {
        let mut black_max_val = i32::MIN;
        let mut black_max_idx = 0;

        for (black_i, &black_v) in nums.iter().enumerate() {
            if black_v > black_max_val {
                black_max_val = black_v;
                black_max_idx = black_i;
            }
        }

        let mut black_rotated1 = Vec::with_capacity(nums.len());
        black_rotated1.extend_from_slice(&nums[black_max_idx..]);
        black_rotated1.extend_from_slice(&nums[..black_max_idx]);

        let mut black_rotated2 = Vec::with_capacity(nums.len());
        black_rotated2.extend_from_slice(&nums[(black_max_idx + 1)..]);
        black_rotated2.extend_from_slice(&nums[..(black_max_idx + 1)]);

        let black_ans1 = Self::black_maximum_profit(&black_rotated1, k);
        let black_ans2 = Self::black_maximum_profit(&black_rotated2, k);

        black_ans1.max(black_ans2)
    }

    fn black_maximum_profit(black_prices: &Vec<i32>, black_k: i32) -> i64 {
        let black_k_usize = black_k as usize;
        let mut black_f = vec![vec![i64::MIN / 2; 3]; black_k_usize + 2];

        for black_j in 1..=black_k_usize + 1 {
            black_f[black_j][0] = 0;
        }

        for &black_p_i32 in black_prices {
            let black_p = black_p_i32 as i64;
            for black_j in (1..=black_k_usize + 1).rev() {
                black_f[black_j][0] = black_f[black_j][0].max(black_f[black_j][1] + black_p).max(black_f[black_j][2] - black_p);
                black_f[black_j][1] = black_f[black_j][1].max(black_f[black_j - 1][0] - black_p);
                black_f[black_j][2] = black_f[black_j][2].max(black_f[black_j - 1][0] + black_p);
            }
        }

        black_f[black_k_usize + 1][0]
    }
}
