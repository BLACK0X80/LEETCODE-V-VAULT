use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Black {
    cost: i64,
    node: usize,
}
impl Ord for Black {
    fn cmp(&self, other: &Self) -> Ordering { other.cost.cmp(&self.cost) }
}
impl PartialOrd for Black {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Solution {
    pub fn find_answer(n: i32, edges: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as usize;
        let mut black_adj = vec![vec![]; n];
        for e in &edges {
            black_adj[e[0] as usize].push((e[1] as usize, e[2] as i64));
            black_adj[e[1] as usize].push((e[0] as usize, e[2] as i64));
        }

        fn black_dij(n: usize, start: usize, adj: &[Vec<(usize, i64)>]) -> Vec<i64> {
            let mut dist = vec![i64::MAX; n];
            dist[start] = 0;
            let mut pq = BinaryHeap::new();
            pq.push(Black { cost: 0, node: start });
            while let Some(Black { cost, node }) = pq.pop() {
                if cost > dist[node] { continue; }
                for &(v, w) in &adj[node] {
                    if cost + w < dist[v] {
                        dist[v] = cost + w;
                        pq.push(Black { cost: dist[v], node: v });
                    }
                }
            }
            dist
        }

        let black_d1 = black_dij(n, 0, &black_adj);
        let black_d2 = black_dij(n, n - 1, &black_adj);
        let black_short = black_d1[n - 1];
        if black_short == i64::MAX {
            return vec![false; edges.len()];
        }

        edges.iter().map(|e| {
            let (u, v, w) = (e[0] as usize, e[1] as usize, e[2] as i64);
            if black_d1[u] != i64::MAX && black_d2[v] != i64::MAX && black_d1[u] + w + black_d2[v] == black_short { return true; }
            if black_d1[v] != i64::MAX && black_d2[u] != i64::MAX && black_d1[v] + w + black_d2[u] == black_short { return true; }
            false
        }).collect()
    }
}
