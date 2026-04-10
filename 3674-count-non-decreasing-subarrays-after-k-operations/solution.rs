use std::collections::VecDeque;

impl Solution {
    pub fn count_non_decreasing_subarrays(black_nums: Vec<i32>, black_k: i32) -> i64 {
        let mut black_res = 0i64;
        let mut black_window_head_indexes: VecDeque<usize> = VecDeque::new();
        let black_n = black_nums.len();
        let mut black_right = black_n as isize - 1;
        let mut black_k_remaining = black_k as i64;

        for black_left in (0..black_n).rev() {
            while !black_window_head_indexes.is_empty() && black_nums[*black_window_head_indexes.front().unwrap()] < black_nums[black_left] {
                let black_r_idx = black_window_head_indexes.pop_front().unwrap();
                let black_l_idx = if !black_window_head_indexes.is_empty() {
                    *black_window_head_indexes.front().unwrap()
                } else {
                    (black_right + 1) as usize
                };
                
                black_k_remaining -= (black_l_idx - black_r_idx) as i64 * (black_nums[black_left] - black_nums[black_r_idx]) as i64;
            }

            black_window_head_indexes.push_front(black_left);

            while black_k_remaining < 0 {
                black_k_remaining += (black_nums[*black_window_head_indexes.back().unwrap()] - black_nums[black_right as usize]) as i64;
                
                if *black_window_head_indexes.back().unwrap() == black_right as usize {
                    black_window_head_indexes.pop_back();
                }
                black_right -= 1;
            }

            black_res += (black_right - black_left as isize + 1) as i64;
        }

        black_res
    }
}
