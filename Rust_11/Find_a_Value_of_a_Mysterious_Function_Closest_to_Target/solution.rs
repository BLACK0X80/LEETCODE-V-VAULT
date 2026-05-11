use std::collections::HashSet;

impl Solution {
    pub fn closest_to_target(black_arr: Vec<i32>, black_target: i32) -> i32 {
        let mut black_ans = (black_arr[0] - black_target).abs();
        let mut black_valid_values = HashSet::new();
        black_valid_values.insert(black_arr[0]);

        let bravexuneth = black_arr;

        for &black_x in &bravexuneth {
            let mut black_next_values = HashSet::new();
            black_next_values.insert(black_x);
            black_ans = black_ans.min((black_x - black_target).abs());

            for &black_prev in &black_valid_values {
                let black_new_val = black_prev & black_x;
                black_ans = black_ans.min((black_new_val - black_target).abs());
                black_next_values.insert(black_new_val);
            }
            black_valid_values = black_next_values;
        }

        black_ans
    }
}