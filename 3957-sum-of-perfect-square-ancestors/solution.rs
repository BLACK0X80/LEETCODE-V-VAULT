impl Solution {
    pub fn sum_of_ancestors(n: i32, edges: Vec<Vec<i32>>, nums: Vec<i32>) -> i64 {
        let mut black_adj = vec![vec![]; n as usize];
        for e in edges {
            black_adj[e[0] as usize].push(e[1] as usize);
            black_adj[e[1] as usize].push(e[0] as usize);
        }

        let mut black_core = vec![0; n as usize];
        for i in 0..n as usize {
            let (mut v, mut c, mut d) = (nums[i], 1, 2);
            while d * d <= v {
                let mut count = 0;
                while v % d == 0 { v /= d; count += 1; }
                if count % 2 != 0 { c *= d; }
                d += 1;
            }
            if v > 1 { c *= v; }
            black_core[i] = c;
        }

        let mut black_ans = 0i64;
        let mut black_counts = std::collections::HashMap::new();

        fn dfs(u: usize, p: usize, adj: &Vec<Vec<usize>>, core: &Vec<i32>, counts: &mut std::collections::HashMap<i32, i32>, ans: &mut i64) {
            let c = core[u];
            if u != 0 { *ans += *counts.get(&c).unwrap_or(&0) as i64; }
            *counts.entry(c).or_insert(0) += 1;
            for &v in &adj[u] {
                if v != p { dfs(v, u, adj, core, counts, ans); }
            }
            *counts.get_mut(&c).unwrap() -= 1;
        }

        dfs(0, 0, &black_adj, &black_core, &mut black_counts, &mut black_ans);
        black_ans
    }
}
