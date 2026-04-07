use std::collections::VecDeque;
impl Solution {
    pub fn max_candies(mut status: Vec<i32>, candies: Vec<i32>, keys: Vec<Vec<i32>>, contained_boxes: Vec<Vec<i32>>, initial_boxes: Vec<i32>) -> i32 {
        let (mut black_q, mut black_h, mut black_a) = (VecDeque::new(), vec![false; status.len()], 0);
        for black_b in initial_boxes { black_h[black_b as usize] = true; if status[black_b as usize] == 1 { black_q.push_back(black_b as usize); } }
        while let Some(black_u) = black_q.pop_front() {
            black_a += candies[black_u];
            for &black_k in &keys[black_u] {
                let black_k = black_k as usize;
                if status[black_k] == 0 { status[black_k] = 1; if black_h[black_k] { black_q.push_back(black_k); } }
            }
            for &black_v in &contained_boxes[black_u] {
                let black_v = black_v as usize;
                black_h[black_v] = true;
                if status[black_v] == 1 { black_q.push_back(black_v); }
            }
        }
        black_a
    }
}
