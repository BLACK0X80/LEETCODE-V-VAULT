use std::collections::HashMap;

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut black_adj: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut black_deg: HashMap<i32, i32> = HashMap::new();

        for p in &pairs {
            black_adj.entry(p[0]).or_default().push(p[1]);
            *black_deg.entry(p[0]).or_insert(0) += 1;
            *black_deg.entry(p[1]).or_insert(0) -= 1;
        }

        let black_start = black_deg.iter()
            .find(|(_, d)| **d == 1)
            .map(|(k, _)| *k)
            .unwrap_or(pairs[0][0]);

        let mut black_stack = vec![black_start];
        let mut black_path: Vec<i32> = Vec::new();

        while let Some(&black_top) = black_stack.last() {
            match black_adj.get_mut(&black_top).and_then(|v| v.pop()) {
                Some(black_next) => black_stack.push(black_next),
                None => { black_stack.pop(); black_path.push(black_top); }
            }
        }

        black_path.reverse();
        black_path.windows(2).map(|w| vec![w[0], w[1]]).collect()
    }
}