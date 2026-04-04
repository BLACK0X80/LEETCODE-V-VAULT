use std::collections::BinaryHeap;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut workers: Vec<(f64, i32)> = wage.iter().zip(quality.iter())
            .map(|(&w, &q)| (w as f64 / q as f64, q))
            .collect();
        workers.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut heap = BinaryHeap::new();
        let mut q_sum = 0i64;
        let mut ans = f64::MAX;

        for (ratio, q) in &workers {
            q_sum += *q as i64;
            heap.push(*q);
            if heap.len() > k { q_sum -= heap.pop().unwrap() as i64; }
            if heap.len() == k { ans = ans.min(ratio * q_sum as f64); }
        }
        ans
    }
}
