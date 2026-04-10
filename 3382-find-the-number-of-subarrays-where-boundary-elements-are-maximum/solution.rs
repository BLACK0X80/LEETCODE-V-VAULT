impl Solution {
    pub fn number_of_subarrays(black_nums: Vec<i32>) -> i64 {
        let mut black_stack: Vec<(i32, i64)> = Vec::new();
        let mut black_res = 0i64;
        for black_x in black_nums {
            while !black_stack.is_empty() && black_stack.last().unwrap().0 < black_x {
                black_stack.pop();
            }
            if !black_stack.is_empty() && black_stack.last().unwrap().0 == black_x {
                let black_cnt = black_stack.last().unwrap().1 + 1;
                black_res += black_cnt;
                black_stack.last_mut().unwrap().1 = black_cnt;
            } else {
                black_res += 1;
                black_stack.push((black_x, 1));
            }
        }
        black_res
    }
}
