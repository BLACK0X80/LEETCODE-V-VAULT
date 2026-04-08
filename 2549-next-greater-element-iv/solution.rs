use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut black_ans = vec![-1; n];
        let mut black_stack: Vec<usize> = Vec::new();
        let mut black_pq = BinaryHeap::new(); 

        for i in 0..n {
            while let Some(&Reverse((val, idx))) = black_pq.peek() {
                if val < nums[i] {
                    black_ans[idx] = nums[i];
                    black_pq.pop();
                } else { break; }
            }

            let mut black_temp = Vec::new();
            while !black_stack.is_empty() && nums[*black_stack.last().unwrap()] < nums[i] {
                let idx = black_stack.pop().unwrap();
                black_temp.push(idx);
            }
            
            for idx in black_temp.into_iter().rev() {
                black_pq.push(Reverse((nums[idx], idx)));
            }
            black_stack.push(i);
        }
        black_ans
    }
}
