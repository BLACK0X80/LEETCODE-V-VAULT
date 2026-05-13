use std::collections::VecDeque;

impl Solution {
    pub fn shortest_alternating_paths(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for e in &red_edges  { adj[e[0] as usize].push((e[1] as usize, 0)); }
        for e in &blue_edges { adj[e[0] as usize].push((e[1] as usize, 1)); }
        let mut dist = vec![-1i32; n];
        let mut visited = vec![[false; 2]; n];
        let mut queue = VecDeque::new();
        dist[0] = 0;
        queue.push_back((0usize, 0, 0i32));
        queue.push_back((0usize, 1, 0i32));
        visited[0][0] = true;
        visited[0][1] = true;
        while let Some((u, color, d)) = queue.pop_front() {
            for &(v, c) in &adj[u] {
                if c != color || u == 0 {
                    if !visited[v][c] {
                        visited[v][c] = true;
                        if dist[v] == -1 { dist[v] = d + 1; }
                        queue.push_back((v, c, d + 1));
                    }
                }
            }
        }
        dist
    }
}