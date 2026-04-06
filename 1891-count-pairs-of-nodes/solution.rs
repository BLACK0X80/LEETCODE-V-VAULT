use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let n = n as usize;
        let mut deg = vec![0i32; n + 1];
        let mut shared: HashMap<(usize, usize), i32> = HashMap::new();

        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            deg[u] += 1; deg[v] += 1;
            let key = (u.min(v), u.max(v));
            *shared.entry(key).or_insert(0) += 1;
        }

        let mut sorted_deg = deg[1..=n].to_vec();
        sorted_deg.sort();

        queries.iter().map(|&q| {
            let mut cnt = 0i32;
            let (mut l, mut r) = (0, n - 1);
            while l < r {
                if sorted_deg[l] + sorted_deg[r] > q {
                    cnt += (r - l) as i32;
                    r -= 1;
                } else { l += 1; }
            }
            for (&(u, v), &s) in &shared {
                if deg[u] + deg[v] > q && deg[u] + deg[v] - s <= q {
                    cnt -= 1;
                }
            }
            cnt
        }).collect()
    }
}
