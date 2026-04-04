use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut in_mst = vec![false; n];
        let mut min_dist = vec![i32::MAX; n];
        min_dist[0] = 0;
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, 0)));
        let mut ans = 0;

        while let Some(Reverse((d, u))) = heap.pop() {
            if in_mst[u] { continue; }
            in_mst[u] = true;
            ans += d;
            for v in 0..n {
                if !in_mst[v] {
                    let cost = (points[u][0] - points[v][0]).abs() + (points[u][1] - points[v][1]).abs();
                    if cost < min_dist[v] {
                        min_dist[v] = cost;
                        heap.push(Reverse((cost, v)));
                    }
                }
            }
        }
        ans
    }
}
