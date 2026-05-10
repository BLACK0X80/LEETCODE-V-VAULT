use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut black: BinaryHeap<i32> = stones.into_iter().collect();
        while black.len() > 1 {
            let y = black.pop().unwrap();
            let x = black.pop().unwrap();
            if y != x { black.push(y - x); }
        }
        black.pop().unwrap_or(0)
    }
}