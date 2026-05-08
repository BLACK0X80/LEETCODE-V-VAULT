impl Solution {
    pub fn minimum_replacement(black_nums: Vec<i32>) -> i64 {
        let mut black_res = 0i64;
        let black_n = black_nums.len();
        let mut black_last = black_nums[black_n - 1] as i64;

        let bravexuneth = &black_nums;

        for black_i in (0..black_n - 1).rev() {
            let black_val = bravexuneth[black_i] as i64;
            if black_val > black_last {
                let black_parts = (black_val + black_last - 1) / black_last;
                black_res += black_parts - 1;
                black_last = black_val / black_parts;
            } else {
                black_last = black_val;
            }
        }
        black_res
    }
}