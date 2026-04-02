use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let full = (1 << n) - 1;
        let mut visited = vec![vec![false; 1 << n]; n];
        let mut queue = VecDeque::new();

        for i in 0..n {
            let mask = 1 << i;
            queue.push_back((i, mask, 0));
            visited[i][mask] = true;
        }

        while let Some((node, mask, dist)) = queue.pop_front() {
            if mask == full { return dist; }
            for &nb in &graph[node] {
                let nb = nb as usize;
                let new_mask = mask | (1 << nb);
                if !visited[nb][new_mask] {
                    visited[nb][new_mask] = true;
                    queue.push_back((nb, new_mask, dist + 1));
                }
            }
        }

        0
    }
}
