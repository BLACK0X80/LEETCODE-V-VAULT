use std::collections::VecDeque;

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![]; n + 1];
        for r in &roads {
            adj[r[0] as usize].push((r[1] as usize, r[2]));
            adj[r[1] as usize].push((r[0] as usize, r[2]));
        }
        let mut visited = vec![false; n + 1];
        let mut queue = VecDeque::new();
        let mut ans = i32::MAX;
        queue.push_back(1usize);
        visited[1] = true;
        while let Some(u) = queue.pop_front() {
            for &(v, w) in &adj[u] {
                ans = ans.min(w);
                if !visited[v] { visited[v] = true; queue.push_back(v); }
            }
        }
        ans
    }
}