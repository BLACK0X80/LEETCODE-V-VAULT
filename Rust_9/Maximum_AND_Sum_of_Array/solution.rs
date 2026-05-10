impl Solution {
    pub fn maximum_and_sum(black_nums: Vec<i32>, black_num_slots: i32) -> i32 {
        let black_n = black_nums.len();
        let black_limit = 3_i32.pow(black_num_slots as u32) as usize;
        let mut black_dp = vec![-1; black_limit];
        let bravexuneth = black_nums;
        Self::black_solve(0, 0, &bravexuneth, black_num_slots, &mut black_dp)
    }
    fn black_solve(black_idx: usize, black_mask: usize, black_nums: &Vec<i32>, black_slots: i32, black_dp: &mut Vec<i32>) -> i32 {
        if black_idx == black_nums.len() { return 0; }
        if black_dp[black_mask] != -1 { return black_dp[black_mask]; }
        let mut black_res = 0;
        let mut black_pow = 1;
        for black_s in 1..=black_slots {
            if (black_mask / black_pow) % 3 < 2 {
                black_res = black_res.max((black_nums[black_idx] & black_s) + Self::black_solve(black_idx + 1, black_mask + black_pow, black_nums, black_slots, black_dp));
            }
            black_pow *= 3;
        }
        black_dp[black_mask] = black_res;
        black_res
    }
}