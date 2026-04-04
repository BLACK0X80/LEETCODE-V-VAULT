use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Graph { adj: Vec<Vec<(usize, i32)>> }

impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut adj = vec![vec![]; n as usize];
        for e in &edges { adj[e[0] as usize].push((e[1] as usize, e[2])); }
        Graph { adj }
    }
    fn add_edge(&mut self, edge: Vec<i32>) {
        self.adj[edge[0] as usize].push((edge[1] as usize, edge[2]));
    }
    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let n = self.adj.len();
        let mut dist = vec![i32::MAX; n];
        dist[node1 as usize] = 0;
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, node1 as usize)));
        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] { continue; }
            for &(v, w) in &self.adj[u] {
                if dist[u] + w < dist[v] { dist[v] = dist[u] + w; heap.push(Reverse((dist[v], v))); }
            }
        }
        if dist[node2 as usize] == i32::MAX { -1 } else { dist[node2 as usize] }
    }
}
