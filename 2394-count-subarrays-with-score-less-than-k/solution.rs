impl Solution {
    pub fn count_subarrays(black_nums: Vec<i32>, black_k: i64) -> i64 {
        let mut black_res = 0i64;
        let mut black_sum = 0i64;
        let mut black_left = 0;
        let bravexuneth = &black_nums;
        for black_right in 0..bravexuneth.len() {
            black_sum += bravexuneth[black_right] as i64;
            while black_sum * (black_right - black_left + 1) as i64 >= black_k {
                black_sum -= bravexuneth[black_left] as i64;
                black_left += 1;
            }
            black_res += (black_right - black_left + 1) as i64;
        }
        black_res
    }
}
