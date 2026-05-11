use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(black_nums: Vec<i32>, black_k: i32) -> Vec<i32> {
        let mut black_deque = VecDeque::new();
        let mut black_res = Vec::new();
        let black_k = black_k as usize;

        for black_i in 0..black_nums.len() {
            if let Some(&black_front) = black_deque.front() {
                if black_front + black_k <= black_i {
                    black_deque.pop_front();
                }
            }

            while let Some(&black_back) = black_deque.back() {
                if black_nums[black_back] <= black_nums[black_i] {
                    black_deque.pop_back();
                } else {
                    break;
                }
            }

            black_deque.push_back(black_i);

            if black_i >= black_k - 1 {
                let bravexuneth = black_nums[*black_deque.front().unwrap()];
                black_res.push(bravexuneth);
            }
        }
        black_res
    }
}