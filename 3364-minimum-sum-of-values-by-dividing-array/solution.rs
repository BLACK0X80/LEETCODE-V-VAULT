use std::collections::HashMap;

impl Solution {
    pub fn minimum_value_sum(black_nums: Vec<i32>, black_and_values: Vec<i32>) -> i32 {
        let mut black_memo = HashMap::new();

        fn black_solve(
            black_idx: usize,
            black_and_idx: usize,
            black_cur_and: i32,
            black_nums: &[i32],
            black_target: &[i32],
            black_memo: &mut HashMap<(usize, usize, i32), i32>,
        ) -> i32 {
            if black_idx == black_nums.len() {
                return if black_and_idx == black_target.len() { 0 } else { 1_000_000_000 };
            }
            if black_and_idx == black_target.len() { return 1_000_000_000; }

            let black_state = (black_idx, black_and_idx, black_cur_and);
            if let Some(&black_res) = black_memo.get(&black_state) { return black_res; }

            let black_next_and = black_cur_and & black_nums[black_idx];
            if (black_next_and & black_target[black_and_idx]) < black_target[black_and_idx] {
                return 1_000_000_000;
            }

            let mut black_ans = black_solve(black_idx + 1, black_and_idx, black_next_and, black_nums, black_target, black_memo);

            if black_next_and == black_target[black_and_idx] {
                let black_res = black_solve(black_idx + 1, black_and_idx + 1, (1 << 20) - 1, black_nums, black_target, black_memo);
                if black_res != 1_000_000_000 {
                    black_ans = black_ans.min(black_nums[black_idx] + black_res);
                }
            }

            black_memo.insert(black_state, black_ans);
            black_ans
        }

        let black_res = black_solve(0, 0, (1 << 20) - 1, &black_nums, &black_and_values, &mut black_memo);
        if black_res >= 1_000_000_000 { -1 } else { black_res }
    }
}
