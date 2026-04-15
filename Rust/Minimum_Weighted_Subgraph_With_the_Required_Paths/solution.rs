use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn minimum_weight(n: i32, edges: Vec<Vec<i32>>, src1: i32, src2: i32, dest: i32) -> i64 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        let mut rg = vec![vec![]; n];
        for e in &edges {
            g[e[0] as usize].push((e[1] as usize, e[2] as i64));
            rg[e[1] as usize].push((e[0] as usize, e[2] as i64));
        }
        let dijkstra = |src: usize, adj: &Vec<Vec<(usize, i64)>>| {
            let mut dist = vec![i64::MAX; n];
            dist[src] = 0;
            let mut heap = BinaryHeap::new();
            heap.push(Reverse((0i64, src)));
            while let Some(Reverse((d, u))) = heap.pop() {
                if d > dist[u] { continue; }
                for &(v, w) in &adj[u] {
                    if dist[u] + w < dist[v] { dist[v] = dist[u] + w; heap.push(Reverse((dist[v], v))); }
                }
            }
            dist
        };
        let d1 = dijkstra(src1 as usize, &g);
        let d2 = dijkstra(src2 as usize, &g);
        let dd = dijkstra(dest as usize, &rg);
        let mut ans = i64::MAX;
        for i in 0..n {
            if d1[i] != i64::MAX && d2[i] != i64::MAX && dd[i] != i64::MAX {
                ans = ans.min(d1[i] + d2[i] + dd[i]);
            }
        }
        if ans == i64::MAX { -1 } else { ans }
    }
}