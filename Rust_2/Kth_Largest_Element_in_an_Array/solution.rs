use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::with_capacity(k as usize + 1);
        for n in nums {
            heap.push(Reverse(n));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        heap.pop().unwrap().0
    }
}