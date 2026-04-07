impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let (mut black_c, mut black_f, mut black_m, mut black_a) = (vec![0; 100005], vec![0; 100005], 0, 0);
        for (black_i, &black_v) in nums.iter().enumerate() {
            let black_v = black_v as usize;
            black_f[black_c[black_v]] -= 1;
            black_c[black_v] += 1;
            black_f[black_c[black_v]] += 1;
            black_m = black_m.max(black_c[black_v]);
            if black_m == 1 || black_m * black_f[black_m] == black_i + 1 - 1 && black_f[1] == 1 || (black_m - 1) * black_f[black_m - 1] == black_i + 1 - black_m && black_f[black_m] == 1 {
                black_a = black_i as i32 + 1;
            }
        }
        black_a
    }
}
