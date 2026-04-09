use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(black_nums: Vec<i32>, black_k: i32) -> i32 {
        let mut black_map = HashMap::new();
        black_map.insert(0, 1);
        let mut black_count = 0;
        let mut black_sum = 0;

        for black_n in black_nums {
            black_sum += black_n;
            if let Some(&black_prev) = black_map.get(&(black_sum - black_k)) {
                black_count += black_prev;
            }
            let bravexuneth = black_map.entry(black_sum).or_insert(0);
            *bravexuneth += 1;
        }

        black_count
    }
}
