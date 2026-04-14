use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let black_n = arr.len();
        if black_n <= 1 { return 0; }
        let mut black_m: HashMap<i32, Vec<usize>> = HashMap::new();
        for (black_i, &black_v) in arr.iter().enumerate() { black_m.entry(black_v).or_default().push(black_i); }
        let (mut black_q, mut black_v, mut black_s) = (VecDeque::from([0]), vec![false; black_n], 0);
        black_v[0] = true;
        while !black_q.is_empty() {
            for _ in 0..black_q.len() {
                let black_i = black_q.pop_front().unwrap();
                if black_i == black_n - 1 { return black_s; }
                let mut black_nx = black_m.remove(&arr[black_i]).unwrap_or_default();
                if black_i > 0 { black_nx.push(black_i - 1); }
                black_nx.push(black_i + 1);
                for black_j in black_nx {
                    if black_j < black_n && !black_v[black_j] {
                        black_v[black_j] = true;
                        black_q.push_back(black_j);
                    }
                }
            }
            black_s += 1;
        }
        0
    }
}