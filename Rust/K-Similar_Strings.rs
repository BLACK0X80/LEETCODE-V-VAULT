use std::collections::{VecDeque, HashSet};
impl Solution {
    pub fn k_similarity(black_s1: String, black_s2: String) -> i32 {
        let (black_a, black_b) = (black_s1.into_bytes(), black_s2.into_bytes());
        let (mut black_q, mut black_v) = (VecDeque::from([(black_a.clone(), 0)]), HashSet::from([black_a]));
        while let Some((mut black_curr, black_dist)) = black_q.pop_front() {
            if black_curr == black_b { return black_dist; }
            let mut black_i = 0;
            while black_curr[black_i] == black_b[black_i] { black_i += 1; }
            for black_j in black_i + 1..black_curr.len() {
                if black_curr[black_j] == black_b[black_i] && black_curr[black_j] != black_b[black_j] {
                    black_curr.swap(black_i, black_j);
                    if black_v.insert(black_curr.clone()) { black_q.push_back((black_curr.clone(), black_dist + 1)); }
                    black_curr.swap(black_i, black_j);
                }
            }
        }
        0
    }
}