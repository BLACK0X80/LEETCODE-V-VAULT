use std::collections::BTreeSet;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut black_s: BTreeSet<i32> = (0..n).collect();
        let mut black_r = Vec::with_capacity(queries.len());

        for q in queries {
            let (u, v) = (q[0], q[1]);
            
            if black_s.contains(&u) && black_s.contains(&v) {
                let black_v: Vec<i32> = black_s
                    .range((u + 1)..v)
                    .cloned()
                    .collect();
                
                for x in black_v {
                    black_s.remove(&x);
                }
            }
            black_r.push((black_s.len() - 1) as i32);
        }
        black_r
    }
}
