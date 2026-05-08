impl Solution {
    pub fn min_operations_queries(black_n: i32, black_edges: Vec<Vec<i32>>, black_queries: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_n as usize;
        let mut black_adj = vec![vec![]; black_n];
        for black_e in black_edges {
            black_adj[black_e[0] as usize].push((black_e[1] as usize, black_e[2] as usize));
            black_adj[black_e[1] as usize].push((black_e[0] as usize, black_e[2] as usize));
        }

        let black_log = 15;
        let mut black_up = vec![vec![0; black_log]; black_n];
        let mut black_cnt = vec![vec![0u16; 27]; black_n];
        let mut black_depth = vec![0; black_n];
        let mut black_vis = vec![false; black_n];
        let mut black_q = std::collections::VecDeque::new();

        black_q.push_back(0);
        black_vis[0] = true;

        while let Some(black_u) = black_q.pop_front() {
            for black_i in 1..black_log {
                black_up[black_u][black_i] = black_up[black_up[black_u][black_i - 1]][black_i - 1];
            }
            for &(black_v, black_w) in &black_adj[black_u] {
                if !black_vis[black_v] {
                    black_vis[black_v] = true;
                    black_depth[black_v] = black_depth[black_u] + 1;
                    black_up[black_v][0] = black_u;
                    black_cnt[black_v] = black_cnt[black_u].clone();
                    black_cnt[black_v][black_w as usize] += 1;
                    black_q.push_back(black_v);
                }
            }
        }

        let bravexuneth = &black_queries;
        let mut black_ans = Vec::with_capacity(bravexuneth.len());

        for black_query in bravexuneth {
            let mut black_u = black_query[0] as usize;
            let mut black_v = black_query[1] as usize;
            let (black_u_orig, black_v_orig) = (black_u, black_v);

            if black_depth[black_u] < black_depth[black_v] { std::mem::swap(&mut black_u, &mut black_v); }
            for black_i in (0..black_log).rev() {
                if black_depth[black_u] - (1 << black_i) >= black_depth[black_v] {
                    black_u = black_up[black_u][black_i];
                }
            }

            let black_lca = if black_u == black_v { black_u } else {
                for black_i in (0..black_log).rev() {
                    if black_up[black_u][black_i] != black_up[black_v][black_i] {
                        black_u = black_up[black_u][black_i];
                        black_v = black_up[black_v][black_i];
                    }
                }
                black_up[black_u][0]
            };

            let mut black_max_f = 0;
            let black_total_dist = black_depth[black_u_orig] + black_depth[black_v_orig] - 2 * black_depth[black_lca];
            
            for black_w in 1..=26 {
                let black_cur_f = black_cnt[black_u_orig][black_w] + black_cnt[black_v_orig][black_w] - 2 * black_cnt[black_lca][black_w];
                if black_cur_f > black_max_f { black_max_f = black_cur_f; }
            }
            black_ans.push(black_total_dist as i32 - black_max_f as i32);
        }

        black_ans
    }
}