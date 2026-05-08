impl Solution {
    pub fn min_operations(mut black_nums: Vec<i32>) -> i32 {
        let black_n = black_nums.len() as i32;
        black_nums.sort_unstable();
        black_nums.dedup();
        let (mut black_mx, mut black_r) = (0, 0);
        for black_l in 0..black_nums.len() {
            while black_r < black_nums.len() && black_nums[black_r] < black_nums[black_l] + black_n {
                black_r += 1;
            }
            black_mx = black_mx.max((black_r - black_l) as i32);
        }
        black_n - black_mx
    }
}