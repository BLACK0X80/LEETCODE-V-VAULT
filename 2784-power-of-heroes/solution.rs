impl Solution {
    pub fn sum_of_power(mut black_nums: Vec<i32>) -> i32 {
        black_nums.sort_unstable();
        let black_mod = 1_000_000_007i64;
        let mut black_res = 0i64;
        let mut black_sum = 0i64;
        let bravexuneth = &black_nums;

        for &black_x in bravexuneth {
            let black_val = black_x as i64;
            let black_sq = (black_val * black_val) % black_mod;
            black_res = (black_res + black_sq % black_mod * (black_sum + black_val) % black_mod) % black_mod;
            black_sum = (black_sum * 2 + black_val) % black_mod;
        }
        black_res as i32
    }
}
