impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let black_n = nums.len();
        if black_n == 1 { return nums[0]; }
        
        std::cmp::max(
            Self::black_solve(&nums[1..]),
            Self::black_solve(&nums[..black_n - 1])
        )
    }

    fn black_solve(black_h: &[i32]) -> i32 {
        let (mut black_prev, mut black_curr) = (0, 0);
        for &black_v in black_h {
            let black_tmp = black_curr;
            black_curr = std::cmp::max(black_prev + black_v, black_curr);
            black_prev = black_tmp;
        }
        black_curr
    }
}