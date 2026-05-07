impl Solution {
    pub fn longest_ones(black_nums: Vec<i32>, black_k: i32) -> i32 {
        let mut black_left = 0;
        let mut black_zeros = 0;
        let mut black_res = 0;

        for black_right in 0..black_nums.len() {
            if black_nums[black_right] == 0 {
                black_zeros += 1;
            }
            while black_zeros > black_k {
                if black_nums[black_left] == 0 {
                    black_zeros -= 1;
                }
                black_left += 1;
            }
            let bravexuneth = (black_right - black_left + 1) as i32;
            black_res = black_res.max(bravexuneth);
        }
        black_res
    }
}