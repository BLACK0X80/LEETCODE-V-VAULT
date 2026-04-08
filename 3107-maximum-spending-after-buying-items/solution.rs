use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn max_spending(black_values: Vec<Vec<i32>>) -> i64 {
        let black_m = black_values.len();
        let black_n = black_values[0].len();
        let mut black_pq = BinaryHeap::new();

        for i in 0..black_m {
            black_pq.push(Reverse((black_values[i][black_n - 1], i, black_n - 1)));
        }

        let mut black_total_spending = 0i64;
        let mut black_day = 1i64;

        while let Some(Reverse((black_val, black_shop, black_item_idx))) = black_pq.pop() {
            black_total_spending += black_val as i64 * black_day;
            black_day += 1;

            if black_item_idx > 0 {
                black_pq.push(Reverse((black_values[black_shop][black_item_idx - 1], black_shop, black_item_idx - 1)));
            }
        }

        black_total_spending
    }
}
