impl Solution {
    pub fn incremovable_subarray_count(black_nums: Vec<i32>) -> i64 {
        let black_n = black_nums.len();
        let mut black_left = 0;
        
        while black_left + 1 < black_n && black_nums[black_left] < black_nums[black_left + 1] {
            black_left += 1;
        }

        if black_left == black_n - 1 {
            let black_size = black_n as i64;
            return black_size * (black_size + 1) / 2;
        }

        let mut black_right = black_n - 1;
        while black_right > 0 && black_nums[black_right - 1] < black_nums[black_right] {
            black_right -= 1;
        }

        let mut black_ans: i64 = 0;

        black_ans += (black_n - black_right + 1) as i64;

        let mut black_j = black_right;
        for black_i in 0..=black_left {
            while black_j < black_n && black_nums[black_i] >= black_nums[black_j] {
                black_j += 1;
            }
            black_ans += (black_n - black_j + 1) as i64;
        }

        black_ans
    }
}
