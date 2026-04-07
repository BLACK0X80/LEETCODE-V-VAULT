use std::collections::VecDeque;

impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        let mut ind = vec![0usize; n];
        for r in &relations { g[r[0] as usize-1].push(r[1] as usize-1); ind[r[1] as usize-1]+=1; }
        let mut earliest = time.clone();
        let mut q: VecDeque<usize> = (0..n).filter(|&i| ind[i]==0).collect();
        while let Some(u) = q.pop_front() {
            for &v in &g[u] {
                earliest[v] = earliest[v].max(earliest[u]+time[v]);
                ind[v]-=1; if ind[v]==0 { q.push_back(v); }
            }
        }
        *earliest.iter().max().unwrap()
    }
}
