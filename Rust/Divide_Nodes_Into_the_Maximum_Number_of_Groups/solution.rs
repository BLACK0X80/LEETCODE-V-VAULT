use std::collections::VecDeque;

impl Solution {
    pub fn magnificent_sets(black_n: i32, black_edges: Vec<Vec<i32>>) -> i32 {
        let black_n = black_n as usize;
        let mut black_adj = vec![vec![]; black_n + 1];
        for black_e in black_edges {
            black_adj[black_e[0] as usize].push(black_e[1] as usize);
            black_adj[black_e[1] as usize].push(black_e[0] as usize);
        }

        let mut black_vis = vec![false; black_n + 1];
        let mut black_res = 0;

        for black_i in 1..=black_n {
            if !black_vis[black_i] {
                let mut black_comp = vec![];
                let mut black_q = VecDeque::from([black_i]);
                black_vis[black_i] = true;
                while let Some(black_u) = black_q.pop_front() {
                    black_comp.push(black_u);
                    for &black_v in &black_adj[black_u] {
                        if !black_vis[black_v] {
                            black_vis[black_v] = true;
                            black_q.push_back(black_v);
                        }
                    }
                }

                let mut black_max = -1;
                for &black_root in &black_comp {
                    let mut black_d = vec![-1i32; black_n + 1];
                    let mut black_bq = VecDeque::from([black_root]);
                    black_d[black_root] = 1;
                    let mut black_cur = 1;
                    while let Some(black_u) = black_bq.pop_front() {
                        for &black_v in &black_adj[black_u] {
                            if black_d[black_v] == -1 {
                                black_d[black_v] = black_d[black_u] + 1;
                                black_cur = black_cur.max(black_d[black_v]);
                                black_bq.push_back(black_v);
                            } else if (black_d[black_v] - black_d[black_u]).abs() != 1 {
                                return -1;
                            }
                        }
                    }
                    black_max = black_max.max(black_cur);
                }
                black_res += black_max;
            }
        }
        black_res
    }
}