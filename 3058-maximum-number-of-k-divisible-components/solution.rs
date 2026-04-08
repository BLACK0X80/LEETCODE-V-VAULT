impl Solution {
    pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
        let black_n = n as usize;
        let mut black_adj = vec![vec![]; black_n];
        for black_e in edges {
            black_adj[black_e[0] as usize].push(black_e[1] as usize);
            black_adj[black_e[1] as usize].push(black_e[0] as usize);
        }

        let mut black_res = 0;
        let black_k = k as i64;
        fn black_dfs(u: usize, p: usize, adj: &Vec<Vec<usize>>, vals: &Vec<i32>, k: i64, res: &mut i32) -> i64 {
            let mut black_sum = vals[u] as i64;
            for &v in &adj[u] {
                if v != p { black_sum += black_dfs(v, u, adj, vals, k, res); }
            }
            if black_sum % k == 0 {
                *res += 1;
                0
            } else {
                black_sum
            }
        }
        black_dfs(0, black_n, &black_adj, &values, black_k, &mut black_res);
        black_res
    }
}
