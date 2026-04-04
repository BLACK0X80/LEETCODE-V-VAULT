use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut black = BinaryHeap::new();
        for i in 0..nums1.len().min(k as usize) {
            black.push(Reverse((nums1[i] + nums2[0], i, 0)));
        }
        let mut res = vec![];
        while res.len() < k as usize && !black.is_empty() {
            let Reverse((_, i, j)) = black.pop().unwrap();
            res.push(vec![nums1[i], nums2[j]]);
            if j + 1 < nums2.len() {
                black.push(Reverse((nums1[i] + nums2[j+1], i, j+1)));
            }
        }
        res
    }
}
