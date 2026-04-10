impl Solution {
    pub fn minimum_operations(black_nums: Vec<i32>, black_target: Vec<i32>) -> i64 {
        let mut black_res = 0i64;
        let mut black_prev = 0i64;
        for black_i in 0..black_nums.len() {
            let black_diff = (black_target[black_i] as i64) - (black_nums[black_i] as i64);
            if (black_diff > 0 && black_prev < 0) || (black_diff < 0 && black_prev > 0) {
                black_res += black_diff.abs();
            } else if black_diff.abs() > black_prev.abs() {
                black_res += (black_diff - black_prev).abs();
            }
            black_prev = black_diff;
        }
        black_res
    }
}
