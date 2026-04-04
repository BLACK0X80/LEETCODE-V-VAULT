use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct KthLargest { k: usize, heap: BinaryHeap<Reverse<i32>> }

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut s = KthLargest { k: k as usize, heap: BinaryHeap::new() };
        for n in nums { s.add(n); }
        s
    }
    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        while self.heap.len() > self.k { self.heap.pop(); }
        self.heap.peek().unwrap().0
    }
}
