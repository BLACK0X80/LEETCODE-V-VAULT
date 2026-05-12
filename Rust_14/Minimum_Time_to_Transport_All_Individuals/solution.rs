use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Black {
    cost: u128,
    mask: usize,
    stage: usize,
    boat: usize,
}

impl Ord for Black {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Black {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn min_time(n: i32, k: i32, m: i32, time: Vec<i32>, mul: Vec<f64>) -> f64 {
        let n = n as usize;
        let k = k as usize;
        let m = m as usize;
        let all = (1 << n) - 1;
        let scale = 1_000_000u128;
        
        let mut dist: HashMap<(usize, usize, usize), u128> = HashMap::new();
        let mut pq = BinaryHeap::new();
        
        dist.insert((all, 0, 0), 0);
        pq.push(Black { cost: 0, mask: all, stage: 0, boat: 0 });
        
        while let Some(Black { cost, mask, stage, boat }) = pq.pop() {
            if let Some(&d) = dist.get(&(mask, stage, boat)) {
                if cost > d { continue; }
            }
            
            if mask == 0 && boat == 1 {
                return cost as f64 / scale as f64;
            }
            
            if boat == 0 {
                let mut sub = mask;
                while sub > 0 {
                    let cnt = sub.count_ones() as usize;
                    if cnt >= 1 && cnt <= k {
                        let mx = (0..n).filter(|&i| (sub >> i) & 1 == 1)
                            .map(|i| time[i] as f64).fold(0.0, f64::max);
                        let cross = mx * mul[stage];
                        let nm = mask & !sub;
                        let ns = (stage + cross.floor() as usize) % m;
                        let nc = cost + (cross * scale as f64).round() as u128;
                        let key = (nm, ns, 1);
                        if !dist.contains_key(&key) || nc < dist[&key] {
                            dist.insert(key, nc);
                            pq.push(Black { cost: nc, mask: nm, stage: ns, boat: 1 });
                        }
                    }
                    sub = (sub - 1) & mask;
                }
            } else if mask != 0 {
                for i in 0..n {
                    if (mask >> i) & 1 == 0 {
                        let ret = time[i] as f64 * mul[stage];
                        let nm = mask | (1 << i);
                        let ns = (stage + ret.floor() as usize) % m;
                        let nc = cost + (ret * scale as f64).round() as u128;
                        let key = (nm, ns, 0);
                        if !dist.contains_key(&key) || nc < dist[&key] {
                            dist.insert(key, nc);
                            pq.push(Black { cost: nc, mask: nm, stage: ns, boat: 0 });
                        }
                    }
                }
            }
        }
        -1.0
    }
}