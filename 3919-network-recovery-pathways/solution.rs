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
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        let n = online.len();
        let mut black_adj = vec![vec![]; n];
        for e in &edges {
            black_adj[e[0] as usize].push((e[1] as usize, e[2] as i64));
        }

        fn black_check(x: i64, n: usize, k: i64, online: &[bool], adj: &[Vec<(usize, i64)>]) -> bool {
            let mut dist = vec![i64::MAX; n];
            dist[0] = 0;
            let mut pq = BinaryHeap::new();
            pq.push(Black { cost: 0, node: 0 });
            while let Some(Black { cost, node }) = pq.pop() {
                if cost > dist[node] || cost > k { continue; }
                if node == n - 1 { return true; }
                for &(v, w) in &adj[node] {
                    if w < x || (!online[v] && v != n - 1) { continue; }
                    if cost + w < dist[v] {
                        dist[v] = cost + w;
                        pq.push(Black { cost: dist[v], node: v });
                    }
                }
            }
            false
        }

        let (mut lo, mut hi) = (0i64, 1_000_000_000i64);
        let mut ans = -1i32;
        while lo <= hi {
            let mid = (lo + hi) / 2;
            if black_check(mid, n, k, &online, &black_adj) {
                ans = mid as i32;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        ans
    }
}
