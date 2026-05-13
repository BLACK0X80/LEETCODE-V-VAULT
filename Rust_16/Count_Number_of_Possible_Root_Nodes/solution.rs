use std::collections::HashSet;

impl Solution {
    pub fn root_count(black_edges: Vec<Vec<i32>>, black_guesses: Vec<Vec<i32>>, black_k: i32) -> i32 {
        let black_n = black_edges.len() + 1;
        let mut black_adj = vec![vec![]; black_n];
        for e in black_edges {
            black_adj[e[0] as usize].push(e[1] as usize);
            black_adj[e[1] as usize].push(e[0] as usize);
        }

        let mut black_gs = HashSet::new();
        for g in black_guesses { black_gs.insert((g[0] as usize, g[1] as usize)); }

        let mut black_initial = 0;
        let mut black_stack = vec![(0, black_n)];
        let mut black_order = vec![];
        let mut black_parent = vec![0; black_n];

        let mut black_q = vec![(0, black_n)];
        while let Some((u, p)) = black_q.pop() {
            black_order.push((u, p));
            for &v in &black_adj[u] {
                if v != p {
                    if black_gs.contains(&(u, v)) { black_initial += 1; }
                    black_q.push((v, u));
                }
            }
        }

        let mut black_res = 0;
        let mut black_current = vec![0; black_n];
        black_current[0] = black_initial;
        
        
        let mut black_q = vec![(0, black_n)];
        while let Some((u, p)) = black_q.pop() {
            if black_current[u] >= black_k { black_res += 1; }
            for &v in &black_adj[u] {
                if v != p {
                    let mut next_score = black_current[u];
                    if black_gs.contains(&(u, v)) { next_score -= 1; }
                    if black_gs.contains(&(v, u)) { next_score += 1; }
                    black_current[v] = next_score;
                    black_q.push((v, u));
                }
            }
        }
        black_res
    }
}