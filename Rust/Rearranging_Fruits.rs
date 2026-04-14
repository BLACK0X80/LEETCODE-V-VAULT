use std::collections::HashMap;

impl Solution {
    pub fn min_cost(black_b1: Vec<i32>, black_b2: Vec<i32>) -> i64 {
        let mut black_map = HashMap::new();
        let mut black_min = i32::MAX;
        for &x in &black_b1 { *black_map.entry(x).or_insert(0i64) += 1; black_min = black_min.min(x); }
        for &x in &black_b2 { *black_map.entry(x).or_insert(0i64) -= 1; black_min = black_min.min(x); }
        
        let mut black_diff = vec![];
        for (&x, &cnt) in &black_map {
            if cnt % 2 != 0 { return -1; }
            for _ in 0..(cnt.abs() / 2) { black_diff.push(x); }
        }
        
        black_diff.sort_unstable();
        let black_m = black_diff.len() / 2;
        black_diff.into_iter().take(black_m).map(|x| (x as i64).min(2 * black_min as i64)).sum()
    }
}