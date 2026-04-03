use std::collections::HashMap;

impl Solution {
    pub fn maximum_points(edges: Vec<Vec<i32>>, coins: Vec<i32>, k: i32) -> i32 {
        let n = coins.len();
        let mut adj = vec![vec![]; n];
        for e in &edges {
            adj[e[0] as usize].push(e[1] as usize);
            adj[e[1] as usize].push(e[0] as usize);
        }

        let mut memo = vec![vec![-1i32; 14]; n];

        fn dfs(u: usize, par: usize, halves: usize, adj: &Vec<Vec<usize>>,
               coins: &Vec<i32>, k: i32, memo: &mut Vec<Vec<i32>>) -> i32 {
            if halves >= 14 { return 0; }
            if memo[u][halves] != -1 { return memo[u][halves]; }
            let c = coins[u] >> halves;
            let mut opt1 = c - k;
            let mut opt2 = c >> 1;
            for &v in &adj[u] {
                if v == par {continue;}
                opt1 += dfs(v, u, halves, adj, coins, k, memo);
                opt2 += dfs(v, u, halves + 1, adj, coins, k, memo);
            }
            memo[u][halves] = opt1.max(opt2);
            memo[u][halves]
        }

        dfs(0, n, 0, &adj, &coins, k, &mut memo)
    }
}
