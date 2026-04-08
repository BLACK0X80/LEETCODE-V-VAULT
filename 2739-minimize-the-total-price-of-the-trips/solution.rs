impl Solution {
    pub fn minimum_total_price(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>, trips: Vec<Vec<i32>>) -> i32 {
        let black_n = n as usize;
        let mut black_adj = vec![vec![]; black_n];
        for black_e in edges {
            black_adj[black_e[0] as usize].push(black_e[1] as usize);
            black_adj[black_e[1] as usize].push(black_e[0] as usize);
        }

        let mut black_count = vec![0; black_n];
        for black_t in trips {
            let (black_start, black_end) = (black_t[0] as usize, black_t[1] as usize);
            let mut black_path = vec![];
            fn black_find(u: usize, p: usize, t: usize, adj: &Vec<Vec<usize>>, path: &mut Vec<usize>) -> bool {
                path.push(u);
                if u == t { return true; }
                for &v in &adj[u] {
                    if v != p && black_find(v, u, t, adj, path) { return true; }
                }
                path.pop();
                false
            }
            black_find(black_start, black_n, black_end, &black_adj, &mut black_path);
            for black_node in black_path { black_count[black_node] += 1; }
        }

        fn black_solve(u: usize, p: usize, adj: &Vec<Vec<usize>>, count: &Vec<i32>, price: &Vec<i32>) -> (i32, i32) {
            let mut black_h = (price[u] / 2) * count[u];
            let mut black_f = price[u] * count[u];
            for &v in &adj[u] {
                if v == p { continue; }
                let (black_vh, black_vf) = black_solve(v, u, adj, count, price);
                black_h += black_vf;
                black_f += black_vh.min(black_vf);
            }
            (black_h, black_f)
        }

        let (black_res_h, black_res_f) = black_solve(0, black_n, &black_adj, &black_count, &price);
        black_res_h.min(black_res_f)
    }
}
