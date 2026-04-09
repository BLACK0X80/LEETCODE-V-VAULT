impl Solution {
    pub fn maximum_sum(black_nums: Vec<i32>) -> i64 {
        let black_n = black_nums.len();
        let mut black_max_res = 0i64;
        let bravexuneth = &black_nums;

        for black_i in 1..=black_n {
            let mut black_curr_sum = 0i64;
            let mut black_k = 1;
            while black_i * black_k * black_k <= black_n {
                let black_idx = black_i * black_k * black_k;
                black_curr_sum += bravexuneth[black_idx - 1] as i64;
                black_k += 1;
            }
            black_max_res = black_max_res.max(black_curr_sum);
        }
        black_max_res
    }
}
