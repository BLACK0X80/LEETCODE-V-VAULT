use std::collections::BinaryHeap;

impl Solution {
    pub fn modified_graph_edges(n: i32, mut e: Vec<Vec<i32>>, s: i32, d: i32, t: i32) -> Vec<Vec<i32>> {
        let (n, s, d, t) = (n as usize, s as usize, d as usize, t as i64);
        let (mut x, mut y) = (vec![vec![]; n], vec![]);
        
        for (i, v) in e.iter_mut().enumerate() {
            if v[2] < 0 { v[2] = 2e9 as i32; y.push(i); }
            x[v[0] as usize].push((v[1] as usize, i));
            x[v[1] as usize].push((v[0] as usize, i));
        }
        
        let mut z = |e: &Vec<Vec<i32>>| {
            let (mut dp, mut pq) = (vec![i64::MAX; n], BinaryHeap::from([(0i64, s)]));
            dp[s] = 0;
            while let Some((c, u)) = pq.pop() {
                if -c > dp[u] { continue; }
                for &(v, i) in &x[u] {
                    let w = e[i][2] as i64;
                    if dp[v] > -c + w {
                        dp[v] = -c + w;
                        pq.push((-dp[v], v));
                    }
                }
            }
            dp[d]
        };
        
        let mut k = z(&e);
        if k < t { return vec![]; }
        
        for i in y {
            if k <= t { break; }
            e[i][2] = 1;
            k = z(&e);
            if k <= t { e[i][2] += (t - k) as i32; }
        }
        
        if k > t { vec![] } else { e }
    }
}