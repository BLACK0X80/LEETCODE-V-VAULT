use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let neighbors = [
            vec![1, 3],
            vec![0, 2, 4],
            vec![1, 5],
            vec![0, 4],
            vec![1, 3, 5],
            vec![2, 4],
        ];

        let mut start = String::new();
        for row in &board {
            for &n in row {
                start.push((b'0' + n as u8) as char);
            }
        }

        let target = "123450";
        if start == target { return 0; }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((start.clone(), 0));
        visited.insert(start);

        while let Some((state, steps)) = queue.pop_front() {
            let zero = state.find('0').unwrap();
            let chars: Vec<char> = state.chars().collect();

            for &nb in &neighbors[zero] {
                let mut next = chars.clone();
                next.swap(zero, nb);
                let ns: String = next.iter().collect();
                if ns == target { return steps + 1; }
                if visited.insert(ns.clone()) {
                    queue.push_back((ns, steps + 1));
                }
            }
        }

        -1
    }
}