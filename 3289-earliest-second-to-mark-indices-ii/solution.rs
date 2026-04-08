use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn earliest_second_to_mark_indices(black_nums: Vec<i32>, black_ci: Vec<i32>) -> i32 {
        let black_m = black_ci.len();
        let mut black_low = 1;
        let mut black_high = black_m as i32;
        let mut black_ans = -1;

        while black_low <= black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            if Self::black_check(&black_nums, &black_ci, black_mid) {
                black_ans = black_mid;
                black_high = black_mid - 1;
            } else {
                black_low = black_mid + 1;
            }
        }
        black_ans
    }

    fn black_check(black_nums: &Vec<i32>, black_ci: &Vec<i32>, black_limit: i32) -> bool {
        let black_n = black_nums.len();
        let mut black_first_occurrence = vec![-1; black_n + 1];
        for black_i in 0..black_limit as usize {
            let black_idx = black_ci[black_i] as usize;
            if black_first_occurrence[black_idx] == -1 {
                black_first_occurrence[black_idx] = black_i as i32;
            }
        }

        let mut black_pq = BinaryHeap::new();
        let mut black_free_time = 0i64;
        let mut black_saved_count = 0i64;
        let mut black_total_reduction = 0i64;

        for black_i in (0..black_limit as usize).rev() {
            let black_idx = black_ci[black_i] as usize;
            if black_i as i32 == black_first_occurrence[black_idx] && black_nums[black_idx - 1] > 0 {
                black_pq.push(Reverse(black_nums[black_idx - 1] as i64));
                black_total_reduction += black_nums[black_idx - 1] as i64;
                black_saved_count += 1;
                
                if black_free_time > 0 {
                    black_free_time -= 1;
                } else {
                    if let Some(Reverse(black_min)) = black_pq.pop() {
                        black_total_reduction -= black_min;
                        black_saved_count -= 1;
                    }
                    black_free_time += 1;
                }
            } else {
                black_free_time += 1;
            }
        }

        let mut black_total_sum = 0i64;
        for &black_v in black_nums {
            black_total_sum += black_v as i64;
        }

        let black_remaining_ops = black_total_sum - black_total_reduction + black_n as i64 - black_saved_count;
        black_remaining_ops <= black_free_time
    }
}
