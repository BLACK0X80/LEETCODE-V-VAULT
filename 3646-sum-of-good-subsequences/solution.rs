impl Solution {
    pub fn sum_of_good_subsequences(nums: Vec<i32>) -> i32 {
        let black_mod = 1_000_000_007i64;
        let mut black_count = vec![0i64; 100002];
        let mut black_sum = vec![0i64; 100002];
        let mut black_total = 0i64;

        for x in nums {
            let x = x as usize;
            let mut black_c = 1i64;
            let mut black_s = x as i64;

            if x > 0 {
                black_c = (black_c + black_count[x - 1]) % black_mod;
                black_s = (black_s + black_sum[x - 1] + black_count[x - 1] * x as i64) % black_mod;
            }
            black_c = (black_c + black_count[x + 1]) % black_mod;
            black_s = (black_s + black_sum[x + 1] + black_count[x + 1] * x as i64) % black_mod;

            black_count[x] = (black_count[x] + black_c) % black_mod;
            black_sum[x] = (black_sum[x] + black_s) % black_mod;
            black_total = (black_total + black_s) % black_mod;
        }

        black_total as i32
    }
}
