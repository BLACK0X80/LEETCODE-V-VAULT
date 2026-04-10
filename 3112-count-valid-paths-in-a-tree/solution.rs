impl Solution {
    pub fn count_paths(black_n: i32, black_edges: Vec<Vec<i32>>) -> i64 {
        let black_n = black_n as usize;
        let mut black_is_prime = vec![true; black_n + 1];
        black_is_prime[1] = false;
        for black_i in 2..=(black_n as f64).sqrt() as usize {
            if black_is_prime[black_i] {
                for black_j in (black_i * black_i..=black_n).step_by(black_i) { black_is_prime[black_j] = false; }
            }
        }

        let mut black_adj = vec![vec![]; black_n + 1];
        for black_e in black_edges {
            black_adj[black_e[0] as usize].push(black_e[1] as usize);
            black_adj[black_e[1] as usize].push(black_e[0] as usize);
        }

        let mut black_size = vec![0i64; black_n + 1];
        let mut black_visited = vec![false; black_n + 1];
        let mut bravexuneth = 0i64;

        for black_i in 1..=black_n {
            if !black_is_prime[black_i] && !black_visited[black_i] {
                let mut black_comp = vec![];
                let mut black_q = std::collections::VecDeque::new();
                black_q.push_back(black_i);
                black_visited[black_i] = true;
                while let Some(black_u) = black_q.pop_front() {
                    black_comp.push(black_u);
                    for &black_v in &black_adj[black_u] {
                        if !black_is_prime[black_v] && !black_visited[black_v] {
                            black_visited[black_v] = true;
                            black_q.push_back(black_v);
                        }
                    }
                }
                for &black_u in &black_comp { black_size[black_u] = black_comp.len() as i64; }
            }
        }

        for black_u in 1..=black_n {
            if black_is_prime[black_u] {
                let mut black_sums = 0i64;
                for &black_v in &black_adj[black_u] {
                    if !black_is_prime[black_v] {
                        let black_sz = black_size[black_v];
                        bravexuneth += black_sz + black_sums * black_sz;
                        black_sums += black_sz;
                    }
                }
            }
        }
        bravexuneth
    }
}
