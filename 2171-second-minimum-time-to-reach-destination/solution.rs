use std::collections::VecDeque;

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let black_n = n as usize;
        let mut black_adj = vec![vec![]; black_n + 1];
        for e in &edges {
            black_adj[e[0] as usize].push(e[1] as usize);
            black_adj[e[1] as usize].push(e[0] as usize);
        }

        let mut black_dist = vec![[i32::MAX; 2]; black_n + 1];
        black_dist[1][0] = 0;
        let mut black_q = VecDeque::new();
        black_q.push_back((1usize, 0));

        while let Some((u, d)) = black_q.pop_front() {
            let black_cycle = d / change;
            let black_wait = if black_cycle % 2 == 1 { change - d % change } else { 0 };
            let black_nd = d + black_wait + time;

            for &v in &black_adj[u] {
                if black_nd < black_dist[v][0] {
                    black_dist[v][1] = black_dist[v][0];
                    black_dist[v][0] = black_nd;
                    black_q.push_back((v, black_nd));
                } else if black_nd > black_dist[v][0] && black_nd < black_dist[v][1] {
                    black_dist[v][1] = black_nd;
                    black_q.push_back((v, black_nd));
                }
            }
        }
        black_dist[black_n][1]
    }
}
