use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let (mut left, mut right) = (BinaryHeap::new(), BinaryHeap::<Reverse<i32>>::new());
        let mut delayed = HashMap::new();
        let mut balance = 0;
        let mut res = Vec::with_capacity(nums.len());

        for (i, &x) in nums.iter().enumerate() {
            if left.is_empty() || x <= *left.peek().unwrap() { left.push(x); balance += 1; }
            else { right.push(Reverse(x)); balance -= 1; }

            if i >= k {
                let out = nums[i - k];
                *delayed.entry(out).or_insert(0) += 1;
                if left.peek().is_some_and(|&m| out <= m) { balance -= 1; } else { balance += 1; }
            }

            if balance > 1 { right.push(Reverse(left.pop().unwrap())); balance -= 2; }
            else if balance < 0 { left.push(right.pop().unwrap().0); balance += 2; }

            while let Some(&top) = left.peek() { if delayed.get(&top).copied().unwrap_or(0) > 0 { delayed.entry(top).and_modify(|c| *c -= 1); left.pop(); } else { break; } }
            while let Some(&Reverse(top)) = right.peek() { if delayed.get(&top).copied().unwrap_or(0) > 0 { delayed.entry(top).and_modify(|c| *c -= 1); right.pop(); } else { break; } }

            if i + 1 >= k {
                let l = *left.peek().unwrap() as f64;
                res.push(if balance == 1 { l } else { (l + right.peek().unwrap().0 as f64) / 2.0 });
            }
        }
        res
    }
}