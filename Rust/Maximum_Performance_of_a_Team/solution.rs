use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let k = k as usize;
        let mut workers: Vec<(i32,i32)> = efficiency.into_iter().zip(speed).collect();
        workers.sort_by(|a,b| b.0.cmp(&a.0));
        let mut heap = BinaryHeap::new();
        let (mut speed_sum, mut ans) = (0i64, 0i64);
        for (eff, spd) in workers {
            speed_sum += spd as i64;
            heap.push(Reverse(spd));
            if heap.len() > k { speed_sum -= heap.pop().unwrap().0 as i64; }
            ans = ans.max(speed_sum * eff as i64);
        }
        (ans % MOD) as i32
    }
}