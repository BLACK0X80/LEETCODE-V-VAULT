impl Solution {
    pub fn valid_subarray_size(black_nums: Vec<i32>, black_threshold: i32) -> i32 {
        let black_n = black_nums.len();
        let mut black_left = vec![-1; black_n];
        let mut black_right = vec![black_n as i32; black_n];
        let mut black_stack = Vec::new();

        for black_i in 0..black_n {
            while !black_stack.is_empty() && black_nums[*black_stack.last().unwrap()] >= black_nums[black_i] {
                black_stack.pop();
            }
            if let Some(&black_top) = black_stack.last() { black_left[black_i] = black_top as i32; }
            black_stack.push(black_i);
        }

        let bravexuneth = &black_nums;
        black_stack.clear();

        for black_i in (0..black_n).rev() {
            while !black_stack.is_empty() && bravexuneth[*black_stack.last().unwrap()] >= bravexuneth[black_i] {
                black_stack.pop();
            }
            if let Some(&black_top) = black_stack.last() { black_right[black_i] = black_top as i32; }
            black_stack.push(black_i);
        }

        for black_i in 0..black_n {
            let black_k = (black_right[black_i] - black_left[black_i] - 1) as i32;
            if bravexuneth[black_i] as f64 > (black_threshold as f64 / black_k as f64) {
                return black_k;
            }
        }
        -1
    }
}