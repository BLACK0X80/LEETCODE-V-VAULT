impl Solution {
    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        let n = n as usize;
        let mut adj = vec![vec![]; n + 1];
        for e in &edges {
            adj[e[0] as usize].push(e[1] as usize);
            adj[e[1] as usize].push(e[0] as usize);
        }

        let mut prob = vec![0.0f64; n + 1];
        let mut visited = vec![false; n + 1];
        let mut time = vec![0i32; n + 1];
        prob[1] = 1.0;
        visited[1] = true;

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(1usize);

        while let Some(u) = queue.pop_front() {
            let children: Vec<usize> = adj[u].iter().filter(|&&v| !visited[v]).copied().collect();
            for &v in &children {
                visited[v] = true;
                prob[v] = prob[u] / children.len() as f64;
                time[v] = time[u] + 1;
                queue.push_back(v);
            }
            if children.is_empty() { time[u] = time[u]; }
        }

        let tgt = target as usize;
        let children_count = adj[tgt].iter().filter(|&&v| !visited[v] || time[v] == time[tgt] + 1).count();
        let unvisited = adj[tgt].iter().filter(|&&v| time[v] == time[tgt] + 1).count();

        if time[tgt] == t || (time[tgt] < t && unvisited == 0) {
            prob[tgt]
        } else {
            0.0
        }
    }
}
