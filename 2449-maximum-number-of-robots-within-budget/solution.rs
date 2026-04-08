use std::collections::VecDeque;

impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let mut black_dq = VecDeque::new();
        let mut black_sum_run = 0i64;
        let mut black_left = 0;
        let mut black_max_robots = 0;

        for black_right in 0..charge_times.len() {
            while !black_dq.is_empty() && charge_times[*black_dq.back().unwrap()] <= charge_times[black_right] {
                black_dq.pop_back();
            }
            black_dq.push_back(black_right);
            black_sum_run += running_costs[black_right] as i64;

            while !black_dq.is_empty() && (charge_times[*black_dq.front().unwrap()] as i64 + (black_right - black_left + 1) as i64 * black_sum_run) > budget {
                if *black_dq.front().unwrap() == black_left {
                    black_dq.pop_front();
                }
                black_sum_run -= running_costs[black_left] as i64;
                black_left += 1;
            }
            black_max_robots = black_max_robots.max(black_right - black_left + 1);
        }
        black_max_robots as i32
    }
}
