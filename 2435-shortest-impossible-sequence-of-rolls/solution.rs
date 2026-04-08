use std::collections::HashSet;
impl Solution {
    pub fn shortest_sequence(black_rolls: Vec<i32>, black_k: i32) -> i32 {
        let (mut black_ans, mut black_set) = (1, HashSet::new());
        for black_x in black_rolls {
            black_set.insert(black_x);
            if black_set.len() == black_k as usize {
                black_ans += 1;
                black_set.clear();
            }
        }
        black_ans
    }
}
