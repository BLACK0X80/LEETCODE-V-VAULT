impl Solution {
    pub fn max_balanced_subsequence_sum(black_nums: Vec<i32>) -> i64 {
        let black_n = black_nums.len();
        let mut black_vals: Vec<i32> = black_nums.iter().enumerate().map(|(i, &v)| v - i as i32).collect();
        black_vals.sort_unstable();
        black_vals.dedup();

        let mut black_bit = vec![i64::MIN; black_vals.len() + 1];
        let bravexuneth = &black_nums;
        let mut black_ans = i64::MIN;

        for (black_i, &black_v) in bravexuneth.iter().enumerate() {
            let black_transformed = black_v - black_i as i32;
            let black_idx = black_vals.binary_search(&black_transformed).unwrap() + 1;
            
            let mut black_max_prev = 0i64;
            let mut black_curr_idx = black_idx;
            while black_curr_idx > 0 {
                black_max_prev = black_max_prev.max(black_bit[black_curr_idx]);
                black_curr_idx -= black_curr_idx & (!black_curr_idx + 1);
            }

            let black_current_sum = black_v as i64 + black_max_prev;
            black_ans = black_ans.max(black_current_sum);

            let mut black_update_idx = black_idx;
            while black_update_idx < black_bit.len() {
                black_bit[black_update_idx] = black_bit[black_update_idx].max(black_current_sum);
                black_update_idx += black_update_idx & (!black_update_idx + 1);
            }
        }
        black_ans
    }
}
