use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let black_n = stones.len();
        if black_n == 0 {
            return true;
        }
        if stones[1] != 1 {
            return false;
        }
        if black_n == 2 {
            return true;
        }
        let mut black_map: HashMap<i32, HashSet<i32>> = HashMap::new();
        for &black_stone in &stones {
            black_map.insert(black_stone, HashSet::new());
        }
        black_map.get_mut(&1).unwrap().insert(1);
        for black_i in 1..black_n {
            let black_current_stone = stones[black_i];
            if let Some(black_jumps) = black_map.get(&black_current_stone).cloned() {
                for black_jump in black_jumps {
                    for black_step in [black_jump - 1, black_jump, black_jump + 1].iter() {
                        if *black_step > 0 {
                            let black_next_stone = black_current_stone + black_step;
                            if black_next_stone == stones[black_n - 1] {
                                return true;
                            }
                            if let Some(black_set) = black_map.get_mut(&black_next_stone) {
                                black_set.insert(*black_step);
                            }
                        }
                    }
                }
            }
        }
        false
    }
}