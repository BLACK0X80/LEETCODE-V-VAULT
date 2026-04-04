use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut black: BinaryHeap<i64> = target.iter().map(|&x| x as i64).collect();
        let mut total: i64 = target.iter().map(|&x| x as i64).sum();
        while let Some(&top) = black.peek() {
            if top == 1 { return true; }
            let rest = total - top;
            if rest == 0 { return false; }
            let prev = top % rest;
            let prev = if prev == 0 { rest } else { prev };
            if prev >= top { return false; }
            total = total - top + prev;
            black.pop();
            black.push(prev);
        }
        false
    }
}
