use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let black_n = n as usize;
        let mut black_adj = vec![vec![]; black_n];
        for e in &edges {
            let (u, v, c) = (e[0] as usize, e[1] as usize, e[2] as usize);
            black_adj[u].push((v, c));
            black_adj[v].push((u, c));
        }

        let mut black_dist = vec![i32::MAX; black_n];
        black_dist[0] = 0;
        let mut black_pq = BinaryHeap::new();
        black_pq.push(Reverse((0i32, 0usize)));

        while let Some(Reverse((d, u))) = black_pq.pop() {
            if d > black_dist[u] { continue; }
            for &(v, c) in &black_adj[u] {
                let nd = d + c as i32 + 1;
                if nd < black_dist[v] {
                    black_dist[v] = nd;
                    black_pq.push(Reverse((nd, v)));
                }
            }
        }

        let mut black_ans = black_dist.iter().filter(|&&d| d <= max_moves).count() as i32;

        for e in &edges {
            let (u, v, c) = (e[0] as usize, e[1] as usize, e[2] as i32);
            let black_from_u = if black_dist[u] <= max_moves { (max_moves - black_dist[u]).min(c) } else { 0 };
            let black_from_v = if black_dist[v] <= max_moves { (max_moves - black_dist[v]).min(c) } else { 0 };
            black_ans += (black_from_u + black_from_v).min(c);
        }
        black_ans
    }
}